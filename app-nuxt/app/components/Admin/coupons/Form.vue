<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? t('admin.coupons.form.titleEdit') : t('admin.coupons.form.titleNew') }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">{{ t('admin.coupons.form.saving') }}</span>
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
          <!-- Code -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.coupons.form.code') }} *</span>
            </label>
            <input
              v-model="code"
              @blur="codeBlur"
              type="text"
              placeholder="PROMO2024"
              class="input input-bordered w-full uppercase"
              :class="{ 'input-error': codeError }"
              required
              :disabled="pending"
            />
            <label v-if="codeError" class="label">
              <span class="label-text-alt text-error">{{ codeError }}</span>
            </label>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Discount Type -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.coupons.form.type') }}</span>
              </label>
              <select               v-model.number="values.discount_type" class="select select-bordered w-full" :disabled="pending">
                <option :value="1">{{ t('admin.coupons.types.percentage') }} (%)</option>
                <option :value="2">{{ t('admin.coupons.types.fixed') }} (R$)</option>
                <option :value="3">{{ t('admin.coupons.types.freeShipping') }}</option>
              </select>
            </div>

            <!-- Discount Value -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.coupons.form.value') }}</span>
              </label>
              <input
                v-model.number="values.discount_value"
                type="number"
                step="0.01"
                min="0"
                placeholder="0.00"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Minimum Amount -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.coupons.form.minAmount') }}</span>
              </label>
              <input
                v-model.number="values.minimum_amount"
                type="number"
                step="0.01"
                min="0"
                placeholder="0.00"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <!-- Maximum Discount -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.coupons.form.maxDiscount') }}</span>
              </label>
              <input
                v-model.number="values.maximum_discount"
                type="number"
                step="0.01"
                min="0"
                placeholder="0.00"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Usage Limit -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.coupons.form.usageLimit') }}</span>
              </label>
              <input
                v-model.number="values.usage_limit"
                type="number"
                min="0"
                :placeholder="t('admin.coupons.form.usageLimitPlaceholder')"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <!-- Expiration Date -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.coupons.form.expiration') }}</span>
              </label>
              <input
                v-model="values.expires_at"
                type="datetime-local"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>
          </div>

          <!-- Active -->
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text font-semibold">{{ t('admin.coupons.form.active') }}</span>
              <input
                v-model="values.active"
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
              {{ isEditing ? t('admin.coupons.form.submitUpdate') : t('admin.coupons.form.submitSave') }} {{ t('admin.coupons.form.submitCoupon') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
import type { Coupon } from '~/types'
import { useForm, useField } from 'vee-validate'

interface Props {
  coupon?: Partial<Coupon>
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', coupon: Coupon): void
  (e: 'cancel'): void
}>()

const { apiFetch } = useApi()

const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
    code: '',
    discount_type: 1,
    discount_value: null as number | null,
    minimum_amount: null as number | null,
    maximum_discount: null as number | null,
    usage_limit: null as number | null,
    expires_at: '',
    active: true
  }
})

const { value: code, errorMessage: codeError, handleBlur: codeBlur } = useField<string>('code', (v) => {
  if (!v?.trim()) return t('admin.coupons.form.validation.codeRequired')
  return true
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

const formatDateTimeLocal = (dateString: string) => {
  const date = new Date(dateString)
  return date.toISOString().slice(0, 16)
}

onMounted(() => {
  if (props.coupon && props.isEditing) {
    setFieldValue('code', props.coupon.code || '')
    setFieldValue('discount_type', props.coupon.discount_type ?? 1)
    setFieldValue('discount_value', props.coupon.discount_value ?? null)
    setFieldValue('minimum_amount', props.coupon.minimum_amount ?? null)
    setFieldValue('maximum_discount', props.coupon.maximum_discount ?? null)
    setFieldValue('usage_limit', props.coupon.usage_limit ?? null)
    setFieldValue('expires_at', props.coupon.expires_at ? formatDateTimeLocal(props.coupon.expires_at) : '')
    setFieldValue('active', props.coupon.active ?? true)
  }
})

watch(() => props.coupon, (newCoupon) => {
  if (newCoupon && props.isEditing) {
    setFieldValue('code', newCoupon.code || '')
    setFieldValue('discount_type', newCoupon.discount_type ?? 1)
    setFieldValue('discount_value', newCoupon.discount_value ?? null)
    setFieldValue('minimum_amount', newCoupon.minimum_amount ?? null)
    setFieldValue('maximum_discount', newCoupon.maximum_discount ?? null)
    setFieldValue('usage_limit', newCoupon.usage_limit ?? null)
    setFieldValue('expires_at', newCoupon.expires_at ? formatDateTimeLocal(newCoupon.expires_at) : '')
    setFieldValue('active', newCoupon.active ?? true)
  }
}, { immediate: true })

const onSubmit = handleSubmit(async () => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      code: values.code.trim().toUpperCase(),
      discount_type: values.discount_type,
      discount_value: values.discount_value,
      minimum_amount: values.minimum_amount,
      maximum_discount: values.maximum_discount,
      usage_limit: values.usage_limit,
      expires_at: values.expires_at || null,
      active: values.active
    }

    const url = props.isEditing
      ? `/api/admin/coupons/${props.coupon?.id}`
      : '/api/admin/coupons'

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await apiFetch<Coupon>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? t('admin.coupons.form.success.updated')
      : t('admin.coupons.form.success.created')

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || t('admin.coupons.form.error')
  } finally {
    pending.value = false
  }
})
</script>

<style scoped></style>
