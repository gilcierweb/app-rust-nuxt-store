export default defineNuxtPlugin(async () => {
  if (import.meta.client) return
  const auth = useAuth()
  await auth.init()
})
