<template>
  <div>
    <AdminAddressesForm 
      :initial-user-id="route.query.user_id ? Number(route.query.user_id) : undefined"
      @saved="handleSaved" 
      @cancel="handleCancel" 
    />
  </div>
</template>

<script setup lang="ts">
import type { Address } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()

const handleSaved = (address: Address) => {
  const returnTo = route.query.return_to as string
  navigateTo(returnTo || '/admin/addresses')
}

const handleCancel = () => {
  const returnTo = route.query.return_to as string
  navigateTo(returnTo || '/admin/addresses')
}
</script>

<style scoped></style>
