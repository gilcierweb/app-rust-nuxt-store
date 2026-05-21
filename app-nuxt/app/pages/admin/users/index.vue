<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.users.title') }}</h1>
      <NuxtLinkLocale to="/admin/users/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.users.add') }}
      </NuxtLinkLocale>
    </div>

    <div class="card shadow-base-300/10 mb-6 shadow-md">
      <form class="card-body" @submit.prevent="handleSearch">
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

    <div v-else-if="users.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">{{ $t('admin.users.notFound') }}</p>
    </div>

    <!-- Users Table -->
    <div v-else class="card shadow-base-300/10 w-full shadow-md overflow-hidden">
      <div class="card-body p-0">
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
              <tr v-for="user in users" :key="user.id" class="row-hover">
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
                    <NuxtLinkLocale
                      :to="`/admin/users/${user.id}/edit`"
                      class="btn btn-circle btn-text btn-sm"
                      :aria-label="$t('common.edit')"
                    >
                      <i class="icon-[tabler--pencil] size-5"></i>
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

        <AdminPagination
          :current-page="currentPage"
          :page-size="pageSize"
          :current-count="users.length"
          :total="data?.total || 0"
          :pending="pending"
          :summary="$t('admin.users.pagination.showing', { current: users.length, total: data?.total || 0 })"
          :previous-label="$t('admin.users.pagination.previous')"
          :next-label="$t('admin.users.pagination.next')"
          @change="changePage"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AdminPaginatedResponse } from '~/types'

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
const { apiFetch, useApiFetch } = useApi()
const toast = useAppToast()
const dialog = useAppDialog()

const searchQuery = ref('')
const appliedSearchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(20)

const apiQuery = reactive({
  page: currentPage,
  page_size: pageSize,
  search: computed(() => appliedSearchQuery.value || undefined)
})

const { pending, data, error, refresh } = await useApiFetch<AdminPaginatedResponse<AdminUser>>(
  '/api/admin/users',
  {
    key: 'admin-users-list',
    query: apiQuery
  }
)

const users = computed(() => data.value?.items || [])

const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric'
  }).format(new Date(dateString))
}

const handleSearch = () => {
  currentPage.value = 1
  appliedSearchQuery.value = searchQuery.value.trim()
}

const changePage = (page: number) => {
  currentPage.value = Math.max(1, page)
}

const confirmDelete = async (user: AdminUser) => {
  const confirmed = await dialog.confirm({
    message: t('common.confirmDelete', { name: user.email }),
    confirmLabel: t('common.delete'),
    tone: 'danger'
  })
  if (!confirmed) return

  try {
    await apiFetch(`/api/admin/users/${user.id}`, {
      method: 'DELETE'
    })
    await refresh()
  } catch (err) {
    toast.error(t('common.errorDelete', { resource: t('admin.users.title').toLowerCase() }))
    console.error(err)
  }
}
</script>
