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
          <p class="text-base-content/60 mt-1">Our handpicked selection for you</p>
        </div>
        <NuxtLink to="/products" class="btn btn-primary btn-sm gap-2">
          Shop All <span class="icon-[tabler--arrow-right] size-4"></span>
        </NuxtLink>
      </div>
      <ProductList />
    </section>

    <!-- Newsletter Section -->
    <section class="relative overflow-hidden rounded-[2.5rem] bg-primary text-primary-content p-10 md:p-20">
      <div class="absolute top-0 right-0 -mt-20 -mr-20 size-80 bg-white/10 rounded-full blur-3xl"></div>
      <div class="absolute bottom-0 left-0 -mb-20 -ml-20 size-80 bg-secondary/20 rounded-full blur-3xl"></div>
      
      <div class="relative z-10 max-w-2xl mx-auto text-center">
        <h2 class="text-3xl md:text-5xl font-bold mb-6">Stay Ahead of the Curve</h2>
        <p class="text-primary-content/80 text-lg mb-10">
          Subscribe to our newsletter and get exclusive access to new arrivals, special offers, and early-bird discounts.
        </p>
        
        <form class="flex flex-col sm:flex-row gap-4">
          <input type="email" placeholder="Enter your email address" 
            class="input input-lg bg-white/10 border-white/20 text-white placeholder:text-white/40 focus:bg-white/20 grow rounded-2xl h-14" />
          <button type="submit" class="btn btn-secondary btn-lg h-14 px-10 rounded-2xl shadow-xl shadow-secondary/20 transition-transform hover:scale-105">
            Subscribe Now
          </button>
        </form>
        
        <p class="text-xs text-primary-content/40 mt-6">
          By subscribing, you agree to our <NuxtLink to="/terms" class="underline">Terms</NuxtLink> and <NuxtLink to="/privacy" class="underline">Privacy Policy</NuxtLink>.
        </p>
      </div>
    </section>

    <!-- Features Section -->
    <section class="grid grid-cols-1 md:grid-cols-3 gap-8 pt-8">
      <div class="flex items-start gap-4 p-6 rounded-3xl bg-base-200/50">
        <div class="size-12 rounded-2xl bg-white shadow-sm flex items-center justify-center shrink-0">
          <span class="icon-[tabler--truck-delivery] size-6 text-primary"></span>
        </div>
        <div>
          <h3 class="font-bold mb-1">Fast & Free Shipping</h3>
          <p class="text-sm text-base-content/60">Free shipping on all orders over R$ 200,00.</p>
        </div>
      </div>
      <div class="flex items-start gap-4 p-6 rounded-3xl bg-base-200/50">
        <div class="size-12 rounded-2xl bg-white shadow-sm flex items-center justify-center shrink-0">
          <span class="icon-[tabler--shield-check] size-6 text-primary"></span>
        </div>
        <div>
          <h3 class="font-bold mb-1">Secure Payments</h3>
          <p class="text-sm text-base-content/60">Your data is always safe and encrypted.</p>
        </div>
      </div>
      <div class="flex items-start gap-4 p-6 rounded-3xl bg-base-200/50">
        <div class="size-12 rounded-2xl bg-white shadow-sm flex items-center justify-center shrink-0">
          <span class="icon-[tabler--help-circle] size-6 text-primary"></span>
        </div>
        <div>
          <h3 class="font-bold mb-1">24/7 Support</h3>
          <p class="text-sm text-base-content/60">Our team is always here to help you.</p>
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