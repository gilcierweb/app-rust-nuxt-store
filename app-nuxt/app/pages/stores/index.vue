<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="text-center mb-16 pt-10">
      <h1 class="h2 mb-4 gradient-text">{{ t('pages.stores.title') }}</h1>
      <p class="text-xl text-base-content/60 max-w-2xl mx-auto">{{ t('pages.stores.description') }}</p>
      
      <div class="max-w-xl mx-auto mt-10">
        <div class="relative">
          <span class="icon-[tabler--search] size-5 absolute left-4 top-1/2 -translate-y-1/2 text-base-content/40"></span>
          <input type="text" placeholder="Search by city or neighborhood..." 
            class="input input-lg bg-base-200/50 border-none rounded-2xl pl-12 w-full h-14" />
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
            <span class="badge badge-primary backdrop-blur-md font-bold px-4 py-3 rounded-xl shadow-lg">Open Now</span>
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
            <button class="btn btn-primary rounded-xl shadow-lg shadow-primary/10">
              Directions
            </button>
            <button class="btn btn-ghost rounded-xl bg-base-200/50">
              Details
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- CTA Section -->
    <div class="mt-20 p-12 bg-base-200 rounded-[3rem] text-center">
      <h2 class="h3 mb-4">Can't find a store near you?</h2>
      <p class="text-base-content/60 mb-8">Don't worry! We ship worldwide and provide 24/7 support online.</p>
      <NuxtLink to="/products" class="btn btn-primary btn-lg px-12 rounded-2xl">Shop Online Now</NuxtLink>
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

const stores = [
  { name: 'São Paulo Flagship', address: 'Av. Paulista, 1000 - Bela Vista', phone: '+55 (11) 9999-9999', hours: 'Mon-Sat: 10:00 - 22:00' },
  { name: 'Rio de Janeiro Central', address: 'Rua Visconde de Pirajá, 500 - Ipanema', phone: '+55 (21) 8888-8888', hours: 'Mon-Sat: 10:00 - 20:00' },
  { name: 'Curitiba Design Hub', address: 'Av. do Batel, 1500 - Batel', phone: '+55 (41) 7777-7777', hours: 'Mon-Sat: 11:00 - 19:00' },
  { name: 'Belo Horizonte Studio', address: 'Rua de Bahia, 1200 - Lourdes', phone: '+55 (31) 6666-6666', hours: 'Mon-Sat: 09:00 - 21:00' },
  { name: 'Porto Alegre Tech Lab', address: 'Av. Nilo Peçanha, 2000 - Boa Vista', phone: '+55 (51) 5555-5555', hours: 'Mon-Fri: 10:00 - 20:00' },
  { name: 'Brasília Modern Store', address: 'SCLS 403 Bloco B - Asa Sul', phone: '+55 (61) 4444-4444', hours: 'Mon-Sat: 10:00 - 22:00' }
]
</script>

