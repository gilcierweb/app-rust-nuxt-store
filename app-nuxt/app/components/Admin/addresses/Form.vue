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
            <select v-model="form.type" class="select select-bordered w-full" :disabled="pending">
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
                v-model="form.first_name"
                type="text"
                :placeholder="t('admin.addresses.form.firstNamePlaceholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.first_name }"
                required
                :disabled="pending"
              />
              <label v-if="errors.first_name" class="label">
                <span class="label-text-alt text-error">{{ errors.first_name }}</span>
              </label>
            </div>

            <!-- Last Name -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.lastName') }} *</span>
              </label>
              <input
                v-model="form.last_name"
                type="text"
                placeholder="Sobrenome"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.last_name }"
                required
                :disabled="pending"
              />
              <label v-if="errors.last_name" class="label">
                <span class="label-text-alt text-error">{{ errors.last_name }}</span>
              </label>
            </div>
          </div>

          <!-- Company -->
          <div class="form-control">
            <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.company') }}</span>
            </label>
            <input
              v-model="form.company"
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
              v-model="form.address1"
              type="text"
              :placeholder="t('admin.addresses.form.address1Placeholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': errors.address1 }"
              required
              :disabled="pending"
            />
            <label v-if="errors.address1" class="label">
              <span class="label-text-alt text-error">{{ errors.address1 }}</span>
            </label>
          </div>

          <!-- Address 2 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.addresses.form.address2') }}</span>
            </label>
            <input
              v-model="form.address2"
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
                v-model="form.city"
                type="text"
                placeholder="Cidade"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.city }"
                required
                :disabled="pending"
              />
              <label v-if="errors.city" class="label">
                <span class="label-text-alt text-error">{{ errors.city }}</span>
              </label>
            </div>

            <!-- State -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.state') }} *</span>
              </label>
              <input
                v-model="form.state"
                type="text"
                :placeholder="t('admin.addresses.form.statePlaceholder')"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.state }"
                required
                :disabled="pending"
              />
              <label v-if="errors.state" class="label">
                <span class="label-text-alt text-error">{{ errors.state }}</span>
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
                v-model="form.zip_code"
                type="text"
                placeholder="00000-000"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.zip_code }"
                required
                :disabled="pending"
              />
              <label v-if="errors.zip_code" class="label">
                <span class="label-text-alt text-error">{{ errors.zip_code }}</span>
              </label>
            </div>

            <!-- Country -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.addresses.form.country') }} *</span>
              </label>
              <input
                v-model="form.country"
                type="text"
                placeholder="País"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.country }"
                required
                :disabled="pending"
              />
              <label v-if="errors.country" class="label">
                <span class="label-text-alt text-error">{{ errors.country }}</span>
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
                v-model="form.phone"
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
                v-model.number="form.user_id"
                type="number"
                :placeholder="t('admin.addresses.form.userIdPlaceholder')"
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

          <!-- Default -->
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text font-semibold">{{ t('admin.addresses.form.default') }}</span>
              <input
                v-model="form.default"
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

// Form state
const form = reactive({
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
})

const errors = reactive({
  first_name: '',
  last_name: '',
  address1: '',
  city: '',
  state: '',
  zip_code: '',
  country: '',
  user_id: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Populate form when editing
onMounted(() => {
  if (props.address && props.isEditing) {
    form.type = props.address.type || ''
    form.first_name = props.address.first_name || ''
    form.last_name = props.address.last_name || ''
    form.company = props.address.company || ''
    form.address1 = props.address.address1 || ''
    form.address2 = props.address.address2 || ''
    form.city = props.address.city || ''
    form.state = props.address.state || ''
    form.zip_code = props.address.zip_code || ''
    form.country = props.address.country || 'Brasil'
    form.phone = props.address.phone || ''
    form.user_id = props.address.user_id ?? null
    form.default = props.address.default ?? false
  }
})

// Watch for address prop changes (in case it loads async)
watch(() => props.address, (newAddress) => {
  if (newAddress && props.isEditing) {
    form.type = newAddress.type || ''
    form.first_name = newAddress.first_name || ''
    form.last_name = newAddress.last_name || ''
    form.company = newAddress.company || ''
    form.address1 = newAddress.address1 || ''
    form.address2 = newAddress.address2 || ''
    form.city = newAddress.city || ''
    form.state = newAddress.state || ''
    form.zip_code = newAddress.zip_code || ''
    form.country = newAddress.country || 'Brasil'
    form.phone = newAddress.phone || ''
    form.user_id = newAddress.user_id ?? null
    form.default = newAddress.default ?? false
  }
}, { immediate: true })

// Validation
const validate = () => {
  let isValid = true
  errors.first_name = ''
  errors.last_name = ''
  errors.address1 = ''
  errors.city = ''
  errors.state = ''
  errors.zip_code = ''
  errors.country = ''
  errors.user_id = ''

  if (!form.first_name?.trim()) {
    errors.first_name = t('admin.addresses.form.validation.firstNameRequired')
    isValid = false
  }

  if (!form.last_name?.trim()) {
    errors.last_name = t('admin.addresses.form.validation.lastNameRequired')
    isValid = false
  }

  if (!form.address1?.trim()) {
    errors.address1 = t('admin.addresses.form.validation.addressRequired')
    isValid = false
  }

  if (!form.city?.trim()) {
    errors.city = t('admin.addresses.form.validation.cityRequired')
    isValid = false
  }

  if (!form.state?.trim()) {
    errors.state = t('admin.addresses.form.validation.stateRequired')
    isValid = false
  }

  if (!form.zip_code?.trim()) {
    errors.zip_code = t('admin.addresses.form.validation.zipCodeRequired')
    isValid = false
  }

  if (!form.country?.trim()) {
    errors.country = t('admin.addresses.form.validation.countryRequired')
    isValid = false
  }

  if (!form.user_id) {
    errors.user_id = t('admin.addresses.form.validation.userIdRequired')
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
      type: form.type || null,
      first_name: form.first_name.trim(),
      last_name: form.last_name.trim(),
      company: form.company?.trim() || null,
      address1: form.address1.trim(),
      address2: form.address2?.trim() || null,
      city: form.city.trim(),
      state: form.state.trim(),
      zip_code: form.zip_code.trim(),
      country: form.country.trim(),
      phone: form.phone?.trim() || null,
      user_id: form.user_id,
      default: form.default
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
}
</script>

<style scoped></style>
