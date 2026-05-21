<template>
  <div>
    <div v-if="pending" class="flex items-center justify-center h-96">
      <span class="loading loading-spinner text-info size-40" />
    </div>

    <div v-else-if="!order" class="flex flex-col items-center justify-center py-20">
      <p class="text-lg text-base-content/60">{{ $t('common.status.unknown') }}</p>
      <NuxtLinkLocale to="/admin/orders" class="btn btn-primary mt-4">
        {{ $t('admin.orders.detail.back') }}
      </NuxtLinkLocale>
    </div>

    <div v-else>
      <div class="mb-6 flex items-center justify-between">
        <div>
          <NuxtLinkLocale to="/admin/orders" class="link link-hover text-sm text-base-content/60">
            &larr; {{ $t('admin.orders.detail.back') }}
          </NuxtLinkLocale>
          <h1 class="h1 mt-1">{{ $t('admin.orders.detail.title') }} - {{ order.order_number || '#' + order.id }}</h1>
        </div>
        <div class="flex gap-2 items-center">
          <span :class="statusBadgeClass(order.status)" class="text-sm">
            {{ statusLabel(order.status) }}
          </span>
          <span :class="paymentBadgeClass(order.payment_status)" class="text-sm">
            {{ paymentLabel(order.payment_status) }}
          </span>
        </div>
      </div>

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <div class="lg:col-span-2 space-y-6">
          <!-- Items Table -->
          <div class="card bg-base-100 shadow-sm border">
            <div class="card-body p-0">
              <div class="p-6 pb-0">
                <h3 class="font-semibold text-lg">{{ $t('admin.orders.table.items') }}</h3>
              </div>
              <div class="overflow-x-auto mt-4">
                <table class="table w-full">
                  <thead>
                    <tr>
                      <th>{{ $t('admin.products.table.name') }}</th>
                      <th>{{ $t('common.actions.quantity') || 'Qtd' }}</th>
                      <th class="text-right">{{ $t('admin.products.table.price') }}</th>
                      <th class="text-right">{{ $t('admin.orders.table.total') }}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="item in order.items || []" :key="item.id">
                      <td>{{ item.product_name || `Produto #${item.product_id}` }}</td>
                      <td>{{ item.quantity }}</td>
                      <td class="text-right">{{ formatNumberBR(item.price) }}</td>
                      <td class="text-right font-semibold">{{ formatNumberBR(item.total) }}</td>
                    </tr>
                  </tbody>
                  <tfoot>
                    <tr>
                      <td colspan="3" class="text-right text-base-content/60">{{ $t('admin.orders.detail.subtotal') }}</td>
                      <td class="text-right">{{ formatNumberBR(order.subtotal) }}</td>
                    </tr>
                    <tr v-if="order.shipping_amount">
                      <td colspan="3" class="text-right text-base-content/60">{{ $t('admin.orders.detail.shipping') }}</td>
                      <td class="text-right">{{ formatNumberBR(order.shipping_amount) }}</td>
                    </tr>
                    <tr v-if="order.discount_amount">
                      <td colspan="3" class="text-right text-base-content/60 text-error">{{ $t('admin.orders.detail.discount') }}</td>
                      <td class="text-right text-error">-{{ formatNumberBR(order.discount_amount) }}</td>
                    </tr>
                    <tr class="text-lg font-bold">
                      <td colspan="3" class="text-right">{{ $t('admin.orders.detail.total') }}</td>
                      <td class="text-right text-primary">{{ formatNumberBR(order.total_amount) }}</td>
                    </tr>
                  </tfoot>
                </table>
              </div>
            </div>
          </div>

          <!-- Customer & Shipping Summary -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Customer Info -->
            <div class="card bg-base-100 shadow-sm border">
              <div class="card-body">
                <h3 class="font-semibold mb-4 flex items-center gap-2">
                  <i class="icon-[tabler--user] size-5"></i>
                  {{ $t('admin.orders.detail.customer') }}
                </h3>
                <div v-if="pendingCustomer" class="flex justify-center py-4">
                  <span class="loading loading-spinner loading-sm"></span>
                </div>
                <div v-else-if="customerProfile" class="space-y-3">
                  <div class="flex items-center gap-3">
                    <div class="avatar avatar-placeholder">
                       <div class="bg-neutral text-neutral-content rounded-full size-10">
                         <span>{{ customerProfile.full_name?.[0] || 'U' }}</span>
                       </div>
                    </div>
                    <div>
                      <div class="font-medium">{{ customerProfile.full_name }}</div>
                      <div class="text-xs text-gray-500">ID: {{ customerProfile.id }}</div>
                    </div>
                  </div>
                  <div class="text-sm space-y-1">
                    <div class="flex items-center gap-2">
                      <i class="icon-[tabler--mail] size-4 text-gray-400"></i>
                      <span>{{ customerProfile.username || 'N/A' }}</span>
                    </div>
                    <div v-if="customerProfile.phone" class="flex items-center gap-2">
                      <i class="icon-[tabler--phone] size-4 text-gray-400"></i>
                      <span>{{ customerProfile.phone }}</span>
                    </div>
                  </div>
                  <NuxtLinkLocale :to="`/admin/customers/${customerProfile.id}`" class="btn btn-xs btn-outline btn-block mt-2">
                    {{ $t('admin.customers.detail.viewProfile', 'Ver Perfil Completo') }}
                  </NuxtLinkLocale>
                </div>
                <div v-else class="text-sm text-gray-500 italic">
                  {{ $t('admin.orders.detail.noCustomerInfo', { id: order.user_id }) }}
                </div>
              </div>
            </div>

            <!-- Shipping Address -->
            <div class="card bg-base-100 shadow-sm border">
              <div class="card-body">
                <h3 class="font-semibold mb-4 flex items-center gap-2">
                  <i class="icon-[tabler--map-pin] size-5"></i>
                  {{ $t('admin.orders.detail.shippingAddress') }}
                </h3>
                <div v-if="pendingAddresses" class="flex justify-center py-4">
                  <span class="loading loading-spinner loading-sm"></span>
                </div>
                <div v-else-if="shippingAddress" class="text-sm space-y-1">
                  <div class="font-medium">{{ shippingAddress.first_name }} {{ shippingAddress.last_name }}</div>
                  <div>{{ shippingAddress.address1 }}</div>
                  <div v-if="shippingAddress.address2">{{ shippingAddress.address2 }}</div>
                  <div>{{ shippingAddress.city }}, {{ shippingAddress.state }}</div>
                  <div>{{ shippingAddress.zip_code }} - {{ shippingAddress.country }}</div>
                  <div v-if="shippingAddress.phone" class="pt-1 text-gray-500">Tel: {{ shippingAddress.phone }}</div>
                </div>
                <div v-else class="text-sm text-gray-500 italic">
                  {{ $t('admin.orders.detail.addressNotFound') }}
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-6">
          <div class="card bg-base-100 shadow-sm border">
            <div class="card-body">
              <h3 class="font-semibold mb-4">{{ $t('order.updateStatus') }}</h3>
              <select v-model="selectedStatus" class="select select-bordered w-full">
                <option value="" disabled>{{ $t('order.selectStatus') }}</option>
                <option v-for="s in availableStatuses" :key="s.value" :value="s.value">
                  {{ s.label }}
                </option>
              </select>
              <button class="btn btn-primary mt-3 w-full" :disabled="!selectedStatus || updating" @click="updateStatus">
                <span v-if="updating" class="loading loading-spinner" />
                {{ $t('common.save') }}
              </button>
              <p v-if="statusMsg" class="mt-2 text-center text-sm" :class="statusMsgType">{{ statusMsg }}</p>
            </div>
          </div>

          <div class="card bg-base-100 shadow-sm border">
            <div class="card-body">
              <h3 class="font-semibold mb-4">{{ $t('admin.orders.detail.summary') }}</h3>
              <div class="space-y-3 text-sm">
                <div class="flex justify-between">
                  <span class="text-base-content/60">{{ $t('common.table.date') }}</span>
                  <span>{{ formatDate(order.created_at) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-base-content/60">{{ $t('admin.orders.detail.currency') }}</span>
                  <span class="uppercase">{{ order.currency }}</span>
                </div>
                <div v-if="order.order_number" class="flex justify-between">
                  <span class="text-base-content/60">{{ $t('admin.orders.table.number', 'Número') }}</span>
                  <span class="font-mono">{{ order.order_number }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Order, Profile, Address } from '~/types'

definePageMeta({ layout: 'admin' })
const { t } = useI18n()
const { apiFetch, useApiFetch } = useApi()
const config = useRuntimeConfig()
const route = useRoute()

const id = route.params.id

// Fetch Order (non-blocking)
const { data: order, pending } = await useApiFetch<Order>(
  `/api/admin/orders/${id}`,
  { key: `admin-order-${id}` }
)

// Fetch Customer Profile (using user_id from order)
const { data: customerProfile, pending: pendingCustomer } = await useAsyncData<Profile>(
  `customer-${id}`,
  async () => {
    if (!order.value?.user_id) return null
    // We need to find the profile that matches this user_id
    const profiles = await apiFetch<Profile[]>('/api/admin/profiles')
    return profiles.find(p => p.user_id === order.value?.user_id) || null
  },
  { watch: [order] }
)

// Fetch Addresses for this customer
const { data: shippingAddress, pending: pendingAddresses } = await useAsyncData<Address>(
  `address-${id}`,
  async () => {
    if (!order.value?.user_id) return null
    const addresses = await apiFetch<Address[]>('/api/admin/addresses')
    // Try to find the default shipping address for this user
    return addresses.find(a => a.user_id === order.value?.user_id && a.default) || 
           addresses.find(a => a.user_id === order.value?.user_id) || null
  },
  { watch: [order] }
)

const selectedStatus = ref('')
const updating = ref(false)
const statusMsg = ref('')
const statusMsgType = ref('')

const statusMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.status.pending'), badge: 'badge-soft badge-warning' },
  2: { label: t('order.status.confirmed'), badge: 'badge-soft badge-info' },
  3: { label: t('order.status.processing'), badge: 'badge-soft badge-info' },
  4: { label: t('order.status.shipped'), badge: 'badge-soft badge-primary' },
  5: { label: t('order.status.delivered'), badge: 'badge-soft badge-success' },
  6: { label: t('order.status.cancelled'), badge: 'badge-soft badge-error' },
}

const paymentMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.paymentStatus.unpaid'), badge: 'badge-soft badge-error' },
  2: { label: t('order.paymentStatus.paid'), badge: 'badge-soft badge-success' },
  3: { label: t('order.paymentStatus.refunded'), badge: 'badge-soft badge-info' },
  4: { label: t('order.paymentStatus.partiallyRefunded'), badge: 'badge-soft badge-warning' },
}

const availableStatuses = computed(() => {
  const current = order.value?.status ?? 1
  const transitions: Record<number, number[]> = {
    1: [2, 6], 2: [3, 6], 3: [4, 6], 4: [5], 5: [], 6: [],
  }
  return (transitions[current] ?? []).map(v => ({ value: v, label: statusMap[v]?.label ?? '-' }))
})

function statusLabel(status: unknown): string {
  if (status == null) return '-'
  return statusMap[status as number]?.label ?? t('common.status.unknown')
}
function statusBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return statusMap[status as number]?.badge ?? 'badge-soft'
}
function paymentLabel(status: unknown): string {
  if (status == null) return '-'
  return paymentMap[status as number]?.label ?? t('common.status.unknown')
}
function paymentBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return paymentMap[status as number]?.badge ?? 'badge-soft'
}

function formatNumberBR(val: any) {
  const n = Number(val) || 0
  return new Intl.NumberFormat(useI18n().locale.value, { style: 'currency', currency: 'BRL' }).format(n)
}

function formatDate(dateString: string) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat(useI18n().locale.value, { dateStyle: 'medium', timeStyle: 'short' }).format(new Date(dateString))
}

async function updateStatus() {
  if (!selectedStatus.value || !order.value) return
  updating.value = true
  statusMsg.value = ''
  try {
    await apiFetch(`/api/admin/orders/${order.value.id}/status`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: { status: Number(selectedStatus.value) },
    })
    order.value.status = Number(selectedStatus.value)
    statusMsg.value = t('common.status.completed')
    statusMsgType.value = 'text-success'
    selectedStatus.value = ''
  } catch (err: any) {
    statusMsg.value = err?.data?.message || err?.message || t('common.status.unknown')
    statusMsgType.value = 'text-error'
  } finally {
    updating.value = false
  }
}
</script>
