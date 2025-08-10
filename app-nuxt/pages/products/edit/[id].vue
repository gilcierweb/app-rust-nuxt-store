<template>
  <div>
    <div class="container mx-auto px-4 py-8">
      <div class="mb-6">
        <NuxtLink to="/products" class="btn btn-outline btn-sm">
          ← Voltar para Produtos
        </NuxtLink>
      </div>
      
      <!-- Loading state while fetching product -->
      <div v-if="pending" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary size-16"></span>
        <span class="ml-4 text-lg">Carregando produto...</span>
      </div>
      
      <!-- Error state -->
      <div v-else-if="error" class="alert alert-error">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span>Erro ao carregar produto: {{ error }}</span>
      </div>
      
      <!-- Product form -->
      <ProductForm
        v-else-if="product"
        :product="product"
        :is-editing="true"
        @saved="handleProductUpdated"
        @cancel="handleCancel"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductApi } from '~/types';

const route = useRoute();
const config = useRuntimeConfig();

const id = route.params.id;

// Fetch product data for editing
const { pending, data: product, error } = await useLazyFetch<ProductApi>(`${config.public.baseURL}/api/products/${id}`);

// Handle successful product update
const handleProductUpdated = (updatedProduct: ProductApi) => {
  console.log('Produto atualizado:', updatedProduct);
  
  // Optionally redirect to the product detail page
  // await navigateTo(`/products/${updatedProduct.id}`);
};

// Handle cancel action
const handleCancel = () => {
  // Redirect back to products list
  navigateTo('/products');
};

// Set page title
useHead({
  title: computed(() => product.value ? `Editar: ${product.value.name}` : 'Editar Produto')
});
</script> 