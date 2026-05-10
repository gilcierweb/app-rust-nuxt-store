<template>
    <div><h1>{{ t('pages.terms.title') }}</h1></div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.terms.title'),
  ogTitle: t('pages.terms.title'),
  description: t('pages.terms.description'),
  ogDescription: t('pages.terms.description'),
})
</script>
