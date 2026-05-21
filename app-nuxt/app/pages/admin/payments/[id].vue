<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/payments" class="btn btn-circle btn-text btn-sm">
          <i class="icon-[tabler--arrow-left] size-5"></i>
        </NuxtLinkLocale>
        <h1 class="h1">{{ $t('admin.payments.detail.title', 'Payment') }} #{{ paymentId }}</h1>
      </div>
      <NuxtLinkLocale to="/admin/payments/refunds" class="btn btn-outline btn-sm">
        <i class="icon-[tabler--receipt-refund] size-4"></i>
        {{ $t('admin.payments.detail.refunds', 'Refunds') }}
      </NuxtLinkLocale>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <span>{{ error.message }}</span>
    </div>

    <!-- Payment Content -->
    <div v-else-if="detail" class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      
      <div class="lg:col-span-2 space-y-6">
        <!-- Overview Card -->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title">{{ $t('admin.payments.detail.overview', 'Overview') }}</h2>
            <div class="grid grid-cols-2 gap-4 mt-4">
              <div>
                <p class="text-sm text-gray-500">{{ $t('admin.payments.detail.amount', 'Amount') }}</p>
                <p class="font-medium text-lg">{{ formatCurrency(detail.payment.amount, detail.payment.currency || 'USD') }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500">{{ $t('admin.payments.detail.status', 'Status') }}</p>
                <span :class="['badge', statusBadge(detail.payment.status)]">
                  {{ statusLabel(detail.payment.status) }}
                </span>
              </div>
              <div>
                <p class="text-sm text-gray-500">{{ $t('admin.payments.detail.orderId', 'Order ID') }}</p>
                <NuxtLinkLocale :to="`/admin/orders/${detail.payment.order_id}`" class="link link-primary">
                  #{{ detail.payment.order_id }}
                </NuxtLinkLocale>
              </div>
              <div>
                <p class="text-sm text-gray-500">{{ $t('admin.payments.detail.transactionId', 'Transaction ID') }}</p>
                <p class="font-mono text-sm">{{ detail.payment.transaction_id || '-' }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500">{{ $t('admin.payments.detail.createdAt', 'Created At') }}</p>
                <p>{{ formatDate(detail.payment.created_at) }}</p>
              </div>
            </div>
            
            <div class="divider"></div>
            
            <div class="flex gap-2">
              <button class="btn btn-primary" @click="capturePayment" :disabled="isActionLoading">
                {{ $t('admin.payments.detail.capture', 'Capture') }}
              </button>
              <button class="btn btn-warning" @click="voidPayment" :disabled="isActionLoading">
                {{ $t('admin.payments.detail.void', 'Void') }}
              </button>
              <button class="btn btn-error" @click="refundPayment" :disabled="isActionLoading">
                {{ $t('admin.payments.detail.refund', 'Refund') }}
              </button>
            </div>
          </div>
        </div>

        <!-- Payment Sessions -->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title">Payment Sessions</h2>
            <div class="overflow-x-auto mt-4">
              <table class="table table-sm">
                <thead>
                  <tr>
                    <th>Session</th>
                    <th>Method</th>
                    <th>Status</th>
                    <th>Expires</th>
                    <th>Completed</th>
                    <th>Created</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="session in paymentSessions" :key="session.id">
                    <td>
                      <div class="font-mono text-sm">#{{ session.id }}</div>
                      <div class="mt-1 max-w-56 truncate font-mono text-xs text-base-content/50">
                        {{ session.external_session_id || '-' }}
                      </div>
                    </td>
                    <td class="font-mono text-sm">#{{ session.payment_method_id }}</td>
                    <td>
                      <span :class="['badge badge-sm badge-soft', sessionStatusBadge(session.status)]">
                        {{ sessionStatusLabel(session.status) }}
                      </span>
                    </td>
                    <td>{{ formatDate(session.expires_at) }}</td>
                    <td>{{ formatDate(session.completed_at) }}</td>
                    <td>{{ formatDate(session.created_at) }}</td>
                  </tr>
                  <tr v-if="paymentSessions.length === 0">
                    <td colspan="6" class="text-center py-4 text-gray-500">No sessions found</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- Refund History -->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title">Refund History</h2>
            <div class="overflow-x-auto mt-4">
              <table class="table table-sm">
                <thead>
                  <tr>
                    <th>Refund</th>
                    <th>Status</th>
                    <th class="text-right">Amount</th>
                    <th>Processed</th>
                    <th>Created</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="refund in paymentRefunds" :key="refund.id">
                    <td>
                      <NuxtLinkLocale :to="`/admin/payments/refunds/${refund.id}`" class="font-mono text-sm link link-primary">
                        #{{ refund.id }}
                      </NuxtLinkLocale>
                      <div class="mt-1 max-w-56 truncate font-mono text-xs text-base-content/50">
                        {{ refund.external_refund_id || refund.idempotency_key }}
                      </div>
                      <div v-if="refund.failure_code || refund.failure_message" class="mt-1 max-w-56 truncate text-xs text-error">
                        {{ refund.failure_code || refund.failure_message }}
                      </div>
                    </td>
                    <td>
                      <span :class="['badge badge-sm badge-soft', refundStatusBadge(refund.status)]">
                        {{ refundStatusLabel(refund.status) }}
                      </span>
                    </td>
                    <td class="text-right font-bold">
                      {{ formatCurrency(refund.amount, refund.currency || detail.payment.currency || 'USD') }}
                    </td>
                    <td>{{ formatDate(refund.processed_at) }}</td>
                    <td>{{ formatDate(refund.created_at) }}</td>
                  </tr>
                  <tr v-if="paymentRefunds.length === 0">
                    <td colspan="5" class="text-center py-4 text-gray-500">No refunds found</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- Events Log -->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title">Webhook Events</h2>
            <div class="overflow-x-auto mt-4">
              <table class="table table-sm">
                <thead>
                  <tr>
                    <th>Type</th>
                    <th>Ext Event ID</th>
                    <th>Status</th>
                    <th>Date</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="event in paymentEvents" :key="event.id">
                    <td><span class="badge badge-soft">{{ event.event_type }}</span></td>
                    <td class="font-mono text-xs">{{ event.external_event_id }}</td>
                    <td>
                      <span :class="['badge badge-sm', eventStatusBadge(event.status)]">
                        {{ eventStatusLabel(event.status) }}
                      </span>
                    </td>
                    <td>{{ formatDate(event.created_at) }}</td>
                  </tr>
                  <tr v-if="paymentEvents.length === 0">
                    <td colspan="4" class="text-center py-4 text-gray-500">No events found</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-6">
        <!-- Gateway Logs -->
        <div class="card bg-base-100 shadow-sm border border-base-200">
          <div class="card-body">
            <h2 class="card-title">Gateway Logs</h2>
            <div class="space-y-4 mt-4 max-h-[600px] overflow-y-auto">
              <div v-for="log in paymentLogs" :key="log.id" class="p-3 bg-base-200 rounded-lg text-sm">
                <div class="flex justify-between mb-1">
                  <span class="font-medium">{{ logLevelLabel(log.level) }}</span>
                  <span class="text-gray-500 text-xs">{{ formatDate(log.created_at) }}</span>
                </div>
                <p class="text-gray-700">{{ log.message || '-' }}</p>
                <pre v-if="log.payload" class="mt-2 p-2 bg-base-300 rounded text-xs overflow-x-auto">{{ log.payload }}</pre>
              </div>
              <div v-if="paymentLogs.length === 0" class="text-center py-4 text-gray-500">
                No logs found
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const paymentId = route.params.id

definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiFetch } = useApi()

interface AdminPaymentDetail {
  payment: {
    id: number
    order_id: number
    payment_method_id: number
    amount?: string | number | null
    currency?: string | null
    status?: number | null
    transaction_id?: string | null
    external_payment_id?: string | null
    created_at?: string | null
  }
  sessions?: AdminPaymentSession[]
  refunds?: AdminPaymentRefund[]
  logs?: AdminPaymentGatewayLog[]
  events?: AdminPaymentGatewayEvent[]
}

interface AdminPaymentSession {
  id: number
  payment_id: number
  payment_method_id: number
  status: number
  external_session_id?: string | null
  expires_at?: string | null
  completed_at?: string | null
  created_at?: string | null
}

interface AdminPaymentRefund {
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

interface AdminPaymentGatewayLog {
  id: number
  level: number
  message?: string | null
  payload?: string | null
  created_at?: string | null
}

interface AdminPaymentGatewayEvent {
  id: number
  event_type: string
  external_event_id: string
  status: number
  created_at?: string | null
}

const { pending, data: detail, error, refresh } = await useApiFetch<AdminPaymentDetail>(
  `/api/admin/payments/${paymentId}`,
  { key: `admin-payment-${paymentId}` }
)

const isActionLoading = ref(false)
const toast = useAppToast()
const dialog = useAppDialog()
const paymentSessions = computed(() => detail.value?.sessions || [])
const paymentRefunds = computed(() => detail.value?.refunds || [])
const paymentLogs = computed(() => detail.value?.logs || [])
const paymentEvents = computed(() => detail.value?.events || [])

const capturePayment = async () => {
  const confirmed = await dialog.confirm({
    title: 'Capture payment',
    message: 'Are you sure you want to capture this payment?',
    confirmLabel: 'Capture'
  })
  if (!confirmed) return
  isActionLoading.value = true
  try {
    await apiFetch(`/api/admin/payments/${paymentId}/capture`, { method: 'POST' })
    toast.success('Payment captured successfully.')
    await refresh()
  } catch (e: any) {
    toast.error(`Capture failed: ${e.message}`)
  } finally {
    isActionLoading.value = false
  }
}

const voidPayment = async () => {
  const confirmed = await dialog.confirm({
    title: 'Void payment',
    message: 'Are you sure you want to void this payment?',
    confirmLabel: 'Void',
    tone: 'danger'
  })
  if (!confirmed) return
  isActionLoading.value = true
  try {
    await apiFetch(`/api/admin/payments/${paymentId}/void`, { method: 'POST' })
    toast.success('Payment voided successfully.')
    await refresh()
  } catch (e: any) {
    toast.error(`Void failed: ${e.message}`)
  } finally {
    isActionLoading.value = false
  }
}

const refundPayment = async () => {
  const amountStr = await dialog.prompt({
    title: 'Refund payment',
    message: 'Enter an amount to refund or leave the field empty to issue a full refund.',
    confirmLabel: 'Refund',
    tone: 'danger',
    input: {
      label: 'Amount',
      placeholder: 'Leave empty for full refund',
      type: 'number',
      min: 0.01,
      step: '0.01'
    }
  })
  if (amountStr === null) return

  let amount = null
  if (amountStr.trim() !== '') {
    amount = parseFloat(amountStr)
    if (isNaN(amount) || amount <= 0) {
      toast.error('Invalid amount')
      return
    }
  }

  isActionLoading.value = true
  try {
    await apiFetch(`/api/admin/payments/${paymentId}/refund`, {
      method: 'POST',
      body: { amount }
    })
    toast.success('Refund requested successfully.')
    await refresh()
  } catch (e: any) {
    toast.error(`Refund failed: ${e.message}`)
  } finally {
    isActionLoading.value = false
  }
}

const formatCurrency = (value: string | number, currency: string) => {
  if (value === null || value === undefined) return '-'
  return new Intl.NumberFormat('pt-BR', { style: 'currency', currency }).format(Number(value))
}

const formatDate = (dateString?: string | null) => {
  if (!dateString) return '-'
  return new Date(dateString).toLocaleString()
}

const statusLabel = (status: number) => {
  switch (status) {
    case 1: return 'Checkout'
    case 2: return 'Pending'
    case 3: return 'Processing'
    case 4: return 'Authorized'
    case 5: return 'Captured'
    case 6: return 'Failed'
    case 7: return 'Voided'
    case 8: return 'Cancelled'
    case 9: return 'Refunded'
    case 10: return 'Partially refunded'
    default: return 'Unknown'
  }
}

const statusBadge = (status: number) => {
  switch (status) {
    case 1: return 'badge-neutral'
    case 2: return 'badge-warning'
    case 3: return 'badge-info'
    case 4: return 'badge-primary'
    case 5: return 'badge-success'
    case 6: return 'badge-error'
    case 7: return 'badge-neutral'
    case 8: return 'badge-neutral'
    case 9: return 'badge-error'
    case 10: return 'badge-warning'
    default: return 'badge-ghost'
  }
}

const eventStatusLabel = (status: number) => {
  switch (status) {
    case 1: return 'Received'
    case 2: return 'Processing'
    case 3: return 'Processed'
    case 4: return 'Failed'
    case 5: return 'Ignored'
    default: return 'Unknown'
  }
}

const eventStatusBadge = (status: number) => {
  switch (status) {
    case 1: return 'badge-info'
    case 2: return 'badge-warning'
    case 3: return 'badge-success'
    case 4: return 'badge-error'
    case 5: return 'badge-neutral'
    default: return 'badge-ghost'
  }
}

const sessionStatusLabel = (status: number) => {
  switch (status) {
    case 1: return 'Pending'
    case 2: return 'Processing'
    case 3: return 'Requires action'
    case 4: return 'Completed'
    case 5: return 'Failed'
    case 6: return 'Cancelled'
    case 7: return 'Expired'
    default: return 'Unknown'
  }
}

const sessionStatusBadge = (status: number) => {
  switch (status) {
    case 1: return 'badge-warning'
    case 2: return 'badge-warning'
    case 3: return 'badge-info'
    case 4: return 'badge-success'
    case 5: return 'badge-error'
    case 6: return 'badge-neutral'
    case 7: return 'badge-neutral'
    default: return 'badge-ghost'
  }
}

const refundStatusLabel = (status: number) => {
  switch (status) {
    case 1: return 'Pending'
    case 2: return 'Processing'
    case 3: return 'Succeeded'
    case 4: return 'Failed'
    case 5: return 'Cancelled'
    default: return 'Unknown'
  }
}

const refundStatusBadge = (status: number) => {
  switch (status) {
    case 1: return 'badge-warning'
    case 2: return 'badge-info'
    case 3: return 'badge-success'
    case 4: return 'badge-error'
    case 5: return 'badge-neutral'
    default: return 'badge-ghost'
  }
}

const logLevelLabel = (level: number) => {
  switch (level) {
    case 1: return 'INFO'
    case 2: return 'WARN'
    case 3: return 'ERROR'
    default: return 'UNKNOWN'
  }
}
</script>
