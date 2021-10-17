import { Plugin } from '@nuxt/types'
import { DiscordAPI } from '@/plugins/handler'

declare module 'vue/types/vue' {
  interface Vue {
    $discord: DiscordAPI
  }
}

declare module '@nuxt/types' {
  interface Context {
    $discord: DiscordAPI
  }
}

const plugin: Plugin = (ctx, inject) => {
  inject('discord', new DiscordAPI())
  ctx.$discord = new DiscordAPI()
}

export default plugin
