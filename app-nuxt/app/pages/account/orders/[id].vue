<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">{{ $t('account.order') }}</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">
          {{ order?.order_number || `${$t('account.orderNumber')}${orderId}` }}
        </h1>
        <p class="text-base-content/60 mt-1">{{ $t('account.orderDetails') }}</p>
      </div>
      <NuxtLinkLocale to="/account/orders" class="btn btn-soft">
        <span class="icon-[tabler--arrow-left] size-5"></span>
        {{ $t('account.back') }}
      </NuxtLinkLocale>
      <button class="btn btn-sm btn-outline" :disabled="downloadingInvoice" @click="downloadInvoice">
        <span v-if="downloadingInvoice" class="loading loading-spinner loading-xs" />
        <span v-else class="icon-[tabler--file-download] size-4" />
        {{ $t('account.downloadInvoice') }}
      </button>
      <button class="btn btn-sm btn-outline" :disabled="downloadingQuotation" @click="downloadQuotation">
        <span v-if="downloadingQuotation" class="loading loading-spinner loading-xs" />
        <span v-else class="icon-[tabler--file-text] size-4" />
        {{ $t('account.downloadQuotation', 'Quotation') }}
      </button>
    </div>

    <div v-if="pending" class="rounded-box border border-base-content/10 bg-base-100 p-8">
      <div class="flex items-center gap-3 text-base-content/60">
        <span class="loading loading-spinner loading-sm"></span>
        {{ $t('account.loadingOrder') }}
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span class="icon-[tabler--alert-circle] size-6"></span>
      <span>{{ $t('account.errorLoadingOrder') }} {{ error.message }}</span>
    </div>

    <div v-else-if="!order" class="rounded-box border border-dashed border-base-content/20 bg-base-100 p-10 text-center">
      <span class="icon-[tabler--package-off] text-base-content/30 mx-auto mb-4 size-12"></span>
      <h2 class="text-lg font-semibold">{{ $t('account.orderNotFound') }}</h2>
      <NuxtLinkLocale to="/account/orders" class="btn btn-primary mt-6">
        {{ $t('account.viewOrders') }}
      </NuxtLinkLocale>
    </div>

    <div v-else class="grid gap-6 lg:grid-cols-3">
      <section class="space-y-6 lg:col-span-2">
        <div class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <div class="mb-5 flex items-center justify-between gap-4">
            <h2 class="text-lg font-semibold">{{ $t('account.orderItems') }}</h2>
            <span class="badge badge-soft">{{ order.items?.length || 0 }} {{ $t('account.items') }}</span>
          </div>

          <div v-if="!order.items?.length" class="rounded-box border border-dashed border-base-content/20 p-8 text-center text-base-content/60">
            {{ $t('account.noItemsReturned') }}
          </div>

          <div v-else class="divide-y divide-base-content/10">
            <div v-for="item in order.items" :key="item.id" class="flex items-center justify-between gap-4 py-4">
              <div class="flex min-w-0 items-center gap-4">
                <div class="bg-base-200 flex size-12 shrink-0 items-center justify-center rounded-lg">
                  <span class="icon-[tabler--package] text-base-content/40 size-6"></span>
                </div>
                <div class="min-w-0">
                  <p class="truncate font-semibold">{{ item.product_name || `${$t('account.productNumber')}${item.product_id}` }}</p>
                  <p class="text-base-content/60 text-sm">{{ item.quantity }} x {{ formatCurrency(item.price, order.currency) }}</p>
                </div>
              </div>
              <p class="font-bold">{{ formatCurrency(item.total, order.currency) }}</p>
            </div>
          </div>
        </div>
      </section>

      <aside class="space-y-6">
        <section class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h2 class="mb-4 text-lg font-semibold">{{ $t('account.summary') }}</h2>
          <dl class="space-y-3">
            <div class="flex justify-between gap-4">
              <dt class="text-base-content/60">{{ $t('account.date') }}</dt>
              <dd class="text-right font-medium">{{ formatDate(order.created_at) }}</dd>
            </div>
            <div class="flex justify-between gap-4">
              <dt class="text-base-content/60">{{ $t('account.status') }}</dt>
              <dd><span :class="['badge', statusBadgeClass(order.status)]">{{ statusLabel(order.status) }}</span></dd>
            </div>
            <div class="flex justify-between gap-4">
              <dt class="text-base-content/60">{{ $t('account.paymentStatus') }}</dt>
              <dd><span :class="['badge', paymentBadgeClass(order.payment_status)]">{{ paymentLabel(order.payment_status) }}</span></dd>
            </div>
          </dl>
        </section>

        <section class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h2 class="mb-4 text-lg font-semibold">{{ $t('account.values') }}</h2>
          <dl class="space-y-3">
            <div class="flex justify-between gap-4">
              <dt class="text-base-content/60">{{ $t('account.subtotal') }}</dt>
              <dd class="font-medium">{{ formatCurrency(order.subtotal, order.currency) }}</dd>
            </div>
            <div class="flex justify-between gap-4">
              <dt class="text-base-content/60">{{ $t('account.shipping') }}</dt>
              <dd class="font-medium">{{ formatCurrency(order.shipping_amount, order.currency) }}</dd>
            </div>
            <div class="flex justify-between gap-4">
              <dt class="text-base-content/60">{{ $t('account.taxes') }}</dt>
              <dd class="font-medium">{{ formatCurrency(order.tax_amount, order.currency) }}</dd>
            </div>
            <div class="flex justify-between gap-4">
              <dt class="text-base-content/60">{{ $t('account.discount') }}</dt>
              <dd class="font-medium">{{ formatCurrency(order.discount_amount, order.currency) }}</dd>
            </div>
            <div class="border-base-content/10 flex justify-between gap-4 border-t pt-4 text-lg font-bold">
              <dt>{{ $t('account.total') }}</dt>
              <dd class="text-primary">{{ formatCurrency(order.total_amount, order.currency) }}</dd>
            </div>
          </dl>
        </section>
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Order } from '~/types'

definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

const route = useRoute()
const { locale, t } = useI18n()
const { useApiLazyFetch, apiFetch } = useApi()

const orderId = computed(() => {
  const id = route.params.id
  return Array.isArray(id) ? id[0] : id
})

useSeoMeta({
  title: () => `${t('account.orderNumber')}${orderId.value}`
})

const { data: order, pending, error } = useApiLazyFetch<Order>(
  `/api/account/orders/${orderId.value}`,
  { key: `account-order-${orderId.value}` }
)

const downloadingInvoice = ref(false)
const downloadingQuotation = ref(false)

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

async function downloadInvoice() {
  if (!order.value) return
  downloadingInvoice.value = true
  try {
    const blob = await apiFetch<Blob>(`/api/account/orders/${orderId.value}/invoice`, {
      responseType: 'blob',
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `invoice-${order.value.order_number || orderId.value}.pdf`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (err: any) {
    console.error('Failed to download invoice:', err)
  } finally {
    downloadingInvoice.value = false
  }
}

async function downloadQuotation() {
  if (!order.value) return
  downloadingQuotation.value = true
  try {
    const blob = await apiFetch<Blob>(`/api/account/orders/${orderId.value}/quotation`, {
      responseType: 'blob',
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `quotation-${order.value.order_number || orderId.value}.pdf`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (err: any) {
    console.error('Failed to download quotation:', err)
  } finally {
    downloadingQuotation.value = false
  }
}
</script>
