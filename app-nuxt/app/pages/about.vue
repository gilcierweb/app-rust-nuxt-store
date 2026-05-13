<template>
  <div class="space-y-20 pb-20">
    <!-- Hero Section -->
    <section class="text-center max-w-4xl mx-auto pt-10">
      <span class="badge badge-primary badge-soft px-4 py-1.5 mb-6">{{ t('pages.about.hero.badge') }}</span>
      <h1 class="h1 mb-6 gradient-text">{{ t('pages.about.hero.title') }}</h1>
      <p class="text-xl text-base-content/60 leading-relaxed">
        {{ t('pages.about.hero.description') }}
      </p>
    </section>

    <!-- Our Story Section -->
    <section class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
      <div class="relative">
        <div class="absolute -top-4 -left-4 size-24 bg-primary/20 rounded-full blur-2xl"></div>
        <div class="absolute -bottom-4 -right-4 size-24 bg-secondary/20 rounded-full blur-2xl"></div>
        <div class="aspect-video rounded-[2.5rem] overflow-hidden shadow-2xl bg-base-300">
           <div class="flex items-center justify-center h-full bg-gradient-to-br from-base-300 to-base-200">
             <span class="icon-[tabler--building-store] size-20 text-base-content/10"></span>
           </div>
        </div>
      </div>
      <div>
        <h2 class="h2 mb-6">{{ t('pages.about.story.title') }}</h2>
        <div class="space-y-4 text-base-content/70 leading-relaxed">
          <p>{{ t('pages.about.story.p1') }}</p>
          <p>{{ t('pages.about.story.p2') }}</p>
          <div class="pt-6 grid grid-cols-2 gap-6">
            <div>
              <div class="text-3xl font-bold text-primary mb-1">{{ t('pages.about.story.stats.customersVal') }}</div>
              <div class="text-sm font-medium opacity-60">{{ t('pages.about.story.stats.customers') }}</div>
            </div>
            <div>
              <div class="text-3xl font-bold text-secondary mb-1">{{ t('pages.about.story.stats.productsVal') }}</div>
              <div class="text-sm font-medium opacity-60">{{ t('pages.about.story.stats.products') }}</div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Mission & Vision -->
    <section class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <div class="card bg-base-100 shadow-soft border border-base-200 hover-lift">
        <div class="card-body p-10">
          <div class="size-14 rounded-2xl bg-primary/10 flex items-center justify-center mb-6">
            <span class="icon-[tabler--target-arrow] size-7 text-primary"></span>
          </div>
          <h3 class="card-title text-2xl mb-4">{{ t('pages.about.mission.title') }}</h3>
          <p class="text-base-content/60 leading-relaxed">
            {{ t('pages.about.mission.description') }}
          </p>
        </div>
      </div>
      <div class="card bg-base-100 shadow-soft border border-base-200 hover-lift">
        <div class="card-body p-10">
          <div class="size-14 rounded-2xl bg-secondary/10 flex items-center justify-center mb-6">
            <span class="icon-[tabler--eye] size-7 text-secondary"></span>
          </div>
          <h3 class="card-title text-2xl mb-4">{{ t('pages.about.vision.title') }}</h3>
          <p class="text-base-content/60 leading-relaxed">
            {{ t('pages.about.vision.description') }}
          </p>
        </div>
      </div>
    </section>

    <!-- Core Values -->
    <section class="text-center py-10">
      <h2 class="h2 mb-12">{{ t('pages.about.values.title') }}</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8">
        <div v-for="value in values" :key="value.title" class="space-y-4">
          <div class="size-16 rounded-full bg-white shadow-md mx-auto flex items-center justify-center">
            <span :class="[value.icon, 'size-8 text-primary']"></span>
          </div>
          <h4 class="font-bold">{{ t(value.title) }}</h4>
          <p class="text-sm text-base-content/50 px-4">{{ t(value.description) }}</p>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="bg-base-200 rounded-[2.5rem] p-12 text-center">
      <h2 class="h3 mb-6">{{ t('pages.about.cta.title') }}</h2>
      <p class="mb-8 text-base-content/60">{{ t('pages.about.cta.description') }}</p>
      <div class="flex flex-wrap justify-center gap-4">
        <NuxtLink to="/contact" class="btn btn-primary">{{ t('pages.about.cta.contact') }}</NuxtLink>
        <NuxtLink to="/products" class="btn btn-ghost">{{ t('pages.about.cta.browse') }}</NuxtLink>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.about.title'),
  ogTitle: t('pages.about.title'),
  description: t('pages.about.description'),
  ogDescription: t('pages.about.description'),
})

const values = [
  { title: 'pages.about.values.quality.title', icon: 'icon-[tabler--award]', description: 'pages.about.values.quality.description' },
  { title: 'pages.about.values.innovation.title', icon: 'icon-[tabler--bulb]', description: 'pages.about.values.innovation.description' },
  { title: 'pages.about.values.customerLove.title', icon: 'icon-[tabler--heart]', description: 'pages.about.values.customerLove.description' },
  { title: 'pages.about.values.integrity.title', icon: 'icon-[tabler--shield-check]', description: 'pages.about.values.integrity.description' }
]
</script>