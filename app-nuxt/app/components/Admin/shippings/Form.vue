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
              v-model="form.name"
              type="text"
              :placeholder="t('admin.shippings.form.namePlaceholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': errors.name }"
              required
              :disabled="pending"
            />
            <label v-if="errors.name" class="label">
              <span class="label-text-alt text-error">{{ errors.name }}</span>
            </label>
          </div>

          <!-- Code -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.shippings.form.code') }} *</span>
            </label>
            <input
              v-model="form.code"
              type="text"
              :placeholder="t('admin.shippings.form.codePlaceholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': errors.code }"
              required
              :disabled="pending"
            />
            <label v-if="errors.code" class="label">
              <span class="label-text-alt text-error">{{ errors.code }}</span>
            </label>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Base Price -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.shippings.form.basePrice') }} *</span>
              </label>
              <input
                v-model.number="form.base_price"
                type="number"
                step="0.01"
                min="0"
                placeholder="0.00"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.base_price }"
                required
                :disabled="pending"
              />
              <label v-if="errors.base_price" class="label">
                <span class="label-text-alt text-error">{{ errors.base_price }}</span>
              </label>
            </div>

            <!-- Free Shipping Threshold -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.shippings.form.freeThreshold') }}</span>
              </label>
              <input
                v-model.number="form.free_shipping_threshold"
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
                v-model="form.active"
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
const { t } = useI18n()
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

const { apiFetch } = useApi()

// Form state
const form = reactive({
  name: '',
  code: '',
  base_price: 0,
  free_shipping_threshold: null as number | null,
  active: true
})

const errors = reactive({
  name: '',
  code: '',
  base_price: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Populate form when editing
onMounted(() => {
  if (props.shipping && props.isEditing) {
    form.name = props.shipping.name || ''
    form.code = props.shipping.code || ''
    form.base_price = props.shipping.base_price ?? 0
    form.free_shipping_threshold = props.shipping.free_shipping_threshold ?? null
    form.active = props.shipping.active ?? true
  }
})

// Watch for shipping prop changes (in case it loads async)
watch(() => props.shipping, (newShipping) => {
  if (newShipping && props.isEditing) {
    form.name = newShipping.name || ''
    form.code = newShipping.code || ''
    form.base_price = newShipping.base_price ?? 0
    form.free_shipping_threshold = newShipping.free_shipping_threshold ?? null
    form.active = newShipping.active ?? true
  }
}, { immediate: true })

// Validation
const validate = () => {
  let isValid = true
  errors.name = ''
  errors.code = ''
  errors.base_price = ''

  if (!form.name.trim()) {
    errors.name = t('admin.shippings.form.validation.nameRequired')
    isValid = false
  }

  if (!form.code.trim()) {
    errors.code = t('admin.shippings.form.validation.codeRequired')
    isValid = false
  }

  if (form.base_price < 0) {
    errors.base_price = t('admin.shippings.form.validation.priceNegative')
    isValid = false
  }

  return isValid
}

// Submit
const onSubmit = async () => {
  if (!validate()) return

  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      name: form.name.trim(),
      code: form.code.trim().toLowerCase(),
      base_price: form.base_price,
      free_shipping_threshold: form.free_shipping_threshold,
      active: form.active
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
}
</script>

<style scoped></style>
