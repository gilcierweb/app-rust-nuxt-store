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
            <!-- <NuxtImg :src="product?.thumbnail" loading="lazy" :alt="product?.title" /> -->
          </figure>
          <div class="card-body">
            <h5 class="card-title mb-2.5">{{ product.name }}</h5>
            <p class="text-success font-weight-bold">{{ formatNumberBR(product?.price) }}</p>
            <p class="mb-4">
              {{ $truncate(product.description, 70, '...') }}
            </p>
            <p> <span class="badge badge-secondary">{{ product.category.name }}</span></p>
            <div class="card-actions">
              <button class="btn btn-primary">Buy Now</button>
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
              <button class="btn btn-primary">Buy Now</button>
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

// const config = useRuntimeConfig();


const { pending: pendingApi, data: productsApi } = await useFetch<ProductApi>(`${config.public.baseURL}/api/products`);
const { pending, data } = await useFetch<Product[]>(`https://dummyjson.com/products`);
const products = data.value?.products || [];
</script>

<style scoped></style>