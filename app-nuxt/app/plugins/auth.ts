export default defineNuxtPlugin(async () => {
  if (import.meta.server) return
  const auth = useAuth()
  await auth.init()
})
