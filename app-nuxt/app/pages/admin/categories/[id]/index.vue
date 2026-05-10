<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLink to="/admin/categories" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLink>
        <div>
          <h1 class="h1">Detalhes da Categoria</h1>
          <p class="text-sm text-gray-500" v-if="category">ID: {{ category.id }}</p>
        </div>
      </div>
      
      <div v-if="category" class="flex gap-2">
        <button @click="deleteCategory" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          Excluir
        </button>
        <NuxtLink :to="`/admin/categories/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          Editar
        </NuxtLink>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">Carregando detalhes da categoria...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Erro ao carregar categoria: {{ error.message }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">Tentar novamente</button>
    </div>

    <!-- Content -->
    <div v-else-if="category" class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      
      <!-- Main Info Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <h2 class="card-title mb-4">Informações Principais</h2>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Nome</span>
              </label>
              <div class="font-medium text-lg">{{ category.name }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Slug</span>
              </label>
              <div class="font-mono bg-base-200 px-3 py-2 rounded-md inline-block">
                {{ category.slug }}
              </div>
            </div>

            <div class="form-control md:col-span-2">
              <label class="label">
                <span class="label-text text-gray-500">Descrição</span>
              </label>
              <div class="prose max-w-none">
                <p v-if="category.description">{{ category.description }}</p>
                <p v-else class="text-gray-400 italic">Sem descrição</p>
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
                <span :class="['badge badge-lg', category.active ? 'badge-success' : 'badge-error']">
                  {{ category.active ? 'Ativo' : 'Inativo' }}
                </span>
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Posição</span>
              </label>
              <div class="font-medium text-lg">{{ category.position ?? '-' }}</div>
            </div>

            <div class="divider my-2"></div>

            <div class="text-xs text-gray-500 space-y-2">
              <div class="flex justify-between">
                <span>Criado em:</span>
                <span class="font-medium">{{ formatDate(category.created_at) }}</span>
              </div>
              <div class="flex justify-between">
                <span>Atualizado em:</span>
                <span class="font-medium">{{ formatDate(category.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Not Found State (if API returns null but no error) -->
    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>Categoria não encontrada.</span>
      <NuxtLink to="/admin/categories" class="btn btn-sm">Voltar para lista</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()
const router = useRouter()

const { pending, data: category, error, refresh } = useFetch<Category>(
  `${config.public.baseURL}/api/categories/${route.params.id}`
)

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

const deleteCategory = async () => {
  if (!category.value) return
  
  if (confirm(`Tem certeza que deseja excluir a categoria "${category.value.name}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/categories/${category.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/categories')
    } catch (err) {
      alert('Erro ao excluir categoria')
      console.error(err)
    }
  }
}
</script>