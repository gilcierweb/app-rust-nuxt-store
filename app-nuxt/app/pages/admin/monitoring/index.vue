<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">{{ t('admin.monitoring.title') }}</h1>
        <p class="text-sm text-base-content/60">{{ t('admin.monitoring.description') }}</p>
      </div>
      <div class="flex flex-wrap gap-2">
        <NuxtLinkLocale to="/admin/payments/events" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--webhook] size-4"></i>
          {{ t('admin.monitoring.actions.events') }}
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/admin/payments/logs" class="btn btn-outline btn-sm">
          <i class="icon-[tabler--list-details] size-4"></i>
          {{ t('admin.monitoring.actions.logs') }}
        </NuxtLinkLocale>
      </div>
    </div>

    <div v-if="isPending" class="card shadow-base-300/10 shadow-md">
      <div class="card-body flex flex-col items-center justify-center py-20">
        <span class="loading loading-spinner text-primary size-12"></span>
        <p class="mt-4 text-base-content/60">{{ t('admin.monitoring.loading') }}</p>
      </div>
    </div>

    <div v-else-if="loadError" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.monitoring.error', { message: loadError.message }) }}</span>
    </div>

    <div v-else class="space-y-6">
      <section class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
        <div v-for="metric in metrics" :key="metric.label" class="card shadow-base-300/10 shadow-md">
          <div class="card-body gap-3">
            <div class="flex items-start justify-between gap-3">
              <div>
                <div class="text-sm text-base-content/60">{{ metric.label }}</div>
                <div class="mt-1 text-3xl font-semibold">{{ metric.value }}</div>
              </div>
              <div :class="['flex size-11 items-center justify-center rounded-full', metric.iconBg]">
                <i :class="[metric.icon, 'size-5']"></i>
              </div>
            </div>
            <p class="text-sm text-base-content/60">{{ metric.help }}</p>
          </div>
        </div>
      </section>

      <section class="grid grid-cols-1 gap-6 xl:grid-cols-[1.15fr_0.85fr]">
        <div class="card shadow-base-300/10 shadow-md overflow-hidden">
          <div class="card-header flex items-center justify-between gap-3">
            <div>
              <h2 class="card-title text-xl">{{ t('admin.monitoring.events.title') }}</h2>
              <p class="text-sm text-base-content/60">{{ t('admin.monitoring.events.description') }}</p>
            </div>
            <NuxtLinkLocale to="/admin/payments/events" class="btn btn-text btn-sm">
              {{ t('common.view') }}
            </NuxtLinkLocale>
          </div>

          <div class="card-body p-0">
            <div class="overflow-x-auto">
              <table class="table">
                <thead>
                  <tr>
                    <th>{{ t('admin.monitoring.events.columns.event') }}</th>
                    <th>{{ t('admin.monitoring.events.columns.gateway') }}</th>
                    <th>{{ t('admin.monitoring.events.columns.status') }}</th>
                    <th>{{ t('admin.monitoring.events.columns.signature') }}</th>
                    <th>{{ t('admin.monitoring.events.columns.received') }}</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="event in eventAlerts" :key="event.id">
                    <td>
                      <div class="font-medium">{{ event.event_type }}</div>
                      <div class="mt-1 max-w-64 truncate font-mono text-xs text-base-content/50">
                        {{ event.external_event_id }}
                      </div>
                      <div v-if="event.failure_message" class="mt-1 max-w-64 truncate text-xs text-error">
                        {{ event.failure_message }}
                      </div>
                    </td>
                    <td>{{ gatewayName(event.payment_gateway_id) }}</td>
                    <td>
                      <span :class="['badge badge-soft badge-sm', eventStatusBadge(event.status)]">
                        {{ eventStatusLabel(event.status) }}
                      </span>
                    </td>
                    <td>
                      <span :class="['badge badge-soft badge-sm', event.signature_valid ? 'badge-success' : 'badge-error']">
                        {{ event.signature_valid ? t('admin.monitoring.events.valid') : t('admin.monitoring.events.invalid') }}
                      </span>
                    </td>
                    <td>{{ formatDateTime(event.created_at) }}</td>
                  </tr>
                  <tr v-if="eventAlerts.length === 0">
                    <td colspan="5" class="py-12 text-center text-base-content/50">
                      {{ t('admin.monitoring.empty') }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div class="card shadow-base-300/10 shadow-md overflow-hidden">
          <div class="card-header flex items-center justify-between gap-3">
            <div>
              <h2 class="card-title text-xl">{{ t('admin.monitoring.logs.title') }}</h2>
              <p class="text-sm text-base-content/60">{{ t('admin.monitoring.logs.description') }}</p>
            </div>
            <NuxtLinkLocale to="/admin/payments/logs" class="btn btn-text btn-sm">
              {{ t('common.view') }}
            </NuxtLinkLocale>
          </div>

          <div class="card-body gap-4">
            <div
              v-for="log in logAlerts"
              :key="log.id"
              class="rounded-lg bg-base-200/70 p-4"
            >
              <div class="flex items-start justify-between gap-3">
                <span :class="['badge badge-soft badge-sm', logLevelBadge(log.level)]">
                  {{ logLevelLabel(log.level) }}
                </span>
                <span class="text-xs text-base-content/50">{{ formatDateTime(log.created_at) }}</span>
              </div>
              <p class="mt-2 text-sm">{{ log.message || '-' }}</p>
              <div class="mt-2 flex flex-wrap gap-2 text-xs text-base-content/60">
                <NuxtLinkLocale
                  v-if="log.payment_id"
                  :to="`/admin/payments/${log.payment_id}`"
                  class="link link-primary"
                >
                  #{{ log.payment_id }}
                </NuxtLinkLocale>
                <span v-if="log.payment_session_id">{{ t('admin.monitoring.logs.session', { id: log.payment_session_id }) }}</span>
                <span v-if="log.payment_gateway_event_id">{{ t('admin.monitoring.logs.event', { id: log.payment_gateway_event_id }) }}</span>
              </div>
            </div>
            <div v-if="logAlerts.length === 0" class="py-8 text-center text-base-content/50">
              {{ t('admin.monitoring.empty') }}
            </div>
          </div>
        </div>
      </section>

      <section class="card shadow-base-300/10 shadow-md overflow-hidden">
        <div class="card-header flex items-center justify-between gap-3">
          <div>
            <h2 class="card-title text-xl">{{ t('admin.monitoring.payments.title') }}</h2>
            <p class="text-sm text-base-content/60">{{ t('admin.monitoring.payments.description') }}</p>
          </div>
          <NuxtLinkLocale to="/admin/payments" class="btn btn-text btn-sm">
            {{ t('common.view') }}
          </NuxtLinkLocale>
        </div>

        <div class="card-body p-0">
          <div class="overflow-x-auto">
            <table class="table">
              <thead>
                <tr>
                  <th>{{ t('admin.monitoring.payments.columns.payment') }}</th>
                  <th>{{ t('admin.monitoring.payments.columns.order') }}</th>
                  <th>{{ t('admin.monitoring.payments.columns.status') }}</th>
                  <th class="text-right">{{ t('admin.monitoring.payments.columns.amount') }}</th>
                  <th>{{ t('admin.monitoring.payments.columns.reason') }}</th>
                  <th>{{ t('admin.monitoring.payments.columns.createdAt') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="payment in paymentAlerts" :key="payment.id">
                  <td>
                    <NuxtLinkLocale :to="`/admin/payments/${payment.id}`" class="link link-primary font-mono">
                      {{ payment.number || `#${payment.id}` }}
                    </NuxtLinkLocale>
                    <div class="mt-1 max-w-52 truncate text-xs text-base-content/50">
                      {{ payment.external_payment_id || payment.transaction_id || payment.idempotency_key || '-' }}
                    </div>
                  </td>
                  <td>
                    <NuxtLinkLocale :to="`/admin/orders/${payment.order_id}`" class="link link-primary font-mono">
                      #{{ payment.order_id }}
                    </NuxtLinkLocale>
                  </td>
                  <td>
                    <span :class="['badge badge-soft badge-sm', paymentStatusBadge(payment.status)]">
                      {{ paymentStatusLabel(payment.status) }}
                    </span>
                  </td>
                  <td class="text-right font-medium">{{ formatCurrency(payment.amount, payment.currency || 'BRL') }}</td>
                  <td class="max-w-64">
                    <div class="truncate text-sm">
                      {{ payment.failure_message || payment.failure_code || t('admin.monitoring.payments.noReason') }}
                    </div>
                  </td>
                  <td>{{ formatDateTime(payment.created_at) }}</td>
                </tr>
                <tr v-if="paymentAlerts.length === 0">
                  <td colspan="6" class="py-12 text-center text-base-content/50">
                    {{ t('admin.monitoring.empty') }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>
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
  idempotency_key?: string | null
  failure_code?: string | null
  failure_message?: string | null
  created_at?: string | null
}

interface AdminPaymentListResponse extends AdminPaginatedResponse<AdminPayment> {
  currencies: string[]
}

interface AdminPaymentGateway {
  id: number
  name: string
}

interface AdminPaymentGatewayEvent {
  id: number
  payment_gateway_id: number
  event_type: string
  external_event_id: string
  status: number
  signature_valid: boolean
  failure_message?: string | null
  created_at?: string | null
}

interface AdminPaymentGatewayLog {
  id: number
  payment_id?: number | null
  payment_session_id?: number | null
  payment_gateway_event_id?: number | null
  direction: number
  level: number
  message?: string | null
  created_at?: string | null
}

const { t, locale } = useI18n()
const { useApiFetch } = useApi()

const { pending: paymentsPending, data: paymentsData, error: paymentsError } = await useApiFetch<AdminPaymentListResponse>(
  '/api/admin/payments',
  { key: 'admin-monitoring-payments' }
)

const { pending: eventsPending, data: eventsData, error: eventsError } = await useApiFetch<AdminPaymentGatewayEvent[]>(
  '/api/admin/payment-gateway-events',
  { key: 'admin-monitoring-events' }
)

const { pending: logsPending, data: logsData, error: logsError } = await useApiFetch<AdminPaymentGatewayLog[]>(
  '/api/admin/payment-gateway-logs',
  { key: 'admin-monitoring-logs' }
)

const { pending: gatewaysPending, data: gatewaysData, error: gatewaysError } = await useApiFetch<AdminPaymentGateway[]>(
  '/api/admin/payment-gateways',
  { key: 'admin-monitoring-gateways' }
)

const isPending = computed(() => paymentsPending.value || eventsPending.value || logsPending.value || gatewaysPending.value)
const loadError = computed(() => paymentsError.value || eventsError.value || logsError.value || gatewaysError.value)

const payments = computed(() => paymentsData.value?.items || [])
const events = computed(() => eventsData.value || [])
const logs = computed(() => logsData.value || [])
const gateways = computed(() => gatewaysData.value || [])

const criticalEventStatuses = new Set([2, 4])
const alertPaymentStatuses = new Set([2, 3, 4, 6])

const eventAlerts = computed(() => {
  return [...events.value]
    .filter(event => criticalEventStatuses.has(event.status) || !event.signature_valid)
    .sort((left, right) => new Date(right.created_at || 0).getTime() - new Date(left.created_at || 0).getTime())
    .slice(0, 8)
})

const logAlerts = computed(() => {
  return [...logs.value]
    .filter(log => log.level >= 2)
    .sort((left, right) => new Date(right.created_at || 0).getTime() - new Date(left.created_at || 0).getTime())
    .slice(0, 6)
})

const paymentAlerts = computed(() => {
  return [...payments.value]
    .filter(payment => payment.status != null && alertPaymentStatuses.has(payment.status))
    .sort((left, right) => new Date(right.created_at || 0).getTime() - new Date(left.created_at || 0).getTime())
    .slice(0, 8)
})

const metrics = computed(() => [
  {
    label: t('admin.monitoring.metrics.failedEvents'),
    value: events.value.filter(event => event.status === 4).length,
    help: t('admin.monitoring.metrics.failedEventsHelp'),
    icon: 'icon-[tabler--webhook-off]',
    iconBg: 'bg-error/15 text-error'
  },
  {
    label: t('admin.monitoring.metrics.invalidSignatures'),
    value: events.value.filter(event => !event.signature_valid).length,
    help: t('admin.monitoring.metrics.invalidSignaturesHelp'),
    icon: 'icon-[tabler--shield-x]',
    iconBg: 'bg-warning/15 text-warning'
  },
  {
    label: t('admin.monitoring.metrics.errorLogs'),
    value: logs.value.filter(log => log.level === 3).length,
    help: t('admin.monitoring.metrics.errorLogsHelp'),
    icon: 'icon-[tabler--alert-octagon]',
    iconBg: 'bg-error/15 text-error'
  },
  {
    label: t('admin.monitoring.metrics.paymentsAtRisk'),
    value: payments.value.filter(payment => payment.status != null && alertPaymentStatuses.has(payment.status)).length,
    help: t('admin.monitoring.metrics.paymentsAtRiskHelp'),
    icon: 'icon-[tabler--credit-card-off]',
    iconBg: 'bg-info/15 text-info'
  }
])

const eventStatuses = [
  { value: 1, label: t('admin.monitoring.status.received'), badge: 'badge-info' },
  { value: 2, label: t('admin.monitoring.status.processing'), badge: 'badge-warning' },
  { value: 3, label: t('admin.monitoring.status.processed'), badge: 'badge-success' },
  { value: 4, label: t('admin.monitoring.status.failed'), badge: 'badge-error' },
  { value: 5, label: t('admin.monitoring.status.ignored'), badge: 'badge-neutral' }
]

const paymentStatuses = [
  { value: 1, label: t('admin.monitoring.status.checkout'), badge: 'badge-neutral' },
  { value: 2, label: t('admin.monitoring.status.pending'), badge: 'badge-warning' },
  { value: 3, label: t('admin.monitoring.status.processing'), badge: 'badge-info' },
  { value: 4, label: t('admin.monitoring.status.authorized'), badge: 'badge-primary' },
  { value: 5, label: t('admin.monitoring.status.captured'), badge: 'badge-success' },
  { value: 6, label: t('admin.monitoring.status.failed'), badge: 'badge-error' },
  { value: 7, label: t('admin.monitoring.status.voided'), badge: 'badge-neutral' },
  { value: 8, label: t('admin.monitoring.status.cancelled'), badge: 'badge-neutral' },
  { value: 9, label: t('admin.monitoring.status.refunded'), badge: 'badge-secondary' },
  { value: 10, label: t('admin.monitoring.status.partiallyRefunded'), badge: 'badge-secondary' }
]

const logLevels = [
  { value: 1, label: t('admin.monitoring.logs.levels.info'), badge: 'badge-info' },
  { value: 2, label: t('admin.monitoring.logs.levels.warning'), badge: 'badge-warning' },
  { value: 3, label: t('admin.monitoring.logs.levels.error'), badge: 'badge-error' }
]

function gatewayName(gatewayId: number) {
  return gateways.value.find(gateway => gateway.id === gatewayId)?.name || `#${gatewayId}`
}

function eventStatusLabel(status: number) {
  return eventStatuses.find(item => item.value === status)?.label || t('admin.statusLabels.unknown')
}

function eventStatusBadge(status: number) {
  return eventStatuses.find(item => item.value === status)?.badge || 'badge-ghost'
}

function paymentStatusLabel(status?: number | null) {
  return paymentStatuses.find(item => item.value === status)?.label || t('admin.statusLabels.unknown')
}

function paymentStatusBadge(status?: number | null) {
  return paymentStatuses.find(item => item.value === status)?.badge || 'badge-ghost'
}

function logLevelLabel(level: number) {
  return logLevels.find(item => item.value === level)?.label || t('admin.statusLabels.unknown')
}

function logLevelBadge(level: number) {
  return logLevels.find(item => item.value === level)?.badge || 'badge-ghost'
}

function formatDateTime(dateString?: string | null) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat(locale.value, {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

function formatCurrency(value?: number | string | null, currency = 'BRL') {
  const amount = Number(value || 0)
  return new Intl.NumberFormat(locale.value, {
    style: 'currency',
    currency
  }).format(amount)
}
</script>
