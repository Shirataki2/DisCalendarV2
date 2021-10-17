<template>
  <v-row justify="center" align="center">
    <v-col cols="12" style="text-align: center">
      <h1
        id="logo"
        class="
          text-h4 text-sm-h2 text-lg-h2 font-weight-bold
          my-3 my-sm-12 my-md-14 my-lg-16
          py-3 py-sm-12 py-md-14 py-lg-16
        "
      >
        DisCalendar
      </h1>
      <p
        class="
          text-subtitle-1 font-weight-medium
          my-3 my-sm-12 my-md-14 my-lg-16
          py-3 py-sm-6 py-md-8 py-lg-12
          mx-sm-12 mx-md-14 mx-lg-16
          px-sm-6 px-md-8 px-lg-12
        "
      >
        DisCalendarはDiscord用のカレンダーアプリです。予定の作成から投稿まで面倒なコマンド操作はほとんど必要ありません。
        使い慣れたブラウザから、どこでも予定の追加や編集することができます。
      </p>
      <v-btn rounded :block="isXS" x-large color="info" @click="invite">
        BOTを導入する
      </v-btn>
      <p class="lr-border text-overline my-4">
        OR 既に導入済みの方は
      </p>
      <v-btn
        v-if="!isLogin"
        :block="isXS"
        rounded
        class="my-2"
        x-large
        color="primary"
        to="/login"
        nuxt
      >
        ログイン
      </v-btn>
      <v-btn
        v-else
        :block="isXS"
        rounded
        class="my-2"
        x-large
        color="primary"
        to="/dashboard"
        nuxt
      >
        サーバー一覧
      </v-btn>
      <v-btn
        ref="noreferrer"
        rounded
        :block="isXS"
        dark
        x-large
        class="my-2"
        color="secondary"
        href="https://discord.gg/MyaZRuze23"
        target="_blank"
      >
        サポートサーバーへ参加
      </v-btn>
      <v-btn
        rounded
        :block="isXS"
        dark
        x-large
        class="my-2"
        color="green"
        to="/docs/gettingstarted"
        nuxt
      >
        使い方を見る
      </v-btn>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'

@Component({
  asyncData: async ({ store, app, $axios }) => {
    if (app.$cookies.get('access_token')) {
    }
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
      }
    }
  }
})
class Index extends Vue {
  get isLogin () {
    return this.$store.getters['auth/user']
  }

  get isXS () {
    return this.$vuetify.breakpoint.xs
  }

  invite () {
    const url = process.env.INVITATION_URL!
    const win = window.open(url, 'Discord', 'menubar')
    if (win) {
      const timer = setInterval(() => {
        if (win.closed) {
          clearInterval(timer)
        }
      }, 100)
    } else {
      location.href = url
    }
  }
}
export default Index
</script>

<style lang="scss">
.lr-border {
  display: flex;
  align-items: center;

  &:before,
  &:after {
      content: "";
      height: 1px;
      flex-grow: 1;
      background-color: #666;
  }

  &:before {
      margin-right: 1rem;
  }

  &:after {
      margin-left: 1rem;
  }
}

@font-face {
  font-family: Logo;
  src: url(/UniSansHeavy.otf);
}

#logo {
  font-family: Logo, sans-serif !important;
}
</style>
