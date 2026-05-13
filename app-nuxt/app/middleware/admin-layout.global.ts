export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.includes('/admin')) {
    const token = useCookie<string | null>('auth_token', { default: () => null })
    const localePath = useLocalePath()
    const auth = useAuth()
    if (!token.value) {
      return navigateTo(localePath('/users/sessions'))
    }

    if (!auth.user.value) {
      await auth.fetchCurrentUser()
    }
    if (!auth.user.value?.can_manage_admin) {
      return navigateTo(localePath('/'))
    }
    to.meta.layout = 'admin'
  }
})
