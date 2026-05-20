<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/orders" class="btn btn-circle btn-text btn-sm">
          <i class="icon-[tabler--arrow-left] size-5"></i>
        </NuxtLinkLocale>
        <h1 class="h1">Payment #{{ paymentId }}</h1>
      </div>
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
            <h2 class="card-title">Overview</h2>
            <div class="grid grid-cols-2 gap-4 mt-4">
              <div>
                <p class="text-sm text-gray-500">Amount</p>
                <p class="font-medium text-lg">{{ formatCurrency(detail.payment.amount, detail.payment.currency || 'USD') }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500">Status</p>
                <span :class="['badge', statusBadge(detail.payment.status)]">
                  {{ statusLabel(detail.payment.status) }}
                </span>
              </div>
              <div>
                <p class="text-sm text-gray-500">Order ID</p>
                <NuxtLinkLocale :to="`/admin/orders/${detail.payment.order_id}`" class="link link-primary">
                  #{{ detail.payment.order_id }}
                </NuxtLinkLocale>
              </div>
              <div>
                <p class="text-sm text-gray-500">Transaction ID</p>
                <p class="font-mono text-sm">{{ detail.payment.transaction_id || '-' }}</p>
              </div>
              <div>
                <p class="text-sm text-gray-500">Created At</p>
                <p>{{ formatDate(detail.payment.created_at) }}</p>
              </div>
            </div>
            
            <div class="divider"></div>
            
            <div class="flex gap-2">
              <button class="btn btn-primary" @click="capturePayment" :disabled="isActionLoading">
                Capture
              </button>
              <button class="btn btn-warning" @click="voidPayment" :disabled="isActionLoading">
                Void
              </button>
              <button class="btn btn-error" @click="refundPayment" :disabled="isActionLoading">
                Refund
              </button>
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
                  <tr v-for="event in detail.events" :key="event.id">
                    <td><span class="badge badge-soft">{{ event.event_type }}</span></td>
                    <td class="font-mono text-xs">{{ event.external_event_id }}</td>
                    <td>
                      <span :class="['badge badge-sm', eventStatusBadge(event.status)]">
                        {{ eventStatusLabel(event.status) }}
                      </span>
                    </td>
                    <td>{{ formatDate(event.created_at) }}</td>
                  </tr>
                  <tr v-if="detail.events.length === 0">
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
              <div v-for="log in detail.logs" :key="log.id" class="p-3 bg-base-200 rounded-lg text-sm">
                <div class="flex justify-between mb-1">
                  <span class="font-medium">{{ logLevelLabel(log.log_level) }}</span>
                  <span class="text-gray-500 text-xs">{{ formatDate(log.created_at) }}</span>
                </div>
                <p class="text-gray-700">{{ log.message }}</p>
                <pre v-if="log.details" class="mt-2 p-2 bg-base-300 rounded text-xs overflow-x-auto">{{ log.details }}</pre>
              </div>
              <div v-if="detail.logs.length === 0" class="text-center py-4 text-gray-500">
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

const { pending, data: detail, error, refresh } = await useApiFetch<any>(
  `/api/admin/payments/${paymentId}`,
  { key: `admin-payment-${paymentId}` }
)

const isActionLoading = ref(false)

const capturePayment = async () => {
  if (!confirm('Are you sure you want to capture this payment?')) return
  isActionLoading.value = true
  try {
    await apiFetch(`/api/admin/payments/${paymentId}/capture`, { method: 'POST' })
    await refresh()
  } catch (e: any) {
    alert(`Capture failed: ${e.message}`)
  } finally {
    isActionLoading.value = false
  }
}

const voidPayment = async () => {
  if (!confirm('Are you sure you want to void this payment?')) return
  isActionLoading.value = true
  try {
    await apiFetch(`/api/admin/payments/${paymentId}/void`, { method: 'POST' })
    await refresh()
  } catch (e: any) {
    alert(`Void failed: ${e.message}`)
  } finally {
    isActionLoading.value = false
  }
}

const refundPayment = async () => {
  const amountStr = prompt('Enter amount to refund (leave empty for full amount):')
  if (amountStr === null) return // Cancelled
  
  let amount = null
  if (amountStr.trim() !== '') {
    amount = parseFloat(amountStr)
    if (isNaN(amount) || amount <= 0) {
      alert('Invalid amount')
      return
    }
  }

  isActionLoading.value = true
  try {
    await apiFetch(`/api/admin/payments/${paymentId}/refund`, {
      method: 'POST',
      body: { amount }
    })
    await refresh()
  } catch (e: any) {
    alert(`Refund failed: ${e.message}`)
  } finally {
    isActionLoading.value = false
  }
}

const formatCurrency = (value: string | number, currency: string) => {
  if (value === null || value === undefined) return '-'
  return new Intl.NumberFormat('pt-BR', { style: 'currency', currency }).format(Number(value))
}

const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Date(dateString).toLocaleString()
}

const statusLabel = (status: number) => {
  switch (status) {
    case 1: return 'Pending'
    case 2: return 'Authorized'
    case 3: return 'Captured'
    case 4: return 'Failed'
    case 5: return 'Cancelled'
    case 6: return 'Refunded'
    default: return 'Unknown'
  }
}

const statusBadge = (status: number) => {
  switch (status) {
    case 1: return 'badge-warning'
    case 2: return 'badge-info'
    case 3: return 'badge-success'
    case 4: return 'badge-error'
    case 5: return 'badge-neutral'
    case 6: return 'badge-soft badge-error'
    default: return 'badge-ghost'
  }
}

const eventStatusLabel = (status: number) => {
  switch (status) {
    case 1: return 'Received'
    case 2: return 'Processed'
    case 3: return 'Failed'
    case 4: return 'Ignored'
    default: return 'Unknown'
  }
}

const eventStatusBadge = (status: number) => {
  switch (status) {
    case 1: return 'badge-info'
    case 2: return 'badge-success'
    case 3: return 'badge-error'
    case 4: return 'badge-neutral'
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
