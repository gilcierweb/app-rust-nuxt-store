<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">{{ $t('account.payment') }}</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">
          {{ detail?.payment.number || `${$t('account.paymentNumber')}${paymentId}` }}
        </h1>
        <p class="text-base-content/60 mt-1">{{ $t('account.paymentDetails') }}</p>
      </div>
      <NuxtLinkLocale to="/account/orders" class="btn btn-soft">
        <span class="icon-[tabler--arrow-left] size-5"></span>
        {{ $t('account.back') }}
      </NuxtLinkLocale>
    </div>

    <div v-if="pending" class="rounded-box border border-base-content/10 bg-base-100 p-8">
      <div class="flex items-center gap-3 text-base-content/60">
        <span class="loading loading-spinner loading-sm"></span>
        {{ $t('account.loadingPayment') }}
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span class="icon-[tabler--alert-circle] size-6"></span>
      <span>{{ $t('account.errorLoadingPayment') }} {{ error.message }}</span>
    </div>

    <div v-else-if="!detail" class="rounded-box border border-dashed border-base-content/20 bg-base-100 p-10 text-center">
      <span class="icon-[tabler--credit-card-off] text-base-content/30 mx-auto mb-4 size-12"></span>
      <h2 class="text-lg font-semibold">{{ $t('account.paymentNotFound') }}</h2>
      <NuxtLinkLocale to="/account/orders" class="btn btn-primary mt-6">
        {{ $t('account.viewOrders') }}
      </NuxtLinkLocale>
    </div>

    <div v-else class="grid gap-6 lg:grid-cols-3">
      <!-- Main Content -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Payment Overview -->
        <div class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h3 class="font-semibold text-lg mb-4">{{ $t('account.paymentOverview') }}</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <p class="text-sm text-base-content/60">{{ $t('account.amount') }}</p>
              <p class="font-medium text-xl mt-1">{{ formatCurrency(detail.payment.amount, detail.payment.currency || 'BRL') }}</p>
            </div>
            <div>
              <p class="text-sm text-base-content/60">{{ $t('account.status') }}</p>
              <span :class="['badge badge-soft mt-1', statusBadgeClass(detail.payment.status)]">
                {{ statusLabel(detail.payment.status) }}
              </span>
            </div>
            <div>
              <p class="text-sm text-base-content/60">{{ $t('account.paymentMethod') }}</p>
              <p class="mt-1">{{ detail.payment_method_name || '-' }}</p>
            </div>
            <div>
              <p class="text-sm text-base-content/60">{{ $t('account.gateway') }}</p>
              <p class="mt-1">{{ detail.gateway_name || '-' }}</p>
            </div>
            <div>
              <p class="text-sm text-base-content/60">{{ $t('account.transactionId') }}</p>
              <p class="font-mono text-sm mt-1">{{ detail.payment.transaction_id || '-' }}</p>
            </div>
            <div>
              <p class="text-sm text-base-content/60">{{ $t('account.paymentDate') }}</p>
              <p class="mt-1">{{ formatDate(detail.payment.processed_at || detail.payment.created_at) }}</p>
            </div>
          </div>
        </div>

        <!-- Order Reference -->
        <div class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h3 class="font-semibold text-lg mb-4">{{ $t('account.orderReference') }}</h3>
          <div class="flex items-center gap-4">
            <div>
              <p class="text-sm text-base-content/60">{{ $t('account.orderNumber') }}</p>
              <NuxtLinkLocale
                :to="`/account/orders/${detail.payment.order_id}`"
                class="link link-hover font-medium text-primary mt-1 inline-block"
              >
                {{ detail.order_number || `#${detail.payment.order_id}` }}
              </NuxtLinkLocale>
            </div>
          </div>
        </div>
      </div>

      <!-- Sidebar -->
      <div class="space-y-6">
        <!-- Actions -->
        <div class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h3 class="font-semibold text-lg mb-4">{{ $t('account.actions') }}</h3>
          <div class="space-y-3">
            <button
              class="btn btn-primary btn-outline w-full"
              :disabled="downloadingReceipt"
              @click="downloadReceipt"
            >
              <span v-if="downloadingReceipt" class="loading loading-spinner loading-xs"></span>
              <span v-else class="icon-[tabler--file-download] size-4"></span>
              {{ $t('account.downloadReceipt') }}
            </button>
            <NuxtLinkLocale
              :to="`/account/orders/${detail.payment.order_id}`"
              class="btn btn-soft w-full"
            >
              <span class="icon-[tabler--receipt] size-4"></span>
              {{ $t('account.viewOrder') }}
            </NuxtLinkLocale>
          </div>
        </div>

        <!-- Payment Info -->
        <div class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h3 class="font-semibold text-lg mb-4">{{ $t('account.paymentInfo') }}</h3>
          <div class="space-y-3 text-sm">
            <div class="flex justify-between">
              <span class="text-base-content/60">{{ $t('account.paymentId') }}</span>
              <span class="font-mono">#{{ detail.payment.id }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-base-content/60">{{ $t('account.currency') }}</span>
              <span>{{ detail.payment.currency || 'BRL' }}</span>
            </div>
            <div v-if="detail.payment.external_payment_id" class="flex justify-between">
              <span class="text-base-content/60">{{ $t('account.externalId') }}</span>
              <span class="font-mono text-xs truncate max-w-32">{{ detail.payment.external_payment_id }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

const { t } = useI18n()
const route = useRoute()
const { apiFetch, useApiLazyFetch } = useApi()

const paymentId = computed(() => {
  const id = route.params.id
  return Array.isArray(id) ? id[0] : id
})

useSeoMeta({ title: () => `${t('account.payment')} #${paymentId.value}` })

interface AccountPaymentDetail {
  payment: {
    id: number
    order_id: number
    amount?: string | number | null
    currency?: string | null
    status?: number | null
    transaction_id?: string | null
    number?: string | null
    external_payment_id?: string | null
    processed_at?: string | null
    created_at: string
  }
  order_number?: string | null
  payment_method_name?: string | null
  gateway_name?: string | null
}

const { data: detail, pending, error } = await useApiLazyFetch<AccountPaymentDetail>(
  `/api/account/payments/${paymentId.value}`,
  { key: `account-payment-${paymentId.value}` }
)

const downloadingReceipt = ref(false)
const toast = useAppToast()

const paymentStatuses = [
  { value: 1, label: 'Checkout', badge: 'badge-neutral' },
  { value: 2, label: 'Pending', badge: 'badge-warning' },
  { value: 3, label: 'Processing', badge: 'badge-info' },
  { value: 4, label: 'Authorized', badge: 'badge-primary' },
  { value: 5, label: 'Captured', badge: 'badge-success' },
  { value: 6, label: 'Failed', badge: 'badge-error' },
  { value: 7, label: 'Voided', badge: 'badge-neutral' },
  { value: 8, label: 'Cancelled', badge: 'badge-neutral' },
  { value: 9, label: 'Refunded', badge: 'badge-info' },
  { value: 10, label: 'Partially refunded', badge: 'badge-warning' },
]

function statusLabel(status?: number | null): string {
  return paymentStatuses.find(item => item.value === status)?.label || 'Unknown'
}

function statusBadgeClass(status?: number | null): string {
  return paymentStatuses.find(item => item.value === status)?.badge || 'badge-ghost'
}

function formatCurrency(value: string | number | null | undefined, currency: string) {
  if (value === null || value === undefined) return '-'
  return new Intl.NumberFormat(getAppLocale(), { style: 'currency', currency }).format(Number(value))
}

function formatDate(dateString?: string | null) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat(getAppLocale(), {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

async function downloadReceipt() {
  if (!detail.value) return
  downloadingReceipt.value = true
  try {
    const blob = await apiFetch<Blob>(`/api/account/payments/${paymentId.value}/receipt`, {
      responseType: 'blob',
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `receipt-${detail.value.payment.number || paymentId.value}.pdf`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch {
    toast.error(t('account.errorDownloadingReceipt'))
  } finally {
    downloadingReceipt.value = false
  }
}
</script>
