export default defineNuxtRouteMiddleware((to) => {
  const auth = useAuth()
  const localePath = useLocalePath()
  if (!auth.isAuthenticated.value) {
    return navigateTo(localePath('/users/sessions'))
  }
})
