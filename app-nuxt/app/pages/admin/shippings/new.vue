<template>
  <div>
    <AdminShippingsForm @saved="handleSaved" @cancel="navigateTo('/admin/shippings')" />
  </div>
</template>

<script setup lang="ts">
import type { ShippingMethod } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (shipping: ShippingMethod) => {
  navigateTo('/admin/shippings')
}
</script>

<style scoped></style>
