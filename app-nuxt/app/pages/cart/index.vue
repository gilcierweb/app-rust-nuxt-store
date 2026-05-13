<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="mb-10 pt-10">
      <h1 class="h2 gradient-text">{{ t('pages.cart.title') }}</h1>
      <p class="text-base-content/60 mt-1">{{ t('pages.cart.description') }}</p>
    </div>

    <!-- Empty State -->
    <div v-if="cartStore.isEmpty" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="alert alert-info max-w-md">
        <div class="flex items-center gap-4">
          <div class="size-16 rounded-full bg-info/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--shopping-cart-off] size-8 text-info" />
          </div>
          <div>
            <h2 class="font-bold text-lg">{{ t('cart.empty') }}</h2>
              {{ t('pages.cart.emptyDescription') }}
          </div>
        </div>
      </div>
      <NuxtLink to="/products" class="btn btn-primary btn-lg mt-8">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <!-- Cart Content -->
    <div v-else class="grid grid-cols-1 lg:grid-cols-12 gap-10">
      <!-- Items List -->
      <div class="lg:col-span-8 space-y-6">
        <div class="card bg-base-100 shadow-soft border border-base-200">
          <div class="card-body p-0 overflow-x-auto">
            <table class="table">
              <thead>
                <tr class="bg-base-200/50">
                  <th class="font-bold py-4 pl-6">{{ t('pages.cart.item') }}</th>
                  <th class="font-bold py-4">{{ t('cart.quantity') }}</th>
                  <th class="font-bold py-4 text-right">{{ t('cart.total') }}</th>
                  <th class="py-4 pr-6"></th>
                </tr>
              </thead>
              <tbody class="divide-y divide-base-200">
                <tr v-for="item in cartStore.items" :key="item.productId" class="hover:bg-base-200/20 transition-colors">
                  <td class="py-4 pl-6">
                    <div class="flex items-center gap-4">
                      <div class="size-20 rounded-xl overflow-hidden bg-base-200 shrink-0 border border-base-200">
                        <NuxtImg v-if="item.image" :src="item.image" class="size-full object-cover" :alt="item.name" />
                        <div v-else class="flex items-center justify-center h-full text-base-content/20">
                          <span class="icon-[tabler--photo] size-6"></span>
                        </div>
                      </div>
                      <div>
                        <NuxtLink v-if="item.slug" :to="`/products/${item.slug || item.productId}`" class="font-bold hover:text-primary transition-colors line-clamp-1">
                          {{ item.name }}
                        </NuxtLink>
                        <p v-else class="font-bold line-clamp-1">{{ item.name }}</p>
                        <p class="text-primary font-semibold text-sm">{{ formatNumberBR(item.price) }}</p>
                      </div>
                    </div>
                  </td>
                  <td class="py-4">
                    <div class="inline-flex items-center p-1 border border-base-200 rounded-lg">
                      <button class="btn btn-soft btn-secondary btn-square btn-sm" 
                        @click="cartStore.updateQuantity(item.productId, item.quantity - 1)"
                        :disabled="item.quantity <= 1">
                        <span class="icon-[tabler--minus] size-4" />
                      </button>
                      <span class="w-10 text-center font-bold">{{ item.quantity }}</span>
                      <button class="btn btn-soft btn-secondary btn-square btn-sm" @click="cartStore.updateQuantity(item.productId, item.quantity + 1)">
                        <span class="icon-[tabler--plus] size-4" />
                      </button>
                    </div>
                  </td>
                  <td class="py-4 text-right font-bold">
                    {{ formatNumberBR(item.price * item.quantity) }}
                  </td>
                  <td class="py-4 pr-6 text-right">
                    <button @click="cartStore.removeItem(item.productId)" class="btn btn-circle btn-soft btn-error btn-sm" aria-label="Remove item">
                      <span class="icon-[tabler--trash] size-4" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        
        <div class="flex flex-wrap items-center justify-between gap-4 px-4">
          <div class="flex flex-wrap gap-4">
            <NuxtLink to="/products" class="btn btn-soft btn-primary gap-2 rounded-xl">
              <span class="icon-[tabler--arrow-left] size-4" />
              {{ t('cart.continueShopping') }}
            </NuxtLink>
            <button class="btn btn-soft btn-error gap-2 rounded-xl" @click="cartStore.clearCart()">
              <span class="icon-[tabler--trash-x] size-4" />
              {{ t('cart.clearCart') }}
            </button>
          </div>
        </div>
      </div>

      <!-- Order Summary -->
      <div class="lg:col-span-4 mt-8 lg:mt-0">
        <div class="card bg-base-100 shadow-xl border border-base-200 sticky top-24">
          <div class="card-body p-8 md:p-10">
            <h5 class="card-title mb-4">{{ t('cart.orderSummary') }}</h5>
          
          <div class="space-y-4 mb-8">
            <div class="flex justify-between text-base-content/60">
              <span>{{ t('cart.subtotal') }}</span>
              <span class="badge badge-ghost">{{ t('cart.itemsCount', { count: cartStore.totalItems }) }}</span>
              <span>{{ formatNumberBR(cartStore.totalPrice) }}</span>
            </div>
            <div class="flex justify-between items-center gap-2">
              <span>{{ t('cart.shipping') }}</span>
              <span class="badge badge-success badge-sm">{{ t('shipping.free') }}</span>
            </div>
          </div>
          
          <div class="pt-6 border-t border-base-200 mb-8">
            <div class="alert alert-info mb-4">
              <div class="flex items-center gap-2">
                <span class="icon-[tabler--info-circle] size-5"></span>
                <span class="text-sm">{{ t('cart.taxesIncluded') }}</span>
              </div>
            </div>
            <div class="flex justify-between items-end">
              <span class="font-bold text-lg text-base-content/60">{{ t('cart.total') }}</span>
              <div class="text-right">
                <span class="block text-3xl font-black text-primary">{{ formatNumberBR(cartStore.totalPrice) }}</span>
              </div>
            </div>
          </div>
          
          <div class="card-actions mt-4 w-full">
            <NuxtLink to="/checkout" class="btn btn-primary w-full">
              {{ t('cart.checkout') }}
              <span class="icon-[tabler--arrow-right] size-5 ml-2" />
            </NuxtLink>
            
            <div class="flex items-center justify-center gap-4 text-base-content/30 mt-6">
              <span class="icon-[tabler--brand-visa] size-8"></span>
              <span class="icon-[tabler--brand-mastercard] size-8"></span>
              <span class="icon-[tabler--brand-paypal] size-8"></span>
              <span class="icon-[logos--apple-pay] size-8"></span>
            </div>
          </div>
        </div>
      </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.cart.title'),
  ogTitle: t('pages.cart.title'),
})
const cartStore = useCartStore()
</script>

