<template>
  <div class="max-w-4xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h1 class="card-title text-2xl font-bold">Editar Categoria</h1>
        
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
        </div>
        
        <div v-else-if="error" class="alert alert-error">
          <span>Erro ao carregar categoria: {{ error.message }}</span>
        </div>
        
        <div v-else-if="category">
          <!-- TODO: Implementar formulário de edição -->
          <p>Categoria: {{ category.name }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()

const { data: category, pending, error } = useLazyFetch(
  `${config.public.baseURL}/api/categories/${route.params.id}`
)
</script>
