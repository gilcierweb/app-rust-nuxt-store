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

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()
  const baseURL = config.public.baseURL
  
  // Get CSRF and headers synchronously inside the plugin setup context
  let ssrCsrfToken: string | null = null
  let ssrCsrfHeaderName: string | null = null
  
  try {
    const csrfObj = useCsrf()
    if (csrfObj && csrfObj.csrf && csrfObj.headerName) {
      ssrCsrfToken = csrfObj.csrf
      ssrCsrfHeaderName = csrfObj.headerName
    }
  } catch (e) {
    // Ignore if useCsrf is not available
  }
  
  const cookieHeader = import.meta.server ? useRequestHeaders(['cookie']) : {}

  async function resolveNuxtCsrfHeader() {
    if (cachedCsrfToken) {
      return cachedCsrfToken
    }

    if (import.meta.client) {
      const response = await $fetch<NuxtCsrfTokenResponse>(NUXT_CSRF_ENDPOINT, {
        method: 'GET',
        credentials: 'include',
        headers: {
          accept: 'application/json'
        }
      })
      const headerName = response.headerName || config.public.csurf?.headerName || 'x-nuxt-csrf-token'
      if (response.token && headerName) {
        cachedCsrfToken = { headerName, token: response.token }
        return cachedCsrfToken
      }
    }

    if (ssrCsrfToken && ssrCsrfHeaderName) {
      // Do not cache globally on SSR to avoid leaking across requests
      return { headerName: ssrCsrfHeaderName, token: ssrCsrfToken }
    }

    return null
  }

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
