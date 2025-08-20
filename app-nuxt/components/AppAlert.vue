<template>
  <div
    v-if="visible"
    :class="['alert', variantClass, 'mb-6', 'flex', 'items-center', 'gap-4']"
    role="alert"
  >
    <span v-html="iconSvg" class="shrink-0 h-6 w-6"></span>
    <div class="flex-1">
      <slot>{{ message }}</slot>
    </div>

    <button
      v-if="dismissible"
      @click="close"
      class="btn btn-ghost btn-sm"
      aria-label="Fechar alerta"
    >
      ✕
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onBeforeUnmount } from 'vue'

const props = defineProps<{
  type?: 'success' | 'error' | 'info' | 'warning'
  message?: string
  show?: boolean
  autoClose?: number | null
  dismissible?: boolean
}>()

const emit = defineEmits<{ (e: 'close'): void }>()

const visible = ref<boolean>(props.show ?? !!props.message)
let timer: number | null = null

const dismissible = computed(() => props.dismissible ?? true)

const variantClass = computed(() => {
  switch (props.type) {
    case 'error':
      return 'alert-error'
    case 'warning':
      return 'alert-warning'
    case 'info':
      return 'alert-info'
    default:
      return 'alert-success'
  }
})

const iconSvg = computed(() => {
  switch (props.type) {
    case 'error':
      return `<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>`
    case 'warning':
      return `<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0zM12 9v4m0 4h.01"/></svg>`
    case 'info':
      return `<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M12 2a10 10 0 100 20 10 10 0 000-20z"/></svg>`
    default:
      return `<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>`
  }
})

function close() {
  visible.value = false
  if (timer) {
    clearTimeout(timer)
    timer = null
  }
  emit('close')
}

function startAutoClose() {
  if (timer) {
    clearTimeout(timer)
    timer = null
  }
  if (props.autoClose && props.autoClose > 0) {
    timer = window.setTimeout(() => {
      close()
    }, props.autoClose)
  }
}

watch(
  () => props.show,
  (v) => {
    visible.value = v ?? !!props.message
    if (visible.value) startAutoClose()
  }
)

watch(
  () => props.message,
  (m) => {
    visible.value = !!m
    if (visible.value) startAutoClose()
  }
)

onBeforeUnmount(() => {
  if (timer) {
    clearTimeout(timer)
    timer = null
  }
})
</script>

<style scoped></style>