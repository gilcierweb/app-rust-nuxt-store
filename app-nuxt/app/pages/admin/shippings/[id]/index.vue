
<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLink to="/admin/shippings" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLink>
        <div>
          <h1 class="h1">{{ t('admin.shippings.detail.title') }}</h1>
          <p class="text-sm text-gray-500" v-if="shipping">ID: {{ shipping.id }}</p>
        </div>
      </div>

      <div v-if="shipping" class="flex gap-2">
        <button @click="deleteShipping" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          {{ t('common.delete') }}
        </button>
        <NuxtLink :to="`/admin/shippings/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          {{ t('common.edit') }}
        </NuxtLink>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">{{ t('admin.shippings.detail.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.shippings.error', { message: error.message }) }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">{{ t('common.actions.tryAgain') }}</button>
    </div>

    <!-- Content -->
    <div v-else-if="shipping" class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Main Info Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <div class="flex items-center gap-3 mb-4">
            <h2 class="card-title">{{ shipping.name }}</h2>
            <span class="badge badge-soft badge-sm font-mono">{{ shipping.code }}</span>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ t('admin.shippings.table.name') }}</span>
              </label>
              <div class="font-medium text-lg">{{ shipping.name }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ t('admin.shippings.table.code') }}</span>
              </label>
              <div class="font-mono bg-base-200 px-3 py-2 rounded-md inline-block">{{ shipping.code }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ t('admin.shippings.table.basePrice') }}</span>
              </label>
              <div class="font-medium text-xl text-primary">{{ formatCurrency(shipping.base_price) }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ t('admin.shippings.table.freeThreshold') }}</span>
              </label>
              <div class="font-medium">
                {{ shipping.free_shipping_threshold ? formatCurrency(shipping.free_shipping_threshold) : 'N/A' }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Status Card -->
      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title mb-4">{{ t('admin.shippings.detail.settings') }}</h2>

          <div class="flex flex-col gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ t('admin.shippings.table.status') }}</span>
              </label>
              <div>
                <span :class="['badge badge-lg', shipping.active ? 'badge-success' : 'badge-error']">
                  {{ shipping.active ? t('admin.shippings.status.active') : t('admin.shippings.status.inactive') }}
                </span>
              </div>
            </div>

            <div class="divider my-2"></div>

            <div class="text-xs text-gray-500 space-y-2">
              <div class="flex justify-between">
                <span>{{ t('pages.categories.item.createdAt') }}</span>
                <span class="font-medium">{{ formatDate(shipping.created_at) }}</span>
              </div>
              <div class="flex justify-between">
                <span>{{ t('pages.categories.item.updatedAt') }}</span>
                <span class="font-medium">{{ formatDate(shipping.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Not Found State -->
    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>{{ t('admin.shippings.notFound') }}</span>
      <NuxtLink to="/admin/shippings" class="btn btn-sm">{{ t('admin.shippings.detail.back') }}</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t, locale } = useI18n()
import type { ShippingMethod } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()
const router = useRouter()

const { pending, data: shipping, error, refresh } = useFetch<ShippingMethod>(
  `${config.public.baseURL}/api/shippings/${route.params.id}`
)

const formatCurrency = (value: number | undefined) => {
  if (value === undefined || value === null) return '-'
  return new Intl.NumberFormat(locale.value, {
    style: 'currency',
    currency: locale.value === 'pt-BR' ? 'BRL' : (locale.value === 'es' ? 'EUR' : 'USD')
  }).format(value)
}

const formatDate = (dateString?: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat(locale.value, {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

const deleteShipping = async () => {
  if (!shipping.value) return

  if (confirm(t('admin.shippings.detail.confirmDelete', { name: shipping.value.name }))) {
    try {
      await $fetch(`${config.public.baseURL}/api/shippings/${shipping.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/shippings')
    } catch (err) {
      alert(t('admin.shippings.detail.errorDelete'))
      console.error(err)
    }
  }
}
</script>
