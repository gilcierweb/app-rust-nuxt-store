import { toValue, computed } from 'vue'
import type { MaybeRefOrGetter } from 'vue'

function assertNoCrossOriginAbsoluteUrl(path: string): void {
  if (!import.meta.client) return
  if (!path.startsWith('http://') && !path.startsWith('https://')) return

  const target = new URL(path)
  if (target.origin !== window.location.origin) {
    throw createError({
      statusCode: 400,
      statusMessage: 'Cross-origin API URL blocked',
      message: `Use relative /api paths via Nuxt proxy. Received: ${target.origin}`
    })
  }
}

function normalizePath(path: string): string {
  if (!path) return ''
  assertNoCrossOriginAbsoluteUrl(path)
  if (path.startsWith('http://') || path.startsWith('https://')) return path
  return path.startsWith('/') ? path : `/${path}`
}

export function useApi() {
  const apiFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const { $api } = useNuxtApp()
    const url = normalizePath(toValue(path))
    return ($api as typeof $fetch)<T>(url, options)
  }

  const useApiFetch = <T>(path: MaybeRefOrGetter<string>, options: any = {}) => {
    const resolvedPath = computed(() => normalizePath(toValue(path)))
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
    const resolvedPath = computed(() => normalizePath(toValue(path)))
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
