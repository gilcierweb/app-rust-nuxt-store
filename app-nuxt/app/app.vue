<template>
  <div>
    <NuxtLoadingIndicator />
    <NuxtRouteAnnouncer />
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <AppConfirmDialog />
    <AppToasts />
  </div>
</template>

<script setup lang="ts">
import AppConfirmDialog from '~/components/ui/AppConfirmDialog.vue'
import AppToasts from '~/components/ui/AppToasts.vue'

useSeoMeta({
  ogTitle: 'App Rust Nuxt Store',
  ogDescription: 'App Rust Nuxt Store - Sua loja online com Rust e Nuxt',
  ogImage: '/og-image.png',
  twitterTitle: 'App Rust Nuxt Store',
  twitterDescription: 'App Rust Nuxt Store - Sua loja online com Rust e Nuxt',
  twitterImage: '/og-image.png',
})
</script>
