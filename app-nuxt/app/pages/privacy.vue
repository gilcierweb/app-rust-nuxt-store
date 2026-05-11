<template>
  <div class="max-w-4xl mx-auto py-12">
    <div class="bg-base-100 p-8 md:p-12 rounded-[2.5rem] shadow-sm border border-base-200">
      <div class="flex items-center justify-between mb-8 pb-8 border-b border-base-200">
        <div>
          <h1 class="h2 mb-2">{{ t('privacy.title') }}</h1>
          <p class="text-sm text-base-content/40 italic">{{ t('privacy.lastUpdated') }}</p>
        </div>
        <div class="badge badge-primary badge-soft">{{ t('privacy.status') }}</div>
      </div>

      <div class="prose prose-lg max-w-none text-base-content/70 space-y-8">
        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">1</span>
            {{ t('privacy.sections.intro.title') }}
          </h2>
          <p>{{ t('privacy.sections.intro.content') }}</p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">2</span>
            {{ t('privacy.sections.collection.title') }}
          </h2>
          <p>{{ t('privacy.sections.collection.content') }}</p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">3</span>
            {{ t('privacy.sections.usage.title') }}
          </h2>
          <p>{{ t('privacy.sections.usage.content') }}</p>
        </section>

        <section class="p-6 bg-base-200/50 rounded-2xl border border-base-200">
          <h3 class="font-bold mb-2">{{ t('privacy.sections.cookies.title') }}</h3>
          <p class="text-sm">{{ t('privacy.sections.cookies.content') }}</p>
        </section>

        <div class="pt-8 border-t border-base-200 text-center">
          <p class="text-sm text-base-content/50">
            <i18n-t keypath="privacy.questions" tag="span">
              <template #contact>
                <NuxtLink to="/contact" class="link link-primary">{{ t('pages.about.cta.contact') }}</NuxtLink>
              </template>
            </i18n-t>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('privacy.title'),
  ogTitle: t('privacy.title'),
  description: t('pages.privacy.description'),
  ogDescription: t('pages.privacy.description'),
})
</script>