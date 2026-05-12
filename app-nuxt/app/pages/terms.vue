<template>
  <div class="max-w-4xl mx-auto py-12">
    <div class="card bg-base-100 shadow-soft border border-base-200">
      <div class="card-body p-8 md:p-12">
        <div class="flex items-center justify-between mb-8 pb-8 border-b border-base-200">
          <div>
            <h1 class="card-title text-3xl mb-2">{{ t('terms.title') }}</h1>
            <p class="text-sm text-base-content/40 italic">{{ t('terms.lastUpdated') }}</p>
          </div>
          <div class="badge badge-secondary badge-soft">{{ t('terms.status') }}</div>
        </div>

        <div class="prose prose-lg max-w-none text-base-content/70 space-y-8">
        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-secondary/10 flex items-center justify-center text-secondary">1</span>
            {{ t('terms.sections.acceptance.title') }}
          </h2>
          <p>{{ t('terms.sections.acceptance.content') }}</p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-secondary/10 flex items-center justify-center text-secondary">2</span>
            {{ t('terms.sections.usage.title') }}
          </h2>
          <p>{{ t('terms.sections.usage.content') }}</p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-secondary/10 flex items-center justify-center text-secondary">3</span>
            {{ t('terms.sections.disclaimer.title') }}
          </h2>
          <p>{{ t('terms.sections.disclaimer.content') }}</p>
        </section>

        <div class="pt-8 border-t border-base-200 text-center">
          <p class="text-sm text-base-content/50">
            <i18n-t keypath="terms.questions" tag="span">
              <template #contact>
                <NuxtLink to="/contact" class="link link-secondary">{{ t('pages.about.cta.contact') }}</NuxtLink>
              </template>
            </i18n-t>
          </p>
        </div>
      </div>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('terms.title'),
  ogTitle: t('terms.title'),
  description: t('pages.terms.description'),
  ogDescription: t('pages.terms.description'),
})
</script>
