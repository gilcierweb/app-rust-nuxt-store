import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  modules: ['@nuxt/eslint', '@nuxt/image', 'shadcn-nuxt', '@pinia/nuxt', 'pinia-plugin-persistedstate/nuxt', '@vite-pwa/nuxt', 'nuxt-toast', '@nuxtjs/i18n','nuxt-security'],
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

    head: {
      title: "App Rust Nuxt Store",
      meta: [
        { charset: "utf-8" },
        { name: "viewport", content: "width=device-width, initial-scale=1" },
        { name: "description", content: "App Rust Nuxt Store" },
        { name: "robots", content: "noindex, nofollow" }, // keep off search engines
        { name: "theme-color", content: "#FF6F00" },
      ],
    },

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
    registerType: 'autoUpdate',
    manifest: {
      id: '/?source=pwa',
      name: 'App Rust Nuxt Store',
      short_name: 'App Rust Nuxt Store',
      description: 'App Rust Nuxt Store',
      start_url: '/?source=twa',
      scope: '/',
      display: 'standalone',
      background_color: '#0A0A0F',
      theme_color: '#00E5FF',
      categories: ['saas', 'monitoring', 'industrial'],
      icons: [
        // regular icons
        { src: '/pwa-icon-192.png', sizes: '192x192', type: 'image/png' },
        { src: '/pwa-icon-512.png', sizes: '512x512', type: 'image/png' },
        // maskable icon required for high-quality install UI
        { src: '/pwa-maskable-512.png', sizes: '512x512', type: 'image/png', purpose: 'maskable' },
      ],
      screenshots: [
        { src: '/screenshot-1.webp', sizes: '1280x720', type: 'image/webp' },
        { src: '/screenshot-2.webp', sizes: '1280x720', type: 'image/webp' },
      ],
    },
    workbox: {
      runtimeCaching: [
        // Example: cache game assets
        {
          urlPattern: /\.(?:png|jpg|jpeg|webp|gif|svg|mp3|wav|ogg|mp4|webm|glb|gltf|bin|ttf|woff2)$/,
          handler: 'CacheFirst',
          options: {
            cacheName: 'assets-cache',
            expiration: { maxEntries: 200, maxAgeSeconds: 60 * 60 * 24 * 30 }, // 30 days
          },
        },
      ],
    },
    devOptions: {
      enabled: false,
    },
  },

  i18n: {
    locales: [
      {
        code: "en",
        iso: "en-US",
        file: "en.json",
        name: "English",
      },
      {
        code: "es",
        iso: "es-ES",
        file: "es.json",
        name: "Español",
      },
      {
        code: "pt-BR",
        iso: "pt-BR",
        file: "pt-BR.json",
        name: "Português Brasil",
      },
    ],
    defaultLocale: "pt-BR",
    strategy: "prefix_except_default",
    lazy: true,
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: "i18n_redirected",
      redirectOn: "root",
    },
  },

  // security: {
  //     csrf: true,
  //   },
})