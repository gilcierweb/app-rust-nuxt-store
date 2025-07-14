<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold mb-8">Categories</h1>
    
    <ul class="menu sm:menu-horizontal bg-base-200 rounded-box">
      <CategoryItemHierarchy
        v-for="category in categories"
        :key="category.id"
        :category="category"
        :level="0"
      />
    </ul>
  </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types';
const config = useRuntimeConfig();

// Buscar categorias da API
const { data: categories } = await useFetch<Category[]>(`${config.public.baseURL}/api/categories/hierarchy`);
</script>

<style scoped>
.container {
  max-width: 1200px;
}
</style>