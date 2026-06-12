<template>
  <div class="card bg-base-100 shadow-sm hover:shadow-xl transition-all duration-300 group hover:-translate-y-1.5 border border-base-200 overflow-hidden">
    <figure class="relative aspect-square overflow-hidden bg-base-200">
      <NuxtLinkLocale :to="`/products/${product.id}`" class="w-full h-full">
        <OptimizedImage
          v-if="image"
          :src="image"
          :alt="product.name"
          class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
        />
        <div v-else class="flex items-center justify-center h-full text-base-content/20">
          <span class="icon-[tabler--photo] size-12"></span>
        </div>
      </NuxtLinkLocale>

      <div class="absolute top-3 left-3 flex flex-col gap-2">
        <span v-if="isNew" class="badge badge-primary badge-sm shadow-sm">{{ t('productCard.new') }}</span>
        <span v-if="category" class="badge badge-secondary badge-sm shadow-sm">{{ category }}</span>
      </div>

      <button
        v-if="showWishlist"
        @click.prevent="toggleWishlist(product.id)"
        class="btn btn-circle btn-sm glass absolute top-3 right-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300 shadow-sm"
        :aria-label="t('productCard.toggleWishlist')"
      >
        <span :class="[isInWishlist(product.id) ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart]', 'size-4']"></span>
      </button>
    </figure>

    <div class="card-body p-5 gap-1">
      <NuxtLinkLocale :to="`/products/${product.id}`" class="group/title">
        <h3 class="card-title text-base font-bold line-clamp-1 group-hover/title:text-primary transition-colors">
          {{ product.name }}
        </h3>
      </NuxtLinkLocale>

      <div class="flex items-center justify-between mt-1">
        <p class="text-primary font-bold text-lg">{{ formatNumberBR(product.price) }}</p>
        <div v-if="rating" class="flex items-center gap-1 text-warning text-xs">
          <span class="icon-[tabler--star-filled] size-3"></span>
          <span class="font-medium">{{ rating }}</span>
        </div>
      </div>

      <p v-if="showDescription && description" class="text-base-content/60 text-sm line-clamp-2 mt-1 h-10">
        {{ description }}
      </p>

      <div class="card-actions mt-4 pt-4 border-t border-base-200">
        <button class="btn btn-primary btn-sm flex-1 gap-2" @click="handleAddToCart">
          <span class="icon-[tabler--shopping-cart] size-4"></span>
          {{ t('productCard.addToCart') }}
        </button>
        <NuxtLinkLocale :to="`/products/${product.id}`" class="btn btn-ghost btn-sm btn-square" :aria-label="t('productCard.viewDetails')">
          <span class="icon-[tabler--eye] size-4"></span>
        </NuxtLinkLocale>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import OptimizedImage from './OptimizedImage.vue'

interface Product {
  id: number | string
  name: string
  price: number
  image?: string
  description?: string
  category?: string
  isNew?: boolean
  rating?: number
}

interface Props {
  product: Product
  image?: string
  category?: string
  description?: string
  isNew?: boolean
  rating?: number
  showWishlist?: boolean
  showDescription?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  image: '',
  category: '',
  description: '',
  isNew: false,
  rating: undefined,
  showWishlist: true,
  showDescription: true
})

const { t } = useI18n()
const { openCart } = useCartUI()
const { addItemSync } = useCartSync()
const { toggleWishlist, isInWishlist } = useWishlist()

function handleAddToCart() {
  addItemSync({
    productId: props.product.id as number,
    name: props.product.name,
    price: props.product.price,
    image: props.image || props.product.image,
  })
  openCart()
}
</script>
