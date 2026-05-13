<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12 pt-10">
      <div>
        <h1 class="h2 gradient-text mb-2">{{ t('pages.wishlist.title') }}</h1>
        <p class="text-base-content/60">{{ t('pages.wishlist.description') }}</p>
      </div>
      <div class="flex items-center gap-3">
        <div class="badge badge-primary badge-soft px-4 py-2 font-bold flex items-center gap-2">
          <span class="icon-[tabler--heart-filled] size-4"></span>
          {{ t('pages.wishlist.savedItems', { count: wishlist.length }) }}
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="flex flex-col items-center justify-center py-32">
      <div class="alert alert-info max-w-md">
        <div class="flex items-center gap-4">
          <div class="loading loading-spinner loading-md"></div>
          <div>
            <p class="font-bold">{{ t('pages.wishlist.loading.title') }}</p>
            <p class="text-sm opacity-80">{{ t('pages.wishlist.loading.description') }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else-if="wishlist.length === 0" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="alert alert-warning max-w-md">
        <div class="flex items-center gap-4">
          <div class="size-16 rounded-full bg-warning/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--heart-off] size-8 text-warning" />
          </div>
          <div>
            <h2 class="font-bold text-lg">{{ t('pages.wishlist.empty') }}</h2>
            <p class="text-sm opacity-80 mt-1">
              {{ t('pages.wishlist.description') }}
            </p>
          </div>
        </div>
      </div>
      <NuxtLink to="/products" class="btn btn-primary btn-lg px-10 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-105 mt-8">
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
             <span class="badge badge-primary badge-soft backdrop-blur-md flex items-center gap-1">
               <span class="icon-[tabler--calendar] size-3"></span>
               {{ t('pages.wishlist.savedAt', { date: new Date(item.created_at).toLocaleDateString(locale) }) }}
             </span>
           </div>
        </div>
        
        <div class="p-8">
          <h3 class="h4 mb-6 group-hover:text-primary transition-colors">{{ t('pages.wishlist.product') }} #{{ item.product_id }}</h3>
          
          <div class="flex gap-3">
            <NuxtLink :to="`/products/${item.product_id}`" class="btn btn-primary grow rounded-xl shadow-lg shadow-primary/10">
              {{ t('pages.wishlist.viewProduct') }}
            </NuxtLink>
            <button class="btn btn-square btn-outline btn-primary rounded-xl" aria-label="Add to cart">
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
