<template>
  <div>
    <AdminBannersForm
      :positions="positions || []"
      @saved="handleSaved"
      @cancel="navigateTo('/admin/banners')"
    />
  </div>
</template>

<script setup lang="ts">
import type { Banner, BannerPosition } from '~/types'

definePageMeta({
  layout: 'admin'
})

const { useApiFetch } = useApi()

const { data: positions } = await useApiFetch<BannerPosition[]>('/api/admin/banners/positions', {
  key: 'admin-banner-positions-form-new'
})

function handleSaved(_banner: Banner) {
  navigateTo('/admin/banners')
}
</script>
