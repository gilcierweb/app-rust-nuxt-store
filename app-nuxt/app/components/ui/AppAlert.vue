<template>
  <Transition name="alert-transition">
    <div
      v-if="visible && message"
      :id="alertId"
      :class="alertClasses"
      role="alert"
    >
      <span :class="iconClasses"></span>
      <div class="flex-1">
        <span v-if="title" class="text-lg font-semibold">{{ title }}:</span>
        <template v-if="title"> </template>{{ message }}
      </div>
      <button
        type="button"
        v-if="dismissible"
        class="ms-auto cursor-pointer leading-none opacity-60 hover:opacity-100 transition-opacity"
        aria-label="Close Button"
        @click="closeAlert"
      >
        <span class="icon-[tabler--x] size-5"></span>
      </button>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { useId } from 'vue'

type AlertTone = 'warning' | 'success' | 'error' | 'info'
type AlertVariant = 'solid' | 'soft' | 'outline'

const props = withDefaults(defineProps<{
  message: string
  title?: string
  tone?: AlertTone
  variant?: AlertVariant
  icon?: string
  dismissible?: boolean
  id?: string
}>(), {
  title: '',
  tone: 'warning',
  variant: 'soft',
  icon: '',
  dismissible: true,
  id: '',
})

const visible = ref(true)
const generatedId = useId()
const alertId = computed(() => {
  if (props.id) return props.id
  return `dismiss-alert-${generatedId.replace(/[^A-Za-z0-9_-]/g, '-')}`
})

const closeAlert = () => {
  visible.value = false
}

watch(
  () => props.message,
  (value) => {
    if (value) visible.value = true
  }
)

const toneClassMap: Record<AlertTone, string> = {
  warning: 'alert-warning',
  success: 'alert-success',
  error: 'alert-error',
  info: 'alert-info',
}

const toneIconMap: Record<AlertTone, string> = {
  warning: 'icon-[tabler--alert-triangle]',
  success: 'icon-[tabler--circle-check]',
  error: 'icon-[tabler--alert-circle]',
  info: 'icon-[tabler--info-circle]',
}

const variantClassMap: Record<AlertVariant, string> = {
  solid: '',
  soft: 'alert-soft',
  outline: 'alert-outline',
}

const alertClasses = computed(() => [
  'alert',
  'flex',
  'items-center',
  'gap-4',
  toneClassMap[props.tone],
  variantClassMap[props.variant],
])

const iconClasses = computed(() => [
  props.icon || toneIconMap[props.tone],
  'shrink-0',
  'size-6',
])
</script>

<style scoped>
.alert-transition-enter-active,
.alert-transition-leave-active {
  transition: all 0.3s ease-in-out;
}
.alert-transition-enter-from,
.alert-transition-leave-to {
  opacity: 0;
  transform: translateX(1.25rem);
}
</style>
