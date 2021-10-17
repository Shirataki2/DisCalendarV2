<template>
  <div>
    <v-container>
      <v-row>
        <v-col cols="12" lg="10" xl="8" offset-lg="1" offset-xl="2">
          <h1 class="text-h4 mt-4 mb-6">
            {{ page.title }}
          </h1>
        </v-col>
      </v-row>
      <v-divider class="mb-3"/>
      <v-row>
        <v-col cols="12" lg="10" xl="8" offset-lg="1" offset-xl="2">
          <nuxt-content id="support" :document="page" />
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<script lang="ts">
import { Vue, Component } from 'vue-property-decorator'
import { Page } from '@/types'

@Component({
  name: 'Support',
  asyncData: async ({ store, app, $axios, $content, params, error }) => {
    const slug = params.slug
    if (!slug) { error({ statusCode: 404, message: 'Page not Found' }) }
    try {
      const page = await $content('support', slug).fetch()
      if (app.$cookies.get('access_token')) {
      }
      if (store.getters['auth/accessToken'] && store.getters['auth/user']) {
        return { page }
      } else if (app.$cookies.get('access_token')) {
        store.dispatch('auth/setAccessToken', app.$cookies.get('access_token'))
        store.dispatch('auth/setRefreshToken', app.$cookies.get('refresh_token'))
        try {
          const { data } = await $axios.get('/discord/api/users/@me')
          store.dispatch('auth/setUser', data)
          return { page }
        } catch (e) {
          return { page }
        }
      }
      return { page }
    } catch {
      error({ statusCode: 404, message: 'Page not Found' })
    }
  }
})
class Support extends Vue {
  page!: Page
}
export default Support
</script>

<style>
#support h2 {
  margin-top: 20px !important;
  margin-bottom: 10px !important;
}

#support li {
  margin: 6px 1px;
}
</style>
