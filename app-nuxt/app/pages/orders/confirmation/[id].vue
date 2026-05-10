<template>
  <div>
    <div v-if="pending" class="flex items-center justify-center h-96">
      <span class="loading loading-spinner text-info size-40" />
    </div>

    <div v-else-if="!order" class="flex flex-col items-center justify-center py-20">
      <span class="icon-[tabler--alert-circle] mb-4 size-20 text-error opacity-60" />
      <p class="text-lg">{{ t('admin.statusLabels.unknown') }}</p>
      <NuxtLink to="/orders" class="btn btn-primary mt-4">
        {{ t('pages.orders.title') }}
      </NuxtLink>
    </div>

    <div v-else class="mx-auto max-w-2xl py-10 text-center">
      <span class="icon-[tabler--circle-check] mx-auto mb-6 size-20 text-success" />
      <h1 class="h1 mb-2">{{ t('pages.confirmation.title') }}</h1>
      <p class="mb-8 text-lg text-base-content/60">{{ t('pages.confirmation.success') }}</p>

      <div class="rounded-box border p-6 text-left">
        <div class="flex justify-between py-2">
          <span class="text-base-content/60">{{ t('pages.confirmation.number') }}</span>
          <span class="font-semibold">{{ order.order_number || '-' }}</span>
        </div>
        <div class="flex justify-between py-2">
          <span class="text-base-content/60">{{ t('order.date') }}</span>
          <span>{{ formatDate(order.created_at) }}</span>
        </div>
        <div class="flex justify-between py-2">
          <span class="text-base-content/60">{{ t('pages.orders.status') }}</span>
          <span class="badge badge-soft badge-primary">{{ t('order.status.pending') }}</span>
        </div>
        <div class="flex justify-between border-t pt-4 mt-2 text-lg font-bold">
          <span>{{ t('order.total') }}</span>
          <span class="text-primary">{{ formatNumberBR(order.total_amount) }}</span>
        </div>
      </div>

      <p class="mt-6 text-sm text-base-content/40">{{ t('pages.confirmation.emailSent') }}</p>

      <div class="mt-8 flex justify-center gap-4">
        <NuxtLink to="/orders" class="btn btn-primary">
          {{ t('pages.orders.title') }}
        </NuxtLink>
        <NuxtLink to="/products" class="btn btn-ghost">
          {{ t('cart.continueShopping') }}
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
const config = useRuntimeConfig()
const route = useRoute()
import type { Order } from '~/types'

const id = route.params.id
const { data: order, pending } = await useFetch<Order>(
  `${config.public.baseURL}/api/orders/${id}`,
  { key: `confirmation-${id}` }
)
</script>
