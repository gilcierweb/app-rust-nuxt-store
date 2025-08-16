import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },
  modules: ['@nuxt/eslint', '@nuxt/image', 'shadcn-nuxt', '@pinia/nuxt', '@vite-pwa/nuxt', 'nuxt-toast'],
  css: ["~/assets/css/main.css"],
  vite: { plugins: [tailwindcss(),], },
  shadcn: {
    /**
     * Prefix for all the imported component
     */
    prefix: '',
    /**
     * Directory that the component lives in.
     * @default "./components/ui"
     */
    componentDir: './components/ui'
  },
  app: {
    pageTransition: {
      name: 'fade',
      mode: 'out-in' // default
    },
    layoutTransition: {
      name: 'layout',
      mode: 'out-in' // default
    }
  },
  runtimeConfig: {
    public: {
      ApiBaseUrl: '',
      ApiRustBaseUrl: '',
      baseURL: process.env.NUXT_BASE_URL || 'http://localhost:5150' // Exposed to the frontend as well.
    }
  },
  pwa: {
     /* PWA options */
  },
})