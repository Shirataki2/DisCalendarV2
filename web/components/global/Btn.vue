<template>
  <div class="mt-2 mb-5 justify-center">
    <v-btn
      v-if="external"
      ref="noopener"
      :block="block"
      :outlined="outlined"
      rounded
      x-large
      :href="to"
      :color="color"
      target="_blank"
    >
      <slot />
    </v-btn>
    <v-btn
      v-else
      nuxt
      :block="block"
      :outlined="outlined"
      rounded
      x-large
      :to="to"
      :color="color"
    >
      <slot />
    </v-btn>
  </div>
</template>

<script lang="ts">
import { Component, Vue, Prop } from 'nuxt-property-decorator'

@Component
class AppFooter extends Vue {
  @Prop({ type: Boolean, default: false })
  external!: boolean

  @Prop({ type: Boolean, default: false })
  block!: boolean

  @Prop({ type: Boolean, default: false })
  outlined!: boolean

  @Prop({ type: String, default: '/' })
  to!: string

  @Prop({ type: String, default: 'primary' })
  color!: string
}
export default AppFooter
</script>
