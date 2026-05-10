<template>
  <div>
    <div v-if="pendingApi">
      <div class="flex items-center justify-center h-96">
        <span>{{ t('pages.products.loading') }}</span> <br>
        <span class="loading loading-spinner text-info size-40"></span>
      </div>
    </div>
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 ">
      <div>
        <div class="w-full md:w-1/2 px-4 mb-8">
          <div class="flex gap-4 py-4 justify-center overflow-x-auto"></div>
        </div>
      </div>
      <div>
        <h1 class="text-base-content text-4xl">{{ productApi?.name }}</h1>
        <p class="text-success font-weight-bold mt-2">{{ formatNumberBR(selectedPrice) }}</p>
        <p class="my-3"><span class="badge badge-secondary">{{ productApi?.category?.name }}</span></p>
        <div v-if="variants && variants.length > 0" class="my-4">
          <label class="label">{{ t('variant.select') }}</label>
          <select v-model="selectedVariantId" class="select select-bordered w-full max-w-xs">
            <option :value="null">{{ t('variant.select') }}...</option>
            <option v-for="v in variants" :key="v.id" :value="v.id">
              {{ v.name || `${v.sku || ''}${v.price ? ' - ' + formatNumberBR(v.price) : ''}` }}
            </option>
          </select>
        </div>
        <div class="flex gap-2">
          <button class="btn btn-primary btn-xl" @click="addToCartApi(productApi!)">
            <span class="icon-[tabler--shopping-cart] size-5"></span>
            {{ t('product.addToCart') }}
          </button>
          <button class="btn btn-ghost btn-circle btn-xl" @click="toggleWishlist(productApi!.id)">
            <span :class="[isInWishlist(productApi!.id) ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart]', 'size-6']"></span>
          </button>
        </div>
      </div>
      <div class="grid grid-cols-1">
        <p>{{ productApi?.description }}</p>
      </div>
    </div>

    <div v-if="pending">
      <div class="flex items-center justify-center h-96">
        <span>{{ t('pages.products.loading') }}</span> <br>
        <span class="loading loading-spinner text-info size-40"></span>
      </div>
    </div>
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 ">
      <div>
        <div class="w-full md:w-1/2 px-4 mb-8">
          <NuxtImg :src="product?.thumbnail" loading="lazy" class="w-full h-auto rounded-lg shadow-md mb-4" :alt="product?.title" />
          <div class="flex gap-4 py-4 justify-center overflow-x-auto">
            <div v-for="(item, index) in product?.images" :key="index">
              <NuxtImg :src="item" loading="lazy" class="size-16 sm:size-20 object-cover rounded-md cursor-pointer opacity-60 hover:opacity-100 transition duration-300" :alt="product?.title" />
            </div>
          </div>
        </div>
      </div>
      <div>
        <h1 class="text-base-content text-4xl">{{ product?.title }}</h1>
        <p class="text-success font-weight-bold mt-2">{{ formatNumberBR(product?.price) }}</p>
        <p class="my-3"><span class="badge badge-secondary">{{ product?.category }}</span></p>
        <div class="flex gap-2">
          <button class="btn btn-primary btn-xl" @click="addToCart(product!)">
            <span class="icon-[tabler--shopping-cart] size-5"></span>
            {{ t('product.addToCart') }}
          </button>
          <button class="btn btn-ghost btn-circle btn-xl" @click="toggleWishlist(product!.id)">
            <span :class="[isInWishlist(product!.id) ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart]', 'size-6']"></span>
          </button>
        </div>
      </div>
      <div class="grid grid-cols-1">
        <p>{{ product?.description }}</p>
      </div>
    </div>

    <!-- Reviews Section - Only show when not loading -->
    <div v-if="!pendingApi && !pending" class="mt-12 border-t pt-8">
      <h2 class="text-2xl font-bold mb-6">{{ t('pages.products.reviews') }}</h2>

      <!-- Existing Reviews -->
      <div v-if="reviews && reviews.length > 0" class="space-y-4 mb-8">
        <div v-for="review in reviews" :key="review.id" class="rounded-box border p-4">
          <div class="flex items-center gap-2 mb-2">
            <div class="flex text-warning">
              <i v-for="n in 5" :key="n"
                 :class="n <= (review.rating || 0) ? 'icon-[tabler--star-filled] text-warning' : 'icon-[tabler--star] text-gray-300'"
                 class="size-5"></i>
            </div>
            <span v-if="review.verified_purchase" class="badge badge-success badge-xs">Compra verificada</span>
          </div>
          <p v-if="review.title" class="font-semibold">{{ review.title }}</p>
          <p v-if="review.comment" class="text-sm text-base-content/70 mt-1">{{ review.comment }}</p>
          <p class="text-xs text-base-content/40 mt-2">{{ new Date(review.created_at).toLocaleDateString('pt-BR') }}</p>
        </div>
      </div>
      <p v-else class="text-base-content/60 mb-8">Nenhuma avaliação ainda. Seja o primeiro!</p>

      <!-- Review Form -->
      <div class="rounded-box border p-6 max-w-xl">
        <h3 class="text-lg font-semibold mb-4">{{ t('pages.products.writeReview') }}</h3>

        <AppAlert v-if="reviewSuccess" type="success" :message="reviewSuccess" :auto-close="3000" @close="reviewSuccess = ''" />
        <AppAlert v-if="reviewError" type="error" :message="reviewError" :auto-close="5000" @close="reviewError = ''" :dismissible="true" />

        <form @submit.prevent="submitReview" class="space-y-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Avaliação</span></label>
            <div class="flex items-center gap-2">
              <input v-model.number="reviewForm.rating" type="range" min="1" max="5" class="range range-primary flex-1" />
              <span class="text-xl font-bold w-6 text-center">{{ reviewForm.rating }}</span>
              <div class="flex text-warning">
                <i v-for="n in 5" :key="n"
                   :class="n <= reviewForm.rating ? 'icon-[tabler--star-filled] text-warning' : 'icon-[tabler--star] text-gray-300'"
                   class="size-6"></i>
              </div>
            </div>
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Título (opcional)</span></label>
            <input v-model="reviewForm.title" type="text" placeholder="Resumo da sua avaliação" class="input input-bordered" />
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Comentário (opcional)</span></label>
            <textarea v-model="reviewForm.comment" class="textarea textarea-bordered" rows="3" placeholder="Conte sua experiência..."></textarea>
          </div>
          <button type="submit" class="btn btn-primary" :disabled="reviewSubmitting">
            <span v-if="reviewSubmitting" class="loading loading-spinner loading-sm"></span>
            Enviar Avaliação
          </button>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Product, ProductApi, ProductVariant, Review } from '~/types'
const { t } = useI18n()
useSeoMeta({
  title: computed(() => productApi.value?.name || product.value?.title || t('pages.products.title')),
  ogTitle: computed(() => productApi.value?.name || product.value?.title || t('pages.products.title')),
  description: computed(() => productApi.value?.description || product.value?.description || ''),
  ogDescription: computed(() => productApi.value?.description || product.value?.description || ''),
  ogImage: computed(() => productApi.value?.images?.[0]?.image || product.value?.thumbnail || ''),
})
const config = useRuntimeConfig()
const route = useRoute()
const cartStore = useCartStore()
const { openCart } = useCartUI()

const id = route.params.id

const selectedVariantId = ref<number | null>(null)
const { data: variants } = await useFetch<ProductVariant[]>(
  `${config.public.baseURL}/api/variants/list?product_id=${id}`
)

const selectedVariant = computed(() => {
  if (!selectedVariantId.value || !variants.value) return null
  return variants.value.find(v => v.id === selectedVariantId.value) || null
})

const selectedPrice = computed(() => {
  if (selectedVariant.value?.price) return selectedVariant.value.price
  return productApi.value?.price
})

function addToCartApi(product: ProductApi) {
  cartStore.addItem({
    productId: product.id,
    name: product.name,
    price: selectedPrice.value ?? product.price,
    image: product.images?.[0]?.image,
    slug: product.slug,
    variantId: selectedVariantId.value ?? undefined,
  })
  openCart()
}

function addToCart(product: Product) {
  cartStore.addItem({
    productId: product.id,
    name: product.title,
    price: product.price,
    image: product.thumbnail,
  })
  openCart()
}

const { fetchWishlist, toggleWishlist, isInWishlist } = useWishlist()
onMounted(() => { fetchWishlist() })

const { data: productApi, pending: pendingApi } = await useFetch<ProductApi>(`${config.public.baseURL}/api/products/${id}`)
const { data: product, pending } = await useLazyFetch<Product>(`https://dummyjson.com/products/${id}`)

const { data: reviews, refresh: refreshReviews } = await useLazyFetch<Review[]>(
  `${config.public.baseURL}/api/reviews?product_id=${id}`
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
  reviewSubmitting.value = true
  reviewError.value = ''
  reviewSuccess.value = ''
  try {
    await $fetch(`${config.public.baseURL}/api/reviews`, {
      method: 'POST',
      body: {
        product_id: Number(id),
        user_id: 1,
        rating: reviewForm.rating,
        title: reviewForm.title.trim() || null,
        comment: reviewForm.comment.trim() || null,
        active: true,
      },
    })
    reviewSuccess.value = 'Avaliação enviada com sucesso!'
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
