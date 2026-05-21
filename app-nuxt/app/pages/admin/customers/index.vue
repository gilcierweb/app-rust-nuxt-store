<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">{{ $t('admin.customers.title') }}</h1>
    </div>

    <div class="card shadow-base-300/10 mb-6 shadow-md">
      <div class="card-body">
        <div class="mb-4 flex items-center justify-between gap-3">
          <div></div>
          <NuxtLinkLocale to="/admin/customers/new" class="btn btn-success">{{ $t('admin.customers.add') }}</NuxtLinkLocale>
        </div>

        <form @submit.prevent="handleSearch">
          <div class="grid grid-cols-1 gap-4 md:grid-cols-[1fr_auto]">
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="$t('admin.customers.searchPlaceholder')"
            class="input input-bordered w-full"
          />
          <button type="submit" class="btn btn-primary">{{ $t('common.search') }}</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.customers.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>{{ $t('admin.customers.error', { message: error.message }) }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="profiles.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">{{ $t('admin.customers.notFound') }}</p>
      <NuxtLinkLocale to="/admin/customers/new" class="btn btn-primary mt-4">{{ $t('admin.customers.createFirst') }}</NuxtLinkLocale>
    </div>

    <!-- Profiles Table -->
    <div v-else class="card shadow-base-300/10 w-full shadow-md overflow-hidden">
      <div class="card-body p-0">
        <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.customers.table.name') }}</th>
              <th>{{ $t('admin.customers.table.username') }}</th>
              <th>{{ $t('admin.customers.table.phone') }}</th>
              <th>{{ $t('admin.customers.table.user') }}</th>
              <th>{{ $t('admin.customers.table.date') }}</th>
              <th>{{ $t('admin.customers.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="profile in profiles" :key="profile.id" class="row-hover">
              <td>
                <div class="flex items-center gap-3">
                  <div v-if="profile.avatar" class="avatar">
                    <div class="mask mask-squircle w-10 h-10">
                      <img :src="profile.avatar" :alt="profile.full_name" />
                    </div>
                  </div>
                  <div v-else class="avatar avatar-placeholder">
                    <div class="bg-neutral text-neutral-content w-10 h-10 rounded-full">
                      <span class="text-lg">{{ getInitials(profile.full_name) }}</span>
                    </div>
                  </div>
                  <div class="font-medium">{{ profile.full_name || `${profile.first_name} ${profile.last_name}` }}</div>
                </div>
              </td>
              <td>{{ profile.username || '-' }}</td>
              <td>{{ profile.phone || '-' }}</td>
              <td>{{ profile.user_id }}</td>
              <td>{{ formatDate(profile.created_at) }}</td>
              <td>
                <NuxtLinkLocale
                  :to="`/admin/customers/${profile.id}`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.view')"
                >
                  <i class="icon-[tabler--eye] size-5"></i>
                </NuxtLinkLocale>
                <NuxtLinkLocale
                  :to="`/admin/customers/${profile.id}/edit`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.edit')"
                >
                  <i class="icon-[tabler--pencil] size-5"></i>
                </NuxtLinkLocale>
                <button
                  type="button"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.delete')"
                  @click="confirmDelete(profile)"
                >
                  <span class="icon-[tabler--trash] size-5"></span>
                </button>
              </td>
            </tr>
          </tbody>
        </table>
        </div>

        <AdminPagination
          :current-page="currentPage"
          :page-size="pageSize"
          :current-count="profiles.length"
          :total="data?.total || 0"
          :pending="pending"
          :summary="$t('admin.customers.pagination.showing', { current: profiles.length, total: data?.total || 0 })"
          :previous-label="$t('admin.customers.pagination.previous')"
          :next-label="$t('admin.customers.pagination.next')"
          @change="changePage"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AdminPaginatedResponse, Profile } from '~/types'

definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiFetch } = useApi()
const { t } = useI18n()
const toast = useAppToast()
const dialog = useAppDialog()

const searchQuery = ref('')
const appliedSearchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(20)

const { pending, data, error, refresh } = await useApiFetch<AdminPaginatedResponse<Profile>>(
  '/api/admin/profiles',
  {
    key: 'admin-customers-list',
    query: computed(() => ({
      page: currentPage.value,
      page_size: pageSize.value,
      search: appliedSearchQuery.value || undefined
    }))
  }
)

const profiles = computed(() => data.value?.items || [])

// Get initials from name
const getInitials = (name?: string) => {
  if (!name) return '?'
  return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2)
}

// Format date
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

const confirmDelete = async (profile: Profile) => {
  const name = profile.full_name || `${profile.first_name} ${profile.last_name}`
  const confirmed = await dialog.confirm({
    message: t('admin.customers.detail.confirmDelete', { name }),
    confirmLabel: t('common.delete'),
    tone: 'danger'
  })
  if (!confirmed) return

  try {
    await apiFetch(`/api/admin/profiles/${profile.id}`, {
      method: 'DELETE'
    })
    await refresh()
  } catch (err) {
    toast.error(t('admin.customers.detail.errorDelete'))
    console.error(err)
  }
}
</script>
