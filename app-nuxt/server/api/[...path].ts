import { resolveBackendBaseUrl } from '../utils/backend-url'

const BACKEND_CSRF_HEADER = 'x-backend-csrf-token'
const BACKEND_CSRF_ENDPOINT = '/api/auth/csrf'
const BACKEND_CSRF_COOKIE = 'backend_csrf'
const SKIPPED_RESPONSE_HEADERS = new Set([
  'content-length',
  'content-encoding',
  'transfer-encoding',
  'access-control-allow-origin',
  'access-control-allow-credentials',
  'access-control-allow-headers',
  'access-control-allow-methods'
])

function isProtectedMethod(method: string): boolean {
  return ['POST', 'PUT', 'PATCH', 'DELETE'].includes(method.toUpperCase())
}

function splitSetCookieHeader(value: string | null): string[] {
  if (!value) return []
  return value
    .split(/,(?=\s*[^;,\s]+=)/)
    .map(cookie => cookie.trim())
    .filter(Boolean)
}

function buildBackendCsrfCookie(token: string, secure: boolean): string {
  const attrs = [
    `${BACKEND_CSRF_COOKIE}=${encodeURIComponent(token)}`,
    'Path=/',
    'HttpOnly',
    'SameSite=Lax'
  ]
  if (secure) {
    attrs.push('Secure')
  }
  return attrs.join('; ')
}

function mergeCookieHeader(existingCookieHeader: string | null | undefined, setCookies: string[]): string | undefined {
  const jar = new Map<string, string>()

  for (const entry of (existingCookieHeader || '').split(';')) {
    const trimmed = entry.trim()
    if (!trimmed) continue
    const separatorIndex = trimmed.indexOf('=')
    if (separatorIndex <= 0) continue
    jar.set(trimmed.slice(0, separatorIndex), trimmed.slice(separatorIndex + 1))
  }

  for (const setCookie of setCookies) {
    const cookiePair = setCookie.split(';', 1)[0]?.trim()
    if (!cookiePair) continue
    const separatorIndex = cookiePair.indexOf('=')
    if (separatorIndex <= 0) continue
    jar.set(cookiePair.slice(0, separatorIndex), cookiePair.slice(separatorIndex + 1))
  }

  if (jar.size === 0) return undefined
  return Array.from(jar.entries())
    .map(([name, value]) => `${name}=${value}`)
    .join('; ')
}

async function ensureBackendCsrfToken(event: any, backendUrl: string, incomingCookieHeader?: string) {
  const response = await $fetch.raw<{ token?: string }>(`${backendUrl}${BACKEND_CSRF_ENDPOINT}`, {
    method: 'GET',
    headers: incomingCookieHeader ? { cookie: incomingCookieHeader, accept: 'application/json' } : { accept: 'application/json' }
  })

  const token = response._data?.token
  if (!token) {
    throw createError({ statusCode: 500, statusMessage: 'Backend CSRF bootstrap failed' })
  }

  const setCookies = splitSetCookieHeader(response.headers.get('set-cookie'))
  const hasBackendCsrfCookie = setCookies.some(cookie =>
    cookie.toLowerCase().startsWith(`${BACKEND_CSRF_COOKIE}=`)
  )

  if (!hasBackendCsrfCookie) {
    const isSecure = getRequestURL(event).protocol === 'https:'
    setCookies.push(buildBackendCsrfCookie(token, isSecure))
  }

  const cookieHeader = mergeCookieHeader(incomingCookieHeader, setCookies)

  for (const cookie of setCookies) {
    appendResponseHeader(event, 'set-cookie', cookie)
  }

  return { token, cookieHeader }
}

function forwardResponseHeaders(event: any, responseHeaders: Headers) {
  for (const [key, value] of responseHeaders.entries()) {
    const headerName = key.toLowerCase()

    if (headerName === 'set-cookie') {
      const cookies = splitSetCookieHeader(String(value))
      for (const cookie of cookies) {
        appendResponseHeader(event, 'set-cookie', cookie)
      }
      continue
    }

    if (!SKIPPED_RESPONSE_HEADERS.has(headerName)) {
      setResponseHeader(event, key, value)
    }
  }
}

function backendErrorPayload(error: any, phase: string, statusText?: string) {
  const data = error?.data ?? error?.response?._data
  if (data && typeof data === 'object') return data

  return {
    error: 'backend_request_failed',
    phase,
    description: typeof data === 'string' ? data : statusText || error?.message || 'Backend request failed'
  }
}

export default defineEventHandler(async (event) => {
  const backend = resolveBackendBaseUrl(event)

  if (!backend.ok) {
    setResponseHeader(event, 'x-api-proxy-error-source', 'nuxt-runtime-config')
    setResponseStatus(event, backend.statusCode, 'Backend URL not configured')

    return {
      error: 'backend_url_not_configured',
      description: backend.message,
      current: backend.current,
      source: backend.source
    }
  }

  const backendUrl = backend.url
  
  const path = event.context.params?.path || ''
  const requestUrl = getRequestURL(event)
  const targetUrl = `${backendUrl}/api/${path}${requestUrl.search}`
  let phase = 'backend-request'
  
  // Forward essential headers
  const incomingHeaders = getRequestHeaders(event)
  const headers = new Headers()
  const allowedHeaders = ['authorization', 'accept', 'content-type', 'cookie', 'user-agent']
  
  for (const headerName of allowedHeaders) {
    const value = incomingHeaders[headerName]
    if (value) {
      headers.set(headerName, Array.isArray(value) ? value.join(headerName === 'cookie' ? '; ' : ', ') : value)
    }
  }

  const method = event.method || 'GET'
  const body = ['GET', 'HEAD'].includes(method) ? undefined : await readRawBody(event, false)

  try {
    if (isProtectedMethod(method) && requestUrl.pathname !== BACKEND_CSRF_ENDPOINT) {
      phase = 'backend-csrf-bootstrap'
      const { token, cookieHeader } = await ensureBackendCsrfToken(event, backendUrl, headers.get('cookie') || undefined)
      headers.set(BACKEND_CSRF_HEADER, token)
      if (cookieHeader) {
        headers.set('cookie', cookieHeader)
      }
    }

    phase = 'backend-request'
    const response = await $fetch.raw(targetUrl, {
      method,
      headers,
      body,
      redirect: 'manual'
    })

    // Forward status and content-type
    setResponseStatus(event, response.status, response.statusText)
    forwardResponseHeaders(event, response.headers)

    return response._data
  } catch (error: any) {
    const response = error?.response
    if (!response) {
      setResponseHeader(event, 'x-api-proxy-error-source', phase)
      setResponseStatus(event, 502, 'Bad Gateway')

      return {
        error: 'backend_unreachable',
        phase,
        description: error?.message || 'Backend request failed'
      }
    }

    forwardResponseHeaders(event, response.headers)
    setResponseHeader(event, 'x-api-proxy-error-source', phase)
    setResponseStatus(event, response.status, response.statusText)
    return backendErrorPayload(error, phase, response.statusText)
  }
})
