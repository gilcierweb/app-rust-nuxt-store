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

        <form class="space-y-6" novalidate @submit.prevent="submitForm">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
            <div class="form-control">
              <label class="label" for="adminUserName">
                <span class="label-text font-semibold">{{ $t('admin.users.form.name') }}</span>
              </label>
              <input
                id="adminUserName"
                v-model="form.name"
                type="text"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.name }"
                :disabled="saving"
                required
              />
              <label v-if="errors.name" class="label">
                <span class="label-text-alt text-error">{{ errors.name }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label" for="adminUserEmail">
                <span class="label-text font-semibold">{{ $t('admin.users.form.email') }}</span>
              </label>
              <input
                id="adminUserEmail"
                v-model="form.email"
                type="email"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.email }"
                :disabled="saving"
                required
              />
              <label v-if="errors.email" class="label">
                <span class="label-text-alt text-error">{{ errors.email }}</span>
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
                v-model="form.role"
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
                v-model="form.password"
                type="password"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.password }"
                :disabled="saving"
                :required="!isEditing"
                autocomplete="new-password"
              />
              <label v-if="errors.password" class="label">
                <span class="label-text-alt text-error">{{ errors.password }}</span>
              </label>
            </div>
          </div>

          <label class="flex items-center gap-3 cursor-pointer">
            <input v-model="form.active" type="checkbox" class="checkbox checkbox-primary" :disabled="saving" />
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

const form = reactive({
  name: '',
  email: '',
  password: '',
  role: 'user',
  active: true
})

const errors = reactive({
  name: '',
  email: '',
  password: ''
})

const roleOptions = computed(() => [
  { value: 'user', label: t('admin.users.roles.user') },
  { value: 'admin', label: t('admin.users.roles.admin') },
  { value: 'store_manager', label: t('admin.users.roles.storeManager') },
  { value: 'editor', label: t('admin.users.roles.editor') },
  { value: 'support', label: t('admin.users.roles.support') },
  { value: 'viewer', label: t('admin.users.roles.viewer') }
])

watch(
  () => props.initialUser,
  (user) => {
    if (!user) return
    form.name = user.name
    form.email = user.email
    form.password = ''
    form.role = normalizeRole(user.role)
    form.active = user.active
  },
  { immediate: true }
)

function normalizeRole(role: string) {
  const normalized = role.trim().toLowerCase().replaceAll(' ', '_')
  return roleOptions.value.some(option => option.value === normalized) ? normalized : 'user'
}

function validateForm() {
  errors.name = ''
  errors.email = ''
  errors.password = ''

  if (form.name.trim().length < 2) {
    errors.name = t('admin.users.form.errors.name')
  }

  if (!form.email.includes('@')) {
    errors.email = t('admin.users.form.errors.email')
  }

  if (!isEditing.value && form.password.trim().length < 8) {
    errors.password = t('admin.users.form.errors.password')
  }

  if (isEditing.value && form.password.trim() && form.password.trim().length < 8) {
    errors.password = t('admin.users.form.errors.password')
  }

  return !errors.name && !errors.email && !errors.password
}

async function submitForm() {
  if (!validateForm()) return

  saving.value = true
  errorMessage.value = ''

  const payload = {
    name: form.name.trim(),
    email: form.email.trim(),
    password: form.password.trim() || undefined,
    role: form.role,
    active: form.active
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
}
</script>
