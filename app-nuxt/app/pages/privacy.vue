<template>
  <div class="max-w-4xl mx-auto py-12">
    <div class="bg-base-100 p-8 md:p-12 rounded-[2.5rem] shadow-sm border border-base-200">
      <div class="flex items-center justify-between mb-8 pb-8 border-b border-base-200">
        <div>
          <h1 class="h2 mb-2">{{ t('pages.privacy.title') }}</h1>
          <p class="text-sm text-base-content/40 italic">Last updated: May 10, 2026</p>
        </div>
        <div class="badge badge-primary badge-soft">Active</div>
      </div>

      <div class="prose prose-lg max-w-none text-base-content/70 space-y-8">
        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">1</span>
            Introduction
          </h2>
          <p>
            At App Rust Nuxt Store, we respect your privacy and are committed to protecting it. This Privacy Policy explains how we collect, use, and safeguard your information when you visit our website.
          </p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">2</span>
            Information Collection
          </h2>
          <p>
            We collect information that you provide directly to us, such as when you create an account, make a purchase, or contact our support team. This may include your name, email, and shipping address.
          </p>
        </section>

        <section>
          <h2 class="h4 text-base-content mb-4 flex items-center gap-2">
            <span class="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">3</span>
            How We Use Your Information
          </h2>
          <p>
            We use the information we collect to process your orders, provide customer support, and send you updates about our services. We do not sell your personal data to third parties.
          </p>
        </section>

        <section class="p-6 bg-base-200/50 rounded-2xl border border-base-200">
          <h3 class="font-bold mb-2">Cookies & Tracking</h3>
          <p class="text-sm">
            We use cookies to enhance your browsing experience and analyze site traffic. You can choose to disable cookies in your browser settings.
          </p>
        </section>

        <div class="pt-8 border-t border-base-200 text-center">
          <p class="text-sm text-base-content/50">
            If you have any questions about this policy, please <NuxtLink to="/contact" class="link link-primary">contact us</NuxtLink>.
          </p>
        </div>
      </div>
    </div>
  </div>
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