<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/shipments" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6" />
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">{{ t('shipping.shipmentDetails') }}</h1>
          <p class="text-sm text-base-content/60" v-if="shipment">#{{ shipment.id }}</p>
        </div>
      </div>
      <div v-if="shipment" class="flex gap-2">
        <button class="btn btn-outline" @click="downloadLabel" :disabled="isLabelLoading">
          <span v-if="isLabelLoading" class="loading loading-spinner loading-xs"></span>
          <i v-else class="icon-[tabler--file-download] size-4"></i>
          {{ $t('admin.shipments.detail.downloadLabel', 'Shipping Label') }}
        </button>
        <button @click="deleteShipment" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2" /> {{ t('common.delete') }}
        </button>
        <NuxtLinkLocale :to="`/admin/shipments/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2" /> {{ t('common.edit') }}
        </NuxtLinkLocale>
      </div>
    </div>

    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12" />
      <span class="mt-4 text-base-content/60">{{ t('common.loading') }}</span>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span>{{ t('common.error') }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">{{ t('common.actions.tryAgain') }}</button>
    </div>

    <div v-else-if="shipment" class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <h2 class="card-title mb-4">{{ t('shipping.shipment') }} #{{ shipment.id }}</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="label"><span class="label-text text-base-content/60">{{ t('shipping.order') }}</span></label>
              <NuxtLinkLocale :to="`/admin/orders/${shipment.order_id}`" class="link link-hover font-mono text-lg">
                #{{ shipment.order_id }}
              </NuxtLinkLocale>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text text-base-content/60">{{ t('shipping.carrier') }}</span></label>
              <div class="font-medium">{{ shipment.carrier || '-' }}</div>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text text-base-content/60">{{ t('shipping.trackingNumber') }}</span></label>
              <div class="font-mono">{{ shipment.tracking_number || '-' }}</div>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text text-base-content/60">{{ t('pages.orders.status') }}</span></label>
              <span :class="statusBadgeClass(shipment.status)">{{ statusLabel(shipment.status) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title mb-4">{{ t('common.table.date') }}</h2>
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-base-content/60">{{ t('common.table.date') }}</span>
              <span>{{ formatDate(shipment.created_at) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="alert alert-warning">
      <span>{{ t('admin.statusLabels.unknown') }}</span>
      <NuxtLinkLocale to="/admin/shipments" class="btn btn-sm">{{ t('common.back') }}</NuxtLinkLocale>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Shipment } from '~/types'

definePageMeta({ layout: 'admin' })

const { t } = useI18n()
const route = useRoute()
const { apiFetch, useApiFetch } = useApi()
const router = useRouter()
const toast = useAppToast()
const dialog = useAppDialog()

const { pending, data: shipment, error, refresh } = await useApiFetch<Shipment>(`/api/admin/shipments/${route.params.id}`)

const isLabelLoading = ref(false)

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

const formatDate = (dateString?: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat(getAppLocale(), { day: '2-digit', month: 'long', year: 'numeric', hour: '2-digit', minute: '2-digit' }).format(new Date(dateString))
}

const deleteShipment = async () => {
  if (!shipment.value) return
  const confirmed = await dialog.confirm({
    message: t('admin.shipments.detail.confirmDelete', { id: shipment.value.id }),
    confirmLabel: t('common.delete'),
    tone: 'danger'
  })
  if (!confirmed) return

  try {
    await apiFetch(`/api/admin/shipments/${shipment.value.id}`, { method: 'DELETE' })
    router.push('/admin/shipments')
  } catch (err) {
    toast.error(t('admin.shipments.detail.errorDelete'))
  }
}

const downloadLabel = async () => {
  if (!shipment.value) return
  isLabelLoading.value = true
  try {
    const blob = await apiFetch(`/api/admin/shipments/${shipment.value.id}/label`, {
      responseType: 'blob'
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `shipping-label-${shipment.value.id}.pdf`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (e: any) {
    toast.error(`Download failed: ${e.message}`)
  } finally {
    isLabelLoading.value = false
  }
}
</script>
