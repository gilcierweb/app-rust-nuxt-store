<template>
  <div>
    <div class="container mx-auto px-4 py-8">
      <div class="mb-6">
        <NuxtLink to="/products" class="btn btn-outline btn-sm">
          ← Voltar para Produtos
        </NuxtLink>
      </div>
      
      <ProductForm
        @saved="handleProductSaved"
        @cancel="handleCancel"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductApi } from '~/types';

// Handle successful product creation
const handleProductSaved = (product: ProductApi) => {
  // You can redirect to the product detail page or show a success message
  console.log('Produto criado:', product);
  
  // Optionally redirect to the products list
  // await navigateTo('/products');
};

// Handle cancel action
const handleCancel = () => {
  // Redirect back to products list
  navigateTo('/products');
};

// Set page title
useHead({
  title: 'Criar Novo Produto'
});
</script> 