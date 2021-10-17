<template>
  <div>
    <v-app-bar elevate-on-scroll app clipped-left outlined>
      <v-app-bar-nav-icon aria-label="Navigation Drawer" @click.stop="drawer = !drawer" />
      <v-toolbar-title :style="visible" @click="$router.push('/')">
        <span id="logo">DisCalendar</span>
      </v-toolbar-title>
      <v-spacer />
      <div v-if="$route.path.match(/\/dashboard\/\d{18,}/g)">
        <CalendarController />
      </div>
      <v-spacer />
      <div v-if="$store.getters['auth/user']">
        <AccountMenu>
          <UserAvatar
            :avatar-id="$store.getters['auth/user'].avatar"
            :user-id="$store.getters['auth/user'].id"
          />
        </AccountMenu>
      </div>
      <div v-else>
        <v-btn text to="/login" nuxt>
          ログイン
        </v-btn>
      </div>
    </v-app-bar>
    <NavigationDrawer v-model="drawer" />
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'
import AccountMenu from '@/components/account/AccountMenu.vue'
import CalendarController from '@/components/calendar/Controller.vue'
import NavigationDrawer from '@/components/header/NavDrawer.vue'
import UserAvatar from '@/components/account/UserAvatar.vue'

@Component({
  components: {
    AccountMenu,
    CalendarController,
    NavigationDrawer,
    UserAvatar,
  }
})
class AppHeader extends Vue {
  drawer = null

  get visible () {
    const defaults = {
      cursor: 'pointer'
    }
    if ((this.$vuetify.breakpoint.xs || this.$vuetify.breakpoint.sm) && this.$route.params.id) {
      return {
        display: 'none',
        ...defaults
      }
    } else {
      return {
        ...defaults
      }
    }
  }
}
export default AppHeader
</script>

<style lang="scss" scoped>
@font-face {
  font-family: Logo;
  src: url(/UniSansHeavy.otf);
}

#logo {
  font-size: 1.2em;
  font-family: Logo, sans-serif;
}

.theme--light.v-app-bar {
    background-color: #fff !important;
    color: rgba(0, 0, 0, 0.87);
}
</style>
