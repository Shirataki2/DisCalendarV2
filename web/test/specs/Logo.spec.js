import test from 'ava'
import { mount } from '@vue/test-utils'
import AppFooter from '@/components/AppFooter.vue'

test('is work', (t) => {
  const wrapper = mount(AppFooter)
  t.truthy(wrapper.vm)
})
