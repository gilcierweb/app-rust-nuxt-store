<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/payments/refunds" class="btn btn-circle btn-text btn-sm">
          <i class="icon-[tabler--arrow-left] size-5"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">{{ t('admin.payments.refunds.detail.title') }} #{{ refundId }}</h1>
          <p class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.description') }}</p>
        </div>
      </div>
      <NuxtLinkLocale
        v-if="detail?.payment.id"
        :to="`/admin/payments/${detail.payment.id}`"
        class="btn btn-outline btn-sm"
      >
        <i class="icon-[tabler--credit-card] size-4"></i>
        {{ t('admin.payments.refunds.detail.paymentLink') }}
      </NuxtLinkLocale>
      <button class="btn btn-outline btn-sm" @click="downloadReceipt" :disabled="isReceiptLoading">
        <span v-if="isReceiptLoading" class="loading loading-spinner loading-xs"></span>
        <i v-else class="icon-[tabler--file-download] size-4"></i>
        {{ t('admin.payments.refunds.detail.downloadReceipt') }}
      </button>
    </div>

    <div v-if="pending" class="card shadow-base-300/10 shadow-md">
      <div class="card-body flex items-center justify-center py-20">
        <span class="loading loading-spinner text-primary size-12"></span>
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ error.message }}</span>
    </div>

    <div v-else-if="detail" class="grid grid-cols-1 gap-6 xl:grid-cols-[1.1fr_0.9fr]">
      <div class="space-y-6">
        <div class="card shadow-base-300/10 shadow-md">
          <div class="card-header">
            <h2 class="card-title text-xl">{{ t('admin.payments.refunds.detail.overview') }}</h2>
          </div>
          <div class="card-body grid grid-cols-1 gap-5 md:grid-cols-2">
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.amount') }}</div>
              <div class="mt-1 text-xl font-semibold">{{ formatCurrency(detail.refund.amount, detail.refund.currency || 'BRL') }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.status') }}</div>
              <div class="mt-1">
                <span :class="['badge badge-soft', refundStatusBadge(detail.refund.status)]">
                  {{ refundStatusLabel(detail.refund.status) }}
                </span>
              </div>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.externalRefundId') }}</div>
              <div class="mt-1 font-mono text-sm">{{ detail.refund.external_refund_id || '-' }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.idempotencyKey') }}</div>
              <div class="mt-1 break-all font-mono text-sm">{{ detail.refund.idempotency_key }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.processedAt') }}</div>
              <div class="mt-1">{{ formatDate(detail.refund.processed_at) }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.createdAt') }}</div>
              <div class="mt-1">{{ formatDate(detail.refund.created_at) }}</div>
            </div>
            <div v-if="detail.refund.failure_code || detail.refund.failure_message" class="md:col-span-2">
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.failure') }}</div>
              <div class="mt-1 rounded-lg bg-error/10 p-3 text-sm text-error">
                {{ detail.refund.failure_code || detail.refund.failure_message }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-6">
        <div class="card shadow-base-300/10 shadow-md">
          <div class="card-header">
            <h2 class="card-title text-xl">{{ t('admin.payments.refunds.detail.parentPayment') }}</h2>
          </div>
          <div class="card-body space-y-4">
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.payment') }}</div>
              <NuxtLinkLocale :to="`/admin/payments/${detail.payment.id}`" class="mt-1 inline-block link link-primary font-mono">
                {{ detail.payment.number || `#${detail.payment.id}` }}
              </NuxtLinkLocale>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.order') }}</div>
              <NuxtLinkLocale :to="`/admin/orders/${detail.payment.order_id}`" class="mt-1 inline-block link link-primary font-mono">
                #{{ detail.payment.order_id }}
              </NuxtLinkLocale>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.gateway') }}</div>
              <div class="mt-1">{{ detail.gateway_name || '-' }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.paymentStatus') }}</div>
              <div class="mt-1">
                <span :class="['badge badge-soft', paymentStatusBadge(detail.payment.status)]">
                  {{ paymentStatusLabel(detail.payment.status) }}
                </span>
              </div>
            </div>
            <div>
              <div class="text-sm text-base-content/60">{{ t('admin.payments.refunds.detail.transaction') }}</div>
              <div class="mt-1 break-all font-mono text-sm">{{ detail.payment.transaction_id || detail.payment.external_payment_id || '-' }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

interface AdminRefundDetail {
  refund: {
    id: number
    payment_id: number
    amount: string | number
    currency: string
    status: number
    external_refund_id?: string | null
    idempotency_key: string
    failure_code?: string | null
    failure_message?: string | null
    processed_at?: string | null
    created_at?: string | null
  }
  payment: {
    id: number
    order_id: number
    status?: number | null
    number?: string | null
    transaction_id?: string | null
    external_payment_id?: string | null
  }
  gateway_name?: string | null
}

const { t } = useI18n()
const route = useRoute()
const refundId = route.params.id
const { apiFetch, useApiFetch } = useApi()

const { pending, data: detail, error } = await useApiFetch<AdminRefundDetail>(
  `/api/admin/payment-refunds/${refundId}`,
  { key: `admin-payment-refund-${refundId}` }
)

const isReceiptLoading = ref(false)
const toast = useAppToast()

const refundStatuses = [
  { value: 1, label: 'Pending', badge: 'badge-warning' },
  { value: 2, label: 'Processing', badge: 'badge-info' },
  { value: 3, label: 'Succeeded', badge: 'badge-success' },
  { value: 4, label: 'Failed', badge: 'badge-error' },
  { value: 5, label: 'Cancelled', badge: 'badge-neutral' }
]

const paymentStatuses = [
  { value: 1, label: 'Checkout', badge: 'badge-neutral' },
  { value: 2, label: 'Pending', badge: 'badge-warning' },
  { value: 3, label: 'Processing', badge: 'badge-info' },
  { value: 4, label: 'Authorized', badge: 'badge-primary' },
  { value: 5, label: 'Captured', badge: 'badge-success' },
  { value: 6, label: 'Failed', badge: 'badge-error' },
  { value: 7, label: 'Voided', badge: 'badge-neutral' },
  { value: 8, label: 'Cancelled', badge: 'badge-neutral' },
  { value: 9, label: 'Refunded', badge: 'badge-secondary' },
  { value: 10, label: 'Partially refunded', badge: 'badge-secondary' }
]

function refundStatusLabel(status: number) {
  return refundStatuses.find(item => item.value === status)?.label || 'Unknown'
}

function refundStatusBadge(status: number) {
  return refundStatuses.find(item => item.value === status)?.badge || 'badge-ghost'
}

function paymentStatusLabel(status?: number | null) {
  return paymentStatuses.find(item => item.value === status)?.label || 'Unknown'
}

function paymentStatusBadge(status?: number | null) {
  return paymentStatuses.find(item => item.value === status)?.badge || 'badge-ghost'
}

function formatCurrency(value: string | number, currency: string) {
  return new Intl.NumberFormat('pt-BR', { style: 'currency', currency }).format(Number(value))
}

function formatDate(dateString?: string | null) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

const downloadReceipt = async () => {
  isReceiptLoading.value = true
  try {
    const blob = await apiFetch(`/api/admin/payment-refunds/${refundId}/receipt`, {
      responseType: 'blob'
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `refund-confirmation-${refundId}.pdf`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (e: any) {
    toast.error(`Download failed: ${e.message}`)
  } finally {
    isReceiptLoading.value = false
  }
}
</script>
