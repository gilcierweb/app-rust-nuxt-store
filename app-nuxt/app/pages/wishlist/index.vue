<template>
  <div>
    <h1 class="h1 my-4">{{ t('pages.wishlist.title') }}</h1>

    <div v-if="loading" class="flex items-center justify-center py-20">
      <span class="loading loading-spinner text-primary size-12"></span>
    </div>

    <div v-else-if="wishlist.length === 0" class="flex flex-col items-center justify-center py-20">
      <span class="icon-[tabler--heart-off] mb-4 size-20 opacity-40" />
      <p class="mb-6 text-lg text-base-content/60">{{ t('pages.wishlist.empty') }}</p>
      <NuxtLink to="/products" class="btn btn-primary">
        {{ t('pages.wishlist.continueShopping') }}
      </NuxtLink>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
      <div v-for="item in wishlist" :key="item.id" class="card bg-white shadow-lg hover:shadow-xl transition-all duration-300 hover:-translate-y-1">
        <div class="card-body">
          <div class="flex items-start justify-between">
            <h5 class="card-title mb-2.5">Produto #{{ item.product_id }}</h5>
            <button class="btn btn-ghost btn-circle btn-sm" @click="removeFromWishlist(item)">
              <span class="icon-[tabler--trash] size-4 text-error"></span>
            </button>
          </div>
          <p class="text-xs text-base-content/40 mb-2">{{ new Date(item.created_at).toLocaleDateString('pt-BR') }}</p>
          <NuxtLink :to="`/products/${item.product_id}`" class="btn btn-primary btn-sm mt-2">
            {{ t('pages.wishlist.viewProduct') }}
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WishlistItem } from '~/types'
const { t } = useI18n()
const { wishlist, fetchWishlist, toggleWishlist } = useWishlist()
const loading = ref(true)

onMounted(async () => {
  await fetchWishlist()
  loading.value = false
})

async function removeFromWishlist(item: WishlistItem) {
  await toggleWishlist(item.product_id)
}
</script>
