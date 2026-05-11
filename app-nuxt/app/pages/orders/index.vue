<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12 pt-10">
      <div>
        <h1 class="h2 gradient-text mb-2">{{ t('pages.orders.title') }}</h1>
        <p class="text-base-content/60">{{ t('pages.orders.description') }}</p>
      </div>
      <div class="flex items-center gap-3">
        <div class="dropdown dropdown-end">
          <button tabindex="0" class="btn btn-ghost btn-square rounded-2xl bg-base-200/50" role="button">
            <span class="icon-[tabler--filter] size-5"></span>
          </button>
          <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-52">
            <li><a>All Orders</a></li>
            <li><a>Pending</a></li>
            <li><a>Completed</a></li>
            <li><a>Cancelled</a></li>
          </ul>
        </div>
        <button class="btn btn-ghost btn-square rounded-2xl bg-base-200/50">
          <span class="icon-[tabler--refresh] size-5"></span>
        </button>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-32">
      <div class="alert alert-info max-w-md">
        <div class="flex items-center gap-4">
          <div class="loading loading-spinner loading-md"></div>
          <div>
            <p class="font-bold">Loading your orders</p>
            <p class="text-sm opacity-80">Please wait while we fetch your order history...</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else-if="orders.length === 0" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="alert alert-warning max-w-md">
        <div class="flex items-center gap-4">
          <div class="size-16 rounded-full bg-warning/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--package-off] size-8 text-warning" />
          </div>
          <div>
            <h2 class="font-bold text-lg">{{ t('pages.orders.empty') }}</h2>
            <p class="text-sm opacity-80 mt-1">
              {{ t('pages.orders.description') }}
            </p>
          </div>
        </div>
      </div>
      <NuxtLink to="/products" class="btn btn-primary btn-lg px-10 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-105 mt-8">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <!-- Orders List -->
    <div v-else class="space-y-6">
      <div v-for="order in orders" :key="order.id" 
        class="group bg-base-100 rounded-[2.5rem] border border-base-200 overflow-hidden hover-shadow-xl hover:shadow-primary/5 transition-all duration-500 hover:border-primary/20">
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
              {{ order.items?.length || 0 }} {{ t('pages.orders.items') }}
            </p>
          </div>
          
          <!-- Status Badges -->
          <div class="lg:w-1/4 flex flex-row lg:flex-col gap-3">
            <div class="flex flex-col gap-1">
              <span class="text-[10px] uppercase tracking-widest text-base-content/30 font-bold">{{ t('pages.orders.status') }}</span>
              <div :class="['badge badge-lg rounded-xl font-bold py-3 px-4', statusBadgeClass(order.status)]">
                <span class="flex items-center gap-2">
                  <span :class="getStatusIcon(order.status)" class="size-4"></span>
                  {{ statusLabel(order.status) }}
                </span>
              </div>
            </div>
          </div>
          
          <div class="lg:w-1/4 flex flex-row lg:flex-col gap-3">
            <div class="flex flex-col gap-1">
              <span class="text-[10px] uppercase tracking-widest text-base-content/30 font-bold">{{ t('pages.orders.payment') }}</span>
              <div :class="['badge badge-lg rounded-xl font-bold py-3 px-4', paymentBadgeClass(order.payment_status)]">
                <span class="flex items-center gap-2">
                  <span :class="getPaymentIcon(order.payment_status)" class="size-4"></span>
                  {{ paymentLabel(order.payment_status) }}
                </span>
              </div>
            </div>
          </div>
          
          <!-- Total & Actions -->
          <div class="lg:w-1/4 flex lg:flex-col lg:items-end justify-between items-center border-t lg:border-t-0 pt-6 lg:pt-0 border-base-200">
            <div class="text-right lg:mb-6">
              <span class="block text-[10px] uppercase tracking-widest text-base-content/30 font-bold">{{ t('pages.orders.amount') }}</span>
              <span class="text-3xl font-black text-primary">{{ formatNumberBR(order.total_amount) }}</span>
            </div>
            <div class="dropdown dropdown-left dropdown-end">
              <button tabindex="0" class="btn btn-ghost hover:bg-primary/10 hover:text-primary rounded-xl px-6 group-hover:translate-x-1 transition-all" role="button">
                {{ t('pages.orders.detail') }}
                <span class="icon-[tabler--arrow-right] size-4 ml-1"></span>
              </button>
              <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-48">
                <li><NuxtLink :to="`/orders/confirmation/${order.id}`">View Details</NuxtLink></li>
                <li><a>Track Order</a></li>
                <li><a>Download Invoice</a></li>
                <li><a class="text-error">Cancel Order</a></li>
              </ul>
            </div>
          </div>
        </div>
        
        <!-- Quick Preview of items (optional) -->
        <div v-if="order.items && order.items.length > 0" class="px-10 py-6 bg-base-200/20 border-t border-base-200 flex items-center gap-4 overflow-x-auto no-scrollbar">
          <div v-for="item in order.items.slice(0, 5)" :key="item.id" class="size-12 rounded-xl bg-white border border-base-200 shrink-0 overflow-hidden">
             <div class="flex items-center justify-center h-full text-base-content/10">
               <span class="icon-[tabler--photo] size-6"></span>
             </div>
          </div>
          <span v-if="order.items.length > 5" class="text-xs font-bold text-base-content/30">+{{ order.items.length - 5 }} {{ t('common.more') || 'mais' }}</span>
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
  return statusMap[status as number]?.label ?? t('common.unknown')
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
  return paymentMap[status as number]?.label ?? t('common.unknown')
}

function paymentBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return paymentMap[status as number]?.badge ?? 'badge-soft'
}

function getStatusIcon(status: unknown): string {
  const iconMap: Record<number, string> = {
    1: 'icon-[tabler--clock]',
    2: 'icon-[tabler--check]',
    3: 'icon-[tabler--loader]',
    4: 'icon-[tabler--truck]',
    5: 'icon-[tabler--circle-check]',
    6: 'icon-[tabler--x]'
  }
  return iconMap[status as number] ?? 'icon-[tabler--help-circle]'
}

function getPaymentIcon(status: unknown): string {
  const iconMap: Record<number, string> = {
    1: 'icon-[tabler--alert-circle]',
    2: 'icon-[tabler--check]',
    3: 'icon-[tabler--refresh]',
    4: 'icon-[tabler--alert-triangle]'
  }
  return iconMap[status as number] ?? 'icon-[tabler--help-circle]'
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
