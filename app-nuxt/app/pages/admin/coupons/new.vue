<template>
  <div>
    <AdminCouponsForm @saved="handleSaved" @cancel="navigateTo('/admin/coupons')" />
  </div>
</template>

<script setup lang="ts">
import type { Coupon } from '~/types'

definePageMeta({
  layout: 'admin'
})

const handleSaved = (coupon: Coupon) => {
  navigateTo('/admin/coupons')
}
</script>

<style scoped></style>
