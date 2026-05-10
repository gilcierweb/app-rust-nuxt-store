<template>
  <div>
    <AdminPostsForm @saved="handleSaved" @cancel="navigateTo('/admin/posts')" />
  </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (post: Post) => {
  navigateTo('/admin/posts')
}
</script>

<style scoped></style>
