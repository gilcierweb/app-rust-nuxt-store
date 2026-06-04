<template>
  <Teleport to="body">
    <Transition name="cart-sidebar">
      <div v-if="isCartOpen" class="fixed inset-0 z-50 flex">
        <div class="fixed inset-0 bg-black/50 transition-opacity" @click="closeCart" />
        <div class="relative ml-auto flex h-full w-full max-w-md flex-col bg-base-100 shadow-xl">
          <div class="flex items-center justify-between border-b px-6 py-4">
            <h2 class="text-xl font-semibold">{{ t('cart.yourCart') }}</h2>
            <button class="btn btn-ghost btn-square btn-sm" @click="closeCart">
              <span class="icon-[tabler--x] size-5" />
            </button>
          </div>

          <div v-if="cartStore.isEmpty" class="flex flex-1 items-center justify-center">
            <div class="text-center">
              <span class="icon-[tabler--shopping-cart-off] mx-auto mb-4 size-16 opacity-40" />
              <p class="text-base-content/60">{{ t('cart.empty') }}</p>
            </div>
          </div>

          <div v-else class="flex-1 overflow-y-auto px-6 py-4">
            <div v-for="item in cartStore.items" :key="item.id" class="mb-4 flex items-center gap-4 rounded-lg border p-3">
              <NuxtImg v-if="item.image" :src="item.image" class="size-16 rounded object-cover" :alt="item.name" />
              <div class="flex-1 min-w-0">
                <p class="truncate font-medium">{{ item.name }}</p>
                <p class="text-sm text-base-content/60">{{ formatNumberBR(item.price) }}</p>
              </div>
              <div class="flex items-center gap-1">
                <button class="btn btn-ghost btn-square btn-xs" @click="updateQuantitySync(item.productId, item.quantity - 1)">
                  <span class="icon-[tabler--minus] size-4" />
                </button>
                <span class="w-8 text-center text-sm">{{ item.quantity }}</span>
                <button class="btn btn-ghost btn-square btn-xs" @click="updateQuantitySync(item.productId, item.quantity + 1)">
                  <span class="icon-[tabler--plus] size-4" />
                </button>
              </div>
              <button class="btn btn-ghost btn-square btn-xs text-error" @click="removeItemSync(item.productId)">
                <span class="icon-[tabler--trash] size-4" />
              </button>
            </div>
          </div>

          <div v-if="!cartStore.isEmpty" class="border-t px-6 py-4">
            <div class="mb-4 flex items-center justify-between">
              <span class="font-semibold">{{ t('cart.subtotal') }}</span>
              <span class="text-xl font-bold text-primary">{{ formatNumberBR(cartStore.totalPrice) }}</span>
            </div>
            <NuxtLinkLocale to="/cart" class="btn btn-primary w-full" @click="closeCart">
              {{ t('cart.viewCart') }}
            </NuxtLinkLocale>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
const { t } = useI18n()
const cartStore = useCartStore()
const { isCartOpen, closeCart } = useCartUI()
const { removeItemSync, updateQuantitySync } = useCartSync()
</script>

<style scoped>
.cart-sidebar-enter-active,
.cart-sidebar-leave-active {
  transition: opacity 0.2s ease;
}
.cart-sidebar-enter-from,
.cart-sidebar-leave-to {
  opacity: 0;
}
</style>
