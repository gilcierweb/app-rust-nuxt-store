<template>
    <div>
        <h1 class="h1">{{ t('pages.contact.title') }}</h1>
    </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.contact.title'),
  ogTitle: t('pages.contact.title'),
  description: t('pages.contact.description'),
  ogDescription: t('pages.contact.description'),
})
</script>