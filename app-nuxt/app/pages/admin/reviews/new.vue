<template>
  <div>
    <AdminReviewsForm @saved="handleSaved" @cancel="navigateTo('/admin/reviews')" />
  </div>
</template>

<script setup lang="ts">
import type { Review } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (review: Review) => {
  navigateTo('/admin/reviews')
}
</script>

<style scoped></style>
