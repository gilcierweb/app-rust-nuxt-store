<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">Payments</h1>
        <p class="text-sm text-base-content/60">Review payment attempts, gateway references, and operational status.</p>
      </div>
      <div class="flex gap-2">
        <NuxtLinkLocale to="/admin/payments/gateways" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--building-bank] size-4"></i>
          Gateways
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/admin/payments/methods" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--credit-card] size-4"></i>
          Methods
        </NuxtLinkLocale>
      </div>
    </div>

    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_180px_180px_180px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Search</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                placeholder="Payment, order, transaction, idempotency..."
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Status</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">All statuses</option>
              <option v-for="status in paymentStatuses" :key="status.value" :value="status.value">
                {{ status.label }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Gateway</span>
            </label>
            <select v-model="selectedGateway" class="select select-bordered w-full">
              <option value="">All gateways</option>
              <option v-for="gateway in gateways" :key="gateway.id" :value="gateway.id">
                {{ gateway.name }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Currency</span>
            </label>
            <select v-model="selectedCurrency" class="select select-bordered w-full">
              <option value="">All currencies</option>
              <option v-for="currency in currencies" :key="currency" :value="currency">
                {{ currency }}
              </option>
            </select>
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">
            Clear
          </button>
        </div>
      </div>
    </div>

    <div v-if="paymentsPending || methodsPending || gatewaysPending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-base-content/60">Loading payments...</p>
    </div>

    <div v-else-if="paymentsError" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Failed to load payments: {{ paymentsError.message }}</span>
    </div>

    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table table-lg">
          <thead class="bg-base-200/50">
            <tr>
              <th>Payment</th>
              <th>Order</th>
              <th>Method</th>
              <th>Gateway</th>
              <th>Status</th>
              <th class="text-right">Amount</th>
              <th>Date</th>
              <th class="text-right">Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="payment in filteredPayments" :key="payment.id" class="hover:bg-base-200/30 transition-colors">
              <td>
                <div class="font-mono text-sm font-bold text-primary">{{ payment.number || `#${payment.id}` }}</div>
                <div class="mt-1 max-w-44 truncate text-xs text-base-content/50">
                  {{ payment.external_payment_id || payment.transaction_id || payment.idempotency_key || '-' }}
                </div>
              </td>
              <td>
                <NuxtLinkLocale :to="`/admin/orders/${payment.order_id}`" class="link link-primary font-mono">
                  #{{ payment.order_id }}
                </NuxtLinkLocale>
              </td>
              <td>
                <div class="font-medium">{{ paymentMethodName(payment.payment_method_id) }}</div>
                <div class="text-xs text-base-content/50">{{ paymentMethodCode(payment.payment_method_id) }}</div>
              </td>
              <td>
                <span class="badge badge-soft badge-info">
                  {{ paymentGatewayName(payment.payment_method_id) }}
                </span>
              </td>
              <td>
                <span :class="['badge badge-soft badge-sm', statusBadgeClass(payment.status)]">
                  {{ statusLabel(payment.status) }}
                </span>
                <div v-if="payment.failure_code || payment.failure_message" class="mt-1 max-w-48 truncate text-xs text-error">
                  {{ payment.failure_code || payment.failure_message }}
                </div>
              </td>
              <td class="text-right font-bold">
                {{ formatCurrency(payment.amount, payment.currency || 'BRL') }}
              </td>
              <td>
                <div class="text-sm font-medium">{{ formatDate(payment.created_at) }}</div>
                <div class="text-xs text-base-content/40">{{ formatTime(payment.created_at) }}</div>
              </td>
              <td class="text-right">
                <NuxtLinkLocale :to="`/admin/payments/${payment.id}`" class="btn btn-circle btn-text btn-sm" aria-label="View payment">
                  <i class="icon-[tabler--eye] size-5"></i>
                </NuxtLinkLocale>
              </td>
            </tr>
            <tr v-if="filteredPayments.length === 0">
              <td colspan="8" class="py-20 text-center text-base-content/50 italic">
                No payments found.
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
  layout: 'admin'
})

interface AdminPayment {
  id: number
  order_id: number
  payment_method_id: number
  amount?: number | string | null
  currency?: string | null
  status?: number | null
  transaction_id?: string | null
  number?: string | null
  external_payment_id?: string | null
  external_status?: string | null
  idempotency_key?: string | null
  failure_code?: string | null
  failure_message?: string | null
  created_at?: string | null
}

interface AdminPaymentMethod {
  id: number
  name?: string | null
  code?: string | null
  payment_gateway_id?: number | null
}

interface AdminPaymentGateway {
  id: number
  name: string
  code: string
}

const { useApiFetch } = useApi()

const searchQuery = ref('')
const selectedStatus = ref('')
const selectedGateway = ref('')
const selectedCurrency = ref('')

const {
  pending: paymentsPending,
  data: paymentsData,
  error: paymentsError
} = await useApiFetch<AdminPayment[]>(
  '/api/admin/payments/',
  { key: 'admin-payments-list' }
)

const {
  pending: methodsPending,
  data: methodsData
} = await useApiFetch<AdminPaymentMethod[]>(
  '/api/admin/payment-methods',
  { key: 'admin-payment-methods-for-payments' }
)

const {
  pending: gatewaysPending,
  data: gatewaysData
} = await useApiFetch<AdminPaymentGateway[]>(
  '/api/admin/payment-gateways',
  { key: 'admin-payment-gateways-for-payments' }
)

const payments = computed(() => paymentsData.value ?? [])
const methods = computed(() => methodsData.value ?? [])
const gateways = computed(() => gatewaysData.value ?? [])

const methodById = computed(() => {
  return new Map(methods.value.map(method => [method.id, method]))
})

const gatewayById = computed(() => {
  return new Map(gateways.value.map(gateway => [gateway.id, gateway]))
})

const currencies = computed(() => {
  return [...new Set(payments.value.map(payment => payment.currency).filter(Boolean) as string[])].sort()
})

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
  { value: 10, label: 'Partially refunded', badge: 'badge-warning' }
]

const statusByValue = new Map(paymentStatuses.map(status => [status.value, status]))

const filteredPayments = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()

  return payments.value
    .filter(payment => {
      const method = methodById.value.get(payment.payment_method_id)
      const gateway = method?.payment_gateway_id ? gatewayById.value.get(method.payment_gateway_id) : null
      const haystack = [
        payment.id,
        payment.number,
        payment.order_id,
        payment.transaction_id,
        payment.external_payment_id,
        payment.external_status,
        payment.idempotency_key,
        method?.name,
        method?.code,
        gateway?.name,
        gateway?.code
      ].filter(Boolean).join(' ').toLowerCase()

      const matchesSearch = !query || haystack.includes(query)
      const matchesStatus = !selectedStatus.value || Number(payment.status) === Number(selectedStatus.value)
      const matchesGateway = !selectedGateway.value || method?.payment_gateway_id === Number(selectedGateway.value)
      const matchesCurrency = !selectedCurrency.value || payment.currency === selectedCurrency.value

      return matchesSearch && matchesStatus && matchesGateway && matchesCurrency
    })
    .sort((a, b) => new Date(b.created_at || 0).getTime() - new Date(a.created_at || 0).getTime())
})

function resetFilters() {
  searchQuery.value = ''
  selectedStatus.value = ''
  selectedGateway.value = ''
  selectedCurrency.value = ''
}

function paymentMethodName(paymentMethodId: number) {
  return methodById.value.get(paymentMethodId)?.name || `Method #${paymentMethodId}`
}

function paymentMethodCode(paymentMethodId: number) {
  return methodById.value.get(paymentMethodId)?.code || '-'
}

function paymentGatewayName(paymentMethodId: number) {
  const method = methodById.value.get(paymentMethodId)
  if (!method?.payment_gateway_id) return 'Manual'
  return gatewayById.value.get(method.payment_gateway_id)?.name || `Gateway #${method.payment_gateway_id}`
}

function statusLabel(status: unknown) {
  return statusByValue.get(Number(status))?.label || 'Unknown'
}

function statusBadgeClass(status: unknown) {
  return statusByValue.get(Number(status))?.badge || 'badge-ghost'
}

function formatCurrency(value: number | string | null | undefined, currency: string) {
  return new Intl.NumberFormat('pt-BR', { style: 'currency', currency }).format(Number(value) || 0)
}

function formatDate(dateString?: string | null) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', { dateStyle: 'medium' }).format(new Date(dateString))
}

function formatTime(dateString?: string | null) {
  if (!dateString) return ''
  return new Intl.DateTimeFormat('pt-BR', { timeStyle: 'short' }).format(new Date(dateString))
}
</script>
