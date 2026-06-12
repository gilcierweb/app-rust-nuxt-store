<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">{{ $t('admin.coupons.title') }}</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="$t('admin.coupons.searchPlaceholder')"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">{{ $t('common.search') }}</button>
        </div>
      </form>

      <NuxtLinkLocale to="/admin/coupons/new" class="btn btn-success">{{ $t('admin.coupons.add') }}</NuxtLinkLocale>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.coupons.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>{{ $t('admin.coupons.error', { message: error.message }) }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredCoupons.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">{{ $t('admin.coupons.notFound') }}</p>
      <NuxtLinkLocale to="/admin/coupons/new" class="btn btn-primary mt-4">{{ $t('admin.coupons.createFirst') }}</NuxtLinkLocale>
    </div>

    <!-- Coupons Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.coupons.table.code') }}</th>
              <th>{{ $t('admin.coupons.table.type') }}</th>
              <th>{{ $t('admin.coupons.table.value') }}</th>
              <th>{{ $t('admin.coupons.table.usage') }}</th>
              <th>{{ $t('admin.coupons.table.expiration') }}</th>
              <th>{{ $t('admin.coupons.table.status') }}</th>
              <th>{{ $t('admin.coupons.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="coupon in filteredCoupons" :key="coupon.id" class="row-hover">
              <td>
                <span class="badge badge-primary badge-lg font-mono">{{ coupon.code }}</span>
              </td>
              <td>{{ discountTypeLabel(coupon.discount_type) }}</td>
              <td>{{ formatDiscountValue(coupon) }}</td>
              <td>
                {{ coupon.used_count || 0 }}/{{ coupon.usage_limit || '∞' }}
                <div class="w-24 h-1 bg-base-200 rounded mt-1">
                  <div 
                    class="h-full bg-primary rounded" 
                    :style="{ width: usagePercentage(coupon) + '%' }"
                  ></div>
                </div>
              </td>
              <td>
                <span v-if="coupon.expires_at" :class="isExpired(coupon.expires_at) ? 'text-error' : ''">
                  {{ formatDate(coupon.expires_at) }}
                </span>
                <span v-else class="text-gray-400">-</span>
              </td>
              <td>
                <span :class="['badge badge-soft text-xs', coupon.active ? 'badge-success' : 'badge-error']">
                  {{ coupon.active ? $t('admin.coupons.detail.active') : $t('admin.coupons.detail.inactive') }}
                </span>
              </td>
              <td>
                <NuxtLinkLocale
                  :to="`/admin/coupons/${coupon.id}`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.view')"
                >
                  <i class="icon-[tabler--eye] size-5"></i>
                </NuxtLinkLocale>
                <NuxtLinkLocale
                  :to="`/admin/coupons/${coupon.id}/edit`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.edit')"
                >
                  <i class="icon-[tabler--pencil] size-5"></i>
                </NuxtLinkLocale>
                <button
                  type="button"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.delete')"
                  @click="confirmDelete(coupon)"
                >
                  <span class="icon-[tabler--trash] size-5"></span>
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Coupon } from '~/types'

definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiFetch } = useApi()
const { t } = useI18n()
const toast = useAppToast()
const dialog = useAppDialog()

const searchQuery = ref('')

const { pending, data: coupons, error, refresh } = await useApiFetch<Coupon[]>(
  '/api/admin/coupons',
  { key: 'admin-coupons-list' }
)

// Filtered coupons based on search
const filteredCoupons = computed(() => {
  if (!coupons.value) return []
  if (!searchQuery.value.trim()) return coupons.value

  const query = searchQuery.value.toLowerCase()
  return coupons.value.filter(coupon =>
    coupon.code?.toLowerCase().includes(query)
  )
})

// Discount type label
const discountTypeLabel = (type?: number) => {
  switch (type) {
    case 1: return t('admin.coupons.types.percentage')
    case 2: return t('admin.coupons.types.fixed')
    case 3: return t('admin.coupons.types.freeShipping')
    default: return t('admin.coupons.types.unknown')
  }
}

const toFiniteNumber = (value?: string | number | null) => {
  const numericValue = Number(value ?? 0)
  return Number.isFinite(numericValue) ? numericValue : 0
}

// Format discount value
const formatDiscountValue = (coupon: Coupon) => {
  if (coupon.discount_type === 1) {
    return `${toFiniteNumber(coupon.discount_value)}%`
  } else if (coupon.discount_type === 2) {
    return `R$ ${toFiniteNumber(coupon.discount_value).toFixed(2)}`
  }
  return '-'
}

// Usage percentage for progress bar
const usagePercentage = (coupon: Coupon) => {
  if (!coupon.usage_limit || coupon.usage_limit === 0) return 0
  const used = coupon.used_count || 0
  return Math.min((used / coupon.usage_limit) * 100, 100)
}

// Check if expired
const isExpired = (expiresAt: string) => {
  return new Date(expiresAt) < new Date()
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
const confirmDelete = async (coupon: Coupon) => {
  const confirmed = await dialog.confirm({
    message: t('admin.coupons.detail.confirmDelete', { name: coupon.code }),
    confirmLabel: t('common.delete'),
    tone: 'danger'
  })
  if (!confirmed) return

  try {
    await apiFetch(`/api/admin/coupons/${coupon.id}`, {
      method: 'DELETE'
    })
    await refresh()
  } catch (err) {
    toast.error(t('admin.coupons.detail.errorDelete'))
  }
}
</script>
