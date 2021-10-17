<template>
  <div style="text-align: center" class="mt-12">
    <Loading />
    <p class="mt-5">
      確認中...
    </p>
  </div>
</template>

<script lang="ts">
import queryString from 'querystring'
import { Component, Vue } from 'nuxt-property-decorator'
import Loading from '@/components/Loading.vue'

@Component({
  components: { Loading },
  asyncData: async ({ store, query, $axios, app }) => {
    const code = query.code
    if (typeof code !== 'string') { return }
    const payload = {
      client_id: process.env.CLIENT_ID!,
      client_secret: process.env.CLIENT_SECRET!,
      grant_type: 'authorization_code',
      redirect_uri: process.env.REDIRECT_URI!,
      scope: 'identify guilds',
      code
    }
    const { data } = await $axios.post(
      '/discord/api/oauth2/token',
      (new URLSearchParams(payload)).toString(),
      {
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
      }
    )
    app.$cookies.set('access_token', data.access_token, { sameSite: 'lax', secure: false })
    app.$cookies.set('refresh_token', data.refresh_token, { sameSite: 'lax', secure: false })
    await store.dispatch('auth/setAccessToken', app.$cookies.get('access_token'))
    await store.dispatch('auth/setRefreshToken', app.$cookies.get('refresh_token'))
    const user = await $axios.get(
      '/discord/api/users/@me',
      {
        headers: {
          Authorization: 'Bearer ' + data.access_token
        }
      }
    )
    await store.dispatch('auth/setUser', user.data)
  }
})
class Callback extends Vue {
  mounted () {
    this.$router.push('/')
  }
}
export default Callback
</script>
