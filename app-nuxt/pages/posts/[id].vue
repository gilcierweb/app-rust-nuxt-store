<template>
    <div>
        <div v-if="status === 'pending'">
            Loading ... <br>
            <span class="loading loading-spinner  size-32"></span>
        </div>
        <div v-else class="w-full">
            <NuxtImg src="https://via.assets.so/game.png?id=1&q=95&w=400&h=400&fit=fill" loading="lazy":alt="post?.title" />
            <h2>{{ post?.title }}</h2>
            <p>{{ post?.content }}</p>
            <p>
                <span :color="post?.status ? 'green' : 'red'" variant="flat">
                    {{ post?.status }}
                </span>
            </p>
            <h1>{{ $route.params.id }} - {{ route.params.id }}</h1>

        </div>
    </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types';

const route = useRoute()
const config = useRuntimeConfig();
// When accessing /posts/1, route.params.id will be 1
console.log(route.params.id)
const id = route.params.id;

const { status, data: post } = await useLazyFetch<Post>(`${config.public.baseURL}/api/posts/${id}`);
</script>

<style scoped></style>