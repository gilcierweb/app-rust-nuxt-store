<template>
    <div>

        <div v-if="pendingApi">
            <div class="flex items-center justify-center h-96">
                <span>Loading ...</span> <br>
                <span class="loading loading-spinner text-info size-40"></span>
            </div>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 ">
            <div>

                <!-- Product Images -->
                <div class="w-full md:w-1/2 px-4 mb-8">                  
                  <!-- <NuxtImg :src="productApi?.thumbnail" loading="lazy" class="w-full h-auto rounded-lg shadow-md mb-4" :alt="productApi?.name" /> -->
                    <div class="flex gap-4 py-4 justify-center overflow-x-auto">
                        <!-- <div v-for="(item, index) in productApi.images" :key="index">
                            <NuxtImg :src="item" loading="lazy" class="size-16 sm:size-20 object-cover rounded-md cursor-pointer opacity-60 hover:opacity-100 transition duration-300" :alt="productApi?.name" />
                        </div> -->
                    </div>
                </div>
            </div>

            <div>
                <h1 class="text-base-content text-4xl">{{ productApi?.name }}</h1>
                <p class="text-success font-weight-bold mt-2">{{ formatNumberBR(productApi?.price) }}</p>
                <p class="my-3"><span class="badge badge-secondary">{{ productApi?.category.name }}</span></p>
                <button class="btn btn-primary btn-xl">Buy Now</button>
            </div>
            <div class="grid grid-cols-1">
                <p>{{ productApi?.description }}</p>
            </div>

        </div>

        <div v-if="pending">
            <div class="flex items-center justify-center h-96">
                <span>Loading ...</span> <br>
                <span class="loading loading-spinner text-info size-40"></span>
            </div>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 ">
            <div>

                <!-- Product Images -->
                <div class="w-full md:w-1/2 px-4 mb-8">                  
                  <NuxtImg :src="product?.thumbnail" loading="lazy" class="w-full h-auto rounded-lg shadow-md mb-4" :alt="product?.title" />
                    <div class="flex gap-4 py-4 justify-center overflow-x-auto">
                        <div v-for="(item, index) in product.images" :key="index">
                            <NuxtImg :src="item" loading="lazy" class="size-16 sm:size-20 object-cover rounded-md cursor-pointer opacity-60 hover:opacity-100 transition duration-300" :alt="product?.title" />
                        </div>
                    </div>
                </div>
            </div>

            <div>
                <h1 class="text-base-content text-4xl">{{ product?.title }}</h1>
                <p class="text-success font-weight-bold mt-2">{{ formatNumberBR(product?.price) }}</p>
                <p class="my-3"><span class="badge badge-secondary">{{ product?.category }}</span></p>
                <button class="btn btn-primary btn-xl">Buy Now</button>
            </div>
            <div class="grid grid-cols-1">
                <p>{{ product?.description }}</p>
            </div>

        </div>       
        
    </div>
</template>

<script setup lang="ts">
import type { ProductApi } from '~/types';
const config = useRuntimeConfig();
const route = useRoute();
// const config = useRuntimeConfig(); 

const id = route.params.id;
console.log(route.params.id);

const { data: productApi, pending: pendingApi } = await useFetch<ProductApi>(`${config.public.baseURL}/api/products/${id}`);
const { data: product, pending } = await useLazyFetch<Product>(`https://dummyjson.com/products/${id}`);
console.log({
    product,
    id,
});
</script>

<style scoped></style>