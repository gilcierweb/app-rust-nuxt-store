<template>
  <div
    :class="[
      'animate-pulse',
      variant === 'circle' ? 'rounded-full' : 'rounded-lg',
      'bg-base-200',
      className
    ]"
    :style="computedStyle"
    v-bind="$attrs"
  />
</template>

<script setup lang="ts">
interface Props {
  variant?: 'text' | 'circle' | 'rect' | 'card'
  width?: string | number
  height?: string | number
  className?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'text',
  className: ''
})

const computedStyle = computed(() => {
  const styles: Record<string, string> = {}

  if (props.width) {
    styles.width = typeof props.width === 'number' ? `${props.width}px` : props.width
  }
  if (props.height) {
    styles.height = typeof props.height === 'number' ? `${props.height}px` : props.height
  }

  if (props.variant === 'text' && !props.height) {
    styles.height = '1rem'
  }
  if (props.variant === 'circle' && !props.width && !props.height) {
    styles.width = '2.5rem'
    styles.height = '2.5rem'
  }
  if (props.variant === 'card' && !props.height) {
    styles.height = '16rem'
  }

  return styles
})
</script>
