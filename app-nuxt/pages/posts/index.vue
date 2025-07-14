<template>
  <div>
    <h1 class="h1">Page Posts</h1>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-4 gap-4 mt-6">

      <div v-if="pending">
        Loading ... <br>
        <span class="loading loading-spinner  size-32"></span>
      </div>
      <div v-else v-for="post in posts" :key="post">
        <div class="card sm:max-w-sm  shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
          <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
          <div class="card-body space-y-3">
            <h5 class="card-title">{{ post.title }}</h5>
            <p>{{ $truncate(post.content, 70, '...') }}</p>
            <div class="card-actions">
              <NuxtLink :to="`/posts/${post.id}`" class="btn btn-primary w-full">Read More</NuxtLink>             
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();

const { pending, data: posts } = await useLazyFetch(`${config.public.baseURL}/api/posts`)
</script>

<style scoped></style>
