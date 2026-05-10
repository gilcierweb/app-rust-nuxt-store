<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">{{ t('order.adminTitle') }}</h1>
    </div>

    <div v-if="pending" class="flex items-center justify-center h-96">
      <span class="loading loading-spinner text-info size-40" />
    </div>

    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>{{ t('pages.orders.number') }}</th>
            <th>{{ t('pages.orders.date') }}</th>
            <th>{{ t('pages.orders.status') }}</th>
            <th>{{ t('order.paymentStatus.paid') }}</th>
            <th class="text-right">{{ t('pages.orders.total') }}</th>
            <th>{{ t('pages.orders.items') }}</th>
            <th />
          </tr>
        </thead>
        <tbody>
          <tr v-for="order in orders" :key="order.id" class="row-hover">
            <td class="font-mono text-sm">{{ order.order_number }}</td>
            <td>{{ new Date(order.created_at).toLocaleDateString() }}</td>
            <td>
              <span :class="statusBadgeClass(order.status)">
                {{ statusLabel(order.status) }}
              </span>
            </td>
            <td>
              <span :class="paymentBadgeClass(order.payment_status)">
                {{ paymentLabel(order.payment_status) }}
              </span>
            </td>
            <td class="text-right font-semibold">{{ formatNumberBR(order.total_amount) }}</td>
            <td>{{ order.items?.length ?? '-' }}</td>
            <td>
              <NuxtLink :to="`/admin/orders/${order.id}`" class="btn btn-circle btn-text btn-sm">
                <i class="icon-[tabler--eye] size-5" />
              </NuxtLink>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })
const { t } = useI18n()
const config = useRuntimeConfig()
import type { Order } from '~/types'

const { data: ordersData, pending } = await useFetch<Order[]>(
  `${config.public.baseURL}/api/orders/list`
)
const orders = computed(() => ordersData.value ?? [])

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
</script>
