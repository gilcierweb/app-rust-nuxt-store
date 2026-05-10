<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLink to="/admin/reviews" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLink>
        <div>
          <h1 class="h1">Detalhes da Avaliação</h1>
          <p class="text-sm text-gray-500" v-if="review">ID: {{ review.id }}</p>
        </div>
      </div>

      <div v-if="review" class="flex gap-2">
        <button @click="deleteReview" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          Excluir
        </button>
        <NuxtLink :to="`/admin/reviews/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          Editar
        </NuxtLink>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">Carregando detalhes da avaliação...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Erro ao carregar avaliação: {{ error.message }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">Tentar novamente</button>
    </div>

    <!-- Content -->
    <div v-else-if="review" class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Main Info Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <h2 class="card-title mb-4">Informações da Avaliação</h2>

          <div class="space-y-4">
            <!-- Rating Display -->
            <div class="flex items-center gap-4 p-4 bg-warning/10 rounded-lg">
              <div class="text-5xl font-bold text-warning">{{ review.rating }}</div>
              <div class="flex text-warning">
                <i v-for="n in 5" :key="n" 
                   :class="['icon-[tabler--star-filled]', n <= (review.rating || 0) ? 'text-warning' : 'text-gray-300']" 
                   class="size-8"></i>
              </div>
            </div>

            <div class="form-control" v-if="review.title">
              <label class="label">
                <span class="label-text text-gray-500">Título</span>
              </label>
              <div class="font-medium text-lg">{{ review.title }}</div>
            </div>

            <div class="form-control" v-if="review.comment">
              <label class="label">
                <span class="label-text text-gray-500">Comentário</span>
              </label>
              <div class="prose max-w-none bg-base-200 p-4 rounded-md">
                <p class="whitespace-pre-wrap">{{ review.comment }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Meta Info Card -->
      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title mb-4">Detalhes</h2>

          <div class="flex flex-col gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Produto ID</span>
              </label>
              <div class="font-medium">{{ review.product_id }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Usuário ID</span>
              </label>
              <div class="font-medium">{{ review.user_id }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Compra Verificada</span>
              </label>
              <div>
                <span v-if="review.verified_purchase" class="badge badge-success badge-lg">
                  <i class="icon-[tabler--check] size-4 mr-1"></i>
                  Sim
                </span>
                <span v-else class="badge badge-ghost badge-lg">Não</span>
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Status</span>
              </label>
              <div>
                <span :class="['badge badge-lg', review.active ? 'badge-success' : 'badge-error']">
                  {{ review.active ? 'Ativa' : 'Inativa' }}
                </span>
              </div>
            </div>

            <div class="divider my-2"></div>

            <div class="text-xs text-gray-500 space-y-2">
              <div class="flex justify-between">
                <span>Criado em:</span>
                <span class="font-medium">{{ formatDate(review.created_at) }}</span>
              </div>
              <div class="flex justify-between">
                <span>Atualizado em:</span>
                <span class="font-medium">{{ formatDate(review.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Not Found State -->
    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>Avaliação não encontrada.</span>
      <NuxtLink to="/admin/reviews" class="btn btn-sm">Voltar para lista</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Review } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()
const router = useRouter()

const { pending, data: review, error, refresh } = useFetch<Review>(
  `${config.public.baseURL}/api/reviews/${route.params.id}`
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

const deleteReview = async () => {
  if (!review.value) return

  if (confirm(`Tem certeza que deseja excluir a avaliação #${review.value.id}?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/reviews/${review.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/reviews')
    } catch (err) {
      alert('Erro ao excluir avaliação')
      console.error(err)
    }
  }
}
</script>
