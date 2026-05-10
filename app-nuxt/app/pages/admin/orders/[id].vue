<template>
  <div>
    <div v-if="pending" class="flex items-center justify-center h-96">
      <span class="loading loading-spinner text-info size-40" />
    </div>

    <div v-else-if="!order" class="flex flex-col items-center justify-center py-20">
      <p class="text-lg text-base-content/60">Order not found</p>
      <NuxtLink to="/admin/orders" class="btn btn-primary mt-4">
        {{ t('pages.orders.backToOrders') }}
      </NuxtLink>
    </div>

    <div v-else>
      <div class="mb-6 flex items-center justify-between">
        <div>
          <NuxtLink to="/admin/orders" class="link link-hover text-sm text-base-content/60">
            &larr; {{ t('pages.orders.backToOrders') }}
          </NuxtLink>
          <h1 class="h1 mt-1">{{ t('pages.orders.detail') }} - {{ order.order_number }}</h1>
        </div>
        <span :class="statusBadgeClass(order.status)" class="text-sm">
          {{ statusLabel(order.status) }}
        </span>
      </div>

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <div class="lg:col-span-2 space-y-6">
          <div class="rounded-box border p-6">
            <h3 class="mb-4 font-semibold">{{ t('pages.orders.items') }}</h3>
            <table class="table w-full">
              <thead>
                <tr>
                  <th>{{ t('pages.products.table.name') }}</th>
                  <th>{{ t('cart.quantity') }}</th>
                  <th class="text-right">{{ t('pages.products.table.price') }}</th>
                  <th class="text-right">{{ t('pages.orders.total') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in order.items" :key="item.id">
                  <td>{{ item.product_name || `Product #${item.product_id}` }}</td>
                  <td>{{ item.quantity }}</td>
                  <td class="text-right">{{ formatNumberBR(item.price) }}</td>
                  <td class="text-right font-semibold">{{ formatNumberBR(item.total) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div class="space-y-6">
          <div class="rounded-box border p-6">
            <h3 class="mb-4 font-semibold">{{ t('order.updateStatus') }}</h3>
            <select v-model="selectedStatus" class="select w-full">
              <option value="" disabled>{{ t('order.selectStatus') }}</option>
              <option v-for="s in availableStatuses" :key="s.value" :value="s.value">
                {{ s.label }}
              </option>
            </select>
            <button class="btn btn-primary mt-3 w-full" :disabled="!selectedStatus || updating" @click="updateStatus">
              <span v-if="updating" class="loading loading-spinner" />
              {{ t('admin.products.edit') }}
            </button>
            <p v-if="statusMsg" class="mt-2 text-center text-sm" :class="statusMsgType">{{ statusMsg }}</p>
          </div>

          <div class="rounded-box border p-6">
            <h3 class="mb-4 font-semibold">{{ t('pages.orders.status') }}</h3>
            <div class="space-y-2 text-sm">
              <div class="flex justify-between">
                <span class="text-base-content/60">{{ t('pages.orders.status') }}</span>
                <span :class="statusBadgeClass(order.status)">{{ statusLabel(order.status) }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">{{ t('order.paymentStatus.paid') }}</span>
                <span :class="paymentBadgeClass(order.payment_status)">
                  {{ paymentLabel(order.payment_status) }}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-base-content/60">{{ t('pages.orders.date') }}</span>
                <span>{{ new Date(order.created_at).toLocaleDateString() }}</span>
              </div>
              <div class="flex justify-between border-t pt-2 mt-2 text-base font-bold">
                <span>{{ t('pages.orders.total') }}</span>
                <span class="text-primary">{{ formatNumberBR(order.total_amount) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })
const { t } = useI18n()
const config = useRuntimeConfig()
const route = useRoute()
import type { Order } from '~/types'

const id = route.params.id
const { data: order, pending } = await useFetch<Order>(
  `${config.public.baseURL}/api/orders/${id}`
)
const selectedStatus = ref('')
const updating = ref(false)
const statusMsg = ref('')
const statusMsgType = ref('')

const statusMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.status.pending'), badge: 'badge-soft badge-warning' },
  2: { label: t('order.status.confirmed'), badge: 'badge-soft badge-info' },
  3: { label: t('order.status.processing'), badge: 'badge-soft badge-info' },
  4: { label: t('order.status.shipped'), badge: 'badge-soft badge-primary' },
  5: { label: t('order.status.delivered'), badge: 'badge-soft badge-success' },
  6: { label: t('order.status.cancelled'), badge: 'badge-soft badge-error' },
}

const paymentMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.paymentStatus.unpaid'), badge: 'badge-soft badge-error' },
  2: { label: t('order.paymentStatus.paid'), badge: 'badge-soft badge-success' },
  3: { label: t('order.paymentStatus.refunded'), badge: 'badge-soft badge-info' },
  4: { label: t('order.paymentStatus.partiallyRefunded'), badge: 'badge-soft badge-warning' },
}

const availableStatuses = computed(() => {
  const current = order.value?.status ?? 1
  const transitions: Record<number, number[]> = {
    1: [2, 6], 2: [3, 6], 3: [4, 6], 4: [5], 5: [], 6: [],
  }
  return (transitions[current] ?? []).map(v => ({ value: v, label: statusMap[v]?.label ?? '' }))
})

function statusLabel(status: number): string {
  return statusMap[status]?.label ?? t('admin.statusLabels.unknown')
}
function statusBadgeClass(status: number): string {
  return statusMap[status]?.badge ?? 'badge-soft'
}
function paymentLabel(status: number): string {
  return paymentMap[status]?.label ?? t('admin.statusLabels.unknown')
}
function paymentBadgeClass(status: number): string {
  return paymentMap[status]?.badge ?? 'badge-soft'
}

async function updateStatus() {
  if (!selectedStatus.value || !order.value) return
  updating.value = true
  statusMsg.value = ''
  try {
    await $fetch(`${config.public.baseURL}/api/orders/${order.value.id}/status`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: { status: Number(selectedStatus.value) },
    })
    order.value.status = Number(selectedStatus.value)
    statusMsg.value = 'Status updated successfully'
    statusMsgType.value = 'text-success'
    selectedStatus.value = ''
  } catch (err: any) {
    statusMsg.value = err?.data?.message || err?.message || 'Error updating status'
    statusMsgType.value = 'text-error'
  } finally {
    updating.value = false
  }
}
</script>
