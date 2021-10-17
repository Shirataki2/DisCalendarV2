<template>
  <div>
    <v-btn v-if="isNotXS" outlined @click="today">
      今日
    </v-btn>
    <v-btn icon aria-label="Prev Date" @click="prev">
      <v-icon>
        mdi-chevron-left
      </v-icon>
    </v-btn>
    <v-menu :close-on-content-click="false">
      <template #activator="{ on, attrs }">
        <v-btn text :large="isNotXS" v-bind="attrs" v-on="on">
          <span class="text-h6" v-text="getDate" />
        </v-btn>
      </template>
      <v-card>
        <v-date-picker color="primary" :value="`${year}-${month}-${day}`" @change="pickDate" />
      </v-card>
    </v-menu>
    <v-btn icon aria-label="Next Date" @click="next">
      <v-icon>
        mdi-chevron-right
      </v-icon>
    </v-btn>
    <v-menu class="ml-5">
      <template #activator="{ on, attrs }">
        <v-btn outlined :small="isXS" v-bind="attrs" v-on="on" v-text="toViewPoint(menu)" />
      </template>
      <v-list offset-y>
        <v-list-item v-for="item in menuItems" :key="item.type" style="cursor: pointer" @click="setType(item)">
          <v-list-item-title v-text="item.title" />
        </v-list-item>
      </v-list>
    </v-menu>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'nuxt-property-decorator'

@Component
class CalendarController extends Vue {
  year = 2020
  month = 1
  day = 1
  menu = '月間表示'
  get menuItems () {
    if (this.isXS) {
      return [
        { title: '月間表示', type: 'month' },
        { title: '4日表示', type: '4day' },
        { title: '1日表示', type: 'day' }
      ]
    } else {
      return [
        { title: '月間表示', type: 'month' },
        { title: '週間表示', type: 'week' },
        { title: '4日表示', type: '4day' },
        { title: '1日表示', type: 'day' }
      ]
    }
  }

  today () {
    this.$store.commit('calendar/TODAY')
  }

  get isNotXS () {
    return !this.$vuetify.breakpoint.xs
  }

  get isXS () {
    return this.$vuetify.breakpoint.xs
  }

  toViewPoint (str: string) {
    if (this.$vuetify.breakpoint.xs) {
      switch (str) {
        case '月間表示':
          return '月'
        case '週間表示':
          return '週'
        case '4日表示':
          return '4日'
        default:
          return '1日'
      }
    } else {
      return str
    }
  }

  get getDate () {
    if (this.$vuetify.breakpoint.xs) {
      return `${this.month}月`
    } else {
      return `${this.year}年${this.month}月`
    }
  }

  setType (item: any) {
    this.menu = item.title
    this.$store.commit('calendar/TYPE', item.type)
  }

  prev () {
    this.$store.commit('calendar/PREV', true)
  }

  next () {
    this.$store.commit('calendar/NEXT', true)
  }

  pickDate (date: string) {
    const d = this.$moment(date)
    this.year = d.year()
    this.month = d.month() + 1
    this.day = d.date()
    this.$store.commit('calendar/DATE', [
      this.year, this.month, this.day
    ])
  }

  mounted () {
    this.$store.subscribe((mut, _) => {
      if (mut.type === 'calendar/DATE') {
        this.year = mut.payload[0]
        this.month = mut.payload[1]
        this.day = mut.payload[2]
      }
    })
  }
}
export default CalendarController
</script>
