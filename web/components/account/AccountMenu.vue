<template>
  <v-menu offset-y>
    <template #activator="{ on, attrs }">
      <v-btn
        v-bind="attrs"
        icon
        large
        v-on="on"
      >
        <slot />
      </v-btn>
    </template>
    <v-list>
      <v-list-item v-for="item in items" :key="item.key" style="cursor: pointer" @click="item.callback">
        <v-list-item-title v-text="item.name()" />
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'
import UserAvatar from '@/components/account/UserAvatar.vue'

@Component({
  components: { UserAvatar }
})
class AccountMenu extends Vue {
  items = [
    {
      key: 'dashboard',
      name: () => 'サーバー一覧',
      callback: this.dashboard
    },
    {
      key: 'theme',
      name: () => `テーマ変更: ${this.theme}`,
      callback: this.themeSwitch
    },
    {
      key: 'logout',
      name: () => 'ログアウト',
      callback: this.logout
    }
  ]

  get theme () {
    return this.$vuetify.theme.dark ? 'ダーク' : 'ライト'
  }

  mounted () {
    const theme = localStorage.getItem('theme')
    if (theme) {
      this.$vuetify.theme.dark = theme === 'dark'
    }
  }

  dashboard () {
    this.$router.push('/dashboard')
  }

  logout () {
    this.$router.push('/logout')
  }

  themeSwitch () {
    this.$vuetify.theme.dark = !this.$vuetify.theme.dark
    localStorage.setItem('theme', this.$vuetify.theme.dark ? 'dark' : 'light')
  }
}
export default AccountMenu
</script>
