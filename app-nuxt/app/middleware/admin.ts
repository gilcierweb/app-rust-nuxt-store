function withLocalePrefix(toPath: string, targetPath: string): string {
  const localeMatch = toPath.match(/^\/(en|es)(\/|$)/)
  if (!localeMatch) return targetPath
  return `/${localeMatch[1]}${targetPath}`
}

export default defineNuxtRouteMiddleware(async (to) => {
  const auth = useAuth()
  await auth.init()

  if (!auth.isAuthenticated.value) {
    return navigateTo(withLocalePrefix(to.path, '/users/sessions'))
  }
})
