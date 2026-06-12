<template>
  <div class="card shadow-base-300/10 shadow-md">
    <div class="card-body p-0">
      <div class="p-4 border-b border-base-200 flex items-center justify-between">
        <h3 class="font-bold">{{ t('admin.inventory.reservations.title') }}</h3>
        <div class="flex gap-2">
          <button class="btn btn-ghost btn-sm" type="button" :disabled="loading" @click="loadReservations">
            <i class="icon-[tabler--refresh] size-4"></i>
          </button>
          <button
            class="btn btn-warning btn-sm"
            type="button"
            :disabled="expiring"
            @click="handleExpireAll"
          >
            <span v-if="expiring" class="loading loading-spinner loading-xs mr-1"></span>
            <i v-else class="icon-[tabler--clock-off] size-4 mr-1"></i>
            {{ t('admin.inventory.reservations.expireAll') }}
          </button>
        </div>
      </div>

      <div v-if="loading" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary"></span>
      </div>

      <div v-else-if="reservations.length === 0" class="py-12 text-center text-base-content/50">
        <i class="icon-[tabler--lock-open] size-12 opacity-20 mb-2 block mx-auto"></i>
        <p>{{ t('admin.inventory.reservations.empty') }}</p>
      </div>

      <div v-else class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>{{ t('admin.inventory.reservations.date') }}</th>
              <th>{{ t('admin.inventory.reservations.item') }}</th>
              <th class="text-right">{{ t('admin.inventory.reservations.quantity') }}</th>
              <th>{{ t('admin.inventory.reservations.status') }}</th>
              <th>{{ t('admin.inventory.reservations.expires') }}</th>
              <th>{{ t('admin.inventory.reservations.context') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="res in reservations" :key="res.id">
              <td class="text-sm">{{ formatDate(res.created_at) }}</td>
              <td>
                <div class="text-sm font-bold">{{ res.product_name || `#${res.variant_id}` }}</div>
                <div class="text-xs text-base-content/50">{{ res.variant_name || res.sku }}</div>
              </td>
              <td class="text-right font-mono font-bold">{{ res.quantity }}</td>
              <td>
                <span :class="['badge badge-sm badge-soft', statusBadgeClass(res.status)]">
                  {{ t(`admin.inventory.reservations.statuses.${res.status}`) }}
                </span>
              </td>
              <td class="text-sm">
                <span v-if="res.expires_at" :class="isExpired(res.expires_at) ? 'text-error' : 'text-base-content/60'">
                  {{ formatDate(res.expires_at) }}
                </span>
                <span v-else class="text-base-content/40">-</span>
              </td>
              <td class="text-sm text-base-content/60">
                <span v-if="res.order_id">Order #{{ res.order_id }}</span>
                <span v-else-if="res.cart_id">Cart #{{ res.cart_id }}</span>
                <span v-else>-</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { StockReservation } from '~/types'

const { t } = useI18n()
const { getReservations, expireReservations } = useInventory()

const loading = ref(false)
const expiring = ref(false)
const reservations = ref<StockReservation[]>([])

function statusBadgeClass(status: string) {
  switch (status) {
    case 'active': return 'badge-info'
    case 'expired': return 'badge-warning'
    case 'released': return 'badge-ghost'
    case 'converted': return 'badge-success'
    default: return 'badge-ghost'
  }
}

function isExpired(dateStr: string) {
  return new Date(dateStr) < new Date()
}

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString(getAppLocale(), {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

async function loadReservations() {
  loading.value = true
  try {
    const result = await getReservations({ status: 'active', page: 1, page_size: 50 })
    reservations.value = result.items || []
  } catch {
    reservations.value = []
  } finally {
    loading.value = false
  }
}

async function handleExpireAll() {
  expiring.value = true
  try {
    await expireReservations()
    await loadReservations()
  } catch {
    // ignore
  } finally {
    expiring.value = false
  }
}

onMounted(() => loadReservations())
</script>
