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
</script>