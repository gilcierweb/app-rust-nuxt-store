<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? 'Editar Cupom' : 'Novo Cupom' }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">Salvando cupom...</span>
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
              <span class="label-text font-semibold">Código do Cupom *</span>
            </label>
            <input
              v-model="form.code"
              type="text"
              placeholder="PROMO2024"
              class="input input-bordered w-full uppercase"
              :class="{ 'input-error': errors.code }"
              required
              :disabled="pending"
            />
            <label v-if="errors.code" class="label">
              <span class="label-text-alt text-error">{{ errors.code }}</span>
            </label>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Discount Type -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Tipo de Desconto</span>
              </label>
              <select v-model.number="form.discount_type" class="select select-bordered w-full" :disabled="pending">
                <option :value="1">Porcentagem (%)</option>
                <option :value="2">Valor Fixo (R$)</option>
                <option :value="3">Frete Grátis</option>
              </select>
            </div>

            <!-- Discount Value -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Valor do Desconto</span>
              </label>
              <input
                v-model.number="form.discount_value"
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
                <span class="label-text font-semibold">Valor Mínimo</span>
              </label>
              <input
                v-model.number="form.minimum_amount"
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
                <span class="label-text font-semibold">Desconto Máximo</span>
              </label>
              <input
                v-model.number="form.maximum_discount"
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
                <span class="label-text font-semibold">Limite de Uso</span>
              </label>
              <input
                v-model.number="form.usage_limit"
                type="number"
                min="0"
                placeholder="0 = ilimitado"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <!-- Expiration Date -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Data de Expiração</span>
              </label>
              <input
                v-model="form.expires_at"
                type="datetime-local"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>
          </div>

          <!-- Active -->
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text font-semibold">Cupom Ativo</span>
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
              Cancelar
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? 'Atualizar' : 'Salvar' }} Cupom
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Coupon } from '~/types'

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

const config = useRuntimeConfig()

// Form state
const form = reactive({
  code: '',
  discount_type: 1,
  discount_value: null as number | null,
  minimum_amount: null as number | null,
  maximum_discount: null as number | null,
  usage_limit: null as number | null,
  expires_at: '',
  active: true
})

const errors = reactive({
  code: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Populate form when editing
onMounted(() => {
  if (props.coupon && props.isEditing) {
    form.code = props.coupon.code || ''
    form.discount_type = props.coupon.discount_type ?? 1
    form.discount_value = props.coupon.discount_value ?? null
    form.minimum_amount = props.coupon.minimum_amount ?? null
    form.maximum_discount = props.coupon.maximum_discount ?? null
    form.usage_limit = props.coupon.usage_limit ?? null
    form.expires_at = props.coupon.expires_at ? formatDateTimeLocal(props.coupon.expires_at) : ''
    form.active = props.coupon.active ?? true
  }
})

// Watch for coupon prop changes (in case it loads async)
watch(() => props.coupon, (newCoupon) => {
  if (newCoupon && props.isEditing) {
    form.code = newCoupon.code || ''
    form.discount_type = newCoupon.discount_type ?? 1
    form.discount_value = newCoupon.discount_value ?? null
    form.minimum_amount = newCoupon.minimum_amount ?? null
    form.maximum_discount = newCoupon.maximum_discount ?? null
    form.usage_limit = newCoupon.usage_limit ?? null
    form.expires_at = newCoupon.expires_at ? formatDateTimeLocal(newCoupon.expires_at) : ''
    form.active = newCoupon.active ?? true
  }
}, { immediate: true })

// Format datetime for input
const formatDateTimeLocal = (dateString: string) => {
  const date = new Date(dateString)
  return date.toISOString().slice(0, 16)
}

// Validation
const validate = () => {
  let isValid = true
  errors.code = ''

  if (!form.code.trim()) {
    errors.code = 'O código do cupom é obrigatório'
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
      code: form.code.trim().toUpperCase(),
      discount_type: form.discount_type,
      discount_value: form.discount_value,
      minimum_amount: form.minimum_amount,
      maximum_discount: form.maximum_discount,
      usage_limit: form.usage_limit,
      expires_at: form.expires_at || null,
      active: form.active
    }

    const url = props.isEditing
      ? `${config.public.baseURL}/api/coupons/${props.coupon?.id}`
      : `${config.public.baseURL}/api/coupons`

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await $fetch<Coupon>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? 'Cupom atualizado com sucesso!'
      : 'Cupom criado com sucesso!'

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || 'Erro ao salvar cupom. Tente novamente.'
  } finally {
    pending.value = false
  }
}
</script>

<style scoped></style>
