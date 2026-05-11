<template>
  <div class="max-w-4xl mx-auto py-12">
    <div class="bg-base-100 p-8 md:p-12 rounded-[2.5rem] shadow-sm border border-base-200">
      <div class="flex items-center justify-between mb-8 pb-8 border-b border-base-200">
        <div>
          <h1 class="h2 mb-2">{{ t('pages.terms.title') }}</h1>
          <p class="text-sm text-base-content/40 italic">Last updated: May 10, 2026</p>
        </div>
        <div class="badge badge-secondary badge-soft">Legal</div>
      </div>

      <div class="prose prose-lg max-w-none text-base-content/70 space-y-8">
        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-secondary/10 flex items-center justify-center text-secondary">1</span>
            Acceptance of Terms
          </h2>
          <p>
            By accessing and using this website, you accept and agree to be bound by the terms and provision of this agreement.
          </p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-secondary/10 flex items-center justify-center text-secondary">2</span>
            Use License
          </h2>
          <p>
            Permission is granted to temporarily download one copy of the materials on App Rust Nuxt Store's website for personal, non-commercial transitory viewing only.
          </p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-secondary/10 flex items-center justify-center text-secondary">3</span>
            Disclaimer
          </h2>
          <p>
            The materials on App Rust Nuxt Store's website are provided on an 'as is' basis. App Rust Nuxt Store makes no warranties, expressed or implied, and hereby disclaims all other warranties.
          </p>
        </section>

        <section class="p-6 bg-base-200/50 rounded-2xl border border-base-200">
          <h3 class="font-bold mb-2">Liability</h3>
          <p class="text-sm">
            In no event shall App Rust Nuxt Store or its suppliers be liable for any damages arising out of the use or inability to use the materials on our website.
          </p>
        </section>

        <div class="pt-8 border-t border-base-200 text-center">
          <p class="text-sm text-base-content/50">
            By continuing to use our site, you acknowledge that you have read and understood these terms.
          </p>
        </div>
      </div>
    </div>
  </div>
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

