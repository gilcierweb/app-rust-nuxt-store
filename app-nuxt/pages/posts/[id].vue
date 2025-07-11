<template>
    <div>
        <div v-if="status === 'pending'">
            Loading ... <br>
            <span class="loading loading-spinner  size-32"></span>
        </div>
        <div v-else class="w-full">
            <img height="400px" src="https://via.assets.so/game.png?id=1&q=95&w=400&h=400&fit=fill" cover></img>
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
const route = useRoute()
const config = useRuntimeConfig();
// When accessing /posts/1, route.params.id will be 1
console.log(route.params.id)
const id = route.params.id;

interface Post {
    id: number;
    title: string;
    content: string;
    status: boolean;
}

const { status, data: post } = await useLazyFetch<Post>(`${config.public.baseURL}/api/posts/${id}`);
</script>

<style scoped></style>