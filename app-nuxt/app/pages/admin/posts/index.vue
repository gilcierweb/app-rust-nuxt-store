<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.posts.title') }}</h1>
      <NuxtLinkLocale to="/admin/posts/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.posts.add') }}
      </NuxtLinkLocale>
    </div>

    <!-- Filters & Search -->
    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <form @submit.prevent="handleSearch" class="flex flex-wrap gap-4 items-end">
          <div class="form-control flex-1 min-w-[240px]">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">Buscar Post</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 group-focus-within:text-primary transition-colors"></span>
              <input 
                v-model="searchQuery" 
                type="text" 
                :placeholder="$t('admin.posts.searchPlaceholder')" 
                class="input input-bordered w-full pl-10" 
              />
            </div>
          </div>
          <button type="submit" class="btn btn-ghost">
            Limpar
          </button>
        </form>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-gray-500">{{ $t('admin.posts.loading') }}</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error mb-6">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.posts.error', { message: error.message }) }}</span>
    </div>

    <!-- Posts Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.posts.table.title') }}</th>
              <th>{{ $t('admin.posts.table.status') }}</th>
              <th>{{ $t('admin.posts.table.author') }}</th>
              <th>{{ $t('admin.posts.table.date') }}</th>
              <th class="text-right">{{ $t('admin.posts.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="post in filteredPosts" :key="post.id" class="row-hover">
              <td>
                <div class="font-medium text-base text-primary">{{ post.title || $t('admin.posts.noTitle') }}</div>
                <div class="text-xs text-gray-400 font-mono">{{ post.slug || '-' }}</div>
              </td>
              <td>
                <span :class="['badge badge-soft text-xs', statusBadgeClass(post.status)]">
                  {{ getStatusLabel(post.status) }}
                </span>
              </td>
              <td>
                 <div class="flex items-center gap-2">
                    <div class="avatar avatar-xs placeholder">
                      <div class="bg-neutral text-neutral-content rounded-full size-6">
                        <span class="text-[10px]">A</span>
                      </div>
                    </div>
                    <span class="text-sm">ID: {{ post.user_id }}</span>
                 </div>
              </td>
              <td>
                <div class="text-sm">{{ formatDate(post.created_at) }}</div>
              </td>
              <td class="text-right">
                <div class="flex justify-end gap-1">
                  <NuxtLinkLocale
                    :to="`/admin/posts/${post.id}`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.view')"
                  >
                    <i class="icon-[tabler--eye] size-5"></i>
                  </NuxtLinkLocale>
                  <NuxtLinkLocale
                    :to="`/admin/posts/${post.id}/edit`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.edit')"
                  >
                    <i class="icon-[tabler--pencil] size-5"></i>
                  </NuxtLinkLocale>
                  <button
                    type="button"
                    class="btn btn-circle btn-text btn-sm text-error"
                    :aria-label="$t('common.delete')"
                    @click="confirmDelete(post)"
                  >
                    <i class="icon-[tabler--trash] size-5"></i>
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="filteredPosts.length === 0">
              <td colspan="5" class="text-center py-20 text-gray-500 italic">
                {{ $t('admin.posts.notFound') }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types'

definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiLazyFetch } = useApi()
const { t } = useI18n()

const searchQuery = ref('')

const { pending, data: posts, error, refresh } = useApiLazyFetch<Post[]>(
  '/api/admin/posts',
  { key: 'admin-posts-list' }
)

// Filtered posts based on search
const filteredPosts = computed(() => {
  if (!posts.value) return []
  if (!searchQuery.value.trim()) return posts.value

  const query = searchQuery.value.toLowerCase()
  return posts.value.filter(post =>
    post.title?.toLowerCase().includes(query) ||
    post.content?.toLowerCase().includes(query)
  )
})

// Status label
const getStatusLabel = (status?: number) => {
  switch (status) {
    case 1: return t('admin.posts.status.draft')
    case 2: return t('admin.posts.status.pending')
    case 3: return t('admin.posts.status.scheduled')
    case 4: return t('admin.posts.status.published')
    case 5: return t('admin.posts.status.private')
    case 6: return t('admin.posts.status.archived')
    case 7: return t('admin.posts.status.trashed')
    case 8: return t('admin.posts.status.rejected')
    default: return t('admin.posts.status.unknown')
  }
}

// Status badge class
const statusBadgeClass = (status?: number) => {
  switch (status) {
    case 4: return 'badge-success' // Published
    case 1: return 'badge-warning' // Draft
    case 2: return 'badge-info' // Pending Review
    case 3: return 'badge-primary' // Scheduled
    case 5: return 'badge-secondary' // Private
    case 6: return 'badge-ghost' // Archived
    case 7: return 'badge-error' // Trashed
    case 8: return 'badge-error' // Rejected
    default: return 'badge-ghost'
  }
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
const confirmDelete = async (post: Post) => {
  if (confirm(t('admin.posts.detail.confirmDelete', { name: post.title || t('admin.posts.noTitle') }))) {
    try {
      await apiFetch(`/api/admin/posts/${post.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert(t('admin.posts.detail.errorDelete'))
      console.error(err)
    }
  }
}
</script>
