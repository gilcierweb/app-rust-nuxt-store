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
        </div>
      </div>
      <div class="grid grid-cols-1">
        <p>{{ product?.description }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Product, ProductApi, ProductVariant } from '~/types'
const { t } = useI18n()
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

const { data: productApi, pending: pendingApi } = await useFetch<ProductApi>(`${config.public.baseURL}/api/products/${id}`)
const { data: product, pending } = await useLazyFetch<Product>(`https://dummyjson.com/products/${id}`)
</script>
