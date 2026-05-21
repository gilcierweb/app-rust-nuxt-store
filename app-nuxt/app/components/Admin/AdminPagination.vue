<template>
  <div class="border-base-content/10 flex items-center justify-between px-6 py-4 border-t">
    <span class="text-xs text-base-content/50">
      {{ summary }}
    </span>

    <div class="join">
      <button
        type="button"
        class="join-item btn btn-sm btn-outline"
        :disabled="isPreviousDisabled"
        @click="$emit('change', currentPage - 1)"
      >
        {{ previousLabel }}
      </button>

      <button type="button" class="join-item btn btn-sm btn-active font-mono">
        {{ currentPage }}
      </button>

      <button
        type="button"
        class="join-item btn btn-sm btn-outline"
        :disabled="isNextDisabled"
        @click="$emit('change', currentPage + 1)"
      >
        {{ nextLabel }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  currentPage: number
  pageSize: number
  currentCount: number
  total: number
  pending?: boolean
  summary: string
  previousLabel: string
  nextLabel: string
}>(), {
  pending: false
})

defineEmits<{
  change: [page: number]
}>()

const isPreviousDisabled = computed(() => props.pending || props.currentPage <= 1)
const isNextDisabled = computed(() => props.pending || (props.currentPage * props.pageSize) >= props.total)
</script>
