const BACKEND_CSRF_HEADER = 'x-backend-csrf-token'
const BACKEND_CSRF_ENDPOINT = '/api/auth/csrf'
const BACKEND_CSRF_COOKIE = 'backend_csrf'

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

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  
  // A URL base do Rust. Pega da config de runtime que é populada pelo .env
  const backendUrl = config.apiRustBaseUrl || 'http://localhost:5150'
  
  const path = event.context.params?.path || ''
  const requestUrl = getRequestURL(event)
  const targetUrl = `${backendUrl}/api/${path}${requestUrl.search}`
  
  // Forward essential headers
  const incomingHeaders = getRequestHeaders(event)
  const headers = new Headers()
  const allowedHeaders = ['authorization', 'accept', 'content-type', 'cookie', 'user-agent', 'origin']
  
  for (const headerName of allowedHeaders) {
    const value = incomingHeaders[headerName]
    if (value) {
      headers.set(headerName, Array.isArray(value) ? value.join(headerName === 'cookie' ? '; ' : ', ') : value)
    }
  }

  const method = event.method || 'GET'
  const body = ['GET', 'HEAD'].includes(method) ? undefined : await readRawBody(event, false)

  if (isProtectedMethod(method) && requestUrl.pathname !== BACKEND_CSRF_ENDPOINT) {
    const { token, cookieHeader } = await ensureBackendCsrfToken(event, backendUrl, headers.get('cookie') || undefined)
    headers.set(BACKEND_CSRF_HEADER, token)
    if (cookieHeader) {
      headers.set('cookie', cookieHeader)
    }
  }

  try {
    const response = await $fetch.raw(targetUrl, {
      method,
      headers,
      body,
      redirect: 'manual'
    })

    // Forward status and content-type
    setResponseStatus(event, response.status, response.statusText)
    
    // Forward headers
    for (const [key, value] of response.headers.entries()) {
      if (key.toLowerCase() === 'set-cookie') {
        // split cookies properly
        const cookies = String(value).split(/,(?=\s*[^;,\s]+=)/).map(c => c.trim()).filter(Boolean)
        for (const cookie of cookies) {
          appendResponseHeader(event, 'set-cookie', cookie)
        }
      } else if (!['content-length', 'content-encoding', 'transfer-encoding'].includes(key.toLowerCase())) {
        setResponseHeader(event, key, value)
      }
    }

    return response._data
  } catch (error: any) {
    const response = error?.response
    if (!response) {
      throw createError({ statusCode: 502, statusMessage: 'Bad Gateway' })
    }
    setResponseStatus(event, response.status, response.statusText)
    return response._data || { error: { message: response.statusText } }
  }
})
