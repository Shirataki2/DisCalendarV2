<template>
  <v-avatar :size="size">
    <img :src="imageSource" alt="User Avatar">
  </v-avatar>
</template>

<script lang="ts">
import { Prop, Component, Vue } from 'nuxt-property-decorator'

@Component
class UserAvatar extends Vue {
  @Prop({ default: '48', type: String })
  size!: string

  @Prop({ required: true, type: String })
  userId!: string

  @Prop({ type: String })
  avatarId!: string

  get imageSource () {
    if (this.avatarId) {
      return `https://cdn.discordapp.com/avatars/${this.userId}/${this.avatarId}.png`
    } else {
      return 'https://leevz.io/img/discord.png'
    }
  }
}
export default UserAvatar
</script>
