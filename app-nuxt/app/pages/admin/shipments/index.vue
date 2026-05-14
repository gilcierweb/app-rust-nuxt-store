<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <h1 class="h1">{{ $t('admin.shipments.title') }}</h1>
      <NuxtLinkLocale to="/admin/shipments/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.shipments.add') }}
      </NuxtLinkLocale>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12" />
      <p class="mt-4 text-gray-500">{{ $t('admin.shipments.loading') }}</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error mb-6">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.shipments.error', { message: error.message }) }}</span>
    </div>

    <!-- Shipments Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th class="w-16">ID</th>
              <th>{{ $t('admin.shipments.table.order') }}</th>
              <th>{{ $t('admin.shipments.table.tracking') }}</th>
              <th>{{ $t('admin.shipments.table.carrier') }}</th>
              <th>{{ $t('admin.shipments.table.status') }}</th>
              <th>{{ $t('admin.shipments.table.date') }}</th>
              <th class="text-right">{{ $t('admin.shipments.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="shipment in shipments" :key="shipment.id" class="row-hover">
              <td><span class="font-mono text-sm">#{{ shipment.id }}</span></td>
              <td>
                <NuxtLinkLocale :to="`/admin/orders/${shipment.order_id}`" class="link link-hover font-bold text-primary font-mono text-sm">
                  #{{ shipment.order_id }}
                </NuxtLinkLocale>
              </td>
              <td><span class="text-xs font-mono bg-base-200 px-2 py-1 rounded">{{ shipment.tracking_number || '-' }}</span></td>
              <td>{{ shipment.carrier || '-' }}</td>
              <td>
                <span :class="['badge badge-soft text-xs', statusBadgeClass(shipment.status)]">
                  {{ statusLabel(shipment.status) }}
                </span>
              </td>
              <td><div class="text-sm">{{ formatDate(shipment.created_at) }}</div></td>
              <td class="text-right">
                <div class="flex justify-end gap-1">
                  <NuxtLinkLocale :to="`/admin/shipments/${shipment.id}`" class="btn btn-circle btn-text btn-sm" :aria-label="$t('common.view')">
                    <i class="icon-[tabler--eye] size-5" />
                  </NuxtLinkLocale>
                  <NuxtLinkLocale :to="`/admin/shipments/${shipment.id}/edit`" class="btn btn-circle btn-text btn-sm" :aria-label="$t('common.edit')">
                    <i class="icon-[tabler--pencil] size-5" />
                  </NuxtLinkLocale>
                  <button type="button" class="btn btn-circle btn-text btn-sm text-error" :aria-label="$t('common.delete')" @click="confirmDelete(shipment)">
                    <i class="icon-[tabler--trash] size-5" />
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="shipments.length === 0">
              <td colspan="7" class="text-center py-20 text-gray-500 italic">
                {{ $t('admin.shipments.notFound') }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Shipment } from '~/types'

definePageMeta({ layout: 'admin' })

const { t } = useI18n()
const config = useRuntimeConfig()

const { pending, data: shipments, error, refresh } = useLazyFetch<Shipment[]>(`${config.public.baseURL}/api/admin/shipments`)

const shipmentStatusMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('shipping.status.pending'), badge: 'badge-warning' },
  2: { label: t('shipping.status.shipped'), badge: 'badge-info' },
  3: { label: t('shipping.status.delivered'), badge: 'badge-success' },
  4: { label: t('shipping.status.cancelled'), badge: 'badge-error' },
}

function statusLabel(status: unknown): string {
  if (status == null) return '-'
  return shipmentStatusMap[status as number]?.label ?? t('common.status.unknown')
}

function statusBadgeClass(status: unknown): string {
  if (status == null) return ''
  return shipmentStatusMap[status as number]?.badge ?? ''
}

const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', { day: '2-digit', month: 'short', year: 'numeric' }).format(new Date(dateString))
}

const confirmDelete = async (shipment: Shipment) => {
  if (confirm(t('admin.shipments.detail.confirmDelete', { id: shipment.id }))) {
    try {
      await $fetch(`${config.public.baseURL}/api/admin/shipments/${shipment.id}`, { method: 'DELETE' })
      await refresh()
    } catch {
      alert(t('admin.shipments.detail.errorDelete'))
    }
  }
}
</script>
