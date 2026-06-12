<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/payments" class="btn btn-circle btn-text btn-sm">
          <i class="icon-[tabler--arrow-left] size-5"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">{{ t('admin.payments.events.title') }}</h1>
          <p class="text-sm text-base-content/60">{{ t('admin.payments.events.description') }}</p>
        </div>
      </div>
      <NuxtLinkLocale to="/admin/payments/logs" class="btn btn-outline btn-sm">
        <i class="icon-[tabler--list-details] size-4"></i>
        {{ t('admin.payments.events.logsLink') }}
      </NuxtLinkLocale>
    </div>

    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_180px_180px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.events.search') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                :placeholder="t('admin.payments.events.searchPlaceholder')"
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.events.gateway') }}</span>
            </label>
            <select v-model="selectedGateway" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.events.allGateways') }}</option>
              <option v-for="gateway in gateways" :key="gateway.id" :value="gateway.id">
                {{ gateway.name }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.payments.events.status') }}</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">{{ t('admin.payments.events.allStatuses') }}</option>
              <option v-for="status in eventStatuses" :key="status.value" :value="status.value">
                {{ status.label }}
              </option>
            </select>
          </div>

          <button class="btn btn-ghost" type="button" @click="resetFilters">
            {{ t('admin.payments.events.clear') }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="eventsPending || gatewaysPending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-base-content/60">{{ t('admin.payments.events.loading') }}</p>
    </div>

    <div v-else-if="eventsError" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.payments.events.error') }} {{ eventsError.message }}</span>
    </div>

    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table table-lg">
          <thead class="bg-base-200/50">
            <tr>
              <th>{{ t('admin.payments.events.event') }}</th>
              <th>{{ t('admin.payments.events.gateway') }}</th>
              <th>{{ t('admin.payments.events.status') }}</th>
              <th>{{ t('admin.payments.events.signature') }}</th>
              <th>{{ t('admin.payments.events.processed') }}</th>
              <th>{{ t('admin.payments.events.received') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="event in filteredEvents" :key="event.id" class="hover:bg-base-200/30 transition-colors">
              <td>
                <div class="font-medium">{{ event.event_type }}</div>
                <div class="mt-1 max-w-72 truncate font-mono text-xs text-base-content/50">
                  {{ event.external_event_id }}
                </div>
                <div v-if="event.failure_message" class="mt-1 max-w-72 truncate text-xs text-error">
                  {{ event.failure_message }}
                </div>
              </td>
              <td>
                <span class="badge badge-soft badge-info">
                  {{ gatewayName(event.payment_gateway_id) }}
                </span>
              </td>
              <td>
                <span :class="['badge badge-soft badge-sm', statusBadgeClass(event.status)]">
                  {{ statusLabel(event.status) }}
                </span>
              </td>
              <td>
                <span :class="['badge badge-soft badge-sm', event.signature_valid ? 'badge-success' : 'badge-error']">
                  {{ event.signature_valid ? t('admin.payments.events.valid') : t('admin.payments.events.invalid') }}
                </span>
              </td>
              <td>{{ formatDate(event.processed_at) }}</td>
              <td>
                <div class="text-sm font-medium">{{ formatDate(event.created_at) }}</div>
                <div class="text-xs text-base-content/40">{{ formatTime(event.created_at) }}</div>
              </td>
            </tr>
            <tr v-if="filteredEvents.length === 0">
              <td colspan="6" class="py-20 text-center text-base-content/50 italic">
                {{ t('admin.payments.events.noEvents') }}
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

interface AdminPaymentGatewayEvent {
  id: number
  payment_gateway_id: number
  event_type: string
  external_event_id: string
  status: number
  signature_valid: boolean
  payload?: string | null
  processed_at?: string | null
  failure_message?: string | null
  created_at?: string | null
}

interface AdminPaymentGateway {
  id: number
  name: string
  code: string
}

const { useApiFetch } = useApi()

const searchQuery = ref('')
const selectedGateway = ref('')
const selectedStatus = ref('')

const {
  pending: eventsPending,
  data: eventsData,
  error: eventsError
} = await useApiFetch<AdminPaymentGatewayEvent[]>(
  '/api/admin/payment-gateway-events',
  { key: 'admin-payment-gateway-events-list' }
)

const {
  pending: gatewaysPending,
  data: gatewaysData
} = await useApiFetch<AdminPaymentGateway[]>(
  '/api/admin/payment-gateways',
  { key: 'admin-payment-gateways-for-events' }
)

const eventStatuses = computed(() => [
  { value: 1, label: t('admin.payments.events.receivedStatus'), badge: 'badge-info' },
  { value: 2, label: t('admin.payments.events.processingStatus'), badge: 'badge-warning' },
  { value: 3, label: t('admin.payments.events.processedStatus'), badge: 'badge-success' },
  { value: 4, label: t('admin.payments.events.failedStatus'), badge: 'badge-error' },
  { value: 5, label: t('admin.payments.events.ignoredStatus'), badge: 'badge-neutral' }
])

const events = computed(() => eventsData.value || [])
const gateways = computed(() => gatewaysData.value || [])

const filteredEvents = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  const gateway = selectedGateway.value ? Number(selectedGateway.value) : null
  const status = selectedStatus.value ? Number(selectedStatus.value) : null

  return events.value.filter((event) => {
    const matchesSearch = !query ||
      event.event_type.toLowerCase().includes(query) ||
      event.external_event_id.toLowerCase().includes(query) ||
      (event.failure_message || '').toLowerCase().includes(query)

    const matchesGateway = !gateway || event.payment_gateway_id === gateway
    const matchesStatus = !status || event.status === status

    return matchesSearch && matchesGateway && matchesStatus
  })
})

const gatewayName = (gatewayId: number) => {
  const gateway = gateways.value.find(item => item.id === gatewayId)
  return gateway?.name || `Gateway #${gatewayId}`
}

const statusLabel = (status: number) => {
  return eventStatuses.find(item => item.value === status)?.label || 'Unknown'
}

const statusBadgeClass = (status: number) => {
  return eventStatuses.find(item => item.value === status)?.badge || 'badge-ghost'
}

const formatDate = (dateString?: string | null) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat(getAppLocale(), {
    day: '2-digit',
    month: 'short',
    year: 'numeric'
  }).format(new Date(dateString))
}

const formatTime = (dateString?: string | null) => {
  if (!dateString) return ''
  return new Intl.DateTimeFormat(getAppLocale(), {
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

const resetFilters = () => {
  searchQuery.value = ''
  selectedGateway.value = ''
  selectedStatus.value = ''
}
</script>
