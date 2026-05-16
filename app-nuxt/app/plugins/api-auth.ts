import { ofetch } from 'ofetch'

function isApiRequest(request: string, baseURL: string): boolean {
  if (request.startsWith('/api/')) return true
  if (request.startsWith(`${baseURL}/api/`)) return true
  return false
}

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()
  const baseURL = config.public.baseURL

  const wrappedFetch = $fetch.create({
    onRequest({ request, options }) {
      const requestUrl = typeof request === 'string' ? request : request.toString()
      
      // Ensure credentials are included for HttpOnly cookies
      options.credentials = 'include'

      // Forward cookies from client to API during SSR
      if (import.meta.server && isApiRequest(requestUrl, baseURL)) {
        options.headers = {
          ...options.headers,
          ...useRequestHeaders(['cookie'])
        }
      }
    }
  })

  return {
    provide: {
      api: wrappedFetch
    }
  }
})
