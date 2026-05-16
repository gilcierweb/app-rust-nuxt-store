export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.includes('/admin')) {
    const localePath = useLocalePath()
    const auth = useAuth()

    await auth.init()

    if (!auth.isAuthenticated.value) {
      return navigateTo(localePath('/users/sessions'))
    }

    if (!auth.user.value?.can_manage_admin) {
      return navigateTo(localePath('/'))
    }
    to.meta.layout = 'admin'
  }
})
