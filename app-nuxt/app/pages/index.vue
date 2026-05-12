<template>
  <div class="space-y-16 pb-20">
    <!-- Hero Section -->
    <section class="relative overflow-hidden rounded-[2rem] bg-base-300 shadow-2xl">
      <div class="absolute inset-0 bg-gradient-to-br from-primary/20 via-transparent to-secondary/20"></div>
      <AppBanner />
    </section>

    <!-- Categories Section -->
    <section>
      <CategoriesList />
    </section>

    <!-- Featured Products -->
    <section>
      <div class="flex items-center justify-between mb-8">
        <div>
          <h2 class="h3 gradient-text">{{ t('pages.home.products') }}</h2>
          <p class="text-base-content/60 mt-1">{{ t('pages.home.productsSubtitle') }}</p>
        </div>
        <NuxtLink to="/products" class="btn btn-primary btn-sm gap-2">
          {{ t('pages.home.shopAll') }} <span class="icon-[tabler--arrow-right] size-4"></span>
        </NuxtLink>
      </div>
      <ProductList />
    </section>

    <!-- Newsletter Section -->
    <section class="relative overflow-hidden rounded-[2.5rem] bg-primary text-primary-content p-10 md:p-20">
      <div class="absolute top-0 right-0 -mt-20 -mr-20 size-80 bg-white/10 rounded-full blur-3xl"></div>
      <div class="absolute bottom-0 left-0 -mb-20 -ml-20 size-80 bg-secondary/20 rounded-full blur-3xl"></div>
      
      <div class="relative z-10 max-w-2xl mx-auto text-center">
        <h2 class="text-3xl md:text-5xl font-bold mb-6">{{ t('pages.home.newsletter.title') }}</h2>
        <p class="text-primary-content/80 text-lg mb-10">
          {{ t('pages.home.newsletter.description') }}
        </p>
        
        <form class="flex flex-col sm:flex-row gap-4">
          <input type="email" :placeholder="t('pages.home.newsletter.placeholder')" 
            class="input input-lg bg-white/10 border-white/20 text-white placeholder:text-white/40 focus:bg-white/20 grow rounded-2xl h-14" />
          <button type="submit" class="btn btn-secondary btn-lg h-14 px-10 rounded-2xl shadow-xl shadow-secondary/20 transition-transform hover:scale-105">
            {{ t('pages.home.newsletter.button') }}
          </button>
        </form>
        
        <p class="text-xs text-primary-content/40 mt-6">
          <i18n-t keypath="privacy.questions" tag="span">
            <template #contact>
              <span>{{ t('pages.privacy.title') }}</span>
            </template>
          </i18n-t>
        </p>
      </div>
    </section>

    <!-- Features Section -->
    <section class="grid grid-cols-1 md:grid-cols-3 gap-8 pt-8">
      <div class="card bg-base-100 shadow-soft border border-base-200">
        <div class="card-body flex-row items-start gap-4 p-6">
          <div class="size-12 rounded-2xl bg-base-200 shadow-sm flex items-center justify-center shrink-0">
            <span class="icon-[tabler--truck-delivery] size-6 text-primary"></span>
          </div>
          <div>
            <h3 class="font-bold mb-1">{{ t('pages.home.features.shipping.title') }}</h3>
            <p class="text-sm text-base-content/60">{{ t('pages.home.features.shipping.description') }}</p>
          </div>
        </div>
      </div>
      <div class="card bg-base-100 shadow-soft border border-base-200">
        <div class="card-body flex-row items-start gap-4 p-6">
          <div class="size-12 rounded-2xl bg-base-200 shadow-sm flex items-center justify-center shrink-0">
            <span class="icon-[tabler--shield-check] size-6 text-primary"></span>
          </div>
          <div>
            <h3 class="font-bold mb-1">{{ t('pages.home.features.payments.title') }}</h3>
            <p class="text-sm text-base-content/60">{{ t('pages.home.features.payments.description') }}</p>
          </div>
        </div>
      </div>
      <div class="card bg-base-100 shadow-soft border border-base-200">
        <div class="card-body flex-row items-start gap-4 p-6">
          <div class="size-12 rounded-2xl bg-base-200 shadow-sm flex items-center justify-center shrink-0">
            <span class="icon-[tabler--help-circle] size-6 text-primary"></span>
          </div>
          <div>
            <h3 class="font-bold mb-1">{{ t('pages.home.features.support.title') }}</h3>
            <p class="text-sm text-base-content/60">{{ t('pages.home.features.support.description') }}</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.home.title'),
  ogTitle: t('pages.home.title'),
  description: t('pages.home.description'),
  ogDescription: t('pages.home.description'),
})
</script>