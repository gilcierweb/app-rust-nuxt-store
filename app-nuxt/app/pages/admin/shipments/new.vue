<template>
  <div>
    <AdminShipmentsForm @saved="handleSaved" @cancel="navigateTo('/admin/shipments')" />
  </div>
</template>

<script setup lang="ts">
import type { Shipment } from '~/types'

definePageMeta({ layout: 'admin' })

const handleSaved = (_shipment: Shipment) => {
  navigateTo('/admin/shipments')
}
</script>
