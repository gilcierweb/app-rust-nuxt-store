<template>
    <div>
<ul class="menu sm:menu-horizontal">
  <li>
    <span class="menu-title">Services</span>
    <ul class="menu" v-for="category in categories" :key="category.id">
      <li class="category">
        <NuxtLink to="/" :title="category.name">
      <strong>{{ category.name }}</strong> ( {{ category.slug }} )
      </NuxtLink>
       
      </li>    
    </ul>
  </li>
  </ul>
    </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();

const { pending, data: categories } = await useLazyFetch(`${config.public.baseURL}/api/categories/hierarchy`)
</script>

<style scoped>

</style>