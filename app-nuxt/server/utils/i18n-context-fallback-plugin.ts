import { defineNitroPlugin } from 'nitropack/runtime'

function buildSafeFallbackContext(defaultLocale: string) {
  return {
    messages: {},
    slp: {},
    localeConfigs: {},
    trackMap: {},
    trackKey(key: string, locale: string) {
      if (!this.trackMap[locale]) {
        this.trackMap[locale] = new Set<string>()
      }
      this.trackMap[locale].add(key)
    },
    vueI18nOptions: {
      locale: defaultLocale,
      defaultLocale,
      fallbackLocale: defaultLocale,
      messages: {}
    }
  }
}

async function ensureI18nContext(event: any) {
  if (!event?.context) return
  if (event.context.nuxtI18n) return

  const defaultLocale =
    event.context.runtimeConfig?.public?.i18n?.defaultLocale ||
    event.context.nitro?.options?.runtimeConfig?.public?.i18n?.defaultLocale ||
    'pt-BR'

  event.context.nuxtI18n = buildSafeFallbackContext(defaultLocale)
}

export default defineNitroPlugin((nitroApp) => {
  nitroApp.hooks.hook('request', ensureI18nContext)
  nitroApp.hooks.hook('render:before', async ({ event }) => {
    await ensureI18nContext(event)
  })
})
