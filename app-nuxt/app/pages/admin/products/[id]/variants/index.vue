<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <div>
        <h1 class="h1">{{ t('variant.list') }}</h1>
        <NuxtLinkLocale :to="`/admin/products/${route.params.id}`" class="text-sm link link-primary">
          ← {{ t('common.back') }}
        </NuxtLinkLocale>
      </div>
      <NuxtLinkLocale :to="`/admin/products/${route.params.id}/variants/new`" class="btn btn-success">
        {{ t('variant.add') }}
      </NuxtLinkLocale>
    </div>

    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
    </div>

    <div v-else-if="variants && variants.length > 0" class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>{{ t('variant.name') }}</th>
            <th>{{ t('variant.sku') }}</th>
            <th>{{ t('variant.price') }}</th>
            <th>{{ t('variant.inventory') }}</th>
            <th>{{ t('variant.active') }}</th>
            <th>{{ t('common.actions.edit') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="v in variants" :key="v.id" class="row-hover">
            <td>{{ v.name || '-' }}</td>
            <td><code>{{ v.sku || '-' }}</code></td>
            <td>{{ v.price ? formatNumberBR(v.price) : '-' }}</td>
            <td>{{ v.inventory_quantity ?? '-' }}</td>
            <td>
              <span :class="['badge', v.active ? 'badge-success' : 'badge-error']">
                {{ v.active ? t('admin.statusLabels.active') : t('admin.statusLabels.inactive') }}
              </span>
            </td>
            <td>
              <NuxtLinkLocale :to="`/admin/products/${route.params.id}/variants/${v.id}/edit`" class="btn btn-circle btn-text btn-sm">
                <span class="icon-[tabler--pencil] size-5"></span>
              </NuxtLinkLocale>
              <button class="btn btn-circle btn-text btn-sm" @click="handleDelete(v.id)">
                <span class="icon-[tabler--trash] size-5"></span>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-else class="flex flex-col items-center justify-center py-20">
      <span class="icon-[tabler--box] mb-4 size-20 opacity-40" />
      <p class="text-lg text-base-content/60">{{ t('variant.noVariants') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductVariant } from '~/types'
definePageMeta({ layout: 'admin' })
const { t } = useI18n()
const route = useRoute()
const config = useRuntimeConfig()
const productId = route.params.id

const { data: variants, pending, refresh } = await useFetch<ProductVariant[]>(
  `${config.public.baseURL}/api/variants/list?product_id=${productId}`,
  { key: `variants-${productId}` }
)

async function handleDelete(variantId: number) {
  if (!confirm(t('variant.deleteConfirm'))) return
  try {
    await $fetch(`${config.public.baseURL}/api/variants/${variantId}`, { method: 'DELETE' })
    refresh()
  } catch (e) {
    console.error(e)
  }
}
</script>
