<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? 'Editar Avaliação' : 'Nova Avaliação' }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">Salvando avaliação...</span>
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
                <span class="label-text font-semibold">Produto ID *</span>
              </label>
              <input
                v-model.number="form.product_id"
                type="number"
                placeholder="ID do produto"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.product_id }"
                required
                :disabled="pending"
              />
              <label v-if="errors.product_id" class="label">
                <span class="label-text-alt text-error">{{ errors.product_id }}</span>
              </label>
            </div>

            <!-- User ID -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Usuário ID *</span>
              </label>
              <input
                v-model.number="form.user_id"
                type="number"
                placeholder="ID do usuário"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.user_id }"
                required
                :disabled="pending"
              />
              <label v-if="errors.user_id" class="label">
                <span class="label-text-alt text-error">{{ errors.user_id }}</span>
              </label>
            </div>
          </div>

          <!-- Rating -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Avaliação (1-5) *</span>
            </label>
            <div class="flex items-center gap-2">
              <input
                v-model.number="form.rating"
                type="range"
                min="1"
                max="5"
                class="range range-primary flex-1"
                :disabled="pending"
              />
              <span class="text-2xl font-bold w-8 text-center">{{ form.rating }}</span>
              <div class="flex text-warning">
                <i v-for="n in 5" :key="n" 
                   :class="['icon-[tabler--star]', n <= form.rating ? 'filled' : '', n <= form.rating ? 'text-warning' : 'text-gray-300']" 
                   class="size-6"></i>
              </div>
            </div>
            <label v-if="errors.rating" class="label">
              <span class="label-text-alt text-error">{{ errors.rating }}</span>
            </label>
          </div>

          <!-- Title -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Título</span>
            </label>
            <input
              v-model="form.title"
              type="text"
              placeholder="Título da avaliação"
              class="input input-bordered w-full"
              :disabled="pending"
            />
          </div>

          <!-- Comment -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Comentário</span>
            </label>
            <textarea
              v-model="form.comment"
              placeholder="Escreva sua avaliação..."
              class="textarea textarea-bordered w-full"
              rows="4"
              :disabled="pending"
            ></textarea>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Verified Purchase -->
            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text font-semibold">Compra Verificada</span>
                <input
                  v-model="form.verified_purchase"
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  :disabled="pending"
                />
              </label>
            </div>

            <!-- Active -->
            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text font-semibold">Ativa</span>
                <input
                  v-model="form.active"
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
              Cancelar
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? 'Atualizar' : 'Salvar' }} Avaliação
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
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

const config = useRuntimeConfig()

// Form state
const form = reactive({
  product_id: null as number | null,
  user_id: null as number | null,
  rating: 5,
  title: '',
  comment: '',
  verified_purchase: true,
  active: true
})

const errors = reactive({
  product_id: '',
  user_id: '',
  rating: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Populate form when editing
onMounted(() => {
  if (props.review && props.isEditing) {
    form.product_id = props.review.product_id ?? null
    form.user_id = props.review.user_id ?? null
    form.rating = props.review.rating ?? 5
    form.title = props.review.title || ''
    form.comment = props.review.comment || ''
    form.verified_purchase = props.review.verified_purchase ?? true
    form.active = props.review.active ?? true
  }
})

// Watch for review prop changes (in case it loads async)
watch(() => props.review, (newReview) => {
  if (newReview && props.isEditing) {
    form.product_id = newReview.product_id ?? null
    form.user_id = newReview.user_id ?? null
    form.rating = newReview.rating ?? 5
    form.title = newReview.title || ''
    form.comment = newReview.comment || ''
    form.verified_purchase = newReview.verified_purchase ?? true
    form.active = newReview.active ?? true
  }
}, { immediate: true })

// Validation
const validate = () => {
  let isValid = true
  errors.product_id = ''
  errors.user_id = ''
  errors.rating = ''

  if (!form.product_id) {
    errors.product_id = 'O ID do produto é obrigatório'
    isValid = false
  }

  if (!form.user_id) {
    errors.user_id = 'O ID do usuário é obrigatório'
    isValid = false
  }

  if (!form.rating || form.rating < 1 || form.rating > 5) {
    errors.rating = 'A avaliação deve ser entre 1 e 5'
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
      product_id: form.product_id,
      user_id: form.user_id,
      rating: form.rating,
      title: form.title?.trim() || null,
      comment: form.comment?.trim() || null,
      verified_purchase: form.verified_purchase,
      active: form.active
    }

    const url = props.isEditing
      ? `${config.public.baseURL}/api/reviews/${props.review?.id}`
      : `${config.public.baseURL}/api/reviews`

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await $fetch<Review>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? 'Avaliação atualizada com sucesso!'
      : 'Avaliação criada com sucesso!'

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || 'Erro ao salvar avaliação. Tente novamente.'
  } finally {
    pending.value = false
  }
}
</script>

<style scoped></style>
