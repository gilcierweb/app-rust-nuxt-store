<template>
  <div>
    <h1 class="h1 my-4">{{ t('pages.orders.title') }}</h1>

    <div v-if="pending" class="flex items-center justify-center h-96">
      <span class="loading loading-spinner text-info size-40" />
    </div>

    <div v-else-if="orders.length === 0" class="flex flex-col items-center justify-center py-20">
      <span class="icon-[tabler--package] mb-4 size-20 opacity-40" />
      <p class="mb-6 text-lg text-base-content/60">{{ t('pages.orders.empty') }}</p>
      <NuxtLink to="/products" class="btn btn-primary">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <div v-else class="overflow-x-auto rounded-box border">
      <table class="table">
        <thead>
          <tr>
            <th>{{ t('pages.orders.number') }}</th>
            <th>{{ t('pages.orders.date') }}</th>
            <th>{{ t('pages.orders.status') }}</th>
            <th class="text-right">{{ t('pages.orders.total') }}</th>
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
            <td class="text-right font-semibold">{{ formatNumberBR(order.total_amount) }}</td>
            <td>
              <NuxtLink :to="`/orders/confirmation/${order.id}`" class="btn btn-ghost btn-sm">
                {{ t('order.details') }}
              </NuxtLink>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
const config = useRuntimeConfig()
import type { Order } from '~/types'

const { data: ordersData, pending } = await useFetch<Order[]>(
  `${config.public.baseURL}/api/orders/my_orders`
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

function statusLabel(status: number): string {
  return statusMap[status]?.label ?? t('admin.statusLabels.unknown')
}

function statusBadgeClass(status: number): string {
  return statusMap[status]?.badge ?? 'badge-soft'
}
</script>
