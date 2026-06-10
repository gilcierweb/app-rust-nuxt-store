<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">{{ t('admin.payments.title') }}</h1>
        <p class="text-sm text-base-content/60">{{ t('admin.payments.description') }}</p>
      </div>
      <div class="flex gap-2">
        <button
          v-if="selectedIds.length > 0"
          class="btn btn-primary btn-sm"
          :disabled="exporting"
          @click="bulkExportReceipts"
        >
          <span v-if="exporting" class="loading loading-spinner loading-xs"></span>
          <span v-else class="icon-[tabler--download] size-4 mr-1"></span>
          {{ t('admin.payments.exportSelected', 'Export Selected') }} ({{ selectedIds.length }})
        </button>
        <NuxtLinkLocale to="/admin/payments/events" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--inbox] size-4"></i>
          {{ t('admin.payments.actions.events') }}
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/admin/payments/logs" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--list-details] size-4"></i>
          {{ t('admin.payments.actions.logs') }}
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/admin/payments/refunds" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--receipt-refund] size-4"></i>
          {{ t('admin.payments.actions.refunds') }}
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/admin/payments/gateways" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--building-bank] size-4"></i>
          {{ t('admin.payments.actions.gateways') }}
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/admin/payments/methods" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--credit-card] size-4"></i>
          {{ t('admin.payments.actions.methods') }}
        </NuxtLinkLocale>
      </div>
    </div>

    <div class="card shadow-base-300/10 mb-6 shadow-md">
      <div class="card-body p-4">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_180px_180px_180px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.filters.search') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                :placeholder="t('admin.payments.filters.searchPlaceholder')"
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.filters.status') }}</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.filters.allStatuses') }}</option>
              <option v-for="status in paymentStatuses" :key="status.value" :value="status.value">
                {{ status.label }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.filters.gateway') }}</span>
            </label>
            <select v-model="selectedGateway" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.filters.allGateways') }}</option>
              <option v-for="gateway in gateways" :key="gateway.id" :value="gateway.id">
                {{ gateway.name }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.filters.currency') }}</span>
            </label>
            <select v-model="selectedCurrency" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.filters.allCurrencies') }}</option>
              <option v-for="currency in currencies" :key="currency" :value="currency">
                {{ currency }}
              </option>
            </select>
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">
            {{ t('admin.payments.filters.clear') }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="paymentsPending || methodsPending || gatewaysPending" class="card shadow-base-300/10 shadow-md">
      <div class="card-body flex flex-col items-center justify-center py-20">
        <span class="loading loading-spinner text-primary size-12"></span>
        <p class="mt-4 text-base-content/60">{{ t('admin.payments.loading') }}</p>
      </div>
    </div>

    <div v-else-if="paymentsError" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.payments.error', { message: paymentsError.message }) }}</span>
    </div>

    <div v-else class="card shadow-base-300/10 w-full shadow-md overflow-hidden">
      <div class="card-body p-0">
        <div class="overflow-x-auto">
        <table class="table table-lg">
          <thead class="bg-base-200/50">
            <tr>
              <th class="w-10">
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm"
                  :checked="allFilteredSelected"
                  @change="toggleSelectAll"
                />
              </th>
              <th>{{ t('admin.payments.table.payment') }}</th>
              <th>{{ t('admin.payments.table.order') }}</th>
              <th>{{ t('admin.payments.table.method') }}</th>
              <th>{{ t('admin.payments.table.gateway') }}</th>
              <th>{{ t('admin.payments.table.status') }}</th>
              <th class="text-right">{{ t('admin.payments.table.amount') }}</th>
              <th>{{ t('admin.payments.table.date') }}</th>
              <th class="text-right">{{ t('admin.payments.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="payment in payments" :key="payment.id" class="hover:bg-base-200/30 transition-colors">
              <td>
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm"
                  :checked="selectedIds.includes(payment.id)"
                  @change="toggleSelect(payment.id)"
                />
              </td>
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
                <NuxtLinkLocale :to="`/admin/payments/${payment.id}`" class="btn btn-circle btn-text btn-sm" :aria-label="t('common.view')">
                  <i class="icon-[tabler--eye] size-5"></i>
                </NuxtLinkLocale>
              </td>
            </tr>
            <tr v-if="payments.length === 0">
              <td colspan="9" class="py-20 text-center text-base-content/50 italic">
                {{ t('admin.payments.empty') }}
              </td>
            </tr>
          </tbody>
        </table>
        </div>

        <AdminPagination
          :current-page="currentPage"
          :page-size="pageSize"
          :current-count="payments.length"
          :total="paymentsData?.total || 0"
          :pending="paymentsPending"
          :summary="t('admin.payments.pagination.showing', { current: payments.length, total: paymentsData?.total || 0 })"
          :previous-label="t('admin.payments.pagination.previous')"
          :next-label="t('admin.payments.pagination.next')"
          @change="changePage"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AdminPaginatedResponse } from '~/types'

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

interface AdminPaymentListResponse extends AdminPaginatedResponse<AdminPayment> {
  currencies: string[]
}

const { t } = useI18n()
const { useApiFetch, apiFetch } = useApi()

const searchQuery = ref('')
const debouncedSearchQuery = ref('')
const selectedStatus = ref('')
const selectedGateway = ref('')
const selectedCurrency = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const selectedIds = ref<number[]>([])
const exporting = ref(false)

const apiQuery = reactive({
  page: currentPage,
  page_size: pageSize,
  search: computed(() => debouncedSearchQuery.value || undefined),
  status: computed(() => selectedStatus.value || undefined),
  gateway_id: computed(() => selectedGateway.value || undefined),
  currency: computed(() => selectedCurrency.value || undefined)
})

const {
  pending: paymentsPending,
  data: paymentsData,
  error: paymentsError,
  refresh
} = await useApiFetch<AdminPaymentListResponse>(
  '/api/admin/payments',
  {
    key: 'admin-payments-list',
    query: apiQuery
  }
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

const payments = computed(() => paymentsData.value?.items ?? [])
const methods = computed(() => methodsData.value ?? [])
const gateways = computed(() => gatewaysData.value ?? [])

const methodById = computed(() => {
  return new Map(methods.value.map(method => [method.id, method]))
})

const gatewayById = computed(() => {
  return new Map(gateways.value.map(gateway => [gateway.id, gateway]))
})

const currencies = computed(() => {
  return paymentsData.value?.currencies ?? []
})

const paymentStatuses = [
  { value: 1, label: t('admin.payments.status.checkout'), badge: 'badge-neutral' },
  { value: 2, label: t('admin.payments.status.pending'), badge: 'badge-warning' },
  { value: 3, label: t('admin.payments.status.processing'), badge: 'badge-info' },
  { value: 4, label: t('admin.payments.status.authorized'), badge: 'badge-primary' },
  { value: 5, label: t('admin.payments.status.captured'), badge: 'badge-success' },
  { value: 6, label: t('admin.payments.status.failed'), badge: 'badge-error' },
  { value: 7, label: t('admin.payments.status.voided'), badge: 'badge-neutral' },
  { value: 8, label: t('admin.payments.status.cancelled'), badge: 'badge-neutral' },
  { value: 9, label: t('admin.payments.status.refunded'), badge: 'badge-info' },
  { value: 10, label: t('admin.payments.status.partiallyRefunded'), badge: 'badge-warning' }
]

const statusByValue = new Map(paymentStatuses.map(status => [status.value, status]))

let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null

watch(searchQuery, (value) => {
  currentPage.value = 1
  if (searchDebounceTimer) clearTimeout(searchDebounceTimer)
  searchDebounceTimer = setTimeout(() => {
    debouncedSearchQuery.value = value.trim()
  }, 250)
})

watch([selectedStatus, selectedGateway, selectedCurrency], () => {
  currentPage.value = 1
})

function resetFilters() {
  searchQuery.value = ''
  debouncedSearchQuery.value = ''
  selectedStatus.value = ''
  selectedGateway.value = ''
  selectedCurrency.value = ''
  currentPage.value = 1
}

function changePage(page: number) {
  currentPage.value = Math.max(1, page)
}

function paymentMethodName(paymentMethodId: number) {
  return methodById.value.get(paymentMethodId)?.name || `Method #${paymentMethodId}`
}

function paymentMethodCode(paymentMethodId: number) {
  return methodById.value.get(paymentMethodId)?.code || '-'
}

function paymentGatewayName(paymentMethodId: number) {
  const method = methodById.value.get(paymentMethodId)
  if (!method?.payment_gateway_id) return t('admin.payments.gateway.manual')
  return gatewayById.value.get(method.payment_gateway_id)?.name || `Gateway #${method.payment_gateway_id}`
}

function statusLabel(status: unknown) {
  return statusByValue.get(Number(status))?.label || t('admin.payments.status.unknown')
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

const allFilteredSelected = computed(() => {
  if (payments.value.length === 0) return false
  return payments.value.every(p => selectedIds.value.includes(p.id))
})

function toggleSelect(id: number) {
  const idx = selectedIds.value.indexOf(id)
  if (idx >= 0) {
    selectedIds.value.splice(idx, 1)
  } else {
    selectedIds.value.push(id)
  }
}

function toggleSelectAll() {
  if (allFilteredSelected.value) {
    const currentIds = new Set(payments.value.map(p => p.id))
    selectedIds.value = selectedIds.value.filter(id => !currentIds.has(id))
  } else {
    const currentIds = payments.value.map(p => p.id)
    const existing = new Set(selectedIds.value)
    for (const id of currentIds) {
      if (!existing.has(id)) {
        selectedIds.value.push(id)
      }
    }
  }
}

async function bulkExportReceipts() {
  if (selectedIds.value.length === 0) return
  exporting.value = true
  try {
    const blob = await apiFetch<Blob>('/api/admin/payments/bulk-export', {
      method: 'POST',
      body: { ids: selectedIds.value },
      responseType: 'blob',
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'receipts.zip'
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (err: any) {
    console.error('Failed to bulk export receipts:', err)
  } finally {
    exporting.value = false
  }
}

onBeforeUnmount(() => {
  if (searchDebounceTimer) clearTimeout(searchDebounceTimer)
})
</script>
