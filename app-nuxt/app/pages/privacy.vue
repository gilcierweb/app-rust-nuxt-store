<template>
    <div><h1>{{ t('pages.privacy.title') }}</h1></div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.privacy.title'),
  ogTitle: t('pages.privacy.title'),
  description: t('pages.privacy.description'),
  ogDescription: t('pages.privacy.description'),
})
</script>