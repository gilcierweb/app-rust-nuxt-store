<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? t('admin.addresses.form.titleEdit') : t('admin.addresses.form.titleNew') }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">{{ t('admin.addresses.form.saving') }}</span>
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
          <!-- Type -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.addresses.form.type') }}</span>
            </label>
            <select v-model="values.type" class="select select-bordered w-full" :disabled="pending">
              <option value="">{{ t('admin.addresses.form.typePlaceholder') }}</option>
              <option value="home">{{ t('admin.addresses.types.home') }}</option>
              <option value="work">{{ t('admin.addresses.types.work') }}</option>
              <option value="other">{{ t('admin.addresses.types.other') }}</option>
            </select>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- First Name -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.firstName') }} *</span>
              </label>
              <input
                v-model="first_name"
                @blur="first_nameBlur"
                type="text"
                :placeholder="t('admin.addresses.form.firstNamePlaceholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': first_nameError }"
                required
                :disabled="pending"
              />
              <label v-if="first_nameError" class="label">
                <span class="label-text-alt text-error">{{ first_nameError }}</span>
              </label>
            </div>

            <!-- Last Name -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.lastName') }} *</span>
              </label>
              <input
                v-model="last_name"
                @blur="last_nameBlur"
                type="text"
                placeholder="Sobrenome"
                class="input input-bordered w-full"
                :class="{ 'input-error': last_nameError }"
                required
                :disabled="pending"
              />
              <label v-if="last_nameError" class="label">
                <span class="label-text-alt text-error">{{ last_nameError }}</span>
              </label>
            </div>
          </div>

          <!-- Company -->
          <div class="form-control">
            <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.company') }}</span>
            </label>
            <input
              v-model="values.company"
              type="text"
              :placeholder="t('admin.addresses.form.companyPlaceholder')"
              class="input input-bordered w-full"
              :disabled="pending"
            />
          </div>

          <!-- Address 1 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.addresses.form.address1') }} *</span>
            </label>
            <input
              v-model="address1"
                @blur="address1Blur"
                type="text"
                :placeholder="t('admin.addresses.form.address1Placeholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': address1Error }"
                required
                :disabled="pending"
              />
              <label v-if="address1Error" class="label">
                <span class="label-text-alt text-error">{{ address1Error }}</span>
            </label>
          </div>

          <!-- Address 2 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.addresses.form.address2') }}</span>
            </label>
            <input
              v-model="values.address2"
              type="text"
              :placeholder="t('admin.addresses.form.address2Placeholder')"
              class="input input-bordered w-full"
              :disabled="pending"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- City -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.city') }} *</span>
              </label>
              <input
                v-model="city"
                @blur="cityBlur"
                type="text"
                placeholder="Cidade"
                class="input input-bordered w-full"
                :class="{ 'input-error': cityError }"
                required
                :disabled="pending"
              />
              <label v-if="cityError" class="label">
                <span class="label-text-alt text-error">{{ cityError }}</span>
              </label>
            </div>

            <!-- State -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.state') }} *</span>
              </label>
              <input
                v-model="state"
                @blur="stateBlur"
                type="text"
                :placeholder="t('admin.addresses.form.statePlaceholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': stateError }"
                required
                :disabled="pending"
              />
              <label v-if="stateError" class="label">
                <span class="label-text-alt text-error">{{ stateError }}</span>
              </label>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Zip Code -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.zipCode') }} *</span>
              </label>
              <input
                v-model="zip_code"
                @blur="zip_codeBlur"
                type="text"
                placeholder="00000-000"
                class="input input-bordered w-full"
                :class="{ 'input-error': zip_codeError }"
                required
                :disabled="pending"
              />
              <label v-if="zip_codeError" class="label">
                <span class="label-text-alt text-error">{{ zip_codeError }}</span>
              </label>
            </div>

            <!-- Country -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.country') }} *</span>
              </label>
              <input
                v-model="country"
                @blur="countryBlur"
                type="text"
                placeholder="País"
                class="input input-bordered w-full"
                :class="{ 'input-error': countryError }"
                required
                :disabled="pending"
              />
              <label v-if="countryError" class="label">
                <span class="label-text-alt text-error">{{ countryError }}</span>
              </label>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Phone -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.phone') }}</span>
              </label>
              <input
                v-model="values.phone"
                type="tel"
                placeholder="(00) 00000-0000"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <!-- User ID -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.userId') }} *</span>
              </label>
              <input
                v-model.number="user_id"
                @blur="user_idBlur"
                type="number"
                :placeholder="t('admin.addresses.form.userIdPlaceholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': user_idError }"
                required
                :disabled="pending"
              />
              <label v-if="user_idError" class="label">
                <span class="label-text-alt text-error">{{ user_idError }}</span>
              </label>
            </div>
          </div>

          <!-- Default -->
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text font-semibold">{{ t('admin.addresses.form.default') }}</span>
              <input
                v-model="values.default"
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
              {{ t('admin.addresses.form.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? t('admin.addresses.form.submitUpdate') : t('admin.addresses.form.submitSave') }} {{ t('admin.addresses.form.submitAddress') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Address } from '~/types'
import { useForm, useField } from 'vee-validate'

interface Props {
  address?: Partial<Address>
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', address: Address): void
  (e: 'cancel'): void
}>()

const { t } = useI18n()
const { apiFetch } = useApi()

const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
    type: '',
    first_name: '',
    last_name: '',
    company: '',
    address1: '',
    address2: '',
    city: '',
    state: '',
    zip_code: '',
    country: 'Brasil',
    phone: '',
    user_id: null as number | null,
    default: false
  }
})

const { value: first_name, errorMessage: first_nameError, handleBlur: first_nameBlur } = useField<string>('first_name', (v) => {
  if (!v?.trim()) return t('admin.addresses.form.validation.firstNameRequired')
  return true
})
const { value: last_name, errorMessage: last_nameError, handleBlur: last_nameBlur } = useField<string>('last_name', (v) => {
  if (!v?.trim()) return t('admin.addresses.form.validation.lastNameRequired')
  return true
})
const { value: address1, errorMessage: address1Error, handleBlur: address1Blur } = useField<string>('address1', (v) => {
  if (!v?.trim()) return t('admin.addresses.form.validation.addressRequired')
  return true
})
const { value: city, errorMessage: cityError, handleBlur: cityBlur } = useField<string>('city', (v) => {
  if (!v?.trim()) return t('admin.addresses.form.validation.cityRequired')
  return true
})
const { value: state, errorMessage: stateError, handleBlur: stateBlur } = useField<string>('state', (v) => {
  if (!v?.trim()) return t('admin.addresses.form.validation.stateRequired')
  return true
})
const { value: zip_code, errorMessage: zip_codeError, handleBlur: zip_codeBlur } = useField<string>('zip_code', (v) => {
  if (!v?.trim()) return t('admin.addresses.form.validation.zipCodeRequired')
  return true
})
const { value: country, errorMessage: countryError, handleBlur: countryBlur } = useField<string>('country', (v) => {
  if (!v?.trim()) return t('admin.addresses.form.validation.countryRequired')
  return true
})
const { value: user_id, errorMessage: user_idError, handleBlur: user_idBlur } = useField<number | null>('user_id', (v) => {
  if (!v) return t('admin.addresses.form.validation.userIdRequired')
  return true
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

onMounted(() => {
  if (props.address && props.isEditing) {
    setFieldValue('type', props.address.type || '')
    setFieldValue('first_name', props.address.first_name || '')
    setFieldValue('last_name', props.address.last_name || '')
    setFieldValue('company', props.address.company || '')
    setFieldValue('address1', props.address.address1 || '')
    setFieldValue('address2', props.address.address2 || '')
    setFieldValue('city', props.address.city || '')
    setFieldValue('state', props.address.state || '')
    setFieldValue('zip_code', props.address.zip_code || '')
    setFieldValue('country', props.address.country || 'Brasil')
    setFieldValue('phone', props.address.phone || '')
    setFieldValue('user_id', props.address.user_id ?? null)
    setFieldValue('default', props.address.default ?? false)
  }
})

watch(() => props.address, (newAddress) => {
  if (newAddress && props.isEditing) {
    setFieldValue('type', newAddress.type || '')
    setFieldValue('first_name', newAddress.first_name || '')
    setFieldValue('last_name', newAddress.last_name || '')
    setFieldValue('company', newAddress.company || '')
    setFieldValue('address1', newAddress.address1 || '')
    setFieldValue('address2', newAddress.address2 || '')
    setFieldValue('city', newAddress.city || '')
    setFieldValue('state', newAddress.state || '')
    setFieldValue('zip_code', newAddress.zip_code || '')
    setFieldValue('country', newAddress.country || 'Brasil')
    setFieldValue('phone', newAddress.phone || '')
    setFieldValue('user_id', newAddress.user_id ?? null)
    setFieldValue('default', newAddress.default ?? false)
  }
}, { immediate: true })

const onSubmit = handleSubmit(async () => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      type: values.type || null,
      first_name: values.first_name.trim(),
      last_name: values.last_name.trim(),
      company: values.company?.trim() || null,
      address1: values.address1.trim(),
      address2: values.address2?.trim() || null,
      city: values.city.trim(),
      state: values.state.trim(),
      zip_code: values.zip_code.trim(),
      country: values.country.trim(),
      phone: values.phone?.trim() || null,
      user_id: values.user_id,
      default: values.default
    }

    const url = props.isEditing
      ? `/api/admin/addresses/${props.address?.id}`
      : '/api/admin/addresses'

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await apiFetch<Address>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? t('admin.addresses.form.successUpdated')
      : t('admin.addresses.form.successCreated')

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || t('admin.addresses.form.error')
  } finally {
    pending.value = false
  }
})
</script>

<style scoped></style>
