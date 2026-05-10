<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">Categorias</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Buscar categorias"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">Buscar</button>
        </div>
      </form>

      <NuxtLink to="/admin/categories/new" class="btn btn-success">Adicionar</NuxtLink>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Carregando categorias...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>Erro ao carregar categorias: {{ error.message }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredCategories.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">Nenhuma categoria encontrada.</p>
      <NuxtLink to="/admin/categories/new" class="btn btn-primary mt-4">Criar primeira categoria</NuxtLink>
    </div>

    <!-- Categories Table -->
    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>Nome</th>
            <th>Slug</th>
            <th>Descrição</th>
            <th>Status</th>
            <th>Posição</th>
            <th>Data</th>
            <th>Ações</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="category in filteredCategories" :key="category.id" class="row-hover">
            <td class="font-medium">{{ category.name }}</td>
            <td class="font-mono text-sm text-gray-500">{{ category.slug }}</td>
            <td>{{ $truncate(category.description || '', 50, '...') }}</td>
            <td>
              <span :class="['badge badge-soft text-xs', category.active ? 'badge-success' : 'badge-error']">
                {{ category.active ? 'Ativo' : 'Inativo' }}
              </span>
            </td>
            <td>{{ category.position ?? '-' }}</td>
            <td>{{ formatDate(category.created_at) }}</td>
            <td>
              <NuxtLink
                :to="`/admin/categories/${category.id}`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Ver detalhes"
              >
                <i class="icon-[tabler--eye] size-5"></i>
              </NuxtLink>
              <NuxtLink
                :to="`/admin/categories/${category.id}/edit`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Editar"
              >
                <i class="icon-[tabler--pencil] size-5"></i>
              </NuxtLink>
              <button
                type="button"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Excluir"
                @click="confirmDelete(category)"
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
import type { Category } from '~/types'

definePageMeta({
  layout: 'admin'
})

const config = useRuntimeConfig()
const { $truncate } = useNuxtApp()

const searchQuery = ref('')

const { pending, data: categories, error, refresh } = useLazyFetch<Category[]>(
  `${config.public.baseURL}/api/categories`
)

// Filtered categories based on search
const filteredCategories = computed(() => {
  if (!categories.value) return []
  if (!searchQuery.value.trim()) return categories.value
  
  const query = searchQuery.value.toLowerCase()
  return categories.value.filter(cat =>
    cat.name.toLowerCase().includes(query) ||
    cat.slug?.toLowerCase().includes(query) ||
    cat.description?.toLowerCase().includes(query)
  )
})

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
const confirmDelete = async (category: Category) => {
  if (confirm(`Tem certeza que deseja excluir a categoria "${category.name}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/categories/${category.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert('Erro ao excluir categoria')
      console.error(err)
    }
  }
}
</script>

<style scoped></style>