<template>
  <div class="container mx-auto">

   <h1 class="h1 my-4">{{ t('pages.categories.title') }}</h1>

    <CategoriesMenu />
    
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
</script>

<style scoped>
</style>