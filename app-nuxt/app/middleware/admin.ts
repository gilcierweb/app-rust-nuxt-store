export default defineNuxtRouteMiddleware((to) => {
  const auth = useAuth()
  if (!auth.isAuthenticated.value) {
    return navigateTo('/users/sessions')
  }
})
