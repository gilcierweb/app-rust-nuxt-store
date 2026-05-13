<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="text-center mb-16 pt-10">
      <h1 class="h2 mb-4 gradient-text">{{ t('pages.stores.title') }}</h1>
      <p class="text-xl text-base-content/60 max-w-2xl mx-auto">{{ t('pages.stores.description') }}</p>
      
      <div class="max-w-xl mx-auto mt-10">
        <div class="flex items-center gap-2 w-full">
          <input type="text" :placeholder="t('pages.stores.search')" class="input grow" id="storeSearch" />
          <button class="btn btn-primary">
            <span class="icon-[tabler--search] size-5"></span>
          </button>
        </div>
      </div>
    </div>

    <!-- Stores Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      <div v-for="store in stores" :key="store.name" 
        class="group bg-base-100 rounded-[2.5rem] border border-base-200 overflow-hidden hover-lift shadow-sm hover:shadow-xl hover:shadow-primary/5 transition-all duration-500">
        
        <!-- Cover/Map Placeholder -->
        <div class="relative h-48 bg-base-200">
          <div class="absolute inset-0 bg-gradient-to-br from-primary/10 to-secondary/10"></div>
          <div class="flex items-center justify-center h-full text-base-content/10 group-hover:scale-110 transition-transform duration-700">
             <span class="icon-[tabler--map-pin] size-16"></span>
          </div>
          <div class="absolute top-4 right-4">
            <span class="badge badge-success backdrop-blur-md font-bold px-4 py-2 rounded-xl shadow-lg flex items-center gap-1">
              <span class="icon-[tabler--circle-check] size-3"></span>
              {{ t('pages.stores.openNow') }}
            </span>
          </div>
        </div>
        
        <!-- Content -->
        <div class="p-8">
          <h3 class="h4 mb-6 group-hover:text-primary transition-colors">{{ store.name }}</h3>
          
          <div class="space-y-4 mb-8">
            <div class="flex items-start gap-3">
              <span class="icon-[tabler--map-2] size-5 text-primary shrink-0"></span>
              <p class="text-sm text-base-content/60">{{ store.address }}</p>
            </div>
            <div class="flex items-center gap-3">
              <span class="icon-[tabler--phone] size-5 text-primary shrink-0"></span>
              <p class="text-sm text-base-content/60">{{ store.phone }}</p>
            </div>
            <div class="flex items-center gap-3">
              <span class="icon-[tabler--clock] size-5 text-primary shrink-0"></span>
              <p class="text-sm text-base-content/60">{{ store.hours }}</p>
            </div>
          </div>
          
          <div class="grid grid-cols-2 gap-4">
            <button class="btn btn-primary gap-2">
              <span class="icon-[tabler--navigation] size-4"></span>
              {{ t('pages.stores.directions') }}
            </button>
            <button class="btn btn-soft w-full">
              {{ t('pages.stores.details') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- CTA Section -->
    <div class="mt-20 p-12 bg-gradient-to-br from-primary/10 to-secondary/10 rounded-[3rem] text-center">
      <div class="alert alert-success max-w-md mx-auto mb-8">
        <div class="flex items-center gap-4">
          <div class="size-12 rounded-full bg-success/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--map-pin] size-6 text-success"></span>
          </div>
          <div>
            <h2 class="font-bold">{{ t('pages.stores.cta.title') }}</h2>
            <p class="text-sm opacity-80 mt-1">{{ t('pages.stores.cta.description') }}</p>
          </div>
        </div>
      </div>
      <NuxtLink to="/products" class="btn btn-primary btn-lg">{{ t('pages.stores.cta.button') }}</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.stores.title'),
  ogTitle: t('pages.stores.title'),
  description: t('pages.stores.description'),
  ogDescription: t('pages.stores.description'),
})

const stores = computed(() => [
  { name: t('pages.stores.locations.sp.name'), address: t('pages.stores.locations.sp.address'), phone: '+55 (11) 9999-9999', hours: t('pages.stores.locations.sp.hours') },
  { name: t('pages.stores.locations.rj.name'), address: t('pages.stores.locations.rj.address'), phone: '+55 (21) 8888-8888', hours: t('pages.stores.locations.rj.hours') },
  { name: t('pages.stores.locations.curitiba.name'), address: t('pages.stores.locations.curitiba.address'), phone: '+55 (41) 7777-7777', hours: t('pages.stores.locations.curitiba.hours') },
  { name: t('pages.stores.locations.bh.name'), address: t('pages.stores.locations.bh.address'), phone: '+55 (31) 6666-6666', hours: t('pages.stores.locations.bh.hours') },
  { name: t('pages.stores.locations.poa.name'), address: t('pages.stores.locations.poa.address'), phone: '+55 (51) 5555-5555', hours: t('pages.stores.locations.poa.hours') },
  { name: t('pages.stores.locations.brasilia.name'), address: t('pages.stores.locations.brasilia.address'), phone: '+55 (61) 4444-4444', hours: t('pages.stores.locations.brasilia.hours') }
])
</script>
