import { proxyRequest } from 'h3'
import { resolveBackendBaseUrl } from '../../utils/backend-url'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig(event)
  const backend = resolveBackendBaseUrl(event)

  if (!backend.ok) {
    setResponseHeader(event, 'x-api-proxy-error-source', 'nuxt-runtime-config')
    setResponseStatus(event, backend.statusCode, 'Backend URL not configured')

    return {
      error: 'backend_url_not_configured',
      description: backend.message,
      current: backend.current,
      source: backend.source
    }
  }
  
  // Extrai o caminho após o /uploads/
  const path = event.context.params?.path || ''
  const requestUrl = getRequestURL(event)
  const target = `${backend.url}/uploads/${path}${requestUrl.search}`

  const apiKey = (config.apiRustApiKey || '').trim()
  return proxyRequest(event, target, apiKey ? { headers: { 'x-api-key': apiKey } } : undefined)
})
