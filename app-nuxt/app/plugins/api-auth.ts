import { ofetch } from 'ofetch'

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

  const wrappedFetch = $fetch.create({
    onRequest({ request, options }) {
      const requestUrl = typeof request === 'string' ? request : request.toString()
      const headers = new Headers(options.headers || {})
      
      // Ensure credentials are included for HttpOnly cookies
      options.credentials = 'include'

      if (isApiRequest(requestUrl, baseURL) && isProtectedMethod(options.method)) {
        const { csrf, headerName } = useCsrf()
        if (csrf && headerName) {
          headers.set(headerName, csrf)
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
    }
  })

  return {
    provide: {
      api: wrappedFetch
    }
  }
})
