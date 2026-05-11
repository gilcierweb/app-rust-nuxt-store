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

      <NuxtLink to="/admin/coupons/new" class="btn btn-success">{{ $t('admin.coupons.add') }}</NuxtLink>
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
      <NuxtLink to="/admin/coupons/new" class="btn btn-primary mt-4">{{ $t('admin.coupons.createFirst') }}</NuxtLink>
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
                <NuxtLink
                  :to="`/admin/coupons/${coupon.id}`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.view')"
                >
                  <i class="icon-[tabler--eye] size-5"></i>
                </NuxtLink>
                <NuxtLink
                  :to="`/admin/coupons/${coupon.id}/edit`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.edit')"
                >
                  <i class="icon-[tabler--pencil] size-5"></i>
                </NuxtLink>
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
