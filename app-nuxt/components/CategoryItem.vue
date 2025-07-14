<template>
    <div>
  <li 
    class="category-item bg-white rounded-lg shadow-sm overflow-hidden border border-gray-100"
    :class="{ 'mt-2': depth > 0 }"
  >
    <div 
      class="category-header p-4 flex items-center cursor-pointer hover:bg-gray-50 transition-colors"
      :class="{ 'bg-blue-50': isExpanded }"
      @click="toggleExpanded"
    >
      <span class="mr-2">
        <span v-if="hasChildren">
          <Icon v-if="isExpanded" name="heroicons:chevron-down" class="text-blue-600" />
          <Icon v-else name="heroicons:chevron-right" class="text-gray-500" />
        </span>
        <span v-else class="w-4 h-4 inline-block"></span>
      </span>
      
      <div class="flex-grow">
        <span class="font-medium text-gray-900">{{ category.name || 'Sem nome' }}</span>
        <span v-if="category.slug" class="ml-2 text-sm text-gray-500">({{ category.slug }})</span>
      </div>
      
      <span 
        class="status-badge px-2 py-1 rounded-full text-xs font-medium"
        :class="category.active ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'"
      >
        {{ category.active ? 'Ativa' : 'Inativa' }}
      </span>
    </div>
    
    <div v-if="isExpanded" class="category-details px-4 pb-4">
      <div v-if="category.description" class="text-gray-700 mb-3">
        {{ category.description }}
      </div>
      
      <div class="meta-grid grid grid-cols-2 gap-2 text-sm text-gray-600">
        <div><span class="font-medium">ID:</span> {{ category.id }}</div>
        <div><span class="font-medium">Posição:</span> {{ category.position || '-' }}</div>
        <div><span class="font-medium">Criado em:</span> {{ formatDate(category.created_at) }}</div>
        <div><span class="font-medium">Atualizado em:</span> {{ formatDate(category.updated_at) }}</div>
      </div>
      
      <div v-if="hasChildren" class="mt-4">
        <ul class="children-list pl-6 border-l-2 border-gray-200 space-y-3">
          <CategoryItem
            v-for="child in filteredChildren"
            :key="child.id"
            :category="child"
            :search-term="searchTerm"
            :depth="depth + 1"
          />
        </ul>
      </div>
    </div>
  </li>
    </div>
</template>

<script setup >

import { computed, ref } from 'vue';

const props = defineProps({
  category: {
    type: Object,
    required: true
  },
  searchTerm: {
    type: String,
    default: ''
  },
  depth: {
    type: Number,
    default: 0
  }
});

const isExpanded = ref(props.depth === 0 || props.searchTerm);

const hasChildren = computed(() => {
  return props.category.children && props.category.children.length > 0;
});

const filteredChildren = computed(() => {
  if (!props.category.children) return [];
  
  if (!props.searchTerm) return props.category.children;
  
  return props.category.children.filter(child => 
    childMatchesSearch(child, props.searchTerm.toLowerCase())
  );
});

function childMatchesSearch(child, termLower) {
  return (
    (child.name && child.name.toLowerCase().includes(termLower)) ||
    (child.slug && child.slug.toLowerCase().includes(termLower)) ||
    (child.description && child.description.toLowerCase().includes(termLower))
  );
}

function toggleExpanded() {
  if (hasChildren.value) {
    isExpanded.value = !isExpanded.value;
  }
}

function formatDate(dateString) {
  if (!dateString) return '-';
  
  try {
    const date = new Date(dateString);
    return date.toLocaleDateString('pt-BR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  } catch {
    return '-';
  }
}

if (props.searchTerm) {
  const termLower = props.searchTerm.toLowerCase();
  const shouldExpand = 
    (props.category.name && props.category.name.toLowerCase().includes(termLower)) ||
    (props.category.slug && props.category.slug.toLowerCase().includes(termLower)) ||
    (props.category.description && props.category.description.toLowerCase().includes(termLower));
  
  if (shouldExpand) {
    isExpanded.value = true;
  }
}

</script>

<style scoped>
</style>