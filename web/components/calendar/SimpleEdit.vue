<template>
  <v-card flat min-width="350px">
    <v-toolbar v-if="event" flat dark :color="event.color">
      <v-toolbar-title v-text="event.name" />
    </v-toolbar>
    <v-container>
      <v-row>
        <v-col class="mt-1" cols="12">
          <v-icon class="mt-n1 mr-1">
            mdi-alarm
          </v-icon>
          <span v-text="date" /></span>
        </v-col>
        <v-col class="mt-n1" cols="12">
          <v-icon class="mt-n1 mr-1">
            mdi-bell
          </v-icon>
          <span v-text="notification" /></span>
        </v-col>
        <v-col class="mt-n1 mb-n2" v-if="event.description" cols="12" style="font-size:0.9em">
          <p v-text="event.description" />
        </v-col>
      </v-row>
    </v-container>
    <v-card-actions class="mt-n1" v-if="!restricted">
      <v-btn text color="primary" @click="edit">
        <v-icon>v-pencil</v-icon>
        編集
      </v-btn>
      <v-spacer />
      <v-btn text color="error" @click="remove">
        <v-icon>v-erase</v-icon>
        削除
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import { Component, Vue, Prop } from 'nuxt-property-decorator'

@Component
class Edit extends Vue {
  @Prop({ required: true })
  event!: any

  @Prop({ default: false, type: Boolean })
  restricted!: boolean

  get date () {
    const start = this.$moment(this.event.start)
    const end = this.$moment(this.event.end)
    if (!this.event.timed) {
      if (start.format('YYYY-MM-DD') !== end.format('YYYY-MM-DD')) {
        return `${start.format('YYYY/MM/DD')} - ${end.format('YYYY/MM/DD')}`
      } else {
        return `${start.format('YYYY/MM/DD')}`
      }
    } else if (start.format('YYYY-MM-DD') !== end.format('YYYY-MM-DD')) {
      return `${start.format('YYYY/MM/DD HH:mm')} - ${end.format('YYYY/MM/DD HH:mm')}`
    } else {
      return `${start.format('YYYY/MM/DD HH:mm')} - ${end.format('HH:mm')}`
    }
  }

  get notification () {
    const notifications: Array<any> = this.event.notifications
    if (notifications.length) {
      return notifications.reduce(
        (acc, notification) => {
          const n = JSON.parse(notification)
          return acc + `${n.num}${n.type}  `
        }, '')
    } else {
      return '-'
    }
  }

  edit () {
    this.$emit('edit', this.event)
  }

  remove () {
    this.$emit('remove', this.event.id)
  }
}
export default Edit
</script>
