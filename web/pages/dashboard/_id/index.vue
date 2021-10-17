<template>
  <v-row justify="center" align="center">
    <v-col v-if="server" class="my-n1 mb-n5" cols="12">
      <div>
        <v-btn :disabled="restricted" large color="secondary" rounded @click.stop="eventDialog = true">
          <v-icon>
            mdi-plus
          </v-icon>
          新規作成
        </v-btn>
      </div>
      <v-dialog v-model="eventDialog" max-width="1000" min-width="80%" persistent>
        <NewEvent
          :endpoint="`/local/api/events/${$route.params.id}`"
          method="POST"
          @submitted="onSubmitted"
          @cancel="eventDialog = false"
        />
      </v-dialog>
      <div
        v-if="server"
        style="
          position: absolute;
          top: 15px;
          right: 30px;
          text-align: right;
        "
      >
        <span class="text-h6" v-text="server.name" />
        <v-btn style="top: -2px" icon @click="serverDialog = true">
          <v-icon>
            mdi-cog
          </v-icon>
        </v-btn>
      </div>
      <v-dialog v-model="serverDialog" max-width="1000" min-width="80%" persistent>
        <ServerSetting
          :server="server"
          @submit:finish="serverDialog = false"
          @cancel="serverDialog = false"
        />
      </v-dialog>
    </v-col>
    <v-col v-if="server" class="mt-2 mb-n5 mx-n3" cols="12">
      <div style="width: 100%">
        <Calendar :restricted="restricted" ref="calendar" @submitted="onSubmitted" />
      </div>
    </v-col>
    <v-col v-else style="text-align: center" cols="12">
      <h1 class="my-5">
        サーバーデータの取得に失敗しました
      </h1>
      <p>以下の事項をご確認ください</p>
      <p>
        ･ BOTがサーバーに導入されているか
      </p>
      <p>
        ･ あなた自身がBOTを導入したサーバーに参加しているか
      </p>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { Component, Vue, Prop } from 'nuxt-property-decorator'
import Calendar from '@/components/calendar/Calendar.vue'
import NewEvent from '@/components/calendar/NewEvent.vue'
import ServerSetting from '@/components/calendar/ServerSetting.vue'
import { Discord } from '@/plugins/handler'
import { auth } from '~/store'

@Component({
  layout: 'authorized',
  components: { Calendar, NewEvent, ServerSetting },
  asyncData: async ({ redirect, store, app, $axios, $discord, params }) => {
    if (store.getters['auth/accessToken'] && store.getters['auth/user']) {
      return { userdata: store.getters['auth/user'] }
    } else if (app.$cookies.get('access_token')) {
      store.commit('auth/SET_ACCESS_TOKEN', app.$cookies.get('access_token'))
      store.commit('auth/SET_REFRESH_TOKEN', app.$cookies.get('refresh_token'))
      try {
        const { data } = await $axios.get('/discord/api/users/@me')
        store.commit('auth/SET_USER', data)
        return { userdata: data }
      } catch (e) {
        redirect(301, '/login')
      }
    }
  }
})
class Index extends Vue {
  @Prop({ type: Object })
  prev!: any

  @Prop({ type: Object })
  next!: any

  eventDialog: boolean = false
  serverDialog: boolean = false
  restricted: boolean = true

  async mounted () {
    const conf = await this.$discord.getGuildConfig(this.$axios, this.$route.params.id)
    if (conf.restricted) {
      const perm = await this.$discord.getUserPermissions(this.$axios, this.$route.params.id)
      const manage = perm.administrator || perm.manage_guild || perm.manage_messages || perm.manage_roles
      this.restricted = !manage
      auth.setRestricted(!manage)
    } else {
      this.restricted = false
      auth.setRestricted(false)
    }
  }

  get server () {
    const server = this.$store.getters['auth/server']
    return server
  }


  async onSubmitted () {
    const cal: any = this.$refs.calendar
    await cal.updateCalendar()
    this.eventDialog = false
  }
}
export default Index
</script>
