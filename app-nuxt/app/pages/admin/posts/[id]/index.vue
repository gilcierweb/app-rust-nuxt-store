<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLink to="/admin/posts" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLink>
        <div>
          <h1 class="h1">Detalhes do Post</h1>
          <p class="text-sm text-gray-500" v-if="post">ID: {{ post.id }}</p>
        </div>
      </div>

      <div v-if="post" class="flex gap-2">
        <button @click="deletePost" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          Excluir
        </button>
        <NuxtLink :to="`/admin/posts/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          Editar
        </NuxtLink>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">Carregando detalhes do post...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Erro ao carregar post: {{ error.message }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">Tentar novamente</button>
    </div>

    <!-- Content -->
    <div v-else-if="post" class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Main Info Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <h2 class="card-title mb-4">Informações Principais</h2>

          <div class="space-y-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Título</span>
              </label>
              <div class="font-medium text-lg">{{ post.title || '(Sem título)' }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Conteúdo</span>
              </label>
              <div class="prose max-w-none bg-base-200 p-4 rounded-md">
                <p v-if="post.content" class="whitespace-pre-wrap">{{ post.content }}</p>
                <p v-else class="text-gray-400 italic">Sem conteúdo</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Settings / Meta Info Card -->
      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title mb-4">Configurações</h2>

          <div class="flex flex-col gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Status</span>
              </label>
              <div>
                <span :class="['badge badge-lg', statusBadgeClass]">
                  {{ PostStatusLabels[post.status || 1] }}
                </span>
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Usuário ID</span>
              </label>
              <div class="font-medium">{{ post.user_id }}</div>
            </div>

            <div class="divider my-2"></div>

            <div class="text-xs text-gray-500 space-y-2">
              <div class="flex justify-between">
                <span>Criado em:</span>
                <span class="font-medium">{{ formatDate(post.created_at) }}</span>
              </div>
              <div class="flex justify-between">
                <span>Atualizado em:</span>
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
      <span>Post não encontrado.</span>
      <NuxtLink to="/admin/posts" class="btn btn-sm">Voltar para lista</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types'
import { PostStatusLabels } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()
const router = useRouter()

const { pending, data: post, error, refresh } = useFetch<Post>(
  `${config.public.baseURL}/api/posts/${route.params.id}`
)

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

  if (confirm(`Tem certeza que deseja excluir o post "${post.value.title || '(Sem título)'}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/posts/${post.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/posts')
    } catch (err) {
      alert('Erro ao excluir post')
      console.error(err)
    }
  }
}
</script>
