export default defineEventHandler((event) => {
  const config = useRuntimeConfig(event)
  const token = event.context.csrfToken
  const headerName = config.public.csurf?.headerName

  setResponseHeader(event, 'cache-control', 'no-store, no-cache, must-revalidate')

  if (!token || !headerName) {
    throw createError({
      statusCode: 500,
      statusMessage: 'Nuxt CSRF token unavailable'
    })
  }

  return {
    token,
    headerName
  }
})
