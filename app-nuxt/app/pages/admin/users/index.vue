<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.users.title') }}</h1>
      <button class="btn btn-primary" type="button" disabled>
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.users.add') }}
      </button>
    </div>

    <div class="mb-6">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-4 md:grid-cols-[1fr_auto]">
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="$t('admin.users.searchPlaceholder')"
            class="input input-bordered w-full"
          />
          <button type="submit" class="btn btn-primary">{{ $t('common.search') }}</button>
        </div>
      </form>
    </div>

    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.users.loading') }}</span>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.users.error', { message: error.message }) }}</span>
    </div>

    <div v-else-if="filteredUsers.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">{{ $t('admin.users.notFound') }}</p>
    </div>

    <!-- Users Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.users.table.email') }}</th>
              <th>{{ $t('admin.users.table.role') }}</th>
              <th>{{ $t('admin.users.table.status') }}</th>
              <th>{{ $t('admin.users.table.date') }}</th>
              <th class="text-right">{{ $t('admin.users.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="user in filteredUsers" :key="user.id" class="row-hover">
              <td>
                <div class="flex items-center gap-3">
                  <div class="avatar avatar-placeholder">
                    <div class="bg-neutral text-neutral-content rounded-full size-10">
                      <span class="text-lg">{{ (user.email.at(0) || '?').toUpperCase() }}</span>
                    </div>
                  </div>
                  <div>
                    <div class="font-medium">{{ user.email }}</div>
                    <div class="text-xs text-gray-500">{{ user.name }}</div>
                  </div>
                </div>
              </td>
              <td>
                <span class="badge badge-soft" :class="user.role === 'Admin' ? 'badge-primary' : 'badge-secondary'">
                  {{ user.role }}
                </span>
              </td>
              <td>
                <span class="badge badge-soft text-xs" :class="user.active ? 'badge-success' : 'badge-error'">
                  {{ user.active ? $t('common.status.active') : $t('common.status.inactive') }}
                </span>
              </td>
              <td>{{ formatDate(user.createdAt) }}</td>
              <td class="text-right">
                <div class="flex justify-end gap-1">
                  <NuxtLinkLocale
                    :to="`/admin/users/${user.id}`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.view')"
                  >
                    <i class="icon-[tabler--eye] size-5"></i>
                  </NuxtLinkLocale>
                  <button
                    class="btn btn-circle btn-text btn-sm text-error"
                    type="button"
                    :aria-label="$t('common.delete')"
                    @click="confirmDelete(user)"
                  >
                    <i class="icon-[tabler--trash] size-5"></i>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

interface AdminUser {
  id: number
  email: string
  name: string
  role: string
  active: boolean
  createdAt: string
}

const { t } = useI18n()
const { apiFetch, useApiLazyFetch } = useApi()

const searchQuery = ref('')

const { pending, data: users, error, refresh } = useApiLazyFetch<AdminUser[]>('/api/admin/users')

const filteredUsers = computed(() => {
  if (!users.value) return []
  if (!searchQuery.value.trim()) return users.value

  const query = searchQuery.value.toLowerCase()
  return users.value.filter(user =>
    user.email.toLowerCase().includes(query) ||
    user.name.toLowerCase().includes(query) ||
    user.role.toLowerCase().includes(query)
  )
})

const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric'
  }).format(new Date(dateString))
}

const handleSearch = () => {
  // Search is handled reactively via computed
}

const confirmDelete = async (user: AdminUser) => {
  if (confirm(t('common.confirmDelete', { name: user.email }))) {
    try {
      await apiFetch(`/api/admin/users/${user.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert(t('common.errorDelete', { resource: t('admin.users.title').toLowerCase() }))
      console.error(err)
    }
  }
}
</script>
