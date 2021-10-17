<template>
  <v-app dark>
    <v-main>
      <AppHeader />
      <v-container>
        <nuxt />
      </v-container>
      <AppFooter />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'
import AppHeader from '@/components/header/AppHeader.vue'
import AppFooter from '@/components/AppFooter.vue'

@Component({
  components: { AppHeader, AppFooter }
})
class DefaultLayout extends Vue {
}
export default DefaultLayout
</script>
