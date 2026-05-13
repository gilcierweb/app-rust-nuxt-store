import { ofetch } from 'ofetch'

function isApiRequest(request: string, baseURL: string): boolean {
  if (request.startsWith('/api/')) return true
  if (request.startsWith(`${baseURL}/api/`)) return true
  return false
}

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()
  const tokenCookie = useCookie<string | null>('auth_token', { default: () => null })
  const baseURL = config.public.baseURL

  const wrappedFetch = ofetch.create({
    onRequest({ request, options }) {
      const requestUrl = typeof request === 'string' ? request : request.toString()
      const token = tokenCookie.value
      if (!token || !isApiRequest(requestUrl, baseURL)) return

      const headers = new Headers(options.headers as HeadersInit | undefined)
      if (!headers.has('Authorization')) {
        headers.set('Authorization', `Bearer ${token}`)
      }
      options.headers = headers
    }
  })

  globalThis.$fetch = wrappedFetch as typeof globalThis.$fetch
})
