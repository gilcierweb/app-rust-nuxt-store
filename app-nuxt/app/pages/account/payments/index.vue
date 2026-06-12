<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">Account</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">{{ t('account.myPayments') }}</h1>
        <p class="text-base-content/60 mt-1">{{ t('account.paymentsHistory') }}</p>
      </div>
      <button class="btn btn-soft" :disabled="pending" @click="refresh()">
        <span class="icon-[tabler--refresh] size-5" :class="{ 'animate-spin': pending }"></span>
        {{ t('account.refresh') }}
      </button>
    </div>

    <div v-if="pending" class="rounded-box border border-base-content/10 bg-base-100 p-8">
      <div class="flex items-center gap-3 text-base-content/60">
        <span class="loading loading-spinner loading-sm"></span>
        {{ t('account.loadingPayments') }}
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span class="icon-[tabler--alert-circle] size-6"></span>
      <span>{{ t('account.errorLoadingPayments') }} {{ error.message }}</span>
    </div>

    <div v-else-if="payments.length === 0" class="rounded-box border border-dashed border-base-content/20 bg-base-100 p-10 text-center">
      <span class="icon-[tabler--credit-card-off] text-base-content/30 mx-auto mb-4 size-12"></span>
      <h2 class="text-lg font-semibold">{{ t('account.noPaymentsFound') }}</h2>
      <p class="text-base-content/60 mt-1">{{ t('account.noPaymentsMessage') }}</p>
      <NuxtLinkLocale to="/products" class="btn btn-primary mt-6">
        {{ t('account.continueShopping') }}
      </NuxtLinkLocale>
    </div>

    <div v-else class="rounded-box overflow-hidden border border-base-content/10 bg-base-100 shadow-sm">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ t('account.paymentNumber') }}</th>
              <th>{{ t('account.orderReference') }}</th>
              <th>{{ t('account.paymentMethod') }}</th>
              <th>{{ t('account.date') }}</th>
              <th>{{ t('account.status') }}</th>
              <th class="text-right">{{ t('account.amount') }}</th>
              <th class="w-16"></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="payment in payments" :key="payment.id">
              <td>
                <div class="font-semibold">{{ payment.transaction_id || `#${payment.id}` }}</div>
              </td>
              <td>
                <NuxtLinkLocale v-if="payment.order_number" :to="`/account/orders/${payment.order_id}`" class="link link-primary">
                  {{ payment.order_number }}
                </NuxtLinkLocale>
                <span v-else class="text-base-content/50">#{{ payment.order_id }}</span>
              </td>
              <td>{{ payment.payment_method_name || '-' }}</td>
              <td>{{ formatDate(payment.processed_at || payment.created_at) }}</td>
              <td>
                <span :class="['badge', paymentBadgeClass(payment.status)]">
                  {{ paymentLabel(payment.status) }}
                </span>
              </td>
              <td class="text-right font-bold text-primary">{{ formatCurrency(payment.amount, payment.currency) }}</td>
              <td>
                <NuxtLinkLocale :to="`/account/payments/${payment.id}`" class="btn btn-square btn-soft btn-sm" :aria-label="t('account.viewPayment')">
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
definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

const { locale, t } = useI18n()
const { useApiLazyFetch } = useApi()

useSeoMeta({
  title: t('account.myPayments')
})

const { data, pending, error, refresh } = useApiLazyFetch<AccountPaymentListItem[]>(
  '/api/account/payments/',
  { key: 'account-payments' }
)

const payments = computed(() => data.value ?? [])

interface AccountPaymentListItem {
  id: number
  order_id: number
  order_number?: string
  amount?: number
  currency?: string
  status?: number
  transaction_id?: string
  payment_method_name?: string
  processed_at?: string
  created_at: string
}

const paymentMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('account.paymentStatus.pending'), badge: 'badge-soft badge-warning' },
  2: { label: t('account.paymentStatus.authorized'), badge: 'badge-soft badge-info' },
  3: { label: t('account.paymentStatus.captured'), badge: 'badge-soft badge-success' },
  4: { label: t('account.paymentStatus.settled'), badge: 'badge-soft badge-success' },
  5: { label: t('account.paymentStatus.failed'), badge: 'badge-soft badge-error' },
  6: { label: t('account.paymentStatus.voided'), badge: 'badge-soft badge-ghost' },
  7: { label: t('account.paymentStatus.refunded'), badge: 'badge-soft badge-info' },
  8: { label: t('account.paymentStatus.partiallyRefunded'), badge: 'badge-soft badge-warning' },
  9: { label: t('account.paymentStatus.cancelled'), badge: 'badge-soft badge-ghost' },
  10: { label: t('account.paymentStatus.pending'), badge: 'badge-soft badge-warning' }
}

function paymentLabel(status: unknown): string {
  return paymentMap[Number(status)]?.label ?? t('account.paymentStatus.unknown')
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
