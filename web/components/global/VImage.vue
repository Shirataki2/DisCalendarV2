<template>
  <div class="mt-3 mb-7" style="width: 100%;">
    <v-img :src="src">
      <template #placeholder>
        <v-skeleton-loader
          class="mx-auto"
          type="image"
        />
      </template>
    </v-img>
  </div>
</template>

<script lang="ts">
import { Component, Vue, Prop } from 'nuxt-property-decorator'

@Component
class VImage extends Vue {
  @Prop({ type: String, required: true })
  src!: string
}
export default VImage
</script>
