<template>
    <div>
        <div class="mb-6">
            <h1 class="h1">Products</h1>
        </div>

        <div class="mb-6 justify-between flex items-center">
            <form action="">
                <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                    <input type="text" placeholder="Search products" class="input input-bordered w-full mb-4" />
                    <button class="btn btn-primary">Search</button>
                </div>
            </form>

            <NuxtLink to="/admin/products/new" class="btn btn-success"> Add</NuxtLink>

        </div>

        <div class="w-full overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Price</th>
                        <th>Description</th>
                        <th>Status</th>
                        <th>Category</th>
                        <th>Date</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>

                    <tr class="row-hover" v-for="product in products" :key="product.id">
                        <td>{{ product.name }}</td>
                        <td>{{ formatNumberBR(product?.price) }}</td>
                        <td>{{ $truncate(product.description, 70, '...') }}</td>
                        <td><span class="badge badge-soft badge-success text-xs">{{ product.status }}</span></td>
                        <td><span class="badge badge-soft badge-success text-xs">{{ product.category.name }}</span></td>
                        <td>March 1, 2024</td>
                        <td>
                            <NuxtLink :to="`/admin/products/${product.id}`" class="btn btn-circle btn-text btn-sm"
                                aria-label="Action button">
                                <i class="icon-[tabler--eye] size-5"></i>
                            </NuxtLink>
                            <NuxtLink :to="`/admin/products/${product.id}/edit`" class="btn btn-circle btn-text btn-sm"
                                aria-label="Action button">
                                <i class="icon-[tabler--pencil] size-5"></i>
                            </NuxtLink>
                            <button class="btn btn-circle btn-text btn-sm" aria-label="Action button"><span
                                    class="icon-[tabler--trash] size-5"></span></button>
                            <button class="btn btn-circle btn-text btn-sm" aria-label="Action button"><span
                                    class="icon-[tabler--dots-vertical] size-5"></span></button>
                        </td>
                    </tr>

                </tbody>
            </table>
        </div>
    </div>
</template>

<script setup lang="ts">

definePageMeta({
    layout: 'admin'
});

const config = useRuntimeConfig();
const { $truncate } = useNuxtApp();
const { pending: pending, data: products } = await useFetch<ProductApi>(`${config.public.baseURL}/api/products`);

</script>

<style scoped></style>