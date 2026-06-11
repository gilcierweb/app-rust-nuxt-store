<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/payments" class="btn btn-circle btn-text btn-sm">
          <i class="icon-[tabler--arrow-left] size-5"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">{{ t('admin.payments.logs.title') }}</h1>
          <p class="text-sm text-base-content/60">{{ t('admin.payments.logs.description') }}</p>
        </div>
      </div>
      <NuxtLinkLocale to="/admin/payments/events" class="btn btn-outline btn-sm">
        <i class="icon-[tabler--inbox] size-4"></i>
        {{ t('admin.payments.logs.eventsLink') }}
      </NuxtLinkLocale>
    </div>

    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_180px_180px_180px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.logs.search') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                :placeholder="t('admin.payments.logs.searchPlaceholder')"
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.logs.level') }}</span>
            </label>
            <select v-model="selectedLevel" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.logs.allLevels') }}</option>
              <option v-for="level in logLevels" :key="level.value" :value="level.value">
                {{ level.label }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.logs.direction') }}</span>
            </label>
            <select v-model="selectedDirection" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.logs.allDirections') }}</option>
              <option v-for="direction in logDirections" :key="direction.value" :value="direction.value">
                {{ direction.label }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.logs.payment') }}</span>
            </label>
            <input
              v-model="selectedPayment"
              class="input input-bordered w-full"
              :placeholder="t('admin.payments.logs.paymentPlaceholder')"
              type="number"
            />
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">
            {{ t('admin.payments.logs.clear') }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="logsPending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-base-content/60">{{ t('admin.payments.logs.loading') }}</p>
    </div>

    <div v-else-if="logsError" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.payments.logs.error') }} {{ logsError.message }}</span>
    </div>

    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table table-lg">
          <thead class="bg-base-200/50">
            <tr>
              <th>{{ t('admin.payments.logs.log') }}</th>
              <th>{{ t('admin.payments.logs.payment') }}</th>
              <th>{{ t('admin.payments.logs.session') }}</th>
              <th>{{ t('admin.payments.logs.event') }}</th>
              <th>{{ t('admin.payments.logs.direction') }}</th>
              <th>{{ t('admin.payments.logs.level') }}</th>
              <th>{{ t('admin.payments.logs.date') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="log in filteredLogs" :key="log.id" class="hover:bg-base-200/30 transition-colors align-top">
              <td>
                <div class="max-w-xl whitespace-normal text-sm">
                  {{ log.message || '-' }}
                </div>
                <pre v-if="log.payload" class="mt-2 max-h-32 max-w-xl overflow-auto rounded bg-base-200 p-2 text-xs">{{ log.payload }}</pre>
              </td>
              <td>
                <NuxtLinkLocale v-if="log.payment_id" :to="`/admin/payments/${log.payment_id}`" class="link link-primary font-mono">
                  #{{ log.payment_id }}
                </NuxtLinkLocale>
                <span v-else>-</span>
              </td>
              <td class="font-mono text-sm">{{ log.payment_session_id || '-' }}</td>
              <td class="font-mono text-sm">{{ log.payment_gateway_event_id || '-' }}</td>
              <td>
                <span class="badge badge-soft badge-info badge-sm">
                  {{ directionLabel(log.direction) }}
                </span>
              </td>
              <td>
                <span :class="['badge badge-soft badge-sm', levelBadgeClass(log.level)]">
                  {{ levelLabel(log.level) }}
                </span>
              </td>
              <td>
                <div class="text-sm font-medium">{{ formatDate(log.created_at) }}</div>
                <div class="text-xs text-base-content/40">{{ formatTime(log.created_at) }}</div>
              </td>
            </tr>
            <tr v-if="filteredLogs.length === 0">
              <td colspan="7" class="py-20 text-center text-base-content/50 italic">
                {{ t('admin.payments.logs.noLogs') }}
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

const { t } = useI18n()

interface AdminPaymentGatewayLog {
  id: number
  payment_id?: number | null
  payment_session_id?: number | null
  payment_gateway_event_id?: number | null
  direction: number
  level: number
  message?: string | null
  payload?: string | null
  created_at?: string | null
}

const { useApiFetch } = useApi()

const searchQuery = ref('')
const selectedLevel = ref('')
const selectedDirection = ref('')
const selectedPayment = ref('')

const {
  pending: logsPending,
  data: logsData,
  error: logsError
} = await useApiFetch<AdminPaymentGatewayLog[]>(
  '/api/admin/payment-gateway-logs',
  { key: 'admin-payment-gateway-logs-list' }
)

const logLevels = computed(() => [
  { value: 1, label: t('admin.payments.logs.info'), badge: 'badge-info' },
  { value: 2, label: t('admin.payments.logs.warning'), badge: 'badge-warning' },
  { value: 3, label: t('admin.payments.logs.errorLevel'), badge: 'badge-error' }
])

const logDirections = computed(() => [
  { value: 1, label: t('admin.payments.logs.inbound') },
  { value: 2, label: t('admin.payments.logs.outbound') },
  { value: 3, label: t('admin.payments.logs.webhook') }
])

const logs = computed(() => logsData.value || [])

const filteredLogs = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  const level = selectedLevel.value ? Number(selectedLevel.value) : null
  const direction = selectedDirection.value ? Number(selectedDirection.value) : null
  const payment = selectedPayment.value ? Number(selectedPayment.value) : null

  return logs.value.filter((log) => {
    const matchesSearch = !query ||
      (log.message || '').toLowerCase().includes(query) ||
      (log.payload || '').toLowerCase().includes(query) ||
      String(log.payment_id || '').includes(query) ||
      String(log.payment_session_id || '').includes(query) ||
      String(log.payment_gateway_event_id || '').includes(query)

    const matchesLevel = !level || log.level === level
    const matchesDirection = !direction || log.direction === direction
    const matchesPayment = !payment || log.payment_id === payment

    return matchesSearch && matchesLevel && matchesDirection && matchesPayment
  })
})

const levelLabel = (level: number) => {
  return logLevels.find(item => item.value === level)?.label || 'Unknown'
}

const levelBadgeClass = (level: number) => {
  return logLevels.find(item => item.value === level)?.badge || 'badge-ghost'
}

const directionLabel = (direction: number) => {
  return logDirections.find(item => item.value === direction)?.label || 'Unknown'
}

const formatDate = (dateString?: string | null) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric'
  }).format(new Date(dateString))
}

const formatTime = (dateString?: string | null) => {
  if (!dateString) return ''
  return new Intl.DateTimeFormat('pt-BR', {
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

const resetFilters = () => {
  searchQuery.value = ''
  selectedLevel.value = ''
  selectedDirection.value = ''
  selectedPayment.value = ''
}
</script>
