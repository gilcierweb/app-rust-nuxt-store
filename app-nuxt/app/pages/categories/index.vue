<template>
  <div class="container mx-auto">

   <h1 class="h1 my-4">{{ t('pages.categories.title') }}</h1>

    <CategoriesMenu />
    
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.categories.title'),
  ogTitle: t('pages.categories.title'),
  description: t('pages.categories.description'),
  ogDescription: t('pages.categories.description'),
})
</script>

<style scoped>
</style>