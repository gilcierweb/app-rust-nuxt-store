<template>
    <div>
        <div v-if="pending">
            Loading ... <br>
            <span class="loading loading-spinner  size-32"></span>
        </div>
        <div v-else class="w-full">
            <h1 class="h1">{{ category.name }} - {{ category.slug }}</h1>
            <p>{{ category.description }}</p>
            <p>{{ category.active }}</p>
            <p>{{ category.position }}</p>
        </div>

    </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types';

const route = useRoute()
const config = useRuntimeConfig();

const id = route.params.id;

const { pending, data: category } = await useLazyFetch<Category>(`${config.public.baseURL}/api/categories/${id}`);

</script>

<style scoped></style>