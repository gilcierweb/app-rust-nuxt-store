// Debounce para evitar rate limiting em múltiplas requisições
let fetchQueue = new Map<string, Promise<any>>()
let debounceTimers = new Map<string, NodeJS.Timeout>()

export function useDebouncedFetch<T>(url: string, options: any = {}, debounceMs = 100) {
  const cacheKey = `${url}-${JSON.stringify(options)}`
  
  // Se já tem uma promise em andamento, retornar ela
  if (fetchQueue.has(cacheKey)) {
    return fetchQueue.get(cacheKey)
  }
  
  // Limpar timer existente
  if (debounceTimers.has(cacheKey)) {
    clearTimeout(debounceTimers.get(cacheKey)!)
  }
  
  // Criar nova promise com debounce
  const promise = new Promise<T>((resolve, reject) => {
    const timer = setTimeout(async () => {
      try {
        const result = await $fetch<T>(url, options)
        resolve(result as T)
        fetchQueue.delete(cacheKey)
        debounceTimers.delete(cacheKey)
      } catch (error) {
        reject(error)
        fetchQueue.delete(cacheKey)
        debounceTimers.delete(cacheKey)
      }
    }, debounceMs)
    
    debounceTimers.set(cacheKey, timer)
  })
  
  fetchQueue.set(cacheKey, promise)
  return promise
}
