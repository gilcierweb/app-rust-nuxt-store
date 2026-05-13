<template>
  <div class="pb-20">
    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-32">
      <div class="relative">
        <div class="size-20 rounded-3xl border-4 border-primary/20 animate-pulse"></div>
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="loading loading-ring loading-lg text-primary"></span>
        </div>
      </div>
      <p class="mt-6 text-base-content/40 font-medium tracking-widest uppercase text-xs">{{ t('pages.categories.detail.loading') }}</p>
    </div>

    <!-- Category Not Found -->
    <div v-else-if="!category" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="size-24 rounded-full bg-base-200 flex items-center justify-center mb-6">
        <span class="icon-[tabler--category-off] size-12 opacity-20" />
      </div>
      <h2 class="h3 mb-2">{{ t('pages.categories.detail.notFound') }}</h2>
      <NuxtLink to="/categories" class="btn btn-primary btn-lg px-10 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-105">
        {{ t('pages.categories.detail.browseAll') }}
      </NuxtLink>
    </div>

    <!-- Category Detail -->
    <div v-else>
      <!-- Hero Section -->
      <section class="relative bg-gradient-to-br from-primary via-secondary to-accent overflow-hidden rounded-[3rem] mb-16">
        <div class="absolute inset-0 bg-black/20"></div>
        <div class="absolute inset-0 bg-gradient-to-r from-primary/90 to-transparent"></div>
        
        <!-- Decorative Elements -->
        <div class="absolute top-0 right-0 w-96 h-96 bg-white/10 rounded-full -translate-y-20 translate-x-20 blur-3xl"></div>
        <div class="absolute bottom-0 left-0 w-72 h-72 bg-white/5 rounded-full translate-y-20 -translate-x-20 blur-2xl"></div>
        
        <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
          <div class="text-center">
            <div class="inline-flex items-center justify-center w-20 h-20 bg-white/20 backdrop-blur-sm rounded-2xl mb-6">
              <span :class="[category.icon || 'icon-[tabler--category]', 'size-10 text-white']"></span>
            </div>
            
            <h1 class="text-4xl md:text-5xl font-bold text-white mb-4">
              {{ category.name }}
            </h1>
            
            <p class="text-xl text-white/90 mb-8 max-w-2xl mx-auto">
              {{ category.description || t('pages.categories.detail.fallbackDescription') }}
            </p>
            
            <div class="flex flex-col sm:flex-row gap-4 justify-center items-center mb-8">
              <div class="flex items-center text-white/80">
                <span class="icon-[tabler--box] size-5 mr-2"></span>
                <span>{{ category.productsCount || 0 }} {{ t('pages.categories.detail.productsLabel') }}</span>
              </div>
              <div class="hidden sm:block w-1 h-1 bg-white/40 rounded-full"></div>
              <div class="flex items-center text-white/80">
                <span class="icon-[tabler--star-filled] size-5 mr-2"></span>
                <span>4.6/5 {{ t('pages.categories.detail.averageRating') }}</span>
              </div>
              <div class="hidden sm:block w-1 h-1 bg-white/40 rounded-full"></div>
              <div class="flex items-center text-white/80">
                <span class="icon-[tabler--truck] size-5 mr-2"></span>
                <span>{{ t('pages.categories.detail.fastDelivery') }}</span>
              </div>
            </div>
            
            <!-- Special Promotion -->
            <div class="alert alert-success max-w-md mx-auto">
              <div class="flex items-center gap-3">
                <div class="size-12 rounded-full bg-success/20 flex items-center justify-center shrink-0">
                  <span class="icon-[tabler--bolt] size-6 text-success"></span>
                </div>
                <div>
                  <h3 class="font-bold">{{ t('pages.categories.detail.specialOffer') }}</h3>
                  <p class="text-sm opacity-80">{{ t('pages.categories.detail.specialOfferDesc') }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Filters Section -->
      <section class="bg-base-100 shadow-lg sticky top-0 z-20">
        <div class="max-w-7xl mx-auto px-6 py-6">
          <div class="flex flex-wrap items-center justify-between gap-4">
            <div class="flex items-center space-x-4">
              <span class="text-base-content/60">{{ t('pages.categories.detail.productsFound', { count: category?.productsCount || 0 }) }}</span>
              <select class="select select-bordered w-full max-w-xs m-1">
                <option disabled selected>{{ t('pages.categories.detail.allBrands') }}</option>
                <option>Apple</option>
                <option>Samsung</option>
                <option>Dell</option>
                <option>HP</option>
              </select>
              <select class="select select-bordered w-full max-w-xs m-1">
                <option disabled selected>{{ t('pages.categories.detail.price') }}</option>
                <option>{{ t('pages.categories.detail.lowToHigh') }}</option>
                <option>{{ t('pages.categories.detail.highToLow') }}</option>
              </select>
              <select class="select select-bordered w-full max-w-xs m-1">
                <option disabled selected>{{ t('pages.categories.detail.rating') }}</option>
                <option>{{ t('pages.categories.detail.stars', { n: 5 }) }}</option>
                <option>{{ t('pages.categories.detail.starsPlus', { n: 4 }) }}</option>
                <option>{{ t('pages.categories.detail.starsPlus', { n: 3 }) }}</option>
              </select>
            </div>
            <div class="flex items-center space-x-4">
              <span class="text-base-content/60">{{ t('pages.categories.detail.filtersLabel') }}</span>
              <div class="btn-group">
                <button class="btn btn-ghost">{{ t('pages.categories.detail.grid') }}</button>
                <button class="btn btn-ghost">{{ t('pages.categories.detail.list') }}</button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Products Grid -->
      <section class="py-16">
        <div class="max-w-7xl mx-auto px-6">
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
            <!-- Static Product Cards for Demo -->
            <div v-for="i in 8" :key="i" class="group bg-base-100 rounded-[2rem] border border-base-200 overflow-hidden hover-lift shadow-sm hover:shadow-xl hover:shadow-primary/5 transition-all duration-500">
              <div class="relative">
                <div class="aspect-video bg-base-300 flex items-center justify-center">
                  <span class="icon-[tabler--photo] size-16 text-base-content/20"></span>
                </div>
                <div class="absolute top-4 right-4">
                  <!-- Actions can be added here if needed -->
                </div>
                <div class="absolute top-4 left-4">
                  <button class="btn btn-ghost btn-sm btn-circle bg-white/90 backdrop-blur-sm hover:bg-white transition-colors">
                    <span class="icon-[tabler--heart] size-4 text-base-content/40"></span>
                  </button>
                </div>
              </div>
              <div class="p-6">
                <div class="h-6 bg-base-300 animate-pulse rounded-lg w-3/4 mb-2"></div>
                <div class="h-4 bg-base-300 animate-pulse rounded-lg w-full mb-4"></div>
                <div class="flex items-center mb-4">
                  <div class="flex text-warning">
                    <span v-for="n in 5" :key="n" class="icon-[tabler--star-filled] size-4"></span>
                  </div>
                  <span class="text-base-content/60 text-sm ml-2">({{ 0 }} {{ t('pages.categories.detail.reviews') }})</span>
                </div>
                <div class="flex items-center justify-between">
                  <div>
                    <span class="text-2xl font-bold text-base-content">{{ formatNumberBR(0) }}</span>
                  </div>
                  <button class="btn btn-primary btn-sm btn-circle shadow-md">
                    <span class="icon-[tabler--shopping-cart] size-5"></span>
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Load More Button -->
          <div class="text-center mt-16">
            <button class="btn btn-outline btn-primary btn-lg rounded-2xl px-12 border-2 hover:bg-primary/5 transition-all">
              <span class="flex items-center gap-2">
                {{ t('pages.categories.detail.loadMore') }}
                <span class="icon-[tabler--arrow-down] size-5"></span>
              </span>
            </button>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types';

const { t } = useI18n()
const route = useRoute()
const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();

const id = route.params.id;

const { pending, data: category } = await useLazyFetch<Category>(`${config.public.baseURL}/api/categories/${id}`);

// SEO optimization for category page
useSeoMeta({
  title: category.value?.name || t('pages.categories.detail.fallbackTitle'),
  ogTitle: category.value?.name || t('pages.categories.detail.fallbackTitle'),
  description: (category.value?.description || t('pages.categories.detail.fallbackDescription')) as string,
  ogDescription: (category.value?.description || t('pages.categories.detail.fallbackDescription')) as string,
  ogImage: 'https://images.unsplash.com/photo-1542744173-8e7e53415bb0?w=1200&h=630&fit=crop',
  ogUrl: `${config.public.baseURL}/categories/${id}`,
  ogType: 'website',
  twitterCard: 'summary_large_image',
  twitterTitle: category.value?.name || t('pages.categories.detail.fallbackTitle'),
  twitterDescription: (category.value?.description || t('pages.categories.detail.fallbackDescription')) as string,
  twitterImage: 'https://images.unsplash.com/photo-1542744173-8e7e53415bb0?w=1200&h=630&fit=crop',
})
</script>
<style scoped>
@import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css');

.gradient-bg {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.card-hover {
    transition: all 0.3s ease;
}

.card-hover:hover {
    transform: translateY(-8px);
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

.floating-animation {
    animation: float 6s ease-in-out infinite;
}

@keyframes float {

    0%,
    100% {
        transform: translateY(0px);
    }

    50% {
        transform: translateY(-20px);
    }
}

.stats-card {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
}

.product-card {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.animate-pulse-slow {
    animation: pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>
