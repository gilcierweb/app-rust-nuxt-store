<template>
  <div>
    <h1>Page Posts</h1>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-4 gap-4 mt-6">

      <div v-if="pending">
        Loading ... <br>
        <span class="loading loading-spinner  size-32"></span>
      </div>
      <div v-else v-for="profile in profiles" :key="profile">
        <div class="card sm:max-w-sm">
          <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
          <div class="card-body space-y-3">
            <h5 class="card-title">{{ profile.nickname }}</h5>
            <p>{{ $truncate(profile.full_name, 70, '...') }}</p>
            <div class="card-actions">
              <NuxtLink :to="`/profiles/${profile.id}`" class="btn btn-primary w-full">Read More</NuxtLink>             
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

const { pending, data: profiles } = await useLazyFetch(`${config.public.baseURL}/api/profiles`)
</script>

<style scoped></style>
