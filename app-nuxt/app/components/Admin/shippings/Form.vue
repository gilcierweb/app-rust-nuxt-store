<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? t('admin.shippings.form.titleEdit') : t('admin.shippings.form.titleNew') }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">{{ t('admin.shippings.form.saving') }}</span>
        </div>

        <!-- Alerts -->
        <AppAlert
          v-if="successMessage"
          type="success"
          :message="successMessage"
          :auto-close="3000"
          @close="successMessage = ''"
        />
        <AppAlert
          v-if="errorMessage"
          type="error"
          :message="errorMessage"
          :auto-close="5000"
          @close="errorMessage = ''"
          :dismissible="true"
        />

        <!-- Form -->
        <form @submit.prevent="onSubmit" class="space-y-6" novalidate>
          <!-- Name -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.shippings.form.name') }} *</span>
            </label>
            <input
              v-model="name"
              type="text"
              :placeholder="t('admin.shippings.form.namePlaceholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': nameError }"
              required
              :disabled="pending"
              @blur="nameBlur"
            />
            <label v-if="nameError" class="label">
              <span class="label-text-alt text-error">{{ nameError }}</span>
            </label>
          </div>

          <!-- Code -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.shippings.form.code') }} *</span>
            </label>
            <input
              v-model="code"
              type="text"
              :placeholder="t('admin.shippings.form.codePlaceholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': codeError }"
              required
              :disabled="pending"
              @blur="codeBlur"
            />
            <label v-if="codeError" class="label">
              <span class="label-text-alt text-error">{{ codeError }}</span>
            </label>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Base Price -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.shippings.form.basePrice') }} *</span>
              </label>
              <input
                v-model.number="base_price"
                type="number"
                step="0.01"
                min="0"
                placeholder="0.00"
                class="input input-bordered w-full"
                :class="{ 'input-error': base_priceError }"
                required
                :disabled="pending"
                @blur="base_priceBlur"
              />
              <label v-if="base_priceError" class="label">
                <span class="label-text-alt text-error">{{ base_priceError }}</span>
              </label>
            </div>

            <!-- Free Shipping Threshold -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.shippings.form.freeThreshold') }}</span>
              </label>
              <input
                v-model.number="free_shipping_threshold"
                type="number"
                step="0.01"
                min="0"
                :placeholder="t('admin.shippings.form.freeThresholdPlaceholder')"
                class="input input-bordered w-full"
                :disabled="pending"
              />
              <label class="label">
                <span class="label-text-alt text-gray-500">{{ t('admin.shippings.form.freeThresholdHint') }}</span>
              </label>
            </div>
          </div>

          <!-- Active -->
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text font-semibold">{{ t('admin.shippings.form.active') }}</span>
              <input
                v-model="active"
                type="checkbox"
                class="checkbox checkbox-primary"
                :disabled="pending"
              />
            </label>
          </div>

          <!-- Action Buttons -->
          <div class="flex justify-end space-x-4 pt-6">
            <button
              type="button"
              class="btn btn-outline"
              :disabled="pending"
              @click="emit('cancel')"
            >
              {{ t('common.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? t('admin.shippings.form.submitUpdate') : t('admin.shippings.form.submitSave') }} {{ t('admin.shippings.form.submitMethod') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useForm, useField } from 'vee-validate'
import type { ShippingMethod } from '~/types'

interface Props {
  shipping?: Partial<ShippingMethod>
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', shipping: ShippingMethod): void
  (e: 'cancel'): void
}>()

const { t } = useI18n()
const { apiFetch } = useApi()

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
    name: '',
    code: '',
    base_price: 0,
    free_shipping_threshold: null as number | null,
    active: true
  }
})

const { value: name, errorMessage: nameError, handleBlur: nameBlur } = useField<string>('name', (v) => {
  if (!v || !v.trim()) return t('admin.shippings.form.validation.nameRequired')
  return true
})

const { value: code, errorMessage: codeError, handleBlur: codeBlur } = useField<string>('code', (v) => {
  if (!v || !v.trim()) return t('admin.shippings.form.validation.codeRequired')
  return true
})

const { value: base_price, errorMessage: base_priceError, handleBlur: base_priceBlur } = useField<number>('base_price', (v) => {
  if (v < 0) return t('admin.shippings.form.validation.priceNegative')
  return true
})

const { value: free_shipping_threshold } = useField<number | null>('free_shipping_threshold')
const { value: active } = useField<boolean>('active')

// Populate form when editing
onMounted(() => {
  if (props.shipping && props.isEditing) {
    setFieldValue('name', props.shipping.name || '')
    setFieldValue('code', props.shipping.code || '')
    setFieldValue('base_price', props.shipping.base_price ?? 0)
    setFieldValue('free_shipping_threshold', props.shipping.free_shipping_threshold ?? null)
    setFieldValue('active', props.shipping.active ?? true)
  }
})

// Watch for shipping prop changes (in case it loads async)
watch(() => props.shipping, (newShipping) => {
  if (newShipping && props.isEditing) {
    setFieldValue('name', newShipping.name || '')
    setFieldValue('code', newShipping.code || '')
    setFieldValue('base_price', newShipping.base_price ?? 0)
    setFieldValue('free_shipping_threshold', newShipping.free_shipping_threshold ?? null)
    setFieldValue('active', newShipping.active ?? true)
  }
}, { immediate: true })

const onSubmit = handleSubmit(async (formValues) => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      name: formValues.name.trim(),
      code: formValues.code.trim().toLowerCase(),
      base_price: formValues.base_price,
      free_shipping_threshold: formValues.free_shipping_threshold,
      active: formValues.active
    }

    const url = props.isEditing
      ? `/api/admin/shippings/${props.shipping?.id}`
      : '/api/admin/shippings'

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await apiFetch<ShippingMethod>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? t('admin.shippings.form.success.updated')
      : t('admin.shippings.form.success.created')

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || t('admin.shippings.form.error')
  } finally {
    pending.value = false
  }
})
</script>

<style scoped></style>
