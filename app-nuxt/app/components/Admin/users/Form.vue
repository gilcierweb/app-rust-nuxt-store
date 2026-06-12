<template>
  <div class="max-w-3xl">
    <div class="card shadow-base-300/10 shadow-md">
      <div class="card-header">
        <h2 class="card-title text-xl">
          {{ isEditing ? $t('admin.users.form.editTitle') : $t('admin.users.form.newTitle') }}
        </h2>
      </div>

      <div class="card-body">
        <AppAlert
          v-if="errorMessage"
          type="error"
          :message="errorMessage"
          :auto-close="5000"
          dismissible
          @close="errorMessage = ''"
        />

        <form class="space-y-6" novalidate @submit.prevent="onSubmit">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
            <div class="form-control">
              <label class="label" for="adminUserName">
                <span class="label-text font-semibold">{{ $t('admin.users.form.name') }}</span>
              </label>
              <input
                id="adminUserName"
                v-model="name"
                type="text"
                class="input input-bordered w-full"
                :class="{ 'input-error': nameError }"
                :disabled="saving"
                required
                @blur="nameBlur"
              />
              <label v-if="nameError" class="label">
                <span class="label-text-alt text-error">{{ nameError }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label" for="adminUserEmail">
                <span class="label-text font-semibold">{{ $t('admin.users.form.email') }}</span>
              </label>
              <input
                id="adminUserEmail"
                v-model="email"
                type="email"
                class="input input-bordered w-full"
                :class="{ 'input-error': emailError }"
                :disabled="saving"
                required
                @blur="emailBlur"
              />
              <label v-if="emailError" class="label">
                <span class="label-text-alt text-error">{{ emailError }}</span>
              </label>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
            <div class="form-control">
              <label class="label" for="adminUserRole">
                <span class="label-text font-semibold">{{ $t('admin.users.form.role') }}</span>
              </label>
              <select
                id="adminUserRole"
                v-model="role"
                class="select select-bordered w-full"
                :disabled="saving"
              >
                <option v-for="role in roleOptions" :key="role.value" :value="role.value">
                  {{ role.label }}
                </option>
              </select>
            </div>

            <div class="form-control">
              <label class="label" for="adminUserPassword">
                <span class="label-text font-semibold">
                  {{ isEditing ? $t('admin.users.form.newPassword') : $t('admin.users.form.password') }}
                </span>
              </label>
              <input
                id="adminUserPassword"
                v-model="password"
                type="password"
                class="input input-bordered w-full"
                :class="{ 'input-error': passwordError }"
                :disabled="saving"
                :required="!isEditing"
                autocomplete="new-password"
                @blur="passwordBlur"
              />
              <label v-if="passwordError" class="label">
                <span class="label-text-alt text-error">{{ passwordError }}</span>
              </label>
            </div>
          </div>

          <label class="flex items-center gap-3 cursor-pointer">
            <input v-model="active" type="checkbox" class="checkbox checkbox-primary" :disabled="saving" />
            <span class="font-medium">{{ $t('admin.users.form.active') }}</span>
          </label>

          <div class="flex flex-col-reverse sm:flex-row sm:justify-end gap-3 pt-2">
            <button type="button" class="btn btn-ghost" :disabled="saving" @click="$emit('cancel')">
              {{ $t('common.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="saving">
              <span v-if="saving" class="loading loading-spinner loading-sm"></span>
              <i v-else class="icon-[tabler--device-floppy] size-5"></i>
              {{ saving ? $t('common.actions.saving') : $t('common.actions.saveChanges') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useForm, useField } from 'vee-validate'

interface AdminUserDetail {
  id: number
  email: string
  name: string
  role: string
  active: boolean
}

const props = defineProps<{
  initialUser?: AdminUserDetail | null
}>()

const emit = defineEmits<{
  saved: [user: AdminUserDetail]
  cancel: []
}>()

const { t } = useI18n()
const { apiFetch } = useApi()

const isEditing = computed(() => Boolean(props.initialUser?.id))
const saving = ref(false)
const errorMessage = ref('')

const roleOptions = computed(() => [
  { value: 'user', label: t('admin.users.roles.user') },
  { value: 'admin', label: t('admin.users.roles.admin') },
  { value: 'store_manager', label: t('admin.users.roles.storeManager') },
  { value: 'editor', label: t('admin.users.roles.editor') },
  { value: 'support', label: t('admin.users.roles.support') },
  { value: 'viewer', label: t('admin.users.roles.viewer') }
])

function normalizeRole(role: string) {
  const normalized = role.trim().toLowerCase().replaceAll(' ', '_')
  return roleOptions.value.some(option => option.value === normalized) ? normalized : 'user'
}

const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
    name: '',
    email: '',
    password: '',
    role: 'user',
    active: true
  }
})

const { value: name, errorMessage: nameError, handleBlur: nameBlur } = useField<string>('name', (v) => {
  if (!v || v.trim().length < 2) return t('admin.users.form.errors.name')
  return true
})

const { value: email, errorMessage: emailError, handleBlur: emailBlur } = useField<string>('email', (v) => {
  if (!v || !v.includes('@')) return t('admin.users.form.errors.email')
  return true
})

const { value: password, errorMessage: passwordError, handleBlur: passwordBlur } = useField<string>('password', (v) => {
  if (!isEditing.value && (!v || v.trim().length < 8)) {
    return t('admin.users.form.errors.password')
  }
  if (isEditing.value && v && v.trim().length < 8) {
    return t('admin.users.form.errors.password')
  }
  return true
})

const { value: role } = useField<string>('role')
const { value: active } = useField<boolean>('active')

watch(
  () => props.initialUser,
  (user) => {
    if (!user) return
    setFieldValue('name', user.name)
    setFieldValue('email', user.email)
    setFieldValue('password', '')
    setFieldValue('role', normalizeRole(user.role))
    setFieldValue('active', user.active)
  },
  { immediate: true }
)

const onSubmit = handleSubmit(async (formValues) => {
  saving.value = true
  errorMessage.value = ''

  const payload = {
    name: formValues.name.trim(),
    email: formValues.email.trim(),
    password: formValues.password.trim() || undefined,
    role: formValues.role,
    active: formValues.active
  }

  try {
    const endpoint = isEditing.value
      ? `/api/admin/users/${props.initialUser?.id}`
      : '/api/admin/users'
    const method = isEditing.value ? 'PATCH' : 'POST'
    const savedUser = await apiFetch<AdminUserDetail>(endpoint, {
      method,
      body: payload
    })
    emit('saved', savedUser)
  } catch (error: any) {
    errorMessage.value = error?.data?.message || error?.message || t('admin.users.form.errors.save')
  } finally {
    saving.value = false
  }
})
</script>
