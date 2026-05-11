<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12 pt-10">
      <div>
        <h1 class="h2 gradient-text mb-2">{{ t('pages.wishlist.title') }}</h1>
        <p class="text-base-content/60">{{ t('pages.wishlist.description') }}</p>
      </div>
      <div class="flex items-center gap-3">
        <span class="badge badge-primary badge-soft px-4 py-2 font-bold">{{ t('pages.wishlist.savedItems', { count: wishlist.length }) }}</span>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="flex flex-col items-center justify-center py-32">
      <div class="relative">
        <div class="size-20 rounded-full border-4 border-primary/20 animate-ping absolute"></div>
        <div class="size-20 rounded-full border-4 border-primary/10 animate-pulse"></div>
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="icon-[tabler--heart-filled] size-8 text-primary animate-bounce"></span>
        </div>
      </div>
      <p class="mt-6 text-base-content/40 font-medium tracking-widest uppercase text-xs">{{ t('pages.wishlist.loading') }}</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="wishlist.length === 0" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="size-24 rounded-full bg-base-200 flex items-center justify-center mb-6">
        <span class="icon-[tabler--heart-off] size-12 opacity-20" />
      </div>
      <h2 class="h3 mb-2">{{ t('pages.wishlist.empty') }}</h2>
      <p class="mb-8 text-base-content/50 max-w-sm text-center">
        {{ t('pages.wishlist.description') }}
      </p>
      <NuxtLink to="/products" class="btn btn-primary btn-lg px-10 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-105">
        {{ t('pages.wishlist.continueShopping') }}
      </NuxtLink>
    </div>

    <!-- Wishlist Grid -->
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
      <div v-for="item in wishlist" :key="item.id" 
        class="group bg-base-100 rounded-[2.5rem] border border-base-200 overflow-hidden hover-lift shadow-sm hover:shadow-xl hover:shadow-primary/5 transition-all duration-500">
        
        <div class="relative h-64 bg-base-200 overflow-hidden">
           <div class="flex items-center justify-center h-full bg-gradient-to-br from-base-200 to-base-300 group-hover:scale-110 transition-transform duration-700">
             <span class="icon-[tabler--photo] size-20 text-base-content/10"></span>
           </div>
           
           <button @click="removeFromWishlist(item)" 
            class="btn btn-square btn-sm btn-error absolute top-4 right-4 rounded-xl opacity-0 group-hover:opacity-100 transition-all duration-300 hover:scale-110">
             <span class="icon-[tabler--trash] size-4"></span>
           </button>
           
           <div class="absolute bottom-4 left-4">
             <span class="badge badge-primary badge-soft backdrop-blur-md">{{ t('pages.wishlist.savedAt', { date: new Date(item.created_at).toLocaleDateString(locale) }) }}</span>
           </div>
        </div>
        
        <div class="p-8">
          <h3 class="h4 mb-6 group-hover:text-primary transition-colors">{{ t('pages.wishlist.product') }} #{{ item.product_id }}</h3>
          
          <div class="flex gap-3">
            <NuxtLink :to="`/products/${item.product_id}`" class="btn btn-primary grow rounded-xl shadow-lg shadow-primary/10">
              {{ t('pages.wishlist.viewProduct') }}
            </NuxtLink>
            <button class="btn btn-square btn-ghost rounded-xl bg-base-200/50">
              <span class="icon-[tabler--shopping-cart-plus] size-5"></span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WishlistItem } from '~/types'
const { t, locale } = useI18n()
const { wishlist, fetchWishlist, toggleWishlist } = useWishlist()
const loading = ref(true)

onMounted(async () => {
  await fetchWishlist()
  loading.value = false
})

async function removeFromWishlist(item: WishlistItem) {
  await toggleWishlist(item.product_id)
}
</script>
