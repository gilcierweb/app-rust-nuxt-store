import { proxyRequest } from 'h3'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  
  // A URL base do Rust. Pega da config de runtime que é populada pelo .env
  const backendUrl = config.public.ApiRustBaseUrl || 'http://localhost:5150'
  
  // Extrai o caminho após o /api/
  // event.context.params.path contém o caminho (o catch-all [...path])
  const path = event.context.params?.path || ''
  const target = `${backendUrl}/api/${path}`
  
  // Realiza o proxy transparente
  return proxyRequest(event, target, {
    fetchOptions: {
      headers: {
        // Preservar IP do cliente (opcional, bom para logs no backend)
        'x-forwarded-for': getRequestHeader(event, 'x-forwarded-for') || getRequestIP(event, { xForwardedFor: true }) || '',
        'x-forwarded-host': getRequestHost(event) || '',
        'x-forwarded-proto': getRequestProtocol(event) || 'http',
      }
    }
  })
})
