<template>
  <div class="max-w-2xl mx-auto">
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
    </div>

    <div v-else>
      <div class="mb-6">
        <h1 class="h1">{{ t('variant.edit') }}</h1>
        <NuxtLinkLocale :to="`/admin/products/${route.params.id}/variants`" class="text-sm link link-primary">
          ← {{ t('common.back') }}
        </NuxtLinkLocale>
      </div>

      <form @submit.prevent="handleSave" class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.name') }}</legend>
            <input v-model="form.name" type="text" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.sku') }}</legend>
            <input v-model="form.sku" type="text" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.price') }}</legend>
            <input v-model.number="form.price" type="number" step="0.01" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.costPrice') }}</legend>
            <input v-model.number="form.cost_price" type="number" step="0.01" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.comparePrice') }}</legend>
            <input v-model.number="form.compare_price" type="number" step="0.01" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.inventory') }}</legend>
            <input v-model.number="form.inventory_quantity" type="number" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.weight') }}</legend>
            <input v-model.number="form.weight" type="number" step="0.01" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.barcode') }}</legend>
            <input v-model="form.barcode" type="text" class="input w-full" />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend">{{ t('variant.position') }}</legend>
            <input v-model.number="form.position" type="number" class="input w-full" />
          </fieldset>
        </div>

        <div class="flex items-center gap-2">
          <input v-model="form.active" type="checkbox" class="checkbox" id="active" />
          <label for="active">{{ t('variant.active') }}</label>
        </div>

        <div class="flex gap-4 pt-4">
          <button type="submit" :disabled="saving" class="btn btn-primary">
            <span v-if="saving" class="loading loading-spinner" />
            {{ t('common.save') }}
          </button>
          <NuxtLinkLocale :to="`/admin/products/${route.params.id}/variants`" class="btn btn-outline">
            {{ t('common.cancel') }}
          </NuxtLinkLocale>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductVariant } from '~/types'
definePageMeta({ layout: 'admin' })
const { t } = useI18n()
const route = useRoute()
const { apiFetch, useApiFetch } = useApi()
const toast = useAppToast()
const router = useRouter()
const productId = route.params.id
const variantId = route.params.variantId
const saving = ref(false)

const { data: variant, pending } = await useApiFetch<ProductVariant>(
  `/api/admin/variants/${variantId}`,
  { key: `variant-${variantId}` }
)

const form = reactive({
  name: '',
  sku: '',
  price: null as number | null,
  cost_price: null as number | null,
  compare_price: null as number | null,
  inventory_quantity: null as number | null,
  weight: null as number | null,
  barcode: '',
  position: null as number | null,
  active: true,
  product_id: Number(productId),
})

watch(variant, (v) => {
  if (!v) return
  form.name = v.name ?? ''
  form.sku = v.sku ?? ''
  form.price = v.price ?? null
  form.cost_price = v.cost_price ?? null
  form.compare_price = v.compare_price ?? null
  form.inventory_quantity = v.inventory_quantity ?? null
  form.weight = v.weight ?? null
  form.barcode = v.barcode ?? ''
  form.position = v.position ?? null
  form.active = v.active ?? true
  form.product_id = v.product_id
}, { immediate: true })

async function handleSave() {
  saving.value = true
  try {
    await apiFetch(`/api/admin/variants/${variantId}`, {
      method: 'PUT',
      body: form,
    })
    router.push(`/admin/products/${productId}/variants`)
  } catch (e) {
    toast.error(t('common.errorSave', { resource: t('variant.title').toLowerCase() }))
  } finally {
    saving.value = false
  }
}
</script>
