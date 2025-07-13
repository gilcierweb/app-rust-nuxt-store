<template>
    <div>
        <div v-if="pending">
            Loading ... <br />
            <span class="loading loading-spinner size-32"></span>
        </div>
        <div v-else class="w-full">
            <img height="400px" :src="profile?.avatar" />
            <h2>{{ profile?.first_name }}</h2>
            <h2>{{ profile?.last_name }}</h2>
            <h2>{{ profile?.full_name }}</h2>
            <p>{{ profile?.username }}</p>
            <p>{{ profile?.nickname }}</p>
            <p>{{ profile?.birth_date }}</p>
            <p>{{ profile?.whatsapp }}</p>
            <p>{{ profile?.phone }}</p>         
            <p>{{ profile?.bio }}</p>
            <h1>{{ $route.params.id }} - {{ route.params.id }}</h1>
        </div>
    </div>
</template>

<script setup lang="ts">
const route = useRoute();
const config = useRuntimeConfig();
// When accessing /posts/1, route.params.id will be 1
console.log(route.params.id);
const id = route.params.id;

interface Profile {
    id: number;
    first_name: string,
    last_name: string,
    full_name: string,
    username: string,
    nickname: string,
    phone:number,
    birth_date: Date,
    avatar: string,
    bio: string,
    whatsapp: number,
    user_id: number,
    title: string;
    content: string;
    status: boolean;
}

const { pending, data: profile } = await useLazyFetch<Profile>(`${config.public.baseURL}/api/profiles/${id}`);
</script>

<style scoped></style>
