<template>
  <div class="pb-20">
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12 pt-10">
      <div>
        <h1 class="h2 gradient-text mb-2">{{ t('pages.products.title') }}</h1>
        <p class="text-base-content/60">{{ t('pages.products.description') }}</p>
      </div>
      
      <div class="flex items-center gap-3 w-full md:w-auto">
        <div class="relative grow md:w-80">
          <span class="icon-[tabler--search] size-5 absolute left-4 top-1/2 -translate-y-1/2 text-base-content/40 z-10"></span>
          <input type="text" :placeholder="t('pages.products.search.placeholder')" 
            class="input input-lg bg-base-200/50 border-none rounded-2xl pl-12 pr-14 w-full h-14" />
          <div class="absolute right-2 top-1/2 -translate-y-1/2">
            <button class="btn btn-ghost btn-square btn-sm rounded-lg">
              <span class="icon-[tabler--adjustments-horizontal] size-5"></span>
            </button>
          </div>
        </div>
        <button class="btn btn-square btn-lg bg-base-200/50 border-none rounded-2xl h-14 w-14 lg:hidden" 
          onclick="document.getElementById('filter-drawer').classList.toggle('overlay-open')"
          aria-haspopup="dialog" aria-expanded="false" aria-controls="filter-drawer">
          <span class="icon-[tabler--filter] size-6"></span>
        </button>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
      <!-- Desktop Sidebar -->
      <aside class="hidden lg:block lg:col-span-3 space-y-8">
        <div class="card bg-base-100 shadow-soft border border-base-200">
          <div class="card-body p-6">
          <h3 class="font-bold mb-6 flex items-center gap-2">
            <span class="icon-[tabler--filter] size-5 text-primary"></span>
            {{ t('pages.products.filters.title') }}
          </h3>
          
          <!-- Categories Filter -->
          <div class="space-y-4">
            <h4 class="text-sm font-semibold uppercase tracking-wider text-base-content/40">{{ t('pages.products.filters.categories') }}</h4>
            <div class="space-y-2">
              <label v-for="cat in categories" :key="cat" class="flex items-center gap-3 cursor-pointer group hover:bg-base-300/30 p-2 rounded-lg transition-colors">
                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm rounded-lg" />
                <span class="text-sm group-hover:text-primary transition-colors">{{ cat }}</span>
                <span class="badge badge-ghost badge-sm ml-auto">12</span>
              </label>
            </div>
          </div>

          <hr class="my-6 border-base-300" />

          <!-- Price Filter -->
          <div class="space-y-4">
            <h4 class="text-sm font-semibold uppercase tracking-wider text-base-content/40">{{ t('pages.products.filters.priceRange') }}</h4>
            <div class="space-y-4">
              <div class="join">
                <input type="range" min="0" max="1000" class="range range-primary range-sm join-item" />
                <input type="number" placeholder="Min" class="input input-sm input-bordered join-item w-20" />
                <input type="number" placeholder="Max" class="input input-sm input-bordered join-item w-20" />
              </div>
              <div class="flex items-center justify-between text-xs font-medium">
                <span>R$ 0</span>
                <span>R$ 1.000+</span>
              </div>
            </div>
          </div>

          <hr class="my-6 border-base-300" />

          <!-- Rating Filter -->
          <div class="space-y-4">
            <h4 class="text-sm font-semibold uppercase tracking-wider text-base-content/40">{{ t('pages.products.filters.rating') }}</h4>
            <div class="space-y-2">
              <label v-for="i in [5, 4, 3, 2, 1]" :key="i" class="flex items-center gap-3 cursor-pointer group hover:bg-base-300/30 p-2 rounded-lg transition-colors">
                <input type="radio" name="rating" class="radio radio-primary radio-sm" />
                <div class="flex items-center gap-1 text-warning">
                  <span v-for="star in 5" :key="star" 
                    :class="[star <= i ? 'icon-[tabler--star-filled]' : 'icon-[tabler--star]', 'size-3.5']"></span>
                </div>
                <span class="text-xs font-medium">{{ t('pages.products.filters.andUp') }}</span>
                <span class="badge badge-ghost badge-xs ml-auto">{{ 6 - i }}k</span>
              </label>
            </div>
          </div>
        </div>
        </div>
        
        <!-- Promotion Card -->
        <div class="alert alert-success rounded-3xl">
          <div class="flex items-start gap-4">
            <div class="size-12 rounded-xl bg-success/20 flex items-center justify-center shrink-0">
              <span class="icon-[tabler--tag] size-6 text-success"></span>
            </div>
            <div>
              <h4 class="font-bold text-lg mb-1">{{ t('pages.products.promo.title') }}</h4>
              <p class="text-sm opacity-80 mb-3">{{ t('pages.products.promo.description') }}</p>
              <button class="btn btn-success btn-sm w-full rounded-xl">{{ t('pages.products.promo.button') }}</button>
            </div>
          </div>
        </div>
      </aside>

      <!-- Product Grid Content -->
      <div class="lg:col-span-9">
        <ProductList />
      </div>
    </div>

    <!-- Mobile Filter Drawer -->
    <div id="filter-drawer" class="overlay overlay-open:translate-x-0 drawer drawer-start hidden" role="dialog" tabindex="-1">
      <div class="drawer-content h-full p-6 bg-base-100 w-80">
        <div class="flex items-center justify-between mb-8">
          <h3 class="font-bold text-lg">{{ t('pages.products.filters.title') }}</h3>
          <button class="btn btn-circle btn-ghost btn-sm" data-overlay="#filter-drawer">
            <span class="icon-[tabler--x] size-5"></span>
          </button>
        </div>
        <div class="space-y-8">
          <div class="space-y-4">
            <h4 class="text-sm font-semibold uppercase tracking-wider text-base-content/40">{{ t('pages.products.filters.categories') }}</h4>
            <div class="space-y-3">
              <label v-for="cat in categories" :key="cat" class="flex items-center gap-3 cursor-pointer">
                <input type="checkbox" class="checkbox checkbox-primary rounded-lg" />
                <span>{{ cat }}</span>
              </label>
            </div>
          </div>
          <button class="btn btn-primary w-full mt-8 rounded-2xl h-14" data-overlay="#filter-drawer">{{ t('pages.products.filters.apply') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.products.title'),
  ogTitle: t('pages.products.title'),
  description: t('pages.products.description'),
  ogDescription: t('pages.products.description'),
})

const categories = ['Fashion', 'Technology', 'Books', 'Home', 'Vehicles', 'Art']
</script>
