<template>
  <v-row>
    <v-col
      offset-sm="1"
      sm="10"
      offset-lg="1"
      lg="10"
      offset-xl="3"
      xl="6"
    >
      <v-card>
        <v-card-title class="justify-center">
          <strong>
            サーバーを選択してください
          </strong>
        </v-card-title>
        <v-card-text>
          <template v-if="loading">
            <Loading />
          </template>
          <template v-else>
            <v-container>
              <v-row>
                <v-col
                  v-for="guild in joinedGuilds"
                  :key="guild.id"
                  cols="6"
                  sm="3"
                  md="3"
                  lg="2"
                >
                  <div>
                    <v-hover #default="{ hover }">
                      <v-img
                        :src="guild.avatar_url"
                        lazy-src="/logo.png"
                        style="border-radius: 50%; cursor: pointer"
                        @error="onImageLoadFailed($event)"
                        @click="$router.push(`/dashboard/${guild.guild_id}`)"
                      >
                        <template #placeholder>
                          <v-row
                            class="fill-height ma-0"
                            align="center"
                            justify="center"
                          >
                            <v-progress-circular
                              indeterminate
                              color="grey lighten-5"
                            />
                          </v-row>
                        </template>
                        <v-fade-transition>
                          <v-row
                            v-if="hover"
                            class="fill-height ma-0"
                            align="center"
                            justify="center"
                            style="background: #333e"
                          >
                            <span
                              style="
                                  color: #fff;
                                  font-size: 1.4rem;
                                  font-weight: 900;
                                "
                            >
                              GO!
                            </span>
                          </v-row>
                        </v-fade-transition>
                      </v-img>
                    </v-hover>
                  </div>
                  <div style="text-align: center; margin-top: 10px">
                    <span v-text="guild.name" />
                  </div>
                </v-col>
                <v-col
                  v-for="guild in invitableGuilds"
                  :key="guild.id"
                  cols="6"
                  sm="3"
                  md="3"
                  lg="2"
                >
                  <div>
                    <v-hover #default="{ hover }">
                      <v-img
                        :src="getIcon(guild)"
                        style="
                            filter: grayscale(1);
                            border-radius: 50%;
                            cursor: pointer;
                          "
                        @click="invite(guild)"
                      >
                        <v-fade-transition>
                          <v-row
                            v-if="hover"
                            class="fill-height ma-0"
                            align="center"
                            justify="center"
                            style="background: #333e"
                          >
                            <span
                              style="
                                  color: #fff;
                                  font-size: 1.4rem;
                                  font-weight: 900;
                                "
                            >
                              招待
                            </span>
                          </v-row>
                        </v-fade-transition>
                      </v-img>
                    </v-hover>
                  </div>
                  <div style="text-align: center; margin-top: 10px">
                    <span v-text="guild.name" />
                  </div>
                </v-col>
              </v-row>
            </v-container>
          </template>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'
import { Discord } from '@/plugins/handler'
import Loading from '@/components/Loading.vue'

@Component({
  components: { Loading },
  asyncData: async ({ redirect, store, app, $axios }) => {
    if (store.getters['auth/accessToken'] && store.getters['auth/user']) {
      return { userdata: store.getters['auth/user'] }
    } else if (app.$cookies.get('access_token')) {
      store.dispatch('auth/setAccessToken', app.$cookies.get('access_token'))
      store.dispatch('auth/setRefreshToken', app.$cookies.get('refresh_token'))
      try {
        const { data } = await $axios.get('/discord/api/users/@me')
        store.dispatch('auth/setUser', data)
        return { userdata: data }
      } catch (e) {
        redirect(301, '/login')
      }
    }
  }
})
class Index extends Vue {
  joinedGuilds: Array<Discord.MinimalGuild> = []
  invitableGuilds: Array<Discord.PartialGuild> = []

  loading = true

  toIconUrl (guildId: bigint, avatarId: string) {
    return `https://cdn.discordapp.com/icons/${guildId}/${avatarId}.png`
  }

  async mounted () {
    const guilds = await this.$discord.getServerList(this.$axios)
    const guildIds = guilds.map(guild => guild.id)
    const joinedGuilds = await this.$discord.getBotJoinedServerList(
      this.$axios,
      guildIds
    )
    this.joinedGuilds = joinedGuilds
    this.invitableGuilds = guilds.filter(guild =>
      guild.permissions & 40 &&
      !joinedGuilds.filter(
        g => JSON.stringify(g.guild_id) === JSON.stringify(guild.id)
      ).length
    )
    this.loading = false
  }

  invite (guild: Discord.PartialGuild) {
    const url = process.env.INVITATION_URL! + `&guild_id=${guild.id}`
    const win = window.open(url, 'Discord', 'menubar')
    if (win) {
      const timer = setInterval(() => {
        if (win.closed) {
          clearInterval(timer)
          this.$router.push(`/dashboard/${guild.id}`)
        }
      }, 100)
    } else {
      location.href = url
    }
  }

  getIcon (guild: Discord.PartialGuild) {
    if (guild.icon) {
      return this.toIconUrl(guild.id, guild.icon)
    } else {
      return '/logo.png'
    }
  }

  onImageLoadFailed (e: any) {
    console.log(e)
  }
}
export default Index
</script>
