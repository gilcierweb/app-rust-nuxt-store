<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">Account</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">{{ $t('account.myOrders') }}</h1>
        <p class="text-base-content/60 mt-1">{{ $t('account.ordersHistory') }}</p>
      </div>
      <button class="btn btn-soft" :disabled="pending" @click="refresh()">
        <span class="icon-[tabler--refresh] size-5" :class="{ 'animate-spin': pending }"></span>
        {{ $t('account.refresh') }}
      </button>
    </div>

    <div v-if="pending" class="rounded-box border border-base-content/10 bg-base-100 p-8">
      <div class="flex items-center gap-3 text-base-content/60">
        <span class="loading loading-spinner loading-sm"></span>
        {{ $t('account.loadingOrders') }}
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span class="icon-[tabler--alert-circle] size-6"></span>
      <span>{{ $t('account.errorLoadingOrders') }} {{ error.message }}</span>
    </div>

    <div v-else-if="orders.length === 0" class="rounded-box border border-dashed border-base-content/20 bg-base-100 p-10 text-center">
      <span class="icon-[tabler--package-off] text-base-content/30 mx-auto mb-4 size-12"></span>
      <h2 class="text-lg font-semibold">{{ $t('account.noOrdersFound') }}</h2>
      <p class="text-base-content/60 mt-1">{{ $t('account.noOrdersMessage') }}</p>
      <NuxtLinkLocale to="/products" class="btn btn-primary mt-6">
        {{ $t('account.continueShopping') }}
      </NuxtLinkLocale>
    </div>

    <div v-else class="rounded-box overflow-hidden border border-base-content/10 bg-base-100 shadow-sm">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('account.order') }}</th>
              <th>{{ $t('account.date') }}</th>
              <th>{{ $t('account.status') }}</th>
              <th>{{ $t('account.paymentStatus') }}</th>
              <th class="text-right">{{ $t('account.total') }}</th>
              <th class="w-16"></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="order in orders" :key="order.id">
              <td>
                <div class="font-semibold">{{ order.order_number || `#${order.id}` }}</div>
                <div class="text-base-content/50 text-xs">{{ order.items?.length || 0 }} {{ $t('account.items') }}</div>
              </td>
              <td>{{ formatDate(order.created_at) }}</td>
              <td><span :class="['badge', statusBadgeClass(order.status)]">{{ statusLabel(order.status) }}</span></td>
              <td><span :class="['badge', paymentBadgeClass(order.payment_status)]">{{ paymentLabel(order.payment_status) }}</span></td>
              <td class="text-right font-bold text-primary">{{ formatCurrency(order.total_amount, order.currency) }}</td>
              <td>
                <NuxtLinkLocale :to="`/account/orders/${order.id}`" class="btn btn-square btn-soft btn-sm" :aria-label="$t('account.viewOrder')">
                  <span class="icon-[tabler--eye] size-4"></span>
                </NuxtLinkLocale>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Order } from '~/types'

definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

const { locale, t } = useI18n()
const { useApiFetch } = useApi()

useSeoMeta({
  title: t('account.myOrders')
})

const { data: ordersData, pending, error, refresh } = await useApiFetch<Order[]>(
  '/api/account/orders',
  { key: 'account-orders' }
)

const orders = computed(() => ordersData.value ?? [])

const statusMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('account.status.pending'), badge: 'badge-soft badge-warning' },
  2: { label: t('account.status.confirmed'), badge: 'badge-soft badge-info' },
  3: { label: t('account.status.processing'), badge: 'badge-soft badge-info' },
  4: { label: t('account.status.shipped'), badge: 'badge-soft badge-primary' },
  5: { label: t('account.status.delivered'), badge: 'badge-soft badge-success' },
  6: { label: t('account.status.cancelled'), badge: 'badge-soft badge-error' }
}

const paymentMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('account.payment.unpaid'), badge: 'badge-soft badge-error' },
  2: { label: t('account.payment.paid'), badge: 'badge-soft badge-success' },
  3: { label: t('account.payment.refunded'), badge: 'badge-soft badge-info' },
  4: { label: t('account.payment.partial'), badge: 'badge-soft badge-warning' }
}

function statusLabel(status: unknown): string {
  return statusMap[Number(status)]?.label ?? t('account.status.unknown')
}

function statusBadgeClass(status: unknown): string {
  return statusMap[Number(status)]?.badge ?? 'badge-soft'
}

function paymentLabel(status: unknown): string {
  return paymentMap[Number(status)]?.label ?? t('account.payment.unknown')
}

function paymentBadgeClass(status: unknown): string {
  return paymentMap[Number(status)]?.badge ?? 'badge-soft'
}

function formatCurrency(value: number, currency = 'BRL'): string {
  return new Intl.NumberFormat(locale.value || 'pt-BR', {
    style: 'currency',
    currency: currency || 'BRL'
  }).format(Number(value || 0))
}

function formatDate(value?: string): string {
  if (!value) return '-'
  return new Intl.DateTimeFormat(locale.value || 'pt-BR', {
    dateStyle: 'short',
    timeStyle: 'short'
  }).format(new Date(value))
}
</script>
