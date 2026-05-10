<template>
    <div>
        <div class="mb-6">
            <h1 class="h1">{{ $t('admin.products.title') }}</h1>
        </div>

        <div class="mb-6 justify-between flex items-center">
            <form @submit.prevent>
                <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                    <input type="text" :placeholder="$t('admin.products.searchPlaceholder')" class="input input-bordered w-full mb-4" />
                    <button class="btn btn-primary">{{ $t('common.search') }}</button>
                </div>
            </form>

            <NuxtLink to="/admin/products/new" class="btn btn-success">{{ $t('admin.products.add') }}</NuxtLink>

        </div>

        <div class="w-full overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>{{ $t('admin.products.table.name') }}</th>
                        <th>{{ $t('admin.products.table.price') }}</th>
                        <th>{{ $t('admin.products.table.description') }}</th>
                        <th>{{ $t('admin.products.table.status') }}</th>
                        <th>{{ $t('admin.products.table.category') }}</th>
                        <th>{{ $t('admin.products.table.date') }}</th>
                        <th>{{ $t('admin.products.table.actions') }}</th>
                    </tr>
                </thead>
                <tbody>

                    <tr class="row-hover" v-for="product in products" :key="product.id">
                        <td>{{ product.name }}</td>
                        <td>{{ formatNumberBR(product?.price) }}</td>
                        <td>{{ $truncate(product.description, 70, '...') }}</td>
                        <td><span class="badge badge-soft badge-success text-xs">{{ product.status }}</span></td>
                        <td><span class="badge badge-soft badge-success text-xs">{{ product.category.name }}</span></td>
                        <td>{{ formatDate(product.created_at) }}</td>
                        <td>
                            <NuxtLink :to="`/admin/products/${product.id}`" class="btn btn-circle btn-text btn-sm"
                                :aria-label="$t('common.view')">
                                <i class="icon-[tabler--eye] size-5"></i>
                            </NuxtLink>
                            <NuxtLink :to="`/admin/products/${product.id}/edit`" class="btn btn-circle btn-text btn-sm"
                                :aria-label="$t('common.edit')">
                                <i class="icon-[tabler--pencil] size-5"></i>
                            </NuxtLink>
                            <button class="btn btn-circle btn-text btn-sm" :aria-label="$t('common.delete')"><span
                                    class="icon-[tabler--trash] size-5"></span></button>
                            <button class="btn btn-circle btn-text btn-sm" aria-label="More"><span
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

const formatNumberBR = (num: number | undefined) => {
    return new Intl.NumberFormat('pt-BR', { style: 'currency', currency: 'BRL' }).format(num || 0);
};

const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Intl.DateTimeFormat('pt-BR', {
        day: '2-digit',
        month: 'short',
        year: 'numeric'
    }).format(new Date(dateString));
};

</script>

<style scoped></style>