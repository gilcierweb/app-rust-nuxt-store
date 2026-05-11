<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="text-center mb-16 pt-10">
      <h1 class="h2 mb-4 gradient-text">{{ t('pages.categories.title') }}</h1>
      <p class="text-xl text-base-content/60 max-w-2xl mx-auto">{{ t('pages.categories.description') }}</p>
      
      <div class="max-w-xl mx-auto mt-10">
        <div class="join w-full">
          <input type="text" :placeholder="t('pages.categories.search.placeholder')" 
            class="input input-lg bg-base-200/50 border-none rounded-l-2xl pl-12 grow join-item h-14" />
          <button class="btn btn-primary join-item rounded-r-2xl h-14 px-6">
            <span class="icon-[tabler--search] size-5"></span>
          </button>
        </div>
      </div>
    </div>

    <!-- Category Grid -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
      <NuxtLink v-for="cat in allCategories" :key="cat.id" 
        :to="`/categories`"
        class="group relative overflow-hidden rounded-[2rem] bg-base-200/50 hover:bg-primary/5 transition-all duration-500 border border-transparent hover:border-primary/20 p-8 flex flex-col h-64 hover-lift">
        
        <div class="size-16 rounded-2xl bg-white shadow-sm flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-500">
          <span :class="[cat.icon || 'icon-[tabler--category]', 'size-8 text-primary']"></span>
        </div>
        
        <div class="mt-auto">
          <h3 class="h4 mb-2 group-hover:text-primary transition-colors">{{ cat.name }}</h3>
          <p class="text-sm text-base-content/50">{{ cat.description || 'Discover our curated selection' }}</p>
        </div>
        
        <div class="absolute top-8 right-8">
          <span class="badge badge-primary badge-soft flex items-center gap-1">
            <span class="icon-[tabler--box] size-3"></span>
            {{ cat.productsCount || 0 }} Products
          </span>
        </div>
        
        <div class="absolute bottom-8 right-8 opacity-0 group-hover:opacity-100 -translate-x-4 group-hover:translate-x-0 transition-all duration-500">
          <span class="icon-[tabler--arrow-right] size-6 text-primary"></span>
        </div>
      </NuxtLink>
    </div>

    <!-- Empty State -->
    <div v-if="allCategories.length === 0" class="text-center py-20">
      <div class="alert alert-warning max-w-md mx-auto">
        <div class="flex items-center gap-4">
          <div class="size-16 rounded-full bg-warning/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--category-2] size-8 text-warning"></span>
          </div>
          <div>
            <h3 class="font-bold text-lg">No categories found</h3>
            <p class="text-sm opacity-80 mt-1">Try adjusting your search or browse all products</p>
          </div>
        </div>
      </div>
      <NuxtLink to="/products" class="btn btn-primary mt-6">Browse All Products</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig()
const { t } = useI18n()

useSeoMeta({
  title: t('pages.categories.title'),
  ogTitle: t('pages.categories.title'),
  description: t('pages.categories.description'),
  ogDescription: t('pages.categories.description'),
})

const { data: categoriesData } = await useFetch<any[]>(`${config.public.baseURL}/api/categories`)

const staticCategories = [
  { id: 'f', name: 'Fashion', icon: 'icon-[tabler--shirt-filled]', description: 'Latest trends and styles for everyone.', productsCount: 124 },
  { id: 't', name: 'Technology', icon: 'icon-[tabler--device-laptop]', description: 'Innovative gadgets and devices.', productsCount: 86 },
  { id: 'b', name: 'Books', icon: 'icon-[tabler--book]', description: 'A world of knowledge and imagination.', productsCount: 210 },
  { id: 'h', name: 'Home', icon: 'icon-[tabler--home-dot]', description: 'Everything for a cozy and modern home.', productsCount: 54 },
  { id: 'v', name: 'Vehicles', icon: 'icon-[tabler--car]', description: 'Automotive accessories and parts.', productsCount: 32 },
  { id: 'a', name: 'Art', icon: 'icon-[tabler--palette]', description: 'Supplies and inspiration for creatives.', productsCount: 45 }
]

const allCategories = computed(() => {
  const apiCats = (categoriesData.value || []).map(c => ({
    id: c.id,
    name: c.name,
    description: c.description,
    productsCount: 0, // Placeholder
    icon: 'icon-[tabler--category]'
  }))
  
  return apiCats.length > 0 ? apiCats : staticCategories
})
</script>