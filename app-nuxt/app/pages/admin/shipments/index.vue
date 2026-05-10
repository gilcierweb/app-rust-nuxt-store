<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <h1 class="h1">{{ t('shipping.list') }}</h1>
      <NuxtLink to="/admin/shipments/new" class="btn btn-success">{{ t('common.actions.add') }}</NuxtLink>
    </div>

    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12" />
      <span class="ml-3">{{ t('common.loading') }}</span>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span>{{ t('common.errorLoad', { resource: t('shipping.list') }) }}</span>
    </div>

    <div v-else-if="shipments.length === 0" class="text-center py-12">
      <p class="text-base-content/60 text-lg">{{ t('shipping.noShipments') }}</p>
      <NuxtLink to="/admin/shipments/new" class="btn btn-primary mt-4">{{ t('shipping.createShipment') }}</NuxtLink>
    </div>

    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>#</th>
            <th>{{ t('shipping.order') }}</th>
            <th>{{ t('shipping.trackingNumber') }}</th>
            <th>{{ t('shipping.carrier') }}</th>
            <th>{{ t('pages.orders.status') }}</th>
            <th>{{ t('common.table.date') }}</th>
            <th>{{ t('common.table.actions') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="shipment in shipments" :key="shipment.id" class="row-hover">
            <td class="font-mono text-sm">{{ shipment.id }}</td>
            <td>
              <NuxtLink :to="`/admin/orders/${shipment.order_id}`" class="link link-hover font-mono text-sm">
                #{{ shipment.order_id }}
              </NuxtLink>
            </td>
            <td>{{ shipment.tracking_number || '-' }}</td>
            <td>{{ shipment.carrier || '-' }}</td>
            <td>
              <span :class="statusBadgeClass(shipment.status)">
                {{ statusLabel(shipment.status) }}
              </span>
            </td>
            <td>{{ formatDate(shipment.created_at) }}</td>
            <td>
              <NuxtLink :to="`/admin/shipments/${shipment.id}`" class="btn btn-circle btn-text btn-sm" :aria-label="t('common.actions.show')">
                <i class="icon-[tabler--eye] size-5" />
              </NuxtLink>
              <NuxtLink :to="`/admin/shipments/${shipment.id}/edit`" class="btn btn-circle btn-text btn-sm" :aria-label="t('common.actions.edit')">
                <i class="icon-[tabler--pencil] size-5" />
              </NuxtLink>
              <button type="button" class="btn btn-circle btn-text btn-sm" :aria-label="t('common.actions.delete')" @click="confirmDelete(shipment)">
                <span class="icon-[tabler--trash] size-5" />
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Shipment } from '~/types'

definePageMeta({ layout: 'admin' })

const { t } = useI18n()
const config = useRuntimeConfig()

const { pending, data: shipments, error, refresh } = useLazyFetch<Shipment[]>(`${config.public.baseURL}/api/shipments`)

const shipmentStatusMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('shipping.status.pending'), badge: 'badge-soft badge-warning' },
  2: { label: t('shipping.status.shipped'), badge: 'badge-soft badge-info' },
  3: { label: t('shipping.status.delivered'), badge: 'badge-soft badge-success' },
  4: { label: t('shipping.status.cancelled'), badge: 'badge-soft badge-error' },
}

function statusLabel(status: unknown): string {
  if (status == null) return '-'
  return shipmentStatusMap[status as number]?.label ?? t('admin.statusLabels.unknown')
}

function statusBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return shipmentStatusMap[status as number]?.badge ?? 'badge-soft'
}

const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', { day: '2-digit', month: 'short', year: 'numeric' }).format(new Date(dateString))
}

const confirmDelete = async (shipment: Shipment) => {
  if (confirm(`Delete shipment #${shipment.id}?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/shipments/${shipment.id}`, { method: 'DELETE' })
      await refresh()
    } catch {
      alert(t('common.error'))
    }
  }
}
</script>
