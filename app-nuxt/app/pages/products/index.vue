<template>
    <div>
        <h1 class="h1 my-4">{{ t('pages.products.title') }}</h1>

        <ProductList />
    </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
import ProductList from "~/components/ProductList.vue";
</script>

<style scoped></style>
