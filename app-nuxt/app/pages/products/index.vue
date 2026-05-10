<template>
    <div>
        <h1 class="h1 my-4">{{ t('pages.products.title') }}</h1>

        <ProductList />
    </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();
import type { ProductApi, Product } from '~/types';
const { t } = useI18n()

const { pending: pendingApi, data: productsData } = await useFetch<ProductApi[]>(`${config.public.baseURL}/api/products`);
const productsApi = computed(() => productsData.value ?? []);
const { pending, data } = await useFetch<{ products: Product[] }>(`https://dummyjson.com/products`);
const products = computed(() => data.value?.products ?? []);

useSeoMeta({
  title: t('pages.products.title'),
  ogTitle: t('pages.products.title'),
  description: t('pages.products.description'),
  ogDescription: t('pages.products.description'),
})
import ProductList from "~/components/ProductList.vue";
</script>

<style scoped></style>
