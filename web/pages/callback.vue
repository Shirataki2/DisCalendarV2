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
import Cookies from 'js-cookie'

import { Component, Vue } from 'nuxt-property-decorator'
import Loading from '@/components/Loading.vue'

@Component({
  components: { Loading },
  asyncData: async ({query}) => {
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
    return { payload }
  }
})
class Callback extends Vue {
  payload: any

  async mounted () {
    console.log(this.payload)
    try {
      const { data } = await this.$axios.post(
        '/discord/api/oauth2/token',
        (new URLSearchParams(this.payload)).toString(),
        {
          headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
        }
      )
      console.log(data)
      console.log(data.access_token)
      // app.$cookies.set('access_token', data.access_token)
      // app.$cookies.set('refresh_token', data.refresh_token)
      this.$store.dispatch('auth/setAccessToken', data.access_token)
      this.$store.dispatch('auth/setRefreshToken', data.refresh_token)

      Cookies.set('access_token', data.access_token, { sameSite: 'lax', secure: false })
      Cookies.set('refresh_token', data.refresh_token, { sameSite: 'lax', secure: false })
      const sleep = (msec: number) => new Promise(resolve => setTimeout(resolve, msec))
      await sleep(500)

      console.log(this.$cookies.getAll())
      const user = await this.$axios.get(
        '/discord/api/users/@me',
        {
          headers: {
            Authorization: 'Bearer ' + data.access_token
          }
        }
      )
      console.log(user.data)
      await this.$store.dispatch('auth/setUser', user.data)

      this.$router.push('/')
    } catch (e) {
      console.error(e)
      this.$router.push('/logout')
    }

  }
}
export default Callback
</script>
