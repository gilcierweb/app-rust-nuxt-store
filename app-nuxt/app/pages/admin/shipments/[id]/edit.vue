<template>
  <div>
    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-base-content/60">{{ t('common.loading') }}</p>
    </div>

    <div v-else-if="error" class="max-w-2xl mx-auto">
      <div class="alert alert-error">
        <span class="icon-[tabler--alert-circle] size-6"></span>
        <span>{{ t('common.errorLoad', { resource: t('shipping.shipment') }) }}</span>
        <NuxtLinkLocale to="/admin/shipments" class="btn btn-sm btn-ghost">
          {{ t('common.back') }}
        </NuxtLinkLocale>
      </div>
    </div>

    <AdminShipmentsForm
      v-else-if="shipment"
      :shipment="shipment"
      :is-editing="true"
      @saved="handleSaved"
      @cancel="navigateTo('/admin/shipments')"
    />
  </div>
</template>

<script setup lang="ts">
import type { Shipment } from '~/types'

definePageMeta({ layout: 'admin' })

const { t } = useI18n()
const route = useRoute()
const { useApiFetch } = useApi()

const { data: shipment, pending, error } = await useApiFetch<Shipment>(
  `/api/admin/shipments/${route.params.id}`,
  { key: `admin-shipment-${route.params.id}` }
)

const handleSaved = (_shipment: Shipment) => {
  navigateTo('/admin/shipments')
}
</script>
