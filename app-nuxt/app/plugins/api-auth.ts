type NuxtCsrfTokenResponse = {
  token?: string
  headerName?: string
}

const NUXT_CSRF_ENDPOINT = '/api/csrf-token'

let cachedCsrfToken: { token: string; headerName: string } | null = null

function isApiRequest(request: string, baseURL: string): boolean {
  if (request.startsWith('/api/')) return true
  if (request.startsWith(`${baseURL}/api/`)) return true
  return false
}

function isProtectedMethod(method?: string): boolean {
  const normalized = (method || 'GET').toUpperCase()
  return ['POST', 'PUT', 'PATCH', 'DELETE'].includes(normalized)
}

async function resolveNuxtCsrfHeader() {
  if (cachedCsrfToken) {
    return cachedCsrfToken
  }

  if (import.meta.client) {
    const config = useRuntimeConfig()
    const response = await $fetch<NuxtCsrfTokenResponse>(NUXT_CSRF_ENDPOINT, {
      method: 'GET',
      credentials: 'include',
      headers: {
        accept: 'application/json'
      }
    })
    const headerName = response.headerName || config.public.csurf?.headerName
    if (response.token && headerName) {
      cachedCsrfToken = { headerName, token: response.token }
      return cachedCsrfToken
    }
  }

  const { csrf, headerName } = useCsrf()
  if (csrf && headerName) {
    cachedCsrfToken = { headerName, token: csrf }
    return cachedCsrfToken
  }

  return null
}

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()
  const baseURL = config.public.baseURL

  const wrappedFetch = $fetch.create({
    async onRequest({ request, options }) {
      const requestUrl = typeof request === 'string' ? request : request.toString()
      const headers = new Headers(options.headers || {})
      
      // Ensure credentials are included for HttpOnly cookies
      options.credentials = 'include'

      if (isApiRequest(requestUrl, baseURL) && isProtectedMethod(options.method)) {
        const csrfHeader = await resolveNuxtCsrfHeader()
        if (csrfHeader) {
          headers.set(csrfHeader.headerName, csrfHeader.token)
        }
      }

      // Forward cookies from client to API during SSR
      if (import.meta.server && isApiRequest(requestUrl, baseURL)) {
        const cookieHeader = useRequestHeaders(['cookie'])
        if (cookieHeader.cookie) {
          headers.set('cookie', cookieHeader.cookie)
        }
      }

      options.headers = headers
    },
    onResponse({ response }) {
      // Invalidate CSRF cache on auth errors — session may have changed.
      if (response.status === 401 || response.status === 403) {
        cachedCsrfToken = null
      }
    }
  })

  return {
    provide: {
      api: wrappedFetch
    }
  }
})
