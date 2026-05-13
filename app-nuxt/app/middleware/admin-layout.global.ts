export default defineNuxtRouteMiddleware((to) => {
  if (to.path.includes('/admin')) {
    const token = useCookie<string | null>('auth_token', { default: () => null })
    const localePath = useLocalePath()
    if (!token.value) {
      return navigateTo(localePath('/users/sessions'))
    }
    to.meta.layout = 'admin'
  }
})
