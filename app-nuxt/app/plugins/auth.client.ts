export default defineNuxtPlugin(async () => {
  const auth = useAuth()
  await auth.init()
})
