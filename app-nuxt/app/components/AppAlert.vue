<template>
  <BaseAppAlert v-bind="$props" />
</template>

<script setup lang="ts">
import BaseAppAlert from '~/components/ui/AppAlert.vue'

withDefaults(defineProps<{
  message: string
  title?: string
  tone?: 'warning' | 'success' | 'error' | 'info'
  variant?: 'solid' | 'soft' | 'outline'
  icon?: string
  dismissible?: boolean
  id?: string
}>(), {
  dismissible: true
})
</script>