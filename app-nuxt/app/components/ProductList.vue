<template>
  <div class="py-8">
    <!-- Loading State -->
    <div v-if="isLoading" class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
      <div v-for="i in 8" :key="i" class="card bg-base-100 shadow-sm animate-pulse">
        <div class="bg-base-200 h-64 w-full rounded-t-xl"></div>
        <div class="card-body gap-3">
          <div class="h-4 bg-base-200 rounded w-3/4"></div>
          <div class="h-4 bg-base-200 rounded w-1/2"></div>
          <div class="h-10 bg-base-200 rounded-lg mt-2"></div>
        </div>
      </div>
    </div>

    <!-- Product Grid -->
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8">
      <div v-for="product in allProducts" :key="product.id" 
        class="card bg-base-100 shadow-sm hover:shadow-xl transition-all duration-300 group hover:-translate-y-1.5 border border-base-200 overflow-hidden">
        
        <!-- Image Area -->
        <figure class="relative aspect-square overflow-hidden bg-base-200">
          <NuxtLinkLocale :to="`/products/${product.id}`" class="w-full h-full">
            <OptimizedImage
              v-if="product.image"
              :src="product.image"
              :alt="product.name"
              class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
            />
            <div v-else class="flex items-center justify-center h-full text-base-content/20">
              <span class="icon-[tabler--photo] size-12"></span>
            </div>
          </NuxtLinkLocale>
          
          <!-- Badges & Actions -->
          <div class="absolute top-3 left-3 flex flex-col gap-2">
            <span v-if="product.isNew" class="badge badge-primary badge-sm shadow-sm">New</span>
            <span v-if="product.category" class="badge badge-secondary badge-sm shadow-sm">{{ product.category }}</span>
          </div>
          
          <button @click="toggleWishlist(product.id)" 
            class="btn btn-circle btn-sm glass absolute top-3 right-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300 shadow-sm">
            <span :class="[isInWishlist(product.id) ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart]', 'size-4']"></span>
          </button>
        </figure>

        <!-- Product Info -->
        <div class="card-body p-5 gap-1">
          <NuxtLinkLocale :to="`/products/${product.id}`" class="group/title">
            <h3 class="card-title text-base font-bold line-clamp-1 group-hover/title:text-primary transition-colors">
              {{ product.name }}
            </h3>
          </NuxtLinkLocale>
          
          <div class="flex items-center justify-between mt-1">
            <p class="text-primary font-bold text-lg">{{ formatNumberBR(product.price) }}</p>
            <div class="flex items-center gap-1 text-warning text-xs">
              <span class="icon-[tabler--star-filled] size-3"></span>
              <span class="font-medium">4.5</span>
            </div>
          </div>

          <p class="text-base-content/60 text-sm line-clamp-2 mt-1 h-10">
            {{ product.description }}
          </p>

          <div class="card-actions mt-4 pt-4 border-t border-base-200">
            <button class="btn btn-primary btn-sm flex-1 gap-2" @click="handleAddToCart(product)">
              <span class="icon-[tabler--shopping-cart] size-4"></span>
              {{ t('product.addToCart') }}
            </button>
            <NuxtLinkLocale :to="`/products/${product.id}`" class="btn btn-ghost btn-sm btn-square" aria-label="View Details">
              <span class="icon-[tabler--eye] size-4"></span>
            </NuxtLinkLocale>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="!isLoading && allProducts.length === 0" class="text-center py-20 bg-base-200/50 rounded-3xl border-2 border-dashed border-base-300">
      <span class="icon-[tabler--package-off] size-16 text-base-content/20 mb-4"></span>
      <h3 class="text-xl font-bold">No products found</h3>
      <p class="text-base-content/60 mt-1">Try adjusting your filters or search criteria.</p>
      <button class="btn btn-primary mt-6">Clear All Filters</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import OptimizedImage from './OptimizedImage.vue'
import type { ProductApi } from '~/types'

const { t } = useI18n()
const { openCart } = useCartUI()
const cartStore = useCartStore()
const { fetchWishlist, toggleWishlist, isInWishlist } = useWishlist()

// Unified Product Type
interface UnifiedProduct {
  id: number | string
  name: string
  description: string
  price: number
  image: string
  category: string
  isNew?: boolean
  source: 'api' | 'dummy'
}

// Data Fetching (non-blocking: page renders instantly with skeleton loaders)
const { useApiLazyFetch } = useApi()
const { pending: pendingApi, data: productsData } = useApiLazyFetch<ProductApi[]>('/api/products')
const { pending: pendingDummy, data: dummyData } = useLazyFetch<{ products: any[] }>(`https://dummyjson.com/products`)

const isLoading = computed(() => pendingApi.value || pendingDummy.value)

// Merged Data
const allProducts = computed<UnifiedProduct[]>(() => {
  const apiItems = (productsData.value || []).map(p => ({
    id: p.id,
    name: p.name,
    description: p.description,
    price: p.price,
    image: p.images?.[0]?.image || '',
    category: p.category?.name || '',
    isNew: true,
    source: 'api' as const
  }))

  const dummyItems = (dummyData.value?.products || []).map(p => ({
    id: `dummy-${p.id}`,
    name: p.title,
    description: p.description,
    price: p.price,
    image: p.thumbnail,
    category: p.category,
    source: 'dummy' as const
  }))

  return [...apiItems, ...dummyItems]
})

function handleAddToCart(product: UnifiedProduct) {
  cartStore.addItem({
    productId: product.id as number,
    name: product.name,
    price: product.price,
    image: product.image,
  })
  openCart()
}

onMounted(() => {
  fetchWishlist()
})
</script>