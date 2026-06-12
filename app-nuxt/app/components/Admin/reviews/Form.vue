<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? t('admin.reviews.form.titleEdit') : t('admin.reviews.form.titleNew') }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">{{ t('admin.reviews.form.saving') }}</span>
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
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Product ID -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.reviews.form.productId') }} *</span>
              </label>
              <input
                v-model.number="productId"
                type="number"
                :placeholder="t('admin.reviews.form.productIdPlaceholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': productIdError }"
                required
                :disabled="pending"
                @blur="productIdBlur"
              />
              <label v-if="productIdError" class="label">
                <span class="label-text-alt text-error">{{ productIdError }}</span>
              </label>
            </div>

            <!-- User ID -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.reviews.form.userId') }} *</span>
              </label>
              <input
                v-model.number="userId"
                type="number"
                :placeholder="t('admin.reviews.form.userIdPlaceholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': userIdError }"
                required
                :disabled="pending"
                @blur="userIdBlur"
              />
              <label v-if="userIdError" class="label">
                <span class="label-text-alt text-error">{{ userIdError }}</span>
              </label>
            </div>
          </div>

          <!-- Rating -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.reviews.form.rating') }} *</span>
            </label>
            <div class="flex items-center gap-2">
              <input
                v-model.number="rating"
                type="range"
                min="1"
                max="5"
                class="range range-primary flex-1"
                :disabled="pending"
                @blur="ratingBlur"
              />
              <span class="text-2xl font-bold w-8 text-center">{{ rating }}</span>
              <div class="flex text-warning">
                <i v-for="n in 5" :key="n" 
                   :class="['icon-[tabler--star]', n <= rating ? 'filled' : '', n <= rating ? 'text-warning' : 'text-gray-300']" 
                   class="size-6"></i>
              </div>
            </div>
            <label v-if="ratingError" class="label">
              <span class="label-text-alt text-error">{{ ratingError }}</span>
            </label>
          </div>

          <!-- Title -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.reviews.form.title') }}</span>
            </label>
            <input
              v-model="values.title"
              type="text"
              :placeholder="t('admin.reviews.form.titlePlaceholder')"
              class="input input-bordered w-full"
              :disabled="pending"
            />
          </div>

          <!-- Comment -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.reviews.form.comment') }}</span>
            </label>
            <textarea
              v-model="values.comment"
              :placeholder="t('admin.reviews.form.commentPlaceholder')"
              class="textarea textarea-bordered w-full"
              rows="4"
              :disabled="pending"
            ></textarea>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Verified Purchase -->
            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text font-semibold">{{ t('admin.reviews.form.verifiedPurchase') }}</span>
                <input
                  v-model="values.verified_purchase"
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  :disabled="pending"
                />
              </label>
            </div>

            <!-- Active -->
            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text font-semibold">{{ t('admin.reviews.form.active') }}</span>
                <input
                  v-model="values.active"
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  :disabled="pending"
                />
              </label>
            </div>
          </div>

          <!-- Action Buttons -->
          <div class="flex justify-end space-x-4 pt-6">
            <button
              type="button"
              class="btn btn-outline"
              :disabled="pending"
              @click="emit('cancel')"
            >
              {{ t('admin.reviews.form.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? t('admin.reviews.form.submitUpdate') : t('admin.reviews.form.submitSave') }} {{ t('admin.reviews.form.submitReview') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useForm, useField } from 'vee-validate'
import type { Review } from '~/types'

interface Props {
  review?: Partial<Review>
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', review: Review): void
  (e: 'cancel'): void
}>()

const { apiFetch } = useApi()
const { t } = useI18n()

// Form state
const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
    product_id: null as number | null,
    user_id: null as number | null,
    rating: 5,
    title: '',
    comment: '',
    verified_purchase: true,
    active: true
  }
})

const { value: productId, errorMessage: productIdError, handleBlur: productIdBlur } = useField<number | null>('product_id', (v) => {
  if (!v) return t('admin.reviews.form.validation.productIdRequired')
  return true
})

const { value: userId, errorMessage: userIdError, handleBlur: userIdBlur } = useField<number | null>('user_id', (v) => {
  if (!v) return t('admin.reviews.form.validation.userIdRequired')
  return true
})

const { value: rating, errorMessage: ratingError, handleBlur: ratingBlur } = useField<number>('rating', (v) => {
  if (!v || v < 1 || v > 5) return t('admin.reviews.form.validation.ratingRange')
  return true
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Populate form when editing
onMounted(() => {
  if (props.review && props.isEditing) {
    setFieldValue('product_id', props.review.product_id ?? null)
    setFieldValue('user_id', props.review.user_id ?? null)
    setFieldValue('rating', props.review.rating ?? 5)
    setFieldValue('title', props.review.title || '')
    setFieldValue('comment', props.review.comment || '')
    setFieldValue('verified_purchase', props.review.verified_purchase ?? true)
    setFieldValue('active', props.review.active ?? true)
  }
})

// Watch for review prop changes (in case it loads async)
watch(() => props.review, (newReview) => {
  if (newReview && props.isEditing) {
    setFieldValue('product_id', newReview.product_id ?? null)
    setFieldValue('user_id', newReview.user_id ?? null)
    setFieldValue('rating', newReview.rating ?? 5)
    setFieldValue('title', newReview.title || '')
    setFieldValue('comment', newReview.comment || '')
    setFieldValue('verified_purchase', newReview.verified_purchase ?? true)
    setFieldValue('active', newReview.active ?? true)
  }
}, { immediate: true })

// Submit
const onSubmit = handleSubmit(async () => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      product_id: values.product_id,
      user_id: values.user_id,
      rating: values.rating,
      title: values.title?.trim() || null,
      comment: values.comment?.trim() || null,
      verified_purchase: values.verified_purchase,
      active: values.active
    }

    const url = props.isEditing
      ? `/api/admin/reviews/${props.review?.id}`
      : '/api/admin/reviews'

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await apiFetch<Review>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? t('admin.reviews.form.successUpdated')
      : t('admin.reviews.form.successCreated')

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || t('admin.reviews.form.error')
  } finally {
    pending.value = false
  }
})
</script>

<style scoped></style>
