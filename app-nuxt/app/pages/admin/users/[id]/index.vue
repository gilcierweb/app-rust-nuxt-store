<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/users" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">{{ $t('admin.users.title') }}</h1>
          <p v-if="user" class="text-sm text-gray-500">ID: {{ user.id }}</p>
        </div>
      </div>

      <div v-if="user" class="flex gap-2">
        <NuxtLinkLocale :to="`/admin/users/${user.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          {{ $t('common.edit') }}
        </NuxtLinkLocale>
        <button type="button" class="btn btn-error btn-outline" @click="deleteUser">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          {{ $t('common.delete') }}
        </button>
      </div>
    </div>

    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">{{ $t('admin.users.loading') }}</span>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.users.error', { message: error.message }) }}</span>
      <button type="button" class="btn btn-sm btn-ghost" @click="() => refresh()">{{ $t('common.actions.tryAgain') }}</button>
    </div>

    <div v-else-if="user" class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="card shadow-base-300/10 shadow-md lg:col-span-1">
        <div class="card-body items-center text-center">
          <div class="avatar avatar-placeholder mb-4">
            <div class="bg-neutral text-neutral-content rounded-full w-24 h-24">
              <span class="text-3xl">{{ (user.email.at(0) || '?').toUpperCase() }}</span>
            </div>
          </div>
          <h2 class="card-title text-xl">{{ user.name }}</h2>
          <p class="text-gray-500">{{ user.email }}</p>

          <div class="mt-3 flex gap-2">
            <span class="badge badge-soft" :class="user.role === 'Admin' ? 'badge-primary' : 'badge-secondary'">
              {{ user.role }}
            </span>
            <span class="badge badge-soft" :class="user.active ? 'badge-success' : 'badge-error'">
              {{ user.active ? $t('common.status.active') : $t('common.status.inactive') }}
            </span>
          </div>

          <div class="divider my-4"></div>

          <div class="text-xs text-gray-500 space-y-2 w-full">
            <div class="flex justify-between">
              <span>{{ $t('admin.users.detail.pid', 'PID') }}</span>
              <span class="font-mono text-[11px]">{{ user.pid }}</span>
            </div>
            <div class="flex justify-between">
              <span>{{ $t('common.table.date') }}</span>
              <span class="font-medium">{{ formatDate(user.createdAt) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="card shadow-base-300/10 shadow-md lg:col-span-2">
        <div class="card-header">
          <h2 class="card-title">{{ $t('common.view') }}</h2>
        </div>

        <div class="card-body">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.users.table.email') }}</span>
              </label>
              <div class="font-medium">{{ user.email }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.users.table.role') }}</span>
              </label>
              <div class="font-medium">{{ user.role }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.users.table.status') }}</span>
              </label>
              <div class="font-medium">
                {{ user.active ? $t('common.status.active') : $t('common.status.inactive') }}
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.users.detail.emailVerified') }}</span>
              </label>
              <div class="font-medium">
                {{ user.emailVerifiedAt ? formatDate(user.emailVerifiedAt) : '-' }}
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.users.detail.createdAt') }}</span>
              </label>
              <div class="font-medium">{{ formatDate(user.createdAt) }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.users.detail.updatedAt') }}</span>
              </label>
              <div class="font-medium">{{ formatDate(user.updatedAt) }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>{{ $t('admin.users.notFound') }}</span>
      <NuxtLinkLocale to="/admin/users" class="btn btn-sm">{{ $t('common.back') }}</NuxtLinkLocale>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

interface AdminUserDetail {
  id: number
  pid: string
  email: string
  name: string
  role: string
  active: boolean
  emailVerifiedAt?: string
  createdAt: string
  updatedAt: string
}

const route = useRoute()
const router = useRouter()
const { t } = useI18n()
const { apiFetch, useApiFetch } = useApi()
const toast = useAppToast()
const dialog = useAppDialog()

const { pending, data: user, error, refresh } = await useApiFetch<AdminUserDetail>(`/api/admin/users/${route.params.id}`)

const formatDate = (dateString?: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

const deleteUser = async () => {
  if (!user.value) return

  const confirmed = await dialog.confirm({
    message: t('common.confirmDelete', { name: user.value.email }),
    confirmLabel: t('common.delete'),
    tone: 'danger'
  })
  if (!confirmed) return

  try {
    await apiFetch(`/api/admin/users/${user.value.id}`, {
      method: 'DELETE'
    })
    router.push('/admin/users')
  } catch (err) {
    toast.error(t('common.errorDelete', { resource: t('admin.users.title').toLowerCase() }))
    console.error(err)
  }
}
</script>
