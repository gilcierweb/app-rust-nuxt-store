import { proxyRequest } from 'h3'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  
  const backendUrl = config.apiRustBaseUrl || 'http://localhost:5150'
  
  // Extrai o caminho após o /uploads/
  const path = event.context.params?.path || ''
  const requestUrl = getRequestURL(event)
  const target = `${backendUrl}/uploads/${path}${requestUrl.search}`
  
  return proxyRequest(event, target)
})
