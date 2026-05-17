<template>
  <div>
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ t('admin.banners.detail.loading') }}</span>
    </div>

    <div v-else-if="error" class="max-w-2xl mx-auto">
      <div class="alert alert-error">
        <i class="icon-[tabler--alert-circle] size-6"></i>
        <span>{{ t('admin.banners.error', { message: error.message }) }}</span>
        <NuxtLinkLocale to="/admin/banners" class="btn btn-sm">{{ t('common.back') }}</NuxtLinkLocale>
      </div>
    </div>

    <AdminBannersForm
      v-else-if="banner"
      :banner="banner"
      :positions="positions || []"
      :is-editing="true"
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

const route = useRoute()
const { t } = useI18n()
const { useApiFetch } = useApi()

const [
  { data: banner, pending, error },
  { data: positions }
] = await Promise.all([
  useApiFetch<Banner>(`/api/admin/banners/${route.params.id}`, {
    key: `admin-banner-edit-${route.params.id}`
  }),
  useApiFetch<BannerPosition[]>('/api/admin/banners/positions', {
    key: 'admin-banner-positions-form-edit'
  })
])

function handleSaved(_banner: Banner) {
  navigateTo('/admin/banners')
}
</script>
