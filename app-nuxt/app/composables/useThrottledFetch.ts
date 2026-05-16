import { ref } from 'vue'

export function useThrottledFetch<T>(url: string, options: any = {}) {
  const { $api } = useNuxtApp()
  const data = ref<T | null>(null)
  const pending = ref(false)
  const error = ref<Error | null>(null)
  
  // Simple throttle: wait 100ms between requests
  let lastFetch = 0
  const throttleDelay = 100
  
  const execute = async () => {
    const now = Date.now()
    if (now - lastFetch < throttleDelay) {
      await new Promise(resolve => setTimeout(resolve, throttleDelay - (now - lastFetch)))
    }
    
    lastFetch = Date.now()
    pending.value = true
    error.value = null
    
    try {
      const response = await ($api as typeof $fetch)<T>(url, options)
      data.value = response
    } catch (err) {
      error.value = err as Error
    } finally {
      pending.value = false
    }
  }
  
  return {
    data,
    pending,
    error,
    execute
  }
}
