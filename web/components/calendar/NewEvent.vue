<template>
  <v-card flat>
    <v-card-title class="text-h5 font-weight-bold" v-text="cardTitle" />
    <v-card-text>
      <v-form ref="form">
        <v-container>
          <v-row>
            <v-col v-if="alertText.length" cols="12" class="mt-n5 mb-n10">
              <v-alert
                type="error"
              >
                <span v-for="text in alertText" :key="text">
                  ・ {{ text }}<br>
                </span>
              </v-alert>
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="title"
                label="タイトルを入力 *"
                :rules="[
                  v => !!v || 'この項目は必須です',
                  v => v.length <= 32 || 'この項目は32文字以下である必要があります'
                ]"
                counter="32"
              />
            </v-col>
            <v-col class="mt-n6" cols="12" sm="5">
              <v-menu
                v-model="startDateMenu"
                :close-on-content-click="false"
                transition="scale-transition"
                offset-y
                min-width="250px"
              >
                <template #activator="{ on, attrs }">
                  <v-combobox
                    v-model="startDate"
                    label="開始日"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-bind="attrs"
                    v-on="on"
                  />
                </template>
                <v-date-picker
                  v-model="startDate"
                  no-title
                  scrollable
                  @change="onStartDateChanged"
                >
                  <v-spacer />
                  <v-btn text color="primary" @click="startDateMenu = false">
                    OK
                  </v-btn>
                </v-date-picker>
              </v-menu>
            </v-col>
            <v-col class="mt-n6" cols="12" sm="5">
              <v-menu
                v-model="startTimeMenu"
                :disabled="isAllDay"
                :close-on-content-click="false"
                transition="scale-transition"
                offset-y
                min-width="250px"
              >
                <template #activator="{ on, attrs }">
                  <v-combobox
                    v-model="startTime"
                    :disabled="isAllDay"
                    label="開始時"
                    prepend-icon="mdi-clock-time-eight-outline"
                    readonly
                    v-bind="attrs"
                    v-on="on"
                  />
                </template>
                <v-time-picker
                  v-model="startTime"
                  :landscape="$vuetify.breakpoint.smAndUp"
                  scrollable
                >
                  <v-spacer />
                  <v-btn text color="primary" @click="startTimeMenu = false">
                    OK
                  </v-btn>
                </v-time-picker>
              </v-menu>
            </v-col>
            <v-col class="mt-n6" cols="12" sm="2">
              <v-checkbox
                v-model="isAllDay"
                label="終日"
              />
            </v-col>
            <v-col class="mt-n9" cols="12" sm="5">
              <v-menu
                v-model="endDateMenu"
                :close-on-content-click="false"
                transition="scale-transition"
                offset-y
                min-width="250px"
              >
                <template #activator="{ on, attrs }">
                  <v-combobox
                    v-model="endDate"
                    label="終了日"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-bind="attrs"
                    v-on="on"
                  />
                </template>
                <v-date-picker
                  v-model="endDate"
                  no-title
                  scrollable
                >
                  <v-spacer />
                  <v-btn text color="primary" @click="endDateMenu = false">
                    OK
                  </v-btn>
                </v-date-picker>
              </v-menu>
            </v-col>
            <v-col class="mt-n9" cols="12" sm="5">
              <v-menu
                v-model="endTimeMenu"
                :disabled="isAllDay"
                :close-on-content-click="false"
                transition="scale-transition"
                offset-y
                min-width="250px"
              >
                <template #activator="{ on, attrs }">
                  <v-combobox
                    v-model="endTime"
                    :disabled="isAllDay"
                    label="終了時"
                    prepend-icon="mdi-clock-time-eight-outline"
                    readonly
                    v-bind="attrs"
                    v-on="on"
                  />
                </template>
                <v-time-picker
                  v-model="endTime"
                  :landscape="$vuetify.breakpoint.smAndUp"
                  scrollable
                >
                  <v-spacer />
                  <v-btn text color="primary" @click="endTimeMenu = false">
                    OK
                  </v-btn>
                </v-time-picker>
              </v-menu>
            </v-col>
            <v-col class="mt-n7" cols="12" sm="2">
              <v-menu :close-on-content-click="false">
                <template #activator="{ on, attrs }">
                  <v-chip class="px-8" :color="color" v-bind="attrs" v-on="on" />
                </template>
                <v-color-picker
                  v-model="color"
                  :swatches="swatches"
                  show-swatches
                />
              </v-menu>
            </v-col>
            <v-col class="mt-n4" cols="12">
              <v-icon>mdi-bell</v-icon>
              <NotificationBtn
                v-for="notification in notifications"
                :key="notification.key"
                class="mx-1"
                :num.sync="notification.num"
                :selected.sync="notification.type"
                @remove="deleteNotification(notification)"
              />
              <v-btn v-if="notifications.length <= 10" icon @click="addNotification">
                <v-icon>
                  mdi-plus
                </v-icon>
              </v-btn>
            </v-col>
            <v-col class="mt-n6" cols="12">
              <v-textarea
                v-model="description"
                label="説明"
                :rules="[
                  v => v.length <= 500 || 'この項目は500文字以下である必要があります'
                ]"
                counter="500"
              />
            </v-col>
          </v-row>
          <v-btn
            :disabled="sending"
            block
            color="primary"
            class="mt-4"
            @click="submit"
          >
            OK
          </v-btn>
          <v-btn
            :disabled="sending"
            block
            color="error"
            class="mt-4"
            @click="cancel"
          >
            キャンセル
          </v-btn>
          <v-btn
            v-if="deletable"
            :disabled="sending"
            block
            color="error"
            class="mt-4"
            @click="deleteEvent"
          >
            削除
          </v-btn>
        </v-container>
      </v-form>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { Component, Vue, Prop } from 'nuxt-property-decorator'
import NotificationBtn from '@/components/buttons/NotificationBtn.vue'

interface Notification {
  key: number
  num: number
  type: string
}

@Component({
  components: { NotificationBtn }
})
class New extends Vue {
  @Prop({ type: Boolean, default: false })
  deletable!: boolean

  @Prop({ type: String, default: '新規作成' })
  cardTitle!: string

  @Prop({ type: String, default: 'POST' })
  method!: string

  @Prop({ type: String, required: true })
  endpoint!: string

  @Prop({ type: Object, default: () => ({}) })
  event!: any

  alertText: Array<String> = []

  title: string = ''
  description: string = ''
  color: string = '#F44336'
  startDateMenu: boolean = false
  startDate: string = this.$moment().format('YYYY-MM-DD')
  startTimeMenu: boolean = false
  startTime: string = this.$moment().format('HH:00')
  isAllDay: boolean = false
  endDateMenu: boolean = false
  endDate: string = this.$moment().format('YYYY-MM-DD')
  endTimeMenu: boolean = false
  endTime: string = this.$moment().format('HH:30')

  notifications: Array<Notification> = [
    { key: 0, num: 1, type: '日前' },
    { key: 1, num: 1, type: '時間前' }
  ]

  swatches = [
    ['#F44336', '#E91E63', '#9C27B0', '#673AB7'],
    ['#3F51B5', '#2196F3', '#03A9F4', '#00BCD4'],
    ['#009688', '#4CAF50', '#8BC34A', '#CDDC39'],
    ['#FFEB3B', '#FFC107', '#FF9800', '#FF5722'],
    ['#9E9E9E', '#212121', '#FF0000', '#0000FF']
  ]

  id = -1
  key = 2
  sending = false

  get form () {
    const ref: any = this.$refs.form
    return ref
  }

  load (event: any) {
    this.id = event.id
    this.title = event.name
    this.description = event.description
    this.color = event.color
    this.isAllDay = !event.timed
    this.startDate = this.$moment(event.start).format('YYYY-MM-DD')
    this.startTime = this.$moment(event.start).format('HH:mm')
    this.endDate = this.$moment(event.end).format('YYYY-MM-DD')
    this.endTime = this.$moment(event.end).format('HH:mm')
    this.notifications = event.notifications.map(
      (notification: any) => JSON.parse(notification)
    )
  }

  onStartDateChanged (e: string) {
    const srart = this.$moment(e).toDate().getTime()
    const end = this.$moment(this.endDate).toDate().getTime()
    if (end < srart) {
      this.endDate = e
    }
  }

  addNotification () {
    const key = this.key
    this.key++
    this.notifications.push({ key, num: key, type: '時間前' })
  }

  deleteNotification (target: Notification) {
    this.notifications = this.notifications.filter(
      notification => notification.key !== target.key
    )
  }

  validateForm () {
    let valid = true
    this.alertText = []
    const formValid: boolean = this.form.validate()
    valid = valid && formValid
    if (!formValid) {
      this.alertText.push('未記入の欄があります')
    }
    const invalidNotifications = this.notifications.filter(
      notification =>
        notification.num <= 0 || notification.num > 100
    )
    valid = valid && invalidNotifications.length === 0
    if (invalidNotifications.length) {
      this.alertText.push('通知時間帯の指定が不正です(1以上100以下の値を入力してください)')
    }
    const start = this.$moment(`${this.startDate} ${this.startTime}`).toDate().getTime()
    const end = this.$moment(`${this.endDate} ${this.endTime}`).toDate().getTime()
    if (start > end) {
      this.alertText.push('終了時間を開始時間より前に設定することはできません')
    }
    valid = valid && start <= end
    return valid
  }

  async submit () {
    if (!this.validateForm()) { return }
    this.sending = true
    const notifications = this.notifications.map(notification => JSON.stringify(notification))
    const fn = this.method === 'POST' ? this.$axios.post : this.$axios.put
    const additionalElem = this.method === 'PUT' ? { id: this.id } : {}
    const endpoint = this.method === 'PUT' ? `${this.endpoint}/${this.id}` : this.endpoint
    await fn(
      endpoint,
      {
        ...additionalElem,
        guild_id: this.$route.params.id,
        name: this.title,
        description: this.description,
        notifications,
        color: this.color,
        is_all_day: this.isAllDay,
        start_at: this.$moment(`${this.startDate} ${this.startTime}`).format('YYYY-MM-DDTHH:mm:ss'),
        end_at: this.$moment(`${this.endDate} ${this.endTime}`).format('YYYY-MM-DDTHH:mm:ss'),
        created_at: this.$moment().format('YYYY-MM-DDTHH:mm:ss')
      }
    )
    this.sending = false
    this.reset()
    this.$emit('submitted')
  }

  cancel () {
    this.reset()
    this.$emit('cancel')
  }

  reset () {
    this.title = ''
    this.description = ''
    this.color = '#F44336'
    this.startDate = this.$moment().format('YYYY-MM-DD')
    this.startTime = this.$moment().format('HH:00')
    this.isAllDay = false
    this.endDate = this.$moment().format('YYYY-MM-DD')
    this.endTime = this.$moment().format('HH:30')
    this.form.resetValidation()
  }

  deleteEvent () {
    this.$emit('remove', this.id)
  }
}
export default New
</script>
