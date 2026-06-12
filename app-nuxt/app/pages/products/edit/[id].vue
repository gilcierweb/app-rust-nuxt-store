<template>
  <div>
    <div class="container mx-auto px-4 py-8">
      <div class="mb-6">
        <NuxtLinkLocale to="/products" class="btn btn-outline btn-sm">
          ← {{ t('pages.products.back') }}
        </NuxtLinkLocale>
      </div>
      
      <!-- Loading state while fetching product -->
      <div v-if="pending" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary size-16"></span>
        <span class="ml-4 text-lg">{{ t('pages.products.edit.loading') }}</span>
      </div>
      
      <!-- Error state -->
      <div v-else-if="error" class="alert alert-error">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span>{{ t('pages.products.edit.error', { message: error }) }}</span>
      </div>
      
      <!-- Product form -->
      <AdminProductsForm
        v-else-if="product"
        :product="product"
        :is-editing="true"
        @saved="handleProductUpdated"
        @cancel="handleCancel"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductApi } from '~/types';

definePageMeta({
  middleware: 'auth'
});

const { t } = useI18n();
const auth = useAuth();

await auth.init();

if (!auth.isAuthenticated.value || !auth.user.value?.can_manage_admin) {
  throw createError({ statusCode: 403, statusMessage: 'Forbidden' });
}

const route = useRoute();
const { useApiLazyFetch } = useApi();

const id = route.params.id;

const { pending, data: product, error } = useApiLazyFetch<ProductApi>(() => `/api/products/${id}`);

const handleProductUpdated = (_updatedProduct: ProductApi) => {
  navigateTo('/products');
};

const handleCancel = () => {
  navigateTo('/products');
};

useHead({
  title: computed(() => product.value ? t('pages.products.edit.titleWithName', { name: product.value.name }) : t('pages.products.edit.title'))
});
</script>