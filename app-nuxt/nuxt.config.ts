import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  modules: ['@nuxt/eslint', '@nuxt/image', 'shadcn-nuxt', '@pinia/nuxt', 'pinia-plugin-persistedstate/nuxt', '@vite-pwa/nuxt', 'nuxt-toast', '@nuxtjs/i18n','nuxt-security', '@nuxtjs/sitemap', 'nuxt-charts'],
  css: ["~/assets/css/main.css"],

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
        { name: "description", content: "App Rust Nuxt Store - Sua loja online com Rust e Nuxt" },
        { name: "robots", content: "index, follow" },
        { name: "theme-color", content: "#FF6F00" },
        { property: "og:type", content: "website" },
        { property: "og:site_name", content: "App Rust Nuxt Store" },
        { name: "twitter:card", content: "summary_large_image" },
        // CSP desabilitado para desenvolvimento - reabilitar em produção
        // { "http-equiv": "Content-Security-Policy", content: "default-src 'self' 'unsafe-inline' 'unsafe-eval' *; script-src 'self' 'unsafe-inline' 'unsafe-eval' blob: data: *; worker-src 'self' blob: data:; style-src 'self' 'unsafe-inline' https://cdnjs.cloudflare.com *; img-src 'self' data: * https://cdn.flyonui.com https://dummyjson.com https://cdn.dummyjson.com; font-src 'self' https://cdnjs.cloudflare.com *; connect-src 'self' http://localhost:5150 https://dummyjson.com https://cdn.dummyjson.com *;" },
      ],
      link: [
        { rel: "canonical", href: "https://app-rust-nuxt-store.com" },
      ],
    },

    // pageTransition: { name: 'fade',  mode: 'out-in' }, // default
    // layoutTransition: { name: 'layout', mode: 'out-in' } // default
  },
  
  runtimeConfig: {
    // private
    // NUXT_BASE_URL: ''
    public: {
      ApiBaseUrl: '',
      ApiRustBaseUrl: '',
      baseURL: process.env.NUXT_PUBLIC_BASE_URL || 'http://localhost:5150' // Exposed to the frontend as well.
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
        files: [
          "en/admin.json", "en/auth.json", "en/cart.json", "en/common.json",
          "en/contact.json", "en/order.json", "en/pages.json", "en/payment.json",
          "en/post.json", "en/privacy.json", "en/product.json", "en/profile.json",
          "en/seed.json", "en/shipping.json", "en/terms.json", "en/variant.json",
          "en/header.json", "en/footer.json"
        ],
        name: "English",
        flag: "🇺🇸"
      },
      {
        code: "es",
        iso: "es-ES",
        files: [
          "es/admin.json", "es/auth.json", "es/cart.json", "es/common.json",
          "es/contact.json", "es/order.json", "es/pages.json", "es/payment.json",
          "es/post.json", "es/privacy.json", "es/product.json", "es/profile.json",
          "es/seed.json", "es/shipping.json", "es/terms.json", "es/variant.json",
          "es/header.json", "es/footer.json"
        ],
        name: "Español",
        flag: "🇪🇸"
      },
      {
        code: "pt-BR",
        iso: "pt-BR",
        files: [
          "pt-BR/admin.json", "pt-BR/auth.json", "pt-BR/cart.json", "pt-BR/common.json",
          "pt-BR/contact.json", "pt-BR/order.json", "pt-BR/pages.json", "pt-BR/payment.json",
          "pt-BR/post.json", "pt-BR/privacy.json", "pt-BR/product.json", "pt-BR/profile.json",
          "pt-BR/seed.json", "pt-BR/shipping.json", "pt-BR/terms.json", "pt-BR/variant.json",
          "pt-BR/header.json", "pt-BR/footer.json"
        ],
        name: "Português Brasil",
        flag: "🇧🇷"
      },
    ],
    langDir: "locales",
    defaultLocale: "pt-BR",
    strategy: "prefix_except_default",
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: "i18n_redirected",
      alwaysRedirect: false,
      fallbackLocale: "pt-BR"
    },
  },

  sitemap: {
    sources: ["/api/__sitemap__/urls"],
    strictNuxtContentPaths: true,
    xslColumns: [
      { label: "URL", width: "60%" },
      { label: "Freq", width: "15%" },
      { label: "Priority", width: "15%" },
    ],
  },

  // Image optimization
  image: {
    quality: 80,
    format: ['webp', 'avif', 'jpg'],
    screens: {
      xs: 320,
      sm: 640,
      md: 768,
      lg: 1024,
      xl: 1280,
      xxl: 1536,
    },
    domains: [],
    presets: {
      product: {
        modifiers: {
          format: 'webp',
          fit: 'cover',
          quality: 80,
        },
      },
      thumbnail: {
        modifiers: {
          format: 'webp',
          fit: 'cover',
          quality: 60,
          width: 300,
          height: 300,
        },
      },
    },
  },

  // Experimental features disabled - causing payload errors in dev
  // experimental: {
  //   payloadExtraction: true,
  //   renderJsonPayloads: true,
  //   clientFallback: true,
  // },

  // Nitro build optimization
  nitro: {
    preset: 'cloudflare_pages',
    compressPublicAssets: true,
    minify: true,
    routeRules: {
      '/api/**': { cors: true, headers: { 'Access-Control-Allow-Methods': 'GET,POST,PUT,DELETE' } },
      '/products/**': { isr: 60, prerender: false },
      // Prerender disabled for dev - enable for production
      // '/': { prerender: true },
      // '/products': { prerender: true },
      // '/about': { prerender: true },
      // '/contact': { prerender: true },
    },
  },

  // Vite build optimization
  vite: {
    plugins: [tailwindcss()],
    css: {
      devSourcemap: true,
    },
    build: {
      cssMinify: true,
      rollupOptions: {

      },
    },
    optimizeDeps: {
      include: ['vue', 'vue-router', 'pinia'],
    },
    // Desabilitar prefetch para evitar rate limiting
    experimental: {
      renderBuiltUrl: (filename) => `/_nuxt/${filename}`,
    },
  },

  security: {
    // csrf: true,
    headers: {
      contentSecurityPolicy: {
        'default-src': ["'self'"],
        'script-src': ["'self'", "'unsafe-inline'", "'unsafe-eval'"],
        'script-src-attr': ["'self'", "'unsafe-inline'"],
        'img-src': ["'self'", 'data:', 'https://cdn.flyonui.com', 'https://dummyjson.com', 'https://cdn.dummyjson.com', 'https://picsum.photos'],
        'style-src': ["'self'", "'unsafe-inline'", "https://cdnjs.cloudflare.com"],
        'font-src': ["'self'", "https://cdnjs.cloudflare.com"],
        'connect-src': ["'self'", "http://localhost:5150", "https://dummyjson.com", "https://cdn.dummyjson.com", "https://romantic-freedom-production-386f.up.railway.app", "https://picsum.photos/800/600"],
      }
    }
  },
})