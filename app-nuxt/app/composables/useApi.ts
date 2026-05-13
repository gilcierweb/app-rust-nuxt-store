type ApiHeaders = Record<string, string>

function normalizePath(baseURL: string, path: string): string {
  if (path.startsWith('http://') || path.startsWith('https://')) return path
  return `${baseURL}${path.startsWith('/') ? path : `/${path}`}`
}

export function useApi() {
  const config = useRuntimeConfig()
  const tokenCookie = useCookie<string | null>('auth_token', { default: () => null })

  const withAuthHeaders = (headers: ApiHeaders = {}): ApiHeaders => {
    if (!tokenCookie.value) return headers
    return {
      ...headers,
      Authorization: `Bearer ${tokenCookie.value}`
    }
  }

  const apiFetch = <T>(path: string, options: any = {}) => {
    const url = normalizePath(config.public.baseURL, path)
    return $fetch<T>(url, {
      ...options,
      headers: withAuthHeaders((options.headers || {}) as ApiHeaders)
    })
  }

  const useApiFetch = <T>(path: string, options: any = {}) => {
    const url = normalizePath(config.public.baseURL, path)
    return useFetch<T>(url, {
      ...options,
      headers: withAuthHeaders((options.headers || {}) as ApiHeaders)
    })
  }

  const useApiLazyFetch = <T>(path: string, options: any = {}) => {
    const url = normalizePath(config.public.baseURL, path)
    return useLazyFetch<T>(url, {
      ...options,
      headers: withAuthHeaders((options.headers || {}) as ApiHeaders)
    })
  }

  return {
    apiFetch,
    useApiFetch,
    useApiLazyFetch,
    withAuthHeaders
  }
}
