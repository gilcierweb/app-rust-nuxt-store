<template>
  <section>
    <h1 class="h1">{{ t('pages.about.title') }}</h1>
    <p>{{ t('pages.about.description') }}</p>
  </section>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.about.title'),
  ogTitle: t('pages.about.title'),
  description: t('pages.about.description'),
  ogDescription: t('pages.about.description'),
})
</script>