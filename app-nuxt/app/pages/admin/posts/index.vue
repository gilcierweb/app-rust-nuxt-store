<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">Posts</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Buscar posts"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">Buscar</button>
        </div>
      </form>

      <NuxtLink to="/admin/posts/new" class="btn btn-success">Adicionar</NuxtLink>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Carregando posts...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>Erro ao carregar posts: {{ error.message }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredPosts.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">Nenhum post encontrado.</p>
      <NuxtLink to="/admin/posts/new" class="btn btn-primary mt-4">Criar primeiro post</NuxtLink>
    </div>

    <!-- Posts Table -->
    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>Título</th>
            <th>Status</th>
            <th>Usuário ID</th>
            <th>Data</th>
            <th>Ações</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="post in filteredPosts" :key="post.id" class="row-hover">
            <td class="font-medium">{{ post.title || '(Sem título)' }}</td>
            <td>
              <span :class="['badge badge-soft text-xs', statusBadgeClass(post.status)]">
                {{ PostStatusLabels[post.status || 1] }}
              </span>
            </td>
            <td>{{ post.user_id }}</td>
            <td>{{ formatDate(post.created_at) }}</td>
            <td>
              <NuxtLink
                :to="`/admin/posts/${post.id}`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Ver detalhes"
              >
                <i class="icon-[tabler--eye] size-5"></i>
              </NuxtLink>
              <NuxtLink
                :to="`/admin/posts/${post.id}/edit`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Editar"
              >
                <i class="icon-[tabler--pencil] size-5"></i>
              </NuxtLink>
              <button
                type="button"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Excluir"
                @click="confirmDelete(post)"
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
import type { Post } from '~/types'
import { PostStatusLabels } from '~/types'

definePageMeta({
  layout: 'admin'
})

const config = useRuntimeConfig()
const { $truncate } = useNuxtApp()

const searchQuery = ref('')

const { pending, data: posts, error, refresh } = useLazyFetch<Post[]>(
  `${config.public.baseURL}/api/posts`
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
  if (confirm(`Tem certeza que deseja excluir o post "${post.title || '(Sem título)'}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/posts/${post.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert('Erro ao excluir post')
      console.error(err)
    }
  }
}
</script>

<style scoped></style>
