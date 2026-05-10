<template>
    <div>
        <h1 class="h1 my-4">{{ t('pages.products.title') }}</h1>

        <ProductList />
    </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.products.title'),
  ogTitle: t('pages.products.title'),
  description: t('pages.products.description'),
  ogDescription: t('pages.products.description'),
})
import ProductList from "~/components/ProductList.vue";
</script>

<style scoped></style>
