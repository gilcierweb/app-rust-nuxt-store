<template>
  <div>
    <div class="mt-6">
      <h1 class="h1">Editar Produto</h1>
    </div>

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

    <!-- Form -->
    <AdminProductsForm
      v-else-if="product"
      :product="product"
      :is-editing="true"
      @saved="handleSaved"
      @cancel="navigateTo('/admin/products')"
    />
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

const handleSaved = (savedProduct: ProductApi) => {
  // Redirecionar após salvar com sucesso
  navigateTo('/admin/products')
}
</script>

<style scoped></style>