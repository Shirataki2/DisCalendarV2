<template>
  <v-timeline align-top :dense="$vuetify.breakpoint.smAndDown">
    <v-timeline-item
      v-for="(item, i) in items"
      :key="i"
      :color="item.color"
      :icon="item.icon"
      fill-dot
    >
      <v-card
        dark
        :color="item.color"
      >
        <v-card-title class="title" v-text="item.title" />
        <v-card-text
          class="pt-5 text--primary"
          :style="style"
        >
          <p v-for="description in item.descriptions" :key="description" v-text="description" />
          <v-btn
            v-if="item.button.show"
            rounded
            large
            :color="item.button.color"
            @click="item.button.onClick"
            v-text="item.button.title"
          />
        </v-card-text>
      </v-card>
    </v-timeline-item>
  </v-timeline>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'

@Component
class Introduction extends Vue {
  items = [
    {
      title: 'ログインし Botをサーバーに導入',
      icon: 'mdi-numeric-1',
      color: 'primary',
      descriptions: [
        'まずはこのサイトにDiscordのアカウントを使用してログインします．',
        '次に，Discordに通知を届けるためにBotをサーバーに導入します．',
        '以下のボタンからBotを招待してください'
      ],
      button: {
        show: true,
        color: 'info',
        title: 'Botを招待',
        onClick: () => {
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
    },
    {
      title: '初期化のためのコマンドを入力',
      icon: 'mdi-numeric-2',
      color: 'red',
      descriptions: [
        'このBotからのメッセージを受信したいチャンネルで以下のコマンドを実行してください',
        'cal init',
        'このコマンドは「メッセージの管理」「サーバーの管理」「ロールの管理」「管理者」のいずれかの役職を持っている方が実行可能です'
      ],
      button: {
        show: false,
        color: 'info',
        title: 'Botを招待',
        onClick: () => { }
      }
    },
    {
      title: 'ブラウザで予定を入力',
      icon: 'mdi-numeric-3',
      color: 'amber darken-3',
      descriptions: [
        'ダッシュボードページから，BOTを導入したサーバーへ移動します．',
        '予定の新規作成から新しい予定を作成してみましょう！',
        '🔔で指定した時刻と開始時刻になった際に自動的にDiscordへメッセージが送信されます',
        '予定を右クリックすることで予定を編集することができます'
      ],
      button: {
        show: true,
        color: 'primary',
        title: 'ダッシュボードページへ',
        onClick: () => {
          this.$router.push('/dashboard')
        }
      }
    }
  ]

  get style () {
    const base = { 'text-align': 'left' }
    if (this.$vuetify.theme.dark) {
      return { 'background-color': '#333', color: '#fff', ...base }
    } else {
      return { 'background-color': '#f3f3f3', color: '#222', ...base }
    }
  }
}
export default Introduction
</script>
