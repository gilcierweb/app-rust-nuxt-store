<template>
  <div>


    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-4 gap-4 mt-6">
      <div v-if="pendingApi">
        <div class="flex items-center justify-center h-96">
          <span>Loading ...</span> <br>
          <span class="loading loading-spinner text-info size-40"></span>
        </div>
      </div>

      <div  v-for="product in productsApi" :key="product.id" class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
          <figure>
            <LazyImage
              v-if="product.images?.[0]?.image"
              :src="product.images[0].image"
              :alt="product.name"
              :width="300"
              :height="300"
              preset="thumbnail"
            />
          </figure>
          <div class="card-body">
            <h5 class="card-title mb-2.5">{{ product.name }}</h5>
            <p class="text-success font-weight-bold">{{ formatNumberBR(product?.price) }}</p>
            <p class="mb-4">
              {{ $truncate(product.description, 70, '...') }}
            </p>
            <p> <span v-if="product.category" class="badge badge-secondary">{{ product.category.name }}</span></p>
            <div class="card-actions">
              <button class="btn btn-primary btn-soft" @click="addToCartApi(product)">
                <span class="icon-[tabler--shopping-cart] size-4"></span>
                {{ t('product.addToCart') }}
              </button>
              <button class="btn btn-ghost btn-circle" @click="toggleWishlist(product.id)">
                <span :class="[isInWishlist(product.id) ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart]', 'size-5']"></span>
              </button>
              <NuxtLink :to="`/products/${product.id}`" class="btn btn-secondary btn-soft">
                Details
              </NuxtLink>
            </div>
          </div>
        </div>

      <div v-if="pending">
        <div class="flex items-center justify-center h-96">
          <span>Loading ...</span> <br>
          <span class="loading loading-spinner text-info size-40"></span>
        </div>
      </div>
     
       <div v-else v-for="product in products" :key="product.id" class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
          <figure>           
            <NuxtImg :src="product?.thumbnail" loading="lazy" :alt="product?.title" />
          </figure>
          <div class="card-body">
            <h5 class="card-title mb-2.5">{{ product.title }}</h5>
            <p class="text-success font-weight-bold">{{ formatNumberBR(product?.price) }}</p>
            <p class="mb-4">
              {{ $truncate(product.description, 70, '...') }}
            </p>
            <p> <span class="badge badge-secondary">{{ product.category }}</span></p>
            <div class="card-actions">
              <button class="btn btn-primary btn-soft" @click="addToCart(product)">
                <span class="icon-[tabler--shopping-cart] size-4"></span>
                {{ t('product.addToCart') }}
              </button>
              <button class="btn btn-ghost btn-circle" @click="toggleWishlist(product.id)">
                <span :class="[isInWishlist(product.id) ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart]', 'size-5']"></span>
              </button>
              <NuxtLink :to="`/products/${product.id}`" class="btn btn-secondary btn-soft">
                Details
              </NuxtLink>
            </div>
          </div>
        </div>
     
      <div v-if="products.length === 0">No products available.</div>

    </div>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();
import type { ProductApi } from '~/types';
const { t } = useI18n()
const cartStore = useCartStore()

interface Product {
  id: number;
  title: string;
  description: string;
  price: number;
  url: string;
  category: string;
  image: string;
  thumbnail: string;
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

function addToCartApi(product: ProductApi) {
  cartStore.addItem({
    productId: product.id,
    name: product.name,
    price: product.price,
    image: product.images?.[0]?.image,
    slug: product.slug,
  })
  openCart()
}

const { openCart } = useCartUI()
const { fetchWishlist, toggleWishlist, isInWishlist } = useWishlist()

onMounted(() => { fetchWishlist() })

const { pending: pendingApi, data: productsData } = await useFetch<ProductApi[]>(`${config.public.baseURL}/api/products`);
const productsApi = computed(() => productsData.value ?? []);
const { pending, data } = await useFetch<Product[]>(`https://dummyjson.com/products`);
const products = data.value?.products || [];
</script>

<style scoped></style>