<template>
  <v-navigation-drawer
    :value="value"
    mobile-breakpoint="1100"
    app
    clipped
    @input="$emit('input', $event)"
  >
    <v-list
      nav
    >
      <div v-for="item in items" :key="item.title">
        <div v-if="!item.external">
          <v-list-item
            v-if="isShow(item.needLogin, item.needLogout, isLogin)"
            link
            nuxt
            color="blue"
            :to="item.to"
          >
            <v-list-item-icon>
              <v-icon v-text="item.icon" />
            </v-list-item-icon>
            <v-list-item-content>
              <v-list-item-title v-text="item.title" />
            </v-list-item-content>
          </v-list-item>
        </div>
        <div v-else>
          <v-list-item
            v-if="isShow(item.needLogin, item.needLogout, isLogin)"
            link
            nuxt
            color="blue"
            target="_blank"
            rel="noreferrer"
            :href="item.to"
          >
            <v-list-item-icon>
              <v-icon v-text="item.icon" />
            </v-list-item-icon>
            <v-list-item-content>
              <v-list-item-title v-text="item.title" />
            </v-list-item-content>
          </v-list-item>
        </div>
      </div>
    </v-list>
    <template #append>
      <div v-for="item in bottomItems" :key="item.title">
        <v-list-item
          v-if="isDashboard(item.onlyDashboard) && isShow(item.needLogin, item.needLogout, isLogin)"
          color="blue"
          @click="item.func()"
        >
          <v-list-item-icon>
            <v-icon v-text="item.icon()" />
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title v-text="item.title" />
          </v-list-item-content>
        </v-list-item>
      </div>
    </template>
  </v-navigation-drawer>
</template>

<script lang="ts">
import { Component, Vue, Prop } from 'nuxt-property-decorator'

@Component
class NavigationDrawer extends Vue {
  @Prop({ type: Boolean, default: null })
  value!: boolean

  get isLogin () {
    return this.$store.getters['auth/user']
  }

  items = [
    {
      title: 'ホーム',
      icon: 'mdi-home',
      to: '/',
      external: false,
      needLogin: false,
      needLogout: false
    },
    {
      title: 'ダッシュボード',
      icon: 'mdi-console',
      to: '/dashboard',
      external: false,
      needLogin: true,
      needLogout: false
    },
    {
      title: 'サポートサーバー',
      icon: 'mdi-discord',
      to: 'https://discord.gg/MyaZRuze23',
      external: true,
      needLogin: false,
      needLogout: false
    },
    {
      title: '使い方',
      icon: 'mdi-help-circle-outline',
      to: '/docs/gettingstarted',
      external: false,
      needLogin: false,
      needLogout: false
    },
    {
      title: '利用規約',
      icon: 'mdi-book-open-page-variant',
      to: '/support/tos',
      external: false,
      needLogin: false,
      needLogout: false
    },
    {
      title: 'プライバシーポリシー',
      icon: 'mdi-account-supervisor',
      to: '/support/privacy',
      external: false,
      needLogin: false,
      needLogout: false
    },
    {
      title: 'GitHub',
      icon: 'mdi-github',
      to: 'https://github.com/Shirataki2/DisCalendarV2',
      external: true,
      needLogin: false,
      needLogout: false
    },
    {
      title: 'ログアウト',
      icon: 'mdi-logout',
      to: '/logout',
      external: false,
      needLogin: true,
      needLogout: false
    },
    {
      title: 'ログイン',
      icon: 'mdi-login',
      to: '/login',
      external: false,
      needLogin: false,
      needLogout: true
    }
  ]

  bottomItems = [
    {
      title: '今日へ移動',
      icon: () => 'mdi-calendar',
      func: () => {
        this.$store.commit('calendar/TODAY')
      },
      onlyDashboard: true,
      needLogin: true,
      needLogout: false
    },
    {
      title: 'テーマ変更',
      icon: () => this.themeIcon(),
      func: () => {
        this.$vuetify.theme.dark = !this.$vuetify.theme.dark
        localStorage.setItem('theme', this.$vuetify.theme.dark ? 'dark' : 'light')
      },
      onlyDashboard: false,
      needLogin: false,
      needLogout: false
    }
  ]

  isDashboard (A: boolean) {
    if (!A) { return true }
    return this.$route.params.id !== undefined
  }

  isShow (A: boolean, B: boolean, C: boolean) {
    if (A && B) { return false }
    if (!A && !B) { return true }
    if (A && !B && !C) { return false }
    if (!A && B && C) { return false }
    if (A && !B && C) { return true }
    if (!A && B && !C) { return true }
  }

  themeIcon () {
    if (this.isDark) {
      return 'mdi-brightness-4'
    } else {
      return 'mdi-brightness-7'
    }
  }

  get isDark () {
    return this.$vuetify.theme.dark
  }
}
export default NavigationDrawer
</script>
