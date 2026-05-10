<template>
  <div class="max-w-4xl mx-auto">
    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Carregando produto...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error mt-6">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>Erro ao carregar produto: {{ error.message }}</span>
      <NuxtLink to="/admin/products" class="btn btn-sm">Voltar</NuxtLink>
    </div>

    <!-- Product Details -->
    <div v-else-if="product" class="card bg-white shadow-lg">
      <div class="card-body">
        <!-- Header -->
        <div class="flex justify-between items-start mb-6">
          <div>
            <h1 class="card-title text-2xl font-bold">{{ product.name }}</h1>
            <p class="text-gray-500">SKU: {{ product.sku }}</p>
          </div>
          <div class="flex gap-2">
            <NuxtLink :to="`/admin/products/${product.id}/edit`" class="btn btn-primary btn-sm">
              Editar
            </NuxtLink>
            <button type="button" class="btn btn-error btn-sm" @click="confirmDelete">
              Excluir
            </button>
          </div>
        </div>

        <!-- Status Badges -->
        <div class="flex gap-2 mb-6">
          <span :class="['badge', product.active ? 'badge-success' : 'badge-error']">
            {{ product.active ? 'Ativo' : 'Inativo' }}
          </span>
          <span v-if="product.featured" class="badge badge-warning">
            Destaque
          </span>
          <span :class="['badge', statusBadgeClass]">
            {{ statusLabel }}
          </span>
        </div>

        <!-- Product Image -->
        <div v-if="coverImage" class="mb-6">
          <img
            :src="`${config.public.baseURL}/uploads/products/${coverImage.image}`"
            :alt="coverImage.alt_text || product.name"
            class="w-full max-w-md rounded-lg shadow-md"
          />
        </div>

        <!-- Details Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Pricing -->
          <div class="space-y-3">
            <h3 class="font-semibold text-lg border-b pb-2">Preços</h3>
            <div class="flex justify-between">
              <span class="text-gray-600">Preço de Venda:</span>
              <span class="font-bold text-primary">{{ formatCurrency(product.price) }}</span>
            </div>
            <div v-if="product.costPrice" class="flex justify-between">
              <span class="text-gray-600">Preço de Custo:</span>
              <span>{{ formatCurrency(product.costPrice) }}</span>
            </div>
            <div v-if="product.comparePrice" class="flex justify-between">
              <span class="text-gray-600">Preço Comparativo:</span>
              <span class="line-through text-gray-400">{{ formatCurrency(product.comparePrice) }}</span>
            </div>
          </div>

          <!-- Info -->
          <div class="space-y-3">
            <h3 class="font-semibold text-lg border-b pb-2">Informações</h3>
            <div v-if="product.slug" class="flex justify-between">
              <span class="text-gray-600">Slug:</span>
              <span class="font-mono text-sm">{{ product.slug }}</span>
            </div>
            <div v-if="product.categoryId" class="flex justify-between">
              <span class="text-gray-600">Categoria ID:</span>
              <span>{{ product.categoryId }}</span>
            </div>
          </div>
        </div>

        <!-- Descriptions -->
        <div v-if="product.shortDescription" class="mt-6">
          <h3 class="font-semibold text-lg border-b pb-2 mb-3">Descrição Curta</h3>
          <p class="text-gray-700">{{ product.shortDescription }}</p>
        </div>

        <div v-if="product.description" class="mt-6">
          <h3 class="font-semibold text-lg border-b pb-2 mb-3">Descrição Completa</h3>
          <p class="text-gray-700 whitespace-pre-wrap">{{ product.description }}</p>
        </div>

        <!-- Images Gallery -->
        <div v-if="product.images && product.images.length > 0" class="mt-6">
          <h3 class="font-semibold text-lg border-b pb-2 mb-3">Galeria de Imagens</h3>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div
              v-for="image in product.images"
              :key="image.id"
              class="relative"
            >
              <img
                :src="`${config.public.baseURL}/uploads/products/${image.image}`"
                :alt="image.alt_text || 'Imagem do produto'"
                class="w-full h-32 object-cover rounded-lg"
              />
              <span v-if="image.cover" class="absolute top-1 left-1 badge badge-primary badge-sm">
                Capa
              </span>
            </div>
          </div>
        </div>

        <!-- Back Button -->
        <div class="mt-8 pt-4 border-t">
          <NuxtLink to="/admin/products" class="btn btn-outline">
            ← Voltar para Lista
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductApi } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()

const { data: product, pending, error } = useLazyFetch<ProductApi>(
  `${config.public.baseURL}/api/products/${route.params.id}`
)

// Computed para imagem de capa
const coverImage = computed(() => {
  if (!product.value?.images) return null
  return product.value.images.find(img => img.cover) || product.value.images[0]
})

// Status helpers
const statusLabel = computed(() => {
  switch (product.value?.status) {
    case 0: return 'Inativo'
    case 1: return 'Ativo'
    case 2: return 'Rascunho'
    default: return 'Desconhecido'
  }
})

const statusBadgeClass = computed(() => {
  switch (product.value?.status) {
    case 0: return 'badge-error'
    case 1: return 'badge-success'
    case 2: return 'badge-warning'
    default: return 'badge-ghost'
  }
})

// Format currency
const formatCurrency = (value: number | undefined) => {
  if (!value) return 'R$ 0,00'
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL'
  }).format(value)
}

// Delete confirmation
const confirmDelete = async () => {
  if (!product.value) return
  
  if (confirm(`Tem certeza que deseja excluir "${product.value.name}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/products/${product.value.id}`, {
        method: 'DELETE'
      })
      navigateTo('/admin/products')
    } catch (err) {
      alert('Erro ao excluir produto')
      console.error(err)
    }
  }
}
</script>

<style scoped></style>