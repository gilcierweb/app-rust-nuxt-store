<template>
  <div class="py-12">
    <div class="flex items-center justify-between mb-8">
      <div>
        <h2 class="h3 gradient-text">{{ t('pages.home.categories') }}</h2>
        <p class="text-base-content/60 mt-1">{{ t('pages.home.categoriesSubtitle') }}</p>
      </div>
      <NuxtLinkLocale to="/categories" class="btn btn-ghost btn-sm gap-2">
        {{ t('pages.home.shopAll') }} <span class="icon-[tabler--arrow-right] size-4"></span>
      </NuxtLinkLocale>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6">
      <NuxtLinkLocale v-for="cat in staticCategories" :key="cat.name" 
        :to="`/categories`"
        class="group flex flex-col items-center gap-4 p-6 rounded-3xl bg-base-200/50 hover:bg-primary/10 transition-all duration-500 hover-lift border border-transparent hover:border-primary/20">
        
        <div class="size-16 rounded-2xl bg-white shadow-sm flex items-center justify-center group-hover:scale-110 transition-transform duration-500">
          <span :class="[cat.icon, 'size-8 text-primary']"></span>
        </div>
        
        <div class="text-center">
          <h3 class="font-bold text-sm md:text-base group-hover:text-primary transition-colors">{{ cat.name }}</h3>
          <span class="text-[10px] uppercase tracking-wider font-semibold text-base-content/40">{{ t('pages.home.productsCount', { count: 12 }) }}</span>
        </div>
      </NuxtLinkLocale>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()

const staticCategories = [
  { name: 'Fashion', icon: 'icon-[tabler--shirt-filled]' },
  { name: 'Technology', icon: 'icon-[tabler--device-laptop]' },
  { name: 'Books', icon: 'icon-[tabler--book]' },
  { name: 'Home', icon: 'icon-[tabler--home-dot]' },
  { name: 'Vehicles', icon: 'icon-[tabler--car]' },
  { name: 'Art', icon: 'icon-[tabler--palette]' }
]
</script>