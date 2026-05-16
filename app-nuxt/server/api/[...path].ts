import { proxyRequest } from 'h3'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  
  // A URL base do Rust. Pega da config de runtime que é populada pelo .env
  const backendUrl = config.public.ApiRustBaseUrl || 'http://localhost:5150'
  
  const path = event.context.params?.path || ''
  const requestUrl = getRequestURL(event)
  const targetUrl = `${backendUrl}/api/${path}${requestUrl.search}`
  
  // Forward essential headers
  const incomingHeaders = getRequestHeaders(event)
  const headers = new Headers()
  const allowedHeaders = ['authorization', 'accept', 'content-type', 'cookie', 'user-agent']
  
  for (const headerName of allowedHeaders) {
    const value = incomingHeaders[headerName]
    if (value) {
      headers.set(headerName, Array.isArray(value) ? value.join(headerName === 'cookie' ? '; ' : ', ') : value)
    }
  }

  const method = event.method || 'GET'
  const body = ['GET', 'HEAD'].includes(method) ? undefined : await readRawBody(event, false)

  try {
    const response = await $fetch.raw(targetUrl, {
      method,
      headers,
      body,
      redirect: 'manual'
    })

    // Forward status and content-type
    setResponseStatus(event, response.status, response.statusText)
    
    // Forward headers
    for (const [key, value] of response.headers.entries()) {
      if (key.toLowerCase() === 'set-cookie') {
        // split cookies properly
        const cookies = String(value).split(/,(?=\s*[^;,\s]+=)/).map(c => c.trim()).filter(Boolean)
        for (const cookie of cookies) {
          appendResponseHeader(event, 'set-cookie', cookie)
        }
      } else if (!['content-length', 'content-encoding', 'transfer-encoding'].includes(key.toLowerCase())) {
        setResponseHeader(event, key, value)
      }
    }

    return response._data
  } catch (error: any) {
    const response = error?.response
    if (!response) {
      throw createError({ statusCode: 502, statusMessage: 'Bad Gateway' })
    }
    setResponseStatus(event, response.status, response.statusText)
    return response._data || { error: { message: response.statusText } }
  }
})
