<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">{{ $t('admin.profiles.title') }}</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="$t('admin.profiles.searchPlaceholder')"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">{{ $t('common.search') }}</button>
        </div>
      </form>

      <NuxtLink to="/admin/profiles/new" class="btn btn-success">{{ $t('admin.profiles.add') }}</NuxtLink>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.profiles.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>{{ $t('admin.profiles.error', { message: error.message }) }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredProfiles.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">{{ $t('admin.profiles.notFound') }}</p>
      <NuxtLink to="/admin/profiles/new" class="btn btn-primary mt-4">{{ $t('admin.profiles.createFirst') }}</NuxtLink>
    </div>

    <!-- Profiles Table -->
    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>{{ $t('admin.profiles.table.name') }}</th>
            <th>{{ $t('admin.profiles.table.username') }}</th>
            <th>{{ $t('admin.profiles.table.phone') }}</th>
            <th>{{ $t('admin.profiles.table.user') }}</th>
            <th>{{ $t('admin.profiles.table.date') }}</th>
            <th>{{ $t('admin.profiles.table.actions') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="profile in filteredProfiles" :key="profile.id" class="row-hover">
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
              <NuxtLink
                :to="`/admin/profiles/${profile.id}`"
                class="btn btn-circle btn-text btn-sm"
                :aria-label="$t('common.view')"
              >
                <i class="icon-[tabler--eye] size-5"></i>
              </NuxtLink>
              <NuxtLink
                :to="`/admin/profiles/${profile.id}/edit`"
                class="btn btn-circle btn-text btn-sm"
                :aria-label="$t('common.edit')"
              >
                <i class="icon-[tabler--pencil] size-5"></i>
              </NuxtLink>
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
  </div>
</template>

<script setup lang="ts">
import type { Profile } from '~/types'

definePageMeta({
  layout: 'admin'
})

const config = useRuntimeConfig()
const { t } = useI18n()

const searchQuery = ref('')

const { pending, data: profiles, error, refresh } = useLazyFetch<Profile[]>(
  `${config.public.baseURL}/api/profiles`
)

// Filtered profiles based on search
const filteredProfiles = computed(() => {
  if (!profiles.value) return []
  if (!searchQuery.value.trim()) return profiles.value

  const query = searchQuery.value.toLowerCase()
  return profiles.value.filter(profile =>
    profile.first_name?.toLowerCase().includes(query) ||
    profile.last_name?.toLowerCase().includes(query) ||
    profile.full_name?.toLowerCase().includes(query) ||
    profile.username?.toLowerCase().includes(query)
  )
})

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

// Search handler
const handleSearch = () => {
  // Search is handled reactively via computed
}

// Delete confirmation
const confirmDelete = async (profile: Profile) => {
  const name = profile.full_name || `${profile.first_name} ${profile.last_name}`
  if (confirm(t('admin.profiles.detail.confirmDelete', { name }))) {
    try {
      await $fetch(`${config.public.baseURL}/api/profiles/${profile.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert(t('admin.profiles.detail.errorDelete'))
      console.error(err)
    }
  }
}
</script>

<style scoped></style>
