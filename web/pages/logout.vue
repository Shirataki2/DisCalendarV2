<template>
  <div style="text-align: center">
    <Loading />
    <p class="mt-5">
      ログアウト中...
    </p>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'
import Loading from '@/components/Loading.vue'

@Component({
  components: { Loading }
})
class Logout extends Vue {
  async mounted () {
    const sleep = (msec: number) => new Promise(resolve => setTimeout(resolve, msec))
    this.$cookies.remove('access_token')
    this.$cookies.remove('refresh_token')
    this.$cookies.remove('user')
    this.$cookies.remove('session')
    this.$store.dispatch('auth/setAccessToken', '')
    this.$store.dispatch('auth/setRefreshToken', '')
    this.$store.dispatch('auth/setUser', null)
    await sleep(500)
    this.$router.push('/')
  }
}
export default Logout
</script>
