<template>
  <div>
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12" />
      <span class="ml-3">{{ t('common.loading') }}</span>
    </div>

    <div v-else-if="error" class="max-w-2xl mx-auto">
      <div class="alert alert-error">
        <span>{{ t('common.errorLoad', { resource: t('shipping.shipment') }) }}</span>
        <NuxtLinkLocale to="/admin/shipments" class="btn btn-sm">{{ t('common.back') }}</NuxtLinkLocale>
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

const route = useRoute()
const config = useRuntimeConfig()

const { data: shipment, pending, error } = useLazyFetch<Shipment>(`${config.public.baseURL}/api/shipments/${route.params.id}`)

const handleSaved = (_shipment: Shipment) => {
  navigateTo('/admin/shipments')
}
</script>
