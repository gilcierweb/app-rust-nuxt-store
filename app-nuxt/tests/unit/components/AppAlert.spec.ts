import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import AppAlert from '../../../app/components/AppAlert.vue'

describe('AppAlert.vue', () => {
  it('renders the message when message prop is provided', () => {
    const message = 'Test Alert Message'
    const wrapper = mount(AppAlert, {
      props: {
        message,
        show: true
      }
    })
    expect(wrapper.text()).toContain(message)
  })

  it('applies the correct class for different types', async () => {
    const types = ['success', 'error', 'info', 'warning']
    for (const type of types) {
      const wrapper = mount(AppAlert, {
        props: {
          type: type as any,
          message: 'Test',
          show: true
        }
      })
      const expectedClass = type === 'success' ? 'alert-success' : `alert-${type}`
      expect(wrapper.find('.alert').classes()).toContain(expectedClass)
    }
  })

  it('hides when show prop is false', () => {
    const wrapper = mount(AppAlert, {
      props: {
        message: 'Test',
        show: false
      }
    })
    expect(wrapper.find('.alert').exists()).toBe(false)
  })

  it('emits close event when close button is clicked', async () => {
    const wrapper = mount(AppAlert, {
      props: {
        message: 'Test',
        show: true,
        dismissible: true
      }
    })
    await wrapper.find('button').trigger('click')
    expect(wrapper.emitted()).toHaveProperty('close')
    expect(wrapper.find('.alert').exists()).toBe(false)
  })

  it('automatically closes after autoClose duration', async () => {
    vi.useFakeTimers()
    const wrapper = mount(AppAlert, {
      props: {
        message: 'Test',
        show: true,
        autoClose: 3000
      }
    })
    
    expect(wrapper.find('.alert').exists()).toBe(true)
    
    vi.advanceTimersByTime(3000)
    await wrapper.vm.$nextTick()
    
    expect(wrapper.find('.alert').exists()).toBe(false)
    vi.useRealTimers()
  })
})
