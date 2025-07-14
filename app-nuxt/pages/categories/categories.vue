<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold mb-6">Categorias</h1>
    
    <div class="mb-6">
      <input 
        v-model="searchTerm"
        placeholder="Buscar categorias..."
        class="w-full p-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
    </div>
    
    <div v-if="pending" class="text-center py-8">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"></div>
      <p class="mt-2 text-gray-600">Carregando categorias...</p>
    </div>
    
    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative">
      Erro ao carregar categorias: {{ error.message }}
    </div>
    
    <div v-else>
      <ul class="space-y-2">
        <CategoryItem
          v-for="category in filteredRootCategories"
          :key="category.id"
          :category="category"
          :search-term="searchTerm"
          :depth="0"
        />
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
const config = useRuntimeConfig();
const searchTerm = ref('');
const { data: categories, pending, error } = await useFetch(`${config.public.baseURL}/api/categories/hierarchy`);

// Filtrar categorias raiz que correspondem à pesquisa
const filteredRootCategories = computed(() => {
  if (!categories.value) return [];
  
  if (!searchTerm.value) {
    return categories.value.filter(cat => !cat.parent_id);
  }
  
  // Filtrar todas as categorias que correspondem ao termo de busca
  return categories.value.filter(category => 
    categoryMatchesSearch(category, searchTerm.value)
  );
});

// Função recursiva para verificar se a categoria ou seus filhos correspondem à pesquisa
function categoryMatchesSearch(category, term) {
  const termLower = term.toLowerCase();
  
  // Verificar se a categoria atual corresponde
  if (
    (category.name && category.name.toLowerCase().includes(termLower)) ||
    (category.slug && category.slug.toLowerCase().includes(termLower)) ||
    (category.description && category.description.toLowerCase().includes(termLower))
  ) {
    return true;
  }
  
  // Verificar se algum filho corresponde
  if (category.children) {
    for (const child of category.children) {
      if (categoryMatchesSearch(child, term)) {
        return true;
      }
    }
  }
  
  return false;
}
</script>

<style scoped>
.container {
  max-width: 800px;
}
</style>