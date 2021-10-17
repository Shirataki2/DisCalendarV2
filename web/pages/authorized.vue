<template>
  <v-row justify="center" align="center">
    <v-col cols="12" sm="8" md="6">
      <h1>You're authorized!</h1>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'

@Component({
  asyncData: async ({ redirect, store, app, $axios }) => {
    if (store.getters['auth/accessToken'] && store.getters['auth/user']) {
      return { userdata: store.getters['auth/user'] }
    } else if (app.$cookies.get('access_token')) {
      store.dispatch('auth/setAccessToken', app.$cookies.get('access_token'))
      store.dispatch('auth/setRefreshToken', app.$cookies.get('refresh_token'))
      try {
        const { data } = await $axios.get('/discord/api/users/@me')
        store.dispatch('auth/setUser', data)
        return { userdata: data }
      } catch (e) {
        redirect(301, '/login')
      }
    }
  }
})
class Index extends Vue {
  async mounted () {
    await this.$axios.get('/discord/api/gateway')
  }
}
export default Index
</script>
