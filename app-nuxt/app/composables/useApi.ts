import { toValue, computed } from 'vue'
import type { MaybeRefOrGetter } from 'vue'

function normalizePath(baseURL: string, path: string): string {
  if (!path) return ''
  if (path.startsWith('http://') || path.startsWith('https://')) return path
  return `${baseURL}${path.startsWith('/') ? path : `/${path}`}`
}

export function useApi() {
  const config = useRuntimeConfig()

  const apiFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const { $api } = useNuxtApp()
    const url = normalizePath(config.public.baseURL, toValue(path))
    return ($api as typeof $fetch)<T>(url, options)
  }

  const useApiFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const resolvedPath = computed(() => normalizePath(config.public.baseURL, toValue(path)))
    const nuxtApp = useNuxtApp()
    return useFetch<T>(resolvedPath, {
      ...options,
      $fetch: nuxtApp.$api as typeof $fetch,
      getCachedData(key) {
        if (options.getCachedData) return options.getCachedData(key)
        return nuxtApp.payload.data[key] || nuxtApp.static?.data[key]
      }
    })
  }

  const useApiLazyFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const resolvedPath = computed(() => normalizePath(config.public.baseURL, toValue(path)))
    const nuxtApp = useNuxtApp()
    return useLazyFetch<T>(resolvedPath, {
      ...options,
      $fetch: nuxtApp.$api as typeof $fetch,
      getCachedData(key) {
        if (options.getCachedData) return options.getCachedData(key)
        return nuxtApp.payload.data[key] || nuxtApp.static?.data[key]
      }
    })
  }

  return {
    apiFetch,
    useApiFetch,
    useApiLazyFetch
  }
}
