<template>
  <div>
    <AdminAddressesForm @saved="handleSaved" @cancel="navigateTo('/admin/addresses')" />
  </div>
</template>

<script setup lang="ts">
import type { Address } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (address: Address) => {
  navigateTo('/admin/addresses')
}
</script>

<style scoped></style>
