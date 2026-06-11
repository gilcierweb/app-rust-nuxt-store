<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.orders.title') }}</h1>
      <div class="flex gap-2">
        <button
          v-if="selectedIds.length > 0"
          class="btn btn-primary btn-sm"
          :disabled="exporting"
          @click="bulkExportInvoices"
        >
          <span v-if="exporting" class="loading loading-spinner loading-xs"></span>
          <span v-else class="icon-[tabler--download] size-4 mr-1"></span>
          {{ $t('admin.orders.exportSelected', 'Export Selected') }} ({{ selectedIds.length }})
        </button>
        <button class="btn btn-outline btn-sm">
          <i class="icon-[tabler--download] size-4 mr-2"></i>
          {{ $t('common.export', 'Exportar') }}
        </button>
      </div>
    </div>

    <!-- Filters & Search -->
    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <div class="flex flex-wrap gap-4 items-end">
          <div class="form-control flex-1 min-w-[240px]">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">{{ $t('common.search', 'Buscar Pedido') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 group-focus-within:text-primary transition-colors"></span>
              <input 
                v-model="searchQuery" 
                type="text" 
                :placeholder="$t('admin.orders.searchPlaceholder', 'Número do pedido...')" 
                class="input input-bordered w-full pl-10" 
              />
            </div>
          </div>

          <div class="form-control w-48">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">{{ $t('common.table.status', 'Status') }}</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">{{ $t('common.all', 'Todos') }}</option>
              <option v-for="(val, key) in statusMap" :key="key" :value="key">
                {{ val.label }}
              </option>
            </select>
          </div>

          <button class="btn btn-ghost" @click="resetFilters">
            {{ $t('common.actions.clear', 'Limpar') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-info size-12"></span>
      <p class="mt-4 text-gray-500">{{ $t('admin.orders.loading', 'Carregando histórico de pedidos...') }}</p>
    </div>

    <!-- Orders Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
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
              <th>{{ $t('admin.orders.table.number') }}</th>
              <th>{{ $t('admin.orders.table.date', 'Data') }}</th>
              <th>{{ $t('admin.orders.table.customer', 'Cliente (ID)') }}</th>
              <th>{{ $t('admin.orders.table.status', 'Status') }}</th>
              <th>{{ $t('admin.orders.table.payment', 'Pagamento') }}</th>
              <th class="text-right">{{ $t('admin.orders.table.total') }}</th>
              <th class="text-right">{{ $t('admin.orders.table.actions', 'Ações') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="order in filteredOrders" :key="order.id" class="hover:bg-base-200/30 transition-colors">
              <td>
                <input
                  type="checkbox"
                  class="checkbox checkbox-sm"
                  :checked="selectedIds.includes(order.id)"
                  @change="toggleSelect(order.id)"
                />
              </td>
              <td class="font-mono text-sm font-bold text-primary">
                {{ order.order_number || `#${order.id}` }}
              </td>
              <td>
                <div class="text-sm font-medium">{{ formatDate(order.created_at) }}</div>
                <div class="text-xs text-gray-400">{{ formatTime(order.created_at) }}</div>
              </td>
              <td>
                <div class="flex items-center gap-2">
                  <div class="avatar avatar-xs placeholder">
                    <div class="bg-neutral text-neutral-content rounded-full size-6">
                      <span class="text-[10px]">U</span>
                    </div>
                  </div>
                  <span class="text-sm">User #{{ order.user_id }}</span>
                </div>
              </td>
              <td>
                <span :class="statusBadgeClass(order.status)" class="badge badge-sm px-2">
                  {{ statusLabel(order.status) }}
                </span>
              </td>
              <td>
                <span :class="paymentBadgeClass(order.payment_status)" class="badge badge-soft badge-sm">
                  {{ paymentLabel(order.payment_status) }}
                </span>
              </td>
              <td class="text-right font-bold text-base">
                {{ formatNumberBR(order.total_amount) }}
              </td>
              <td class="text-right">
                <NuxtLinkLocale :to="`/admin/orders/${order.id}`" class="btn btn-circle btn-text btn-sm" :aria-label="$t('common.view')">
                  <i class="icon-[tabler--eye] size-5" />
                </NuxtLinkLocale>
              </td>
            </tr>
            <tr v-if="filteredOrders.length === 0">
              <td colspan="8" class="text-center py-20 text-gray-500 italic">
                {{ $t('admin.orders.notFound', 'Nenhum pedido encontrado.') }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Order } from '~/types'

definePageMeta({ layout: 'admin' })
const { t } = useI18n()
const { useApiFetch, apiFetch } = useApi()

// Filters
const searchQuery = ref('')
const selectedStatus = ref('')
const selectedIds = ref<number[]>([])
const exporting = ref(false)

const { data: ordersData, pending } = await useApiFetch<Order[]>(
  '/api/admin/orders',
  { key: 'admin-orders-list' }
)

const orders = computed(() => ordersData.value ?? [])

const filteredOrders = computed(() => {
  return orders.value.filter(o => {
    const matchesSearch = o.order_number?.toLowerCase().includes(searchQuery.value.toLowerCase()) || 
                         o.id.toString().includes(searchQuery.value)
    const matchesStatus = !selectedStatus.value || o.status === Number(selectedStatus.value)
    return matchesSearch && matchesStatus
  }).sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
})

const allFilteredSelected = computed(() => {
  if (filteredOrders.value.length === 0) return false
  return filteredOrders.value.every(o => selectedIds.value.includes(o.id))
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
    const filteredIds = new Set(filteredOrders.value.map(o => o.id))
    selectedIds.value = selectedIds.value.filter(id => !filteredIds.has(id))
  } else {
    const filteredIds = filteredOrders.value.map(o => o.id)
    const existing = new Set(selectedIds.value)
    for (const id of filteredIds) {
      if (!existing.has(id)) {
        selectedIds.value.push(id)
      }
    }
  }
}

const resetFilters = () => {
  searchQuery.value = ''
  selectedStatus.value = ''
}

async function bulkExportInvoices() {
  if (selectedIds.value.length === 0) return
  exporting.value = true
  try {
    const blob = await apiFetch<Blob>('/api/admin/orders/bulk-export', {
      method: 'POST',
      body: { ids: selectedIds.value },
      responseType: 'blob',
    })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'invoices.zip'
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (err: any) {
    console.error('Failed to bulk export invoices:', err)
  } finally {
    exporting.value = false
  }
}

const statusMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.status.pending'), badge: 'badge-warning' },
  2: { label: t('order.status.confirmed'), badge: 'badge-info' },
  3: { label: t('order.status.processing'), badge: 'badge-info' },
  4: { label: t('order.status.shipped'), badge: 'badge-primary' },
  5: { label: t('order.status.delivered'), badge: 'badge-success' },
  6: { label: t('order.status.cancelled'), badge: 'badge-error' },
}

const paymentMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.paymentStatus.unpaid'), badge: 'badge-error' },
  2: { label: t('order.paymentStatus.paid'), badge: 'badge-success' },
  3: { label: t('order.paymentStatus.refunded'), badge: 'badge-info' },
  4: { label: t('order.paymentStatus.partiallyRefunded'), badge: 'badge-warning' },
}

function statusLabel(status: unknown): string {
  if (status == null) return '-'
  return statusMap[status as number]?.label ?? 'Desconhecido'
}
function statusBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return statusMap[status as number]?.badge ?? 'badge-soft'
}
function paymentLabel(status: unknown): string {
  if (status == null) return '-'
  return paymentMap[status as number]?.label ?? 'Desconhecido'
}
function paymentBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return paymentMap[status as number]?.badge ?? 'badge-soft'
}

function formatNumberBR(val: any) {
  return new Intl.NumberFormat('pt-BR', { style: 'currency', currency: 'BRL' }).format(Number(val) || 0)
}

function formatDate(dateString: string) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', { dateStyle: 'medium' }).format(new Date(dateString))
}

function formatTime(dateString: string) {
  if (!dateString) return ''
  return new Intl.DateTimeFormat('pt-BR', { timeStyle: 'short' }).format(new Date(dateString))
}
</script>
