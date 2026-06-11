<template>
  <div>
    <div class="container mx-auto px-4 py-8">
      <div class="mb-6">
        <NuxtLinkLocale to="/products" class="btn btn-outline btn-sm">
          ← {{ t('pages.products.back') }}
        </NuxtLinkLocale>
      </div>
      
      <ProductForm
        @saved="handleProductSaved"
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

const handleProductSaved = (product: ProductApi) => {
  console.log('Produto criado:', product);
};

const handleCancel = () => {
  navigateTo('/products');
};

useHead({
  title: t('pages.products.create.title')
});
</script>