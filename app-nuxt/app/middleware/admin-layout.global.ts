function withLocalePrefix(toPath: string, targetPath: string): string {
  const localeMatch = toPath.match(/^\/(en|es)(\/|$)/)
  if (!localeMatch) return targetPath
  return `/${localeMatch[1]}${targetPath}`
}

export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.includes('/admin')) {
    const auth = useAuth()

    await auth.init()

    if (!auth.isAuthenticated.value) {
      return navigateTo(withLocalePrefix(to.path, '/users/sessions'))
    }

    if (!auth.user.value?.can_manage_admin) {
      return navigateTo(withLocalePrefix(to.path, '/'))
    }
    to.meta.layout = 'admin'
  }
})
