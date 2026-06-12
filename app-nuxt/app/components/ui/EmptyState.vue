<template>
  <div class="rounded-box border border-dashed border-base-content/20 bg-base-100 p-10 text-center">
    <span :class="[icon, 'text-base-content/30 mx-auto mb-4 size-12']"></span>
    <h2 class="text-lg font-semibold">{{ title }}</h2>
    <p v-if="description" class="text-base-content/60 mt-1">{{ description }}</p>
    <slot />
    <NuxtLinkLocale v-if="actionTo" :to="actionTo" class="btn btn-primary mt-6">
      {{ actionLabel }}
    </NuxtLinkLocale>
    <button v-else-if="actionLabel" class="btn btn-primary mt-6" @click="$emit('action')">
      {{ actionLabel }}
    </button>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon?: string
  title: string
  description?: string
  actionTo?: string
  actionLabel?: string
}

withDefaults(defineProps<Props>(), {
  icon: 'icon-[tabler--inbox-off]',
  description: '',
  actionTo: '',
  actionLabel: ''
})

defineEmits<{
  action: []
}>()
</script>
