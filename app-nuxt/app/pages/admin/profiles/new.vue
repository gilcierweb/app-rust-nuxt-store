<template>
  <div>
    <AdminProfilesForm @saved="handleSaved" @cancel="navigateTo('/admin/profiles')" />
  </div>
</template>

<script setup lang="ts">
import type { Profile } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (profile: Profile) => {
  navigateTo('/admin/profiles')
}
</script>

<style scoped></style>
