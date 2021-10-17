<template>
    <v-card flat>
    <v-card-title class="text-h5 font-weight-bold" v-text="cardTitle" />
    <v-card-text>
      <v-form ref="form">
        <v-container>
          <v-row>
            <div  v-if="!editable">
              <span class="ma-4">
                サーバーの設定の変更には「管理者」「サーバー管理」「ロールの管理」「メッセージの管理」のいずれかの権限を持っている必要があります
              </span>

            </div>
            <span class="ma-4">
              ユーザーの権限の設定を変更した場合には「再読込」ボタンを押してください
            </span>
            <v-checkbox :disabled="!editable" v-model="restricted" label="予定の追加,編集,削除を「管理者」「サーバー管理」「ロールの管理」「メッセージの管理」のいずれかの権限を持ったユーザーに限定する" />
          </v-row>
          <v-btn
            :loading="checking"
            block
            color="primary"
            class="mt-4"
            @click="checkEditable"
          >
            再読込
          </v-btn>
          <v-btn
            :disabled="sending || !editable || checking"
            block
            color="primary"
            class="mt-4"
            @click="submit"
          >
            保存
          </v-btn>
          <v-btn
            :disabled="sending"
            block
            color="error"
            class="mt-4"
            @click="cancel"
          >
            キャンセル
          </v-btn>
        </v-container>
      </v-form>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'nuxt-property-decorator'
import { Discord } from '~/plugins/handler'

interface Server {
  icon: string,
  id: bigint,
  name: string,
  owner: boolean,
  permissions_new: null,
}

@Component({})
class ServerSetting extends Vue {
  sending: boolean = false

  checking: boolean = false

  editable: boolean = false

  restricted: boolean = false

  @Prop({ type: Object, default: () => ({}) })
  server!: Server

  get cardTitle () {
    return "サーバー設定"
  }

  async mounted () {
    await this.loadSetting()
    await this.checkEditable()
  }

  async loadSetting() {
    const guild_id = this.$route.params.id
    const config = await this.$discord.getGuildConfig(this.$axios, guild_id)
    this.restricted = config.restricted
  }

  async checkEditable() {
    this.checking = true
    const guild_id = this.$route.params.id
    const perms = await this.$discord.getUserPermissions(this.$axios, guild_id)
    this.editable = perms.administrator
      || perms.manage_guild
      || perms.manage_messages
      || perms.manage_roles
    setTimeout(() => {
      this.checking = false
    }, 3000)
  }

  async submit() {
    this.sending = true
    const guild_id = this.$route.params.id
    const payload: Discord.GuildConfigQuery = {
      restricted: this.restricted
    }
    await this.$discord.setGuildConfig(this.$axios, guild_id, payload)
    this.$emit('submit:finish')
    this.sending = false
    this.$router.push('/dashboard')
  }

  cancel() {
    this.$emit('cancel')
  }
}
export default ServerSetting
</script>
