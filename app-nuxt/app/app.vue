<template>
    <div>
        <NuxtLoadingIndicator />
        <NuxtRouteAnnouncer />
        <NuxtLayout>
            <NuxtPage />
        </NuxtLayout>
    </div>
</template>

<script setup lang="ts">
useSeoMeta({
  ogTitle: 'App Rust Nuxt Store',
  ogDescription: 'App Rust Nuxt Store - Sua loja online com Rust e Nuxt',
  ogImage: '/og-image.png',
  twitterTitle: 'App Rust Nuxt Store',
  twitterDescription: 'App Rust Nuxt Store - Sua loja online com Rust e Nuxt',
  twitterImage: '/og-image.png',
})
</script>
