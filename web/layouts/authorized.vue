<template>
  <v-app dark>
    <v-main>
      <AppHeader />
      <v-container>
        <div v-if="$fetchState.pending">
          <div style="text-align: center">
            <Loading />
            <p class="mt-5">
              認証中...
            </p>
          </div>
        </div>
        <div v-else-if="$fetchState.error">
          <p>Error ouucred while fetching user data.</p>
        </div>
        <div v-else>
          <nuxt />
        </div>
      </v-container>
      <AppFooter />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'
import AppHeader from '@/components/header/AppHeader.vue'
import AppFooter from '@/components/AppFooter.vue'
import Loading from '@/components/Loading.vue'

@Component({
  components: { Loading, AppHeader, AppFooter }
})
class DefaultLayout extends Vue {
  async fetch () {
    const id = this.$route.params.id
    if (!id) { this.$router.push('/') }
    // const sleep = (msec: number) => new Promise(resolve => setTimeout(resolve, msec))
    try {
      const server = await this.$discord.getServer(this.$axios, id)
      this.$store.commit('auth/SET_SERVER', server)
    } catch {
      this.$store.commit('auth/SET_SERVER', null)
    }
  }
}
export default DefaultLayout
</script>
