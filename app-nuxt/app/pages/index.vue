<template>
  <div>

    <AppBanner />

    <h1 class="h1 my-4">{{ t('pages.home.categories') }}</h1>

    <CategoriesMenu />

    <CategoriesList />

    <h1 class="h1 my-4">{{ t('pages.home.products') }}</h1>
    <ProductList />

    <AppAlert>
      {{ t('pages.home.autoImport') }}
    </AppAlert>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.home.title'),
  ogTitle: t('pages.home.title'),
  description: t('pages.home.description'),
  ogDescription: t('pages.home.description'),
})
</script>