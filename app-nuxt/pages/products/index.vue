<template>
  <div>
    <h1>Page Products</h1>

    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-4 gap-4 mt-6">

      <div v-if="pending">
        <div class="flex items-center justify-center h-96">
          <span>Loading ...</span> <br>
          <span class="loading loading-spinner text-info size-40"></span>
        </div>
      </div>

      <div v-else v-for="product in products" :key="product.id">
        <div class="card sm:max-w-sm">
          <figure>
            <img :src="product?.thumbnail" alt="image" />
          </figure>
          <div class="card-body">
            <h5 class="card-title mb-2.5">{{ product.title }}</h5>
            <p class="text-success font-weight-bold">{{ formatNumberBR(product?.price) }}</p>
            <p class="mb-4">
              {{ $truncate(product.description, 70, '...') }}
            </p>
            <p> {{ product.category }}</p>
            <div class="card-actions">
              <button class="btn btn-primary">Buy Now</button>
              <NuxtLink :to="`/products/${product.id}`">
                <button class="btn btn-secondary btn-soft btn-block">Details</button>
              </NuxtLink>
            </div>
          </div>
        </div>

          <div v-if="products.length === 0">No products available.</div>
      </div>

    

    </div>
  </div>
</template>

<script setup lang="ts">
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

// const { pending, data } = await useLazyFetch<Product[]>("https://dummyjson.com/products");
const { pending, data } = await useFetch<Product[]>(`https://dummyjson.com/products`);
const products = data.value?.products || [];
</script>

<style scoped></style>