<template>
  <li>
    <template v-if="hasChildren">
      <span class="menu-title">{{ category.name }}</span>
      <ul class="menu">
        <CategoryItemHierarchy
          v-for="child in category.children"
          :key="child.id"
          :category="child"
          :level="level + 1"
        />
      </ul>
    </template>
    
    <template v-else>
      <NuxtLink :to="`/categories/${category.id}`">{{ category.name }}</NuxtLink>
    </template>
  </li>
</template>

<script setup lang="ts">
import type { Category } from '~/types';

const props = defineProps({
  category: {
    type: Object as () => Category,
    required: true
  },
  level: {
    type: Number,
    default: 0
  }
});

const hasChildren = computed(() => {
  return props.category.children && props.category.children.length > 0;
});
</script>