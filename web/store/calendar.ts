import { Module, VuexModule, Mutation } from 'vuex-module-decorators'

@Module({
  name: 'auth',
  stateFactory: true,
  namespaced: true
})
export default class CalendarModule extends VuexModule {
  prev: boolean = false
  next: boolean = false
  type: 'month' | 'week' | 'day' | '4day' = 'month'
  date = [2020, 1, 1]

  @Mutation
  TODAY () { }

  @Mutation
  TYPE (type: 'month' | 'week' | 'day' | '4day') {
    this.type = type
  }

  @Mutation
  PREV (flag: boolean) {
    this.prev = flag
  }

  @Mutation
  NEXT (flag: boolean) {
    this.next = flag
  }

  @Mutation
  DATE (date: [number, number, number]) {
    this.date = date
  }
}
