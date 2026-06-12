export function getAppLocale(): string {
  const nuxtApp = tryUseNuxtApp()
  if (nuxtApp && nuxtApp.$i18n && nuxtApp.$i18n.locale) {
    return nuxtApp.$i18n.locale.value || 'pt-BR'
  }
  return 'pt-BR'
}
