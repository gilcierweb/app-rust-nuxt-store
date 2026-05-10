<template>
  <div>
    <AdminCategoriesForm @saved="handleSaved" @cancel="navigateTo('/admin/categories')" />
  </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (category: Category) => {
  navigateTo('/admin/categories')
}
</script>

<style scoped></style>