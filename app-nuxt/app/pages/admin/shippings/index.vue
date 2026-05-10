<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">{{ $t('admin.shippings.title') }}</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="$t('admin.shippings.searchPlaceholder')"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">{{ $t('common.search') }}</button>
        </div>
      </form>

      <NuxtLink to="/admin/shippings/new" class="btn btn-success">{{ $t('admin.shippings.add') }}</NuxtLink>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.shippings.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>{{ $t('admin.shippings.error', { message: error.message }) }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredShippings.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">{{ $t('admin.shippings.notFound') }}</p>
      <NuxtLink to="/admin/shippings/new" class="btn btn-primary mt-4">{{ $t('admin.shippings.createFirst') }}</NuxtLink>
    </div>

    <!-- Shipping Methods Table -->
    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>{{ $t('admin.shippings.table.name') }}</th>
            <th>{{ $t('admin.shippings.table.code') }}</th>
            <th>{{ $t('admin.shippings.table.basePrice') }}</th>
            <th>{{ $t('admin.shippings.table.freeThreshold') }}</th>
            <th>{{ $t('admin.shippings.table.status') }}</th>
            <th>{{ $t('admin.shippings.table.date') }}</th>
            <th>{{ $t('admin.shippings.table.actions') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="shipping in filteredShippings" :key="shipping.id" class="row-hover">
            <td class="font-medium">{{ shipping.name }}</td>
            <td>
              <span class="badge badge-soft badge-sm font-mono">{{ shipping.code }}</span>
            </td>
            <td>{{ formatCurrency(shipping.base_price) }}</td>
            <td>
              <span v-if="shipping.free_shipping_threshold" class="text-success">
                {{ $t('admin.shippings.status.above', { threshold: formatCurrency(shipping.free_shipping_threshold) }) }}
              </span>
              <span v-else class="text-gray-400">-</span>
            </td>
            <td>
              <span :class="['badge badge-soft text-xs', shipping.active ? 'badge-success' : 'badge-error']">
                {{ shipping.active ? $t('admin.shippings.status.active') : $t('admin.shippings.status.inactive') }}
              </span>
            </td>
            <td>{{ formatDate(shipping.created_at) }}</td>
            <td>
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
                class="btn btn-circle btn-text btn-sm"
                :aria-label="$t('common.delete')"
                @click="confirmDelete(shipping)"
              >
                <span class="icon-[tabler--trash] size-5"></span>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
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
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL'
  }).format(value)
}

// Format date
const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
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

<style scoped></style>
