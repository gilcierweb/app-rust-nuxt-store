<template>
  <div class="pb-20">
    <!-- Loading State -->
    <div v-if="pendingApi" class="flex flex-col items-center justify-center py-32">
      <div class="relative">
        <div class="size-20 rounded-3xl border-4 border-primary/20 animate-pulse"></div>
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="loading loading-ring loading-lg text-primary"></span>
        </div>
      </div>
      <p class="mt-6 text-base-content/40 font-medium tracking-widest uppercase text-xs">{{ t('pages.products.loading') }}</p>
    </div>

    <!-- Product Not Found -->
    <div v-else-if="!productApi" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="size-24 rounded-full bg-base-200 flex items-center justify-center mb-6">
        <span class="icon-[tabler--package-off] size-12 opacity-20" />
      </div>
      <h2 class="h3 mb-2">{{ t('product.notFound') }}</h2>
      <NuxtLinkLocale to="/products" class="btn btn-primary btn-lg px-10 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-105">
        {{ t('cart.continueShopping') }}
      </NuxtLinkLocale>
    </div>

    <!-- Product Detail -->
    <div v-else-if="productApi">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 pt-10">
        <!-- Gallery -->
        <div class="space-y-6">
          <div class="aspect-square bg-base-100 rounded-[3rem] border border-base-200 overflow-hidden shadow-sm group">
            <NuxtImg 
              v-if="productApi.images && productApi.images.length > 0" 
              :src="productApi.images[0].image" 
              class="size-full object-cover group-hover:scale-105 transition-transform duration-700" 
              :alt="productApi.name" 
            />
            <div v-else class="flex items-center justify-center h-full text-base-content/10">
              <span class="icon-[tabler--photo] size-32"></span>
            </div>
          </div>
          
          <div v-if="productApi.images && productApi.images.length > 1" class="flex gap-4 overflow-x-auto no-scrollbar pb-2">
            <div v-for="(img, index) in productApi.images" :key="index" 
              class="size-24 shrink-0 rounded-2xl border-2 border-base-200 overflow-hidden cursor-pointer hover:border-primary transition-colors">
              <NuxtImg :src="img.image" class="size-full object-cover" :alt="productApi.name" />
            </div>
          </div>
        </div>

        <!-- Info -->
        <div class="flex flex-col">
          <div class="mb-8">
            <div class="flex items-center gap-3 mb-4">
              <span class="badge badge-primary badge-soft rounded-lg px-3 py-3 font-bold">{{ productApi.category?.name || 'Uncategorized' }}</span>
              <div class="flex items-center gap-1 text-warning">
                <span class="icon-[tabler--star-filled] size-4"></span>
                <span class="text-sm font-black text-base-content">4.8</span>
                <span class="text-xs text-base-content/40">(120 reviews)</span>
              </div>
            </div>
            <h1 class="h1 mb-4">{{ productApi.name }}</h1>
            <div class="flex items-baseline gap-4">
              <span class="text-4xl font-black text-primary">{{ formatNumberBR(selectedPrice) }}</span>
            </div>
          </div>

          <div class="prose prose-sm text-base-content/60 mb-10 leading-relaxed max-w-none">
            {{ productApi.description }}
          </div>

          <!-- Variants -->
          <div v-if="variants && variants.length > 0" class="mb-10 space-y-4">
            <label class="text-sm uppercase tracking-widest font-black text-base-content/40">{{ t('variant.select') }}</label>
            <div class="flex flex-wrap gap-3">
              <button 
                v-for="v in variants" :key="v.id"
                @click="selectedVariantId = v.id"
                class="px-6 py-3 rounded-2xl border-2 transition-all duration-300 font-bold"
                :class="selectedVariantId === v.id ? 'border-primary bg-primary/5 text-primary ring-4 ring-primary/10' : 'border-base-200 hover:border-base-300'"
              >
                {{ v.name || `${v.sku || 'Variant'}` }}
              </button>
            </div>
          </div>

          <!-- Stock Status -->
          <div v-if="selectedVariant" class="mb-6">
            <div v-if="!isInStock" class="alert alert-error">
              <span class="icon-[tabler--alert-circle] size-5"></span>
              <span>{{ t('product.outOfStock') }}</span>
            </div>
            <div v-else-if="isLowStock" class="alert alert-warning">
              <span class="icon-[tabler--alert-triangle] size-5"></span>
              <span>{{ t('product.lowStock', { count: selectedVariant.inventory_quantity }) }}</span>
            </div>
            <div v-else class="flex items-center gap-2 text-sm text-success">
              <span class="icon-[tabler--check-circle] size-4"></span>
              <span>{{ t('product.inStock') }}</span>
            </div>
          </div>

          <!-- Actions -->
          <div class="mt-auto flex flex-col sm:flex-row gap-4">
            <button 
              class="btn btn-primary btn-lg grow h-16 rounded-[1.5rem] shadow-xl shadow-primary/20 text-lg transition-transform hover:scale-[1.02]" 
              :disabled="!isInStock"
              @click="addToCartApi(productApi)"
            >
              <span class="icon-[tabler--shopping-cart-plus] size-6 mr-2"></span>
              {{ isInStock ? t('product.addToCart') : t('product.outOfStock') }}
            </button>
            <button 
              class="btn btn-square btn-lg h-16 w-16 rounded-[1.5rem] border-2 border-base-200 hover:border-error/20 hover:bg-error/5 hover:text-error group transition-all"
              @click="toggleWishlist(productApi.id)"
            >
              <span :class="[isInWishlist(productApi.id) ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart]', 'size-7 group-hover:scale-110 transition-transform']"></span>
            </button>
          </div>

          <!-- Features -->
          <div class="grid grid-cols-2 gap-4 mt-10">
            <div class="flex items-center gap-3 p-4 bg-base-200/50 rounded-2xl">
              <span class="icon-[tabler--truck] text-primary size-5"></span>
              <span class="text-xs font-bold">{{ t('shipping.free') }}</span>
            </div>
            <div class="flex items-center gap-3 p-4 bg-base-200/50 rounded-2xl">
              <span class="icon-[tabler--shield-check] text-primary size-5"></span>
              <span class="text-xs font-bold">2 Year Warranty</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Reviews Section -->
      <div class="mt-24 pt-16 border-t border-base-200">
        <div class="flex flex-col lg:flex-row gap-16">
          <div class="lg:w-1/3">
            <h2 class="h2 mb-6">{{ t('pages.products.reviews') }}</h2>
            <div class="bg-base-200/50 p-8 rounded-[2.5rem] text-center">
              <span class="text-6xl font-black text-primary">4.8</span>
              <div class="flex justify-center text-warning my-4">
                <span v-for="n in 5" :key="n" class="icon-[tabler--star-filled] size-6"></span>
              </div>
              <p class="text-base-content/40 font-bold uppercase tracking-widest text-xs">Based on 120 reviews</p>
            </div>
          </div>

          <div class="lg:w-2/3">
            <div v-if="reviews && reviews.length > 0" class="space-y-6">
              <div v-for="review in reviews" :key="review.id" class="bg-base-100 p-8 rounded-[2rem] border border-base-200 shadow-sm hover:shadow-md transition-shadow">
                <div class="flex items-center justify-between mb-4">
                  <div class="flex items-center gap-2">
                    <div class="flex text-warning">
                      <span v-for="n in 5" :key="n" 
                        :class="n <= (review.rating || 0) ? 'icon-[tabler--star-filled]' : 'icon-[tabler--star] opacity-20'" 
                        class="size-4"></span>
                    </div>
                    <span v-if="review.verified_purchase" class="badge badge-success badge-xs rounded-md">{{ t('product.verifiedPurchase') }}</span>
                  </div>
                  <span class="text-xs text-base-content/30 font-bold">{{ new Date(review.created_at).toLocaleDateString('pt-BR') }}</span>
                </div>
                <h4 v-if="review.title" class="font-black text-lg mb-2">{{ review.title }}</h4>
                <p v-if="review.comment" class="text-base-content/60 leading-relaxed">{{ review.comment }}</p>
              </div>
            </div>
            <div v-else class="py-12 text-center bg-base-200/30 rounded-[2rem] border-2 border-dashed border-base-200">
              <span class="icon-[tabler--message-off] size-12 opacity-20 mb-4"></span>
              <p class="text-base-content/60 font-medium">{{ t('product.noReviews') }}</p>
            </div>

            <!-- Review Form -->
            <div class="mt-12 bg-base-100 p-8 md:p-10 rounded-[2.5rem] border border-base-200 shadow-xl shadow-primary/5">
              <h3 class="h4 mb-8">{{ t('pages.products.writeReview') }}</h3>

              <AppAlert v-if="reviewSuccess" type="success" :message="reviewSuccess" :auto-close="3000" @close="reviewSuccess = ''" />
              <AppAlert v-if="reviewError" type="error" :message="reviewError" :auto-close="5000" @close="reviewError = ''" :dismissible="true" />

              <form @submit.prevent="submitReview" class="space-y-6">
                <div class="form-control">
                  <label class="form-label mb-2">{{ t('product.rating') }}</label>
                  <div class="flex items-center gap-6 p-4 bg-base-200/50 rounded-2xl">
                    <input v-model.number="reviewForm.rating" type="range" min="1" max="5" class="range range-primary flex-1" />
                    <div class="flex items-center gap-2 text-warning w-24 justify-end">
                      <span class="text-2xl font-black text-base-content mr-2">{{ reviewForm.rating }}</span>
                      <span class="icon-[tabler--star-filled] size-6"></span>
                    </div>
                  </div>
                </div>
                <div class="form-control">
                  <label class="form-label">{{ t('product.reviewTitle') }}</label>
                  <input v-model="reviewForm.title" type="text" placeholder="Excellent product!" class="input input-lg bg-base-200/50 border-none rounded-2xl" />
                </div>
                <div class="form-control">
                  <label class="form-label">{{ t('product.reviewComment') }}</label>
                  <textarea v-model="reviewForm.comment" class="textarea bg-base-200/50 border-none rounded-2xl min-h-32 p-4" placeholder="Share your experience with this item..."></textarea>
                </div>
                <button type="submit" class="btn btn-primary btn-lg w-full rounded-2xl shadow-lg shadow-primary/10 h-14" :disabled="reviewSubmitting">
                  <span v-if="reviewSubmitting" class="loading loading-spinner loading-sm mr-2"></span>
                  {{ t('product.submitReview') }}
                </button>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductApi, ProductVariant, Review } from '~/types'
const { t } = useI18n()
const { apiFetch, useApiLazyFetch } = useApi()
const config = useRuntimeConfig()
const route = useRoute()
const cartStore = useCartStore()
const { openCart } = useCartUI()

const productId = computed(() => route.params.id as string)

// Fetching Product (non-blocking: renders immediately with loading state)
const { data: productApi, pending: pendingApi } = useApiLazyFetch<ProductApi>(
  () => `/api/products/${productId.value}`,
  { key: `product-${productId.value}` }
)

// SEO - Moved after productApi is defined
useSeoMeta({
  title: computed(() => {
    if (!productApi.value) return t('pages.products.title')
    return productApi.value.name || t('pages.products.title')
  }),
  ogTitle: computed(() => {
    if (!productApi.value) return t('pages.products.title')
    return productApi.value.name || t('pages.products.title')
  }),
  description: computed(() => productApi.value?.description || ''),
  ogDescription: computed(() => productApi.value?.description || ''),
  ogImage: computed(() => productApi.value?.images?.[0]?.image || ''),
})

// Fetching Variants (non-blocking)
const { data: variants } = useApiLazyFetch<ProductVariant[]>(
  () => `/api/variants/list?product_id=${productId.value}`,
  { key: `variants-${productId.value}` }
)

const selectedVariantId = ref<number | null>(null)

const selectedVariant = computed(() => {
  if (!selectedVariantId.value || !variants.value) return null
  return variants.value.find(v => v.id === selectedVariantId.value) || null
})

const selectedPrice = computed(() => {
  if (selectedVariant.value?.price) return selectedVariant.value.price
  if (!productApi.value) return 0
  return productApi.value.price ?? 0
})

const selectedStock = computed(() => {
  if (!selectedVariant.value) return null
  return selectedVariant.value.inventory_quantity ?? null
})

const isInStock = computed(() => {
  if (!selectedVariant.value) return true
  if (selectedVariant.value.track_inventory === false) return true
  if (selectedVariant.value.allow_backorder) return true
  const qty = selectedVariant.value.inventory_quantity ?? 0
  const reserved = selectedVariant.value.reserved_quantity ?? 0
  return (qty - reserved) > 0
})

const isLowStock = computed(() => {
  if (!selectedVariant.value) return false
  if (selectedVariant.value.track_inventory === false) return false
  const qty = selectedVariant.value.inventory_quantity ?? 0
  const reserved = selectedVariant.value.reserved_quantity ?? 0
  const available = qty - reserved
  const threshold = selectedVariant.value.low_stock_threshold ?? 5
  return available > 0 && available <= threshold
})

watch(variants, (newVariants) => {
  if (newVariants && newVariants.length > 0 && !selectedVariantId.value) {
    selectedVariantId.value = newVariants[0].id
  }
}, { immediate: true })

// Cart & Wishlist
const { addItemSync } = useCartSync()

function addToCartApi(product: ProductApi) {
  if (!product.id) return
  
  addItemSync({
    productId: product.id,
    name: product.name ?? 'Unknown Product',
    price: selectedPrice.value,
    image: (product.images?.[0]?.image ?? ''),
    slug: (product.slug ?? ''),
    variantId: selectedVariantId.value ?? undefined,
  })
  openCart()
}

const { fetchWishlist, toggleWishlist, isInWishlist } = useWishlist()
onMounted(() => { fetchWishlist() })

// Reviews (non-blocking)
const { data: reviews, refresh: refreshReviews } = useApiLazyFetch<Review[]>(
  () => `/api/reviews?product_id=${productId.value}`,
  { key: `reviews-${productId.value}` }
)

const reviewForm = reactive({
  rating: 5,
  title: '',
  comment: '',
})
const reviewSubmitting = ref(false)
const reviewError = ref('')
const reviewSuccess = ref('')

async function submitReview() {
  if (!productApi.value?.id) return
  reviewSubmitting.value = true
  reviewError.value = ''
  reviewSuccess.value = ''
  try {
    await apiFetch('/api/reviews', {
      method: 'POST',
      body: {
        product_id: Number(productId.value),
        user_id: 1, // Static user for now, should be from auth store
        rating: reviewForm.rating,
        title: reviewForm.title.trim() || null,
        comment: reviewForm.comment.trim() || null,
        active: true,
      },
    })
    reviewSuccess.value = t('product.reviewSuccess')
    reviewForm.rating = 5
    reviewForm.title = ''
    reviewForm.comment = ''
    await refreshReviews()
  } catch (err: any) {
    reviewError.value = err?.data?.message || err?.message || 'Erro ao enviar avaliação'
  } finally {
    reviewSubmitting.value = false
  }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
