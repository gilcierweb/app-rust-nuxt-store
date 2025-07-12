<template>
  <div>

    <AppBanner />

    <CategoriesList />

    <h1 class="h1 my-4">Products</h1>
    <ProductList /> 

    <AppAlert>
      This is an auto-imported component
    </AppAlert>
  </div>
</template>

<script setup lang="ts">
import ProductList from '~/components/ProductList.vue';

</script>