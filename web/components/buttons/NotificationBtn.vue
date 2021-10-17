<template>
  <v-chip large label outlined>
    <v-btn icon class="ml-n2" @click="remove">
      <v-icon>mdi-close</v-icon>
    </v-btn>
    <input
      v-model.number="numVal"
      class="ml-1"
      :style="style"
      type="number"
      min="1"
      max="100"
    >
    <v-menu offset-y>
      <template v-slot:activator="{ on, attrs }">
        <span
          v-bind="attrs"
          v-on="on"
        >
          {{ selectedVal }}
          <v-icon>
            mdi-chevron-down
          </v-icon>
        </span>
      </template>
      <v-list>
        <v-list-item
          v-for="item in items"
          :key="item.key"
          @click="selectedVal = item.title"
        >
          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-chip>
</template>

<script lang="ts">
import { Component, Vue, PropSync } from 'vue-property-decorator'

@Component
class NotificationBtn extends Vue {
  @PropSync('num', { type: Number, default: 1 })
  readonly numVal!: number

  @PropSync('selected', { type: String, default: '時間前' })
  readonly selectedVal!: string

  items = [
    { key: 'week', title: '週間前' },
    { key: 'day', title: '日前' },
    { key: 'hour', title: '時間前' },
    { key: 'minute', title: '分前' }
  ]

  get style () {
    const base = { width: '38px' }
    if (this.$vuetify.theme.dark) {
      return { color: '#ffffff', ...base }
    } else {
      return base
    }
  }

  remove () {
    this.$emit('remove')
  }
}
export default NotificationBtn
</script>
