<template>
  <div class="container mx-auto my-6">
       
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

const { data: categories } = await useFetch<Category[]>('/api/categories/hierarchy');
</script>

<style scoped>
</style>
