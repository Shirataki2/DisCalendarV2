<template>
  <div style="text-align: center">
    <Loading />
    <p class="mt-5">
      リダイレクト中...
    </p>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'
import Loading from '@/components/Loading.vue'

@Component({
  components: { Loading }
})
class Index extends Vue {
  mounted () {
    location.href = process.env.OAUTH2_URL!
  }
}
export default Index
</script>
