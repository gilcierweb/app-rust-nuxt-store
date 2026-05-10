<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">Avaliações</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Buscar avaliações"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">Buscar</button>
        </div>
      </form>

      <NuxtLink to="/admin/reviews/new" class="btn btn-success">Adicionar</NuxtLink>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Carregando avaliações...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>Erro ao carregar avaliações: {{ error.message }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredReviews.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">Nenhuma avaliação encontrada.</p>
      <NuxtLink to="/admin/reviews/new" class="btn btn-primary mt-4">Criar primeira avaliação</NuxtLink>
    </div>

    <!-- Reviews Table -->
    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>Produto ID</th>
            <th>Usuário ID</th>
            <th>Avaliação</th>
            <th>Título</th>
            <th>Verificada</th>
            <th>Status</th>
            <th>Data</th>
            <th>Ações</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="review in filteredReviews" :key="review.id" class="row-hover">
            <td>{{ review.product_id }}</td>
            <td>{{ review.user_id }}</td>
            <td>
              <div class="flex items-center gap-1">
                <span class="font-bold text-warning">{{ review.rating }}</span>
                <i class="icon-[tabler--star-filled] text-warning size-4"></i>
              </div>
            </td>
            <td>{{ review.title || '-' }}</td>
            <td>
              <span v-if="review.verified_purchase" class="badge badge-success badge-sm">
                <i class="icon-[tabler--check] size-3 mr-1"></i>
                Sim
              </span>
              <span v-else class="badge badge-ghost badge-sm">Não</span>
            </td>
            <td>
              <span :class="['badge badge-soft text-xs', review.active ? 'badge-success' : 'badge-error']">
                {{ review.active ? 'Ativa' : 'Inativa' }}
              </span>
            </td>
            <td>{{ formatDate(review.created_at) }}</td>
            <td>
              <NuxtLink
                :to="`/admin/reviews/${review.id}`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Ver detalhes"
              >
                <i class="icon-[tabler--eye] size-5"></i>
              </NuxtLink>
              <NuxtLink
                :to="`/admin/reviews/${review.id}/edit`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Editar"
              >
                <i class="icon-[tabler--pencil] size-5"></i>
              </NuxtLink>
              <button
                type="button"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Excluir"
                @click="confirmDelete(review)"
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
import type { Review } from '~/types'

definePageMeta({
  layout: 'admin'
})

const config = useRuntimeConfig()

const searchQuery = ref('')

const { pending, data: reviews, error, refresh } = useLazyFetch<Review[]>(
  `${config.public.baseURL}/api/reviews`
)

// Filtered reviews based on search
const filteredReviews = computed(() => {
  if (!reviews.value) return []
  if (!searchQuery.value.trim()) return reviews.value

  const query = searchQuery.value.toLowerCase()
  return reviews.value.filter(review =>
    review.title?.toLowerCase().includes(query) ||
    review.comment?.toLowerCase().includes(query)
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
const confirmDelete = async (review: Review) => {
  if (confirm(`Tem certeza que deseja excluir a avaliação #${review.id}?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/reviews/${review.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert('Erro ao excluir avaliação')
      console.error(err)
    }
  }
}
</script>

<style scoped></style>
