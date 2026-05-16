import { toValue, computed } from 'vue'
import type { MaybeRefOrGetter } from 'vue'

type ApiHeaders = Record<string, string>

function normalizePath(baseURL: string, path: string): string {
  if (!path) return ''
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

  const apiFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const url = normalizePath(config.public.baseURL, toValue(path))
    return $fetch<T>(url, {
      ...options,
      headers: withAuthHeaders((options.headers || {}) as ApiHeaders)
    })
  }

  const useApiFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const resolvedPath = computed(() => normalizePath(config.public.baseURL, toValue(path)))
    return useFetch<T>(resolvedPath, {
      ...options,
      headers: withAuthHeaders((options.headers || {}) as ApiHeaders)
    })
  }

  const useApiLazyFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const resolvedPath = computed(() => normalizePath(config.public.baseURL, toValue(path)))
    return useLazyFetch<T>(resolvedPath, {
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
