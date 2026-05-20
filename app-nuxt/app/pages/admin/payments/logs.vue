<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/payments" class="btn btn-circle btn-text btn-sm">
          <i class="icon-[tabler--arrow-left] size-5"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">Gateway Logs</h1>
          <p class="text-sm text-base-content/60">Sanitized provider requests, responses, webhook processing notes, and failures.</p>
        </div>
      </div>
      <NuxtLinkLocale to="/admin/payments/events" class="btn btn-outline btn-sm">
        <i class="icon-[tabler--inbox] size-4"></i>
        Gateway Events
      </NuxtLinkLocale>
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
                placeholder="Message, payment, session, event..."
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Level</span>
            </label>
            <select v-model="selectedLevel" class="select select-bordered w-full">
              <option value="">All levels</option>
              <option v-for="level in logLevels" :key="level.value" :value="level.value">
                {{ level.label }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Direction</span>
            </label>
            <select v-model="selectedDirection" class="select select-bordered w-full">
              <option value="">All directions</option>
              <option v-for="direction in logDirections" :key="direction.value" :value="direction.value">
                {{ direction.label }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">Payment</span>
            </label>
            <input
              v-model="selectedPayment"
              class="input input-bordered w-full"
              placeholder="Payment ID"
              type="number"
            />
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">
            Clear
          </button>
        </div>
      </div>
    </div>

    <div v-if="logsPending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-base-content/60">Loading gateway logs...</p>
    </div>

    <div v-else-if="logsError" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Failed to load gateway logs: {{ logsError.message }}</span>
    </div>

    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table table-lg">
          <thead class="bg-base-200/50">
            <tr>
              <th>Log</th>
              <th>Payment</th>
              <th>Session</th>
              <th>Event</th>
              <th>Direction</th>
              <th>Level</th>
              <th>Date</th>
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
                No gateway logs found.
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
  '/api/admin/payment-gateway-logs/',
  { key: 'admin-payment-gateway-logs-list' }
)

const logLevels = [
  { value: 1, label: 'Info', badge: 'badge-info' },
  { value: 2, label: 'Warning', badge: 'badge-warning' },
  { value: 3, label: 'Error', badge: 'badge-error' }
]

const logDirections = [
  { value: 1, label: 'Inbound' },
  { value: 2, label: 'Outbound' },
  { value: 3, label: 'Webhook' }
]

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
