<template>
  <div>
    <AdminProfilesForm @saved="handleSaved" @cancel="navigateTo('/admin/customers')" />
  </div>
</template>

<script setup lang="ts">
import type { Profile } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (profile: Profile) => {
  navigateTo('/admin/customers')
}
</script>

<style scoped></style>
