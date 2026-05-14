<template>
  <div class="max-w-2xl mx-auto">
    <div class="mb-6">
      <h1 class="h1">{{ t('variant.add') }}</h1>
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
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })
const { t } = useI18n()
const route = useRoute()
const config = useRuntimeConfig()
const router = useRouter()
const productId = route.params.id
const saving = ref(false)

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

async function handleSave() {
  saving.value = true
  try {
    await $fetch(`${config.public.baseURL}/api/admin/variants/`, {
      method: 'POST',
      body: form,
    })
    router.push(`/admin/products/${productId}/variants`)
  } catch (e) {
    console.error(e)
  } finally {
    saving.value = false
  }
}
</script>
