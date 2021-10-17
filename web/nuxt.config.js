import colors from 'vuetify/es5/util/colors'
import config from './siteconfig'
const ENV = process.env

export default {
  server: {
    host: '0.0.0.0',
    port: 6655
  },
  // Global page headers (https://go.nuxtjs.dev/config-head)
  head: {
    titleTemplate: 'DisCalendar',
    title: 'DisCalendar',
    htmlAttrs: {
      lang: 'ja'
    },
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: config.description },
      { hid: 'keywords', name: 'keywords', content: config.kw, 'xml:lang': 'ja', lang: 'ja' }
    ],
    link: [
      { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
      {
        rel: 'stylesheet',
        href: 'https://fonts.googleapis.com/css?family=Courier+Prime:wght@700&display=swap'
      }
    ]
  },

  // Global CSS (https://go.nuxtjs.dev/config-css)
  css: ['@/assets/style.scss'],

  // Plugins to run before rendering page (https://go.nuxtjs.dev/config-plugins)
  plugins: [
    '@/plugins/axios',
    '@/plugins/discord'
  ],

  // Auto import components (https://go.nuxtjs.dev/config-components)
  components: true,

  // Modules for dev and build (recommended) (https://go.nuxtjs.dev/config-modules)
  buildModules: [
    // https://go.nuxtjs.dev/typescript
    '@nuxt/typescript-build',
    // https://go.nuxtjs.dev/vuetify
    '@nuxtjs/vuetify',
    '@nuxtjs/moment',
    '@nuxtjs/google-analytics',
    '@nuxtjs/pwa'
  ],

  pwa: {
    meta: {
      name: config.sitename,
      author: config.author,
      description: config.description,
      theme_color: config.themeColor,
      language: config.lang,
      ogType: config.ogType,
      ogHost: config.ogHost,
      ogImage: config.ogImage
    },
    manifest: {
      name: config.sitename,
      lang: config.lang,
      short_name: config.sitename,
      description: config.description,
      theme_color: config.themeColor
    },
    workbox: {
      offlineAnalytics: true,
      runtimeCaching: [
        {
          urlPattern: '^https://fonts.(?:googleapis|gstatic).com/(.*)',
          handler: 'cacheFirst',
          strategyOptions: {
            cacheName: 'image-cache',
            cacheExpiration: {
              maxEntries: 100,
              maxAgeSeconds: 24 * 60 * 60 * 180 // 0.5 year
            }
          }
        }
      ]
    }
  },

  googleAnalytics: {
    id: process.env.GA_ID
  },

  // Modules (https://go.nuxtjs.dev/config-modules)
  modules: [
    // https://go.nuxtjs.dev/axios
    '@nuxtjs/proxy',
    '@nuxtjs/axios',
    'cookie-universal-nuxt',
    '@nuxt/content'
  ],

  moment: {
    defaultLocale: 'ja',
    locales: ['ja']
  },

  // Axios module configuration (https://go.nuxtjs.dev/config-axios)
  axios: {
    proxy: true
  },

  proxy: {
    '/discord/api/': {
      target: 'https://discord.com/api',
      pathRewrite: {
        '^/discord/api/': '/'
      }
    },
    '/local/api/': {
      target: 'http://api:5000',
      pathRewrite: {
        '^/local/api/': '/'
      }
    }
  },

  // Vuetify module configuration (https://go.nuxtjs.dev/config-vuetify)
  vuetify: {
    customVariables: ['@/assets/variables.scss'],
    treeShake: true,
    theme: {
      dark: true,
      themes: {
        dark: {
          primary: colors.blue.darken2,
          accent: colors.grey.darken3,
          secondary: colors.amber.darken3,
          info: colors.teal.lighten1,
          warning: colors.amber.base,
          error: colors.deepOrange.accent4,
          success: colors.green.accent3
        }
      }
    }
  },

  env: { ...ENV },

  // Build Configuration (https://go.nuxtjs.dev/config-build)
  build: {
  }
}
