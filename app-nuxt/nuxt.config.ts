import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },
  modules: ['@nuxt/eslint', '@nuxt/image'],
  css: ['~/assets/css/main.css'],
  vite: { plugins: [tailwindcss(),], },
  app: {
    pageTransition: {
      name: 'fade',
      mode: 'out-in' // default
    },
    layoutTransition: {
      name: 'slide',
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
})