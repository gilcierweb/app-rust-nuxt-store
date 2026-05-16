<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/posts" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">{{ $t('admin.posts.detail.title') }}</h1>
          <p class="text-sm text-gray-500" v-if="post">ID: {{ post.id }}</p>
        </div>
      </div>

      <div v-if="post" class="flex gap-2">
        <button @click="deletePost" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          {{ $t('common.delete') }}
        </button>
        <NuxtLinkLocale :to="`/admin/posts/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          {{ $t('common.edit') }}
        </NuxtLinkLocale>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">{{ $t('admin.posts.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.posts.error', { message: error.message }) }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">{{ $t('common.actions.tryAgain') }}</button>
    </div>

    <!-- Content -->
    <div v-else-if="post" class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Main Info Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <h2 class="card-title mb-4">{{ $t('admin.categories.detail.mainInfo') }}</h2>

          <div class="space-y-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.posts.table.title') }}</span>
              </label>
              <div class="font-medium text-lg">{{ post.title || $t('admin.posts.noTitle') }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.posts.detail.content') }}</span>
              </label>
              <div class="prose max-w-none bg-base-200 p-4 rounded-md">
                <p v-if="post.content" class="whitespace-pre-wrap">{{ post.content }}</p>
                <p v-else class="text-gray-400 italic">{{ $t('admin.posts.detail.noContent') }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Settings / Meta Info Card -->
      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title mb-4">{{ $t('admin.categories.detail.settings') }}</h2>

          <div class="flex flex-col gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.posts.table.status') }}</span>
              </label>
              <div>
                <span :class="['badge badge-lg', statusBadgeClass]">
                  {{ getStatusLabel(post.status) }}
                </span>
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">{{ $t('admin.posts.table.author') }}</span>
              </label>
              <div class="font-medium">{{ post.user_id }}</div>
            </div>

            <div class="divider my-2"></div>

            <div class="text-xs text-gray-500 space-y-2">
              <div class="flex justify-between">
                <span>{{ $t('admin.categories.detail.createdAt') }}</span>
                <span class="font-medium">{{ formatDate(post.created_at) }}</span>
              </div>
              <div class="flex justify-between">
                <span>{{ $t('admin.categories.detail.updatedAt') }}</span>
                <span class="font-medium">{{ formatDate(post.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Not Found State -->
    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>{{ $t('admin.posts.notFound') }}</span>
      <NuxtLinkLocale to="/admin/posts" class="btn btn-sm">{{ $t('admin.posts.detail.back') }}</NuxtLinkLocale>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const { apiFetch, useApiFetch } = useApi()
const router = useRouter()
const { t } = useI18n()

const { pending, data: post, error, refresh } = useApiFetch<Post>(
  `/api/admin/posts/${route.params.id}`
)

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

const statusBadgeClass = computed(() => {
  switch (post.value?.status) {
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
})

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

const deletePost = async () => {
  if (!post.value) return

  if (confirm(t('admin.posts.detail.confirmDelete', { name: post.value.title || t('admin.posts.noTitle') }))) {
    try {
      await apiFetch(`/api/admin/posts/${post.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/posts')
    } catch (err) {
      alert(t('admin.posts.detail.errorDelete'))
      console.error(err)
    }
  }
}
</script>
