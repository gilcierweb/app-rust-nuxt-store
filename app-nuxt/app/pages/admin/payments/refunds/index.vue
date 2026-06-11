<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">{{ t('admin.payments.refunds.title') }}</h1>
        <p class="text-sm text-base-content/60">{{ t('admin.payments.refunds.description') }}</p>
      </div>
      <div class="flex gap-2">
        <NuxtLinkLocale to="/admin/payments" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--credit-card] size-4"></i>
          {{ t('admin.payments.refunds.paymentsLink') }}
        </NuxtLinkLocale>
      </div>
    </div>

    <div class="card shadow-base-300/10 mb-6 shadow-md">
      <div class="card-body">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_180px_180px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.refunds.search') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                :placeholder="t('admin.payments.refunds.searchPlaceholder')"
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.refunds.status') }}</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.refunds.allStatuses') }}</option>
              <option v-for="status in refundStatuses" :key="status.value" :value="status.value">
                {{ status.label }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.refunds.gateway') }}</span>
            </label>
            <select v-model="selectedGateway" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.refunds.allGateways') }}</option>
              <option v-for="gateway in gateways" :key="gateway" :value="gateway">
                {{ gateway }}
              </option>
            </select>
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">{{ t('admin.payments.refunds.clear') }}</button>
        </div>
      </div>
    </div>

    <div v-if="pending" class="card shadow-base-300/10 shadow-md">
      <div class="card-body flex flex-col items-center justify-center py-20">
        <span class="loading loading-spinner text-primary size-12"></span>
        <p class="mt-4 text-base-content/60">{{ t('admin.payments.refunds.loading') }}</p>
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.payments.refunds.error') }} {{ error.message }}</span>
    </div>

    <div v-else class="card shadow-base-300/10 shadow-md overflow-hidden">
      <div class="card-body p-0">
        <div class="overflow-x-auto">
          <table class="table table-lg">
            <thead class="bg-base-200/50">
              <tr>
                <th>{{ t('admin.payments.refunds.refund') }}</th>
                <th>{{ t('admin.payments.refunds.payment') }}</th>
                <th>{{ t('admin.payments.refunds.gateway') }}</th>
                <th>{{ t('admin.payments.refunds.status') }}</th>
                <th class="text-right">{{ t('admin.payments.refunds.amount') }}</th>
                <th>{{ t('admin.payments.refunds.processed') }}</th>
                <th class="text-right">{{ t('admin.payments.refunds.actions') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in filteredRefunds" :key="item.refund.id" class="hover:bg-base-200/30 transition-colors">
                <td>
                  <div class="font-mono text-sm font-bold text-primary">#{{ item.refund.id }}</div>
                  <div class="mt-1 max-w-52 truncate text-xs text-base-content/50">
                    {{ item.refund.external_refund_id || item.refund.idempotency_key }}
                  </div>
                  <div v-if="item.refund.failure_code || item.refund.failure_message" class="mt-1 max-w-52 truncate text-xs text-error">
                    {{ item.refund.failure_code || item.refund.failure_message }}
                  </div>
                </td>
                <td>
                  <NuxtLinkLocale :to="`/admin/payments/${item.refund.payment_id}`" class="link link-primary font-mono">
                    {{ item.payment_number || `#${item.refund.payment_id}` }}
                  </NuxtLinkLocale>
                  <div class="mt-1 text-xs text-base-content/50">
                    Order #{{ item.order_id }}
                  </div>
                </td>
                <td>
                  <span class="badge badge-soft badge-info">
                    {{ item.gateway_name || '-' }}
                  </span>
                </td>
                <td>
                  <span :class="['badge badge-soft badge-sm', refundStatusBadge(item.refund.status)]">
                    {{ refundStatusLabel(item.refund.status) }}
                  </span>
                </td>
                <td class="text-right font-bold">
                  {{ formatCurrency(item.refund.amount, item.refund.currency || 'BRL') }}
                </td>
                <td>{{ formatDate(item.refund.processed_at || item.refund.created_at) }}</td>
                <td class="text-right">
                  <NuxtLinkLocale :to="`/admin/payments/refunds/${item.refund.id}`" class="btn btn-circle btn-text btn-sm" aria-label="View refund">
                    <i class="icon-[tabler--eye] size-5"></i>
                  </NuxtLinkLocale>
                </td>
              </tr>
              <tr v-if="filteredRefunds.length === 0">
                <td colspan="7" class="py-20 text-center text-base-content/50 italic">
                  {{ t('admin.payments.refunds.noRefunds') }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

interface AdminRefundListItem {
  refund: {
    id: number
    payment_id: number
    amount: number | string
    currency: string
    status: number
    external_refund_id?: string | null
    idempotency_key: string
    failure_code?: string | null
    failure_message?: string | null
    processed_at?: string | null
    created_at?: string | null
  }
  order_id: number
  payment_number?: string | null
  payment_status?: number | null
  gateway_name?: string | null
}

const { t } = useI18n()
const { useApiFetch } = useApi()

const searchQuery = ref('')
const selectedStatus = ref('')
const selectedGateway = ref('')

const { pending, data, error } = await useApiFetch<AdminRefundListItem[]>(
  '/api/admin/payment-refunds',
  { key: 'admin-payment-refunds-list' }
)

const refunds = computed(() => data.value || [])
const gateways = computed(() => {
  return [...new Set(refunds.value.map(item => item.gateway_name).filter(Boolean) as string[])].sort()
})

const refundStatuses = [
  { value: 1, label: 'Pending', badge: 'badge-warning' },
  { value: 2, label: 'Processing', badge: 'badge-info' },
  { value: 3, label: 'Succeeded', badge: 'badge-success' },
  { value: 4, label: 'Failed', badge: 'badge-error' },
  { value: 5, label: 'Cancelled', badge: 'badge-neutral' }
]

const filteredRefunds = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  const status = selectedStatus.value ? Number(selectedStatus.value) : null
  const gateway = selectedGateway.value || null

  return refunds.value.filter((item) => {
    const matchesSearch = !query ||
      String(item.refund.id).includes(query) ||
      String(item.refund.payment_id).includes(query) ||
      (item.payment_number || '').toLowerCase().includes(query) ||
      (item.refund.external_refund_id || '').toLowerCase().includes(query) ||
      item.refund.idempotency_key.toLowerCase().includes(query) ||
      (item.refund.failure_code || '').toLowerCase().includes(query) ||
      (item.refund.failure_message || '').toLowerCase().includes(query)

    const matchesStatus = !status || item.refund.status === status
    const matchesGateway = !gateway || item.gateway_name === gateway

    return matchesSearch && matchesStatus && matchesGateway
  })
})

function refundStatusLabel(status: number) {
  return refundStatuses.find(item => item.value === status)?.label || 'Unknown'
}

function refundStatusBadge(status: number) {
  return refundStatuses.find(item => item.value === status)?.badge || 'badge-ghost'
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

function resetFilters() {
  searchQuery.value = ''
  selectedStatus.value = ''
  selectedGateway.value = ''
}
</script>
