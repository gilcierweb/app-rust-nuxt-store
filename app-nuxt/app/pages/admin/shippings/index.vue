<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.shippings.title') }}</h1>
      <NuxtLink to="/admin/shippings/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.shippings.add') }}
      </NuxtLink>
    </div>

    <!-- Filters & Search -->
    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <form @submit.prevent="handleSearch" class="flex flex-wrap gap-4 items-end">
          <div class="form-control flex-1 min-w-[240px]">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">{{ t('admin.shippings.searchLabel') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 group-focus-within:text-primary transition-colors"></span>
              <input 
                v-model="searchQuery" 
                type="text" 
                :placeholder="$t('admin.shippings.searchPlaceholder')" 
                class="input input-bordered w-full pl-10" 
              />
            </div>
          </div>
          <button type="submit" class="btn btn-ghost">
            {{ t('admin.shippings.clearSearch') }}
          </button>
        </form>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12" />
      <p class="mt-4 text-gray-500">{{ $t('admin.shippings.loading') }}</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error mb-6">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.shippings.error', { message: error.message }) }}</span>
    </div>

    <!-- Shipping Methods Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.shippings.table.name') }}</th>
              <th>{{ $t('admin.shippings.table.code') }}</th>
              <th>{{ $t('admin.shippings.table.basePrice') }}</th>
              <th>{{ $t('admin.shippings.table.freeThreshold') }}</th>
              <th>{{ $t('admin.shippings.table.status') }}</th>
              <th>{{ $t('admin.shippings.table.date') }}</th>
              <th class="text-right">{{ $t('admin.shippings.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="shipping in filteredShippings" :key="shipping.id" class="row-hover">
              <td class="font-medium text-primary">{{ shipping.name }}</td>
              <td>
                <span class="badge badge-soft badge-sm font-mono">{{ shipping.code }}</span>
              </td>
              <td class="font-semibold">{{ formatCurrency(shipping.base_price) }}</td>
              <td>
                <span v-if="shipping.free_shipping_threshold" class="badge badge-soft badge-success text-[10px]">
                   + {{ formatCurrency(shipping.free_shipping_threshold) }}
                </span>
                <span v-else class="text-gray-400 text-xs">-</span>
              </td>
              <td>
                <span :class="['badge badge-soft text-xs', shipping.active ? 'badge-success' : 'badge-error']">
                  {{ shipping.active ? $t('admin.shippings.status.active') : $t('admin.shippings.status.inactive') }}
                </span>
              </td>
              <td><div class="text-sm">{{ formatDate(shipping.created_at) }}</div></td>
              <td class="text-right">
                <div class="flex justify-end gap-1">
                  <NuxtLink
                    :to="`/admin/shippings/${shipping.id}`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.view')"
                  >
                    <i class="icon-[tabler--eye] size-5"></i>
                  </NuxtLink>
                  <NuxtLink
                    :to="`/admin/shippings/${shipping.id}/edit`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.edit')"
                  >
                    <i class="icon-[tabler--pencil] size-5"></i>
                  </NuxtLink>
                  <button
                    type="button"
                    class="btn btn-circle btn-text btn-sm text-error"
                    :aria-label="$t('common.delete')"
                    @click="confirmDelete(shipping)"
                  >
                    <i class="icon-[tabler--trash] size-5"></i>
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="filteredShippings.length === 0">
              <td colspan="7" class="text-center py-20 text-gray-500 italic">
                {{ $t('admin.shippings.notFound') }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ShippingMethod } from '~/types'

definePageMeta({
  layout: 'admin'
})

const config = useRuntimeConfig()
const { t } = useI18n()

const searchQuery = ref('')

const { pending, data: shippings, error, refresh } = useLazyFetch<ShippingMethod[]>(
  `${config.public.baseURL}/api/shippings`
)

// Filtered shippings based on search
const filteredShippings = computed(() => {
  if (!shippings.value) return []
  if (!searchQuery.value.trim()) return shippings.value

  const query = searchQuery.value.toLowerCase()
  return shippings.value.filter(shipping =>
    shipping.name?.toLowerCase().includes(query) ||
    shipping.code?.toLowerCase().includes(query)
  )
})

// Format currency
const formatCurrency = (value: number | undefined) => {
  if (value === undefined || value === null) return '-'
  return new Intl.NumberFormat(locale.value, {
    style: 'currency',
    currency: locale.value === 'pt-BR' ? 'BRL' : (locale.value === 'es' ? 'EUR' : 'USD')
  }).format(value)
}

// Format date
const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat(locale.value, {
    day: '2-digit',
    month: 'short',
    year: 'numeric'
  }).format(new Date(dateString))
}

// Search handler
const handleSearch = () => {
  // Search is handled reactively via computed
}

// Delete confirmation
const confirmDelete = async (shipping: ShippingMethod) => {
  if (confirm(t('admin.shippings.detail.confirmDelete', { name: shipping.name }))) {
    try {
      await $fetch(`${config.public.baseURL}/api/shippings/${shipping.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert(t('admin.shippings.detail.errorDelete'))
      console.error(err)
    }
  }
}
</script>
