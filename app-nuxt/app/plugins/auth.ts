export default defineNuxtPlugin(async () => {
  if (import.meta.server) return
  const auth = useAuth()
  await auth.init()

  if (auth.isAuthenticated.value) {
    const { fetchCart } = useCartSync()
    await fetchCart()
  }
})
