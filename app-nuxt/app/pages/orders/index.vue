<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12 pt-10">
      <div>
        <h1 class="h2 gradient-text mb-2">{{ t('pages.orders.title') }}</h1>
        <p class="text-base-content/60">Track your recent purchases and order history.</p>
      </div>
      <div class="flex items-center gap-3">
        <button class="btn btn-ghost btn-square rounded-2xl bg-base-200/50">
          <span class="icon-[tabler--filter] size-5"></span>
        </button>
        <button class="btn btn-ghost btn-square rounded-2xl bg-base-200/50">
          <span class="icon-[tabler--refresh] size-5"></span>
        </button>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-32">
      <div class="relative">
        <div class="size-20 rounded-3xl border-4 border-primary/20 animate-pulse"></div>
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="loading loading-ring loading-lg text-primary"></span>
        </div>
      </div>
      <p class="mt-6 text-base-content/40 font-medium tracking-widest uppercase text-xs">Syncing your orders...</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="orders.length === 0" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="size-24 rounded-full bg-base-200 flex items-center justify-center mb-6">
        <span class="icon-[tabler--package-off] size-12 opacity-20" />
      </div>
      <h2 class="h3 mb-2">{{ t('pages.orders.empty') }}</h2>
      <p class="mb-8 text-base-content/50 max-w-sm text-center">
        You haven't placed any orders yet. Start shopping to see your history here!
      </p>
      <NuxtLink to="/products" class="btn btn-primary btn-lg px-10 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-105">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <!-- Orders List -->
    <div v-else class="space-y-6">
      <div v-for="order in orders" :key="order.id" 
        class="group bg-base-100 rounded-[2.5rem] border border-base-200 overflow-hidden hover:shadow-xl hover:shadow-primary/5 transition-all duration-500 hover:border-primary/20">
        <div class="p-8 md:p-10 flex flex-col lg:flex-row lg:items-center gap-8">
          
          <!-- Order Info -->
          <div class="lg:w-1/4 space-y-2">
            <div class="flex items-center gap-2 mb-4">
              <span class="icon-[tabler--hash] text-primary size-5"></span>
              <span class="font-black text-xl tracking-tight">{{ order.order_number || '-' }}</span>
            </div>
            <p class="text-sm text-base-content/40 flex items-center gap-2">
              <span class="icon-[tabler--calendar] size-4"></span>
              {{ formatDate(order.created_at) }}
            </p>
            <p class="text-sm text-base-content/40 flex items-center gap-2">
              <span class="icon-[tabler--package] size-4"></span>
              {{ order.items?.length || 0 }} Items
            </p>
          </div>
          
          <!-- Status Badges -->
          <div class="lg:w-1/4 flex flex-row lg:flex-col gap-3">
            <div class="flex flex-col gap-1">
              <span class="text-[10px] uppercase tracking-widest text-base-content/30 font-bold">Order Status</span>
              <div :class="['badge badge-lg rounded-xl font-bold py-5 px-6', statusBadgeClass(order.status)]">
                {{ statusLabel(order.status) }}
              </div>
            </div>
          </div>
          
          <div class="lg:w-1/4 flex flex-row lg:flex-col gap-3">
            <div class="flex flex-col gap-1">
              <span class="text-[10px] uppercase tracking-widest text-base-content/30 font-bold">Payment Status</span>
              <div :class="['badge badge-lg rounded-xl font-bold py-5 px-6', paymentBadgeClass(order.payment_status)]">
                {{ paymentLabel(order.payment_status) }}
              </div>
            </div>
          </div>
          
          <!-- Total & Actions -->
          <div class="lg:w-1/4 flex lg:flex-col lg:items-end justify-between items-center border-t lg:border-t-0 pt-6 lg:pt-0 border-base-200">
            <div class="text-right lg:mb-6">
              <span class="block text-[10px] uppercase tracking-widest text-base-content/30 font-bold">Total Amount</span>
              <span class="text-3xl font-black text-primary">{{ formatNumberBR(order.total_amount) }}</span>
            </div>
            <NuxtLink :to="`/orders/confirmation/${order.id}`" class="btn btn-ghost hover:bg-primary/10 hover:text-primary rounded-xl px-6 group-hover:translate-x-1 transition-all">
              {{ t('order.details') }}
              <span class="icon-[tabler--arrow-right] size-4 ml-1"></span>
            </NuxtLink>
          </div>
        </div>
        
        <!-- Quick Preview of items (optional) -->
        <div v-if="order.items && order.items.length > 0" class="px-10 py-6 bg-base-200/20 border-t border-base-200 flex items-center gap-4 overflow-x-auto no-scrollbar">
          <div v-for="item in order.items.slice(0, 5)" :key="item.id" class="size-12 rounded-xl bg-white border border-base-200 shrink-0 overflow-hidden">
             <!-- If we had item images in the order, we'd show them here -->
             <div class="flex items-center justify-center h-full text-base-content/10">
               <span class="icon-[tabler--photo] size-6"></span>
             </div>
          </div>
          <span v-if="order.items.length > 5" class="text-xs font-bold text-base-content/30">+{{ order.items.length - 5 }} more</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.orders.title'),
  ogTitle: t('pages.orders.title'),
})
const config = useRuntimeConfig()
import type { Order } from '~/types'

const { data: ordersData, pending } = await useFetch<Order[]>(
  `${config.public.baseURL}/api/orders/my_orders`,
  { key: 'my-orders' }
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

function statusLabel(status: unknown): string {
  if (status == null) return '-'
  return statusMap[status as number]?.label ?? t('admin.statusLabels.unknown')
}

function statusBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return statusMap[status as number]?.badge ?? 'badge-soft'
}

const paymentMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.paymentStatus.unpaid'), badge: 'badge-soft badge-error' },
  2: { label: t('order.paymentStatus.paid'), badge: 'badge-soft badge-success' },
  3: { label: t('order.paymentStatus.refunded'), badge: 'badge-soft badge-info' },
  4: { label: t('order.paymentStatus.partiallyRefunded'), badge: 'badge-soft badge-warning' },
}

function paymentLabel(status: unknown): string {
  if (status == null) return '-'
  return paymentMap[status as number]?.label ?? t('admin.statusLabels.unknown')
}

function paymentBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return paymentMap[status as number]?.badge ?? 'badge-soft'
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
