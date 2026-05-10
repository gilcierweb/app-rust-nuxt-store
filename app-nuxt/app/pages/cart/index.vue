<template>
  <div>
    <h1 class="h1 my-4">{{ t('pages.cart.title') }}</h1>

    <div v-if="cartStore.isEmpty" class="flex flex-col items-center justify-center py-20">
      <span class="icon-[tabler--shopping-cart-off] mb-4 size-20 opacity-40" />
      <p class="mb-6 text-lg text-base-content/60">{{ t('cart.empty') }}</p>
      <NuxtLink to="/products" class="btn btn-primary">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <div v-else class="grid grid-cols-1 gap-8 lg:grid-cols-3">
      <div class="lg:col-span-2">
        <div class="overflow-x-auto rounded-box border">
          <table class="table">
            <thead>
              <tr>
                <th>{{ t('pages.cart.item') }}</th>
                <th>{{ t('cart.quantity') }}</th>
                <th class="text-right">{{ t('cart.total') }}</th>
                <th />
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in cartStore.items" :key="item.id">
                <td>
                  <div class="flex items-center gap-3">
                    <NuxtImg v-if="item.image" :src="item.image" class="size-20 rounded object-cover" :alt="item.name" />
                    <div>
                      <NuxtLink v-if="item.slug" :to="`/products/${item.slug || item.productId}`" class="font-medium link link-hover">
                        {{ item.name }}
                      </NuxtLink>
                      <p v-else class="font-medium">{{ item.name }}</p>
                      <p class="text-sm text-base-content/60">{{ formatNumberBR(item.price) }}</p>
                    </div>
                  </div>
                </td>
                <td>
                  <div class="flex items-center gap-1">
                    <button class="btn btn-ghost btn-square btn-sm" @click="cartStore.updateQuantity(item.productId, item.quantity - 1)">
                      <span class="icon-[tabler--minus] size-4" />
                    </button>
                    <input type="number" :value="item.quantity" min="1" class="input input-sm w-16 text-center"
                      @change="cartStore.updateQuantity(item.productId, Number(($event.target as HTMLInputElement).value))" />
                    <button class="btn btn-ghost btn-square btn-sm" @click="cartStore.updateQuantity(item.productId, item.quantity + 1)">
                      <span class="icon-[tabler--plus] size-4" />
                    </button>
                  </div>
                </td>
                <td class="text-right font-semibold">
                  {{ formatNumberBR(item.price * item.quantity) }}
                </td>
                <td>
                  <button class="btn btn-ghost btn-square btn-sm text-error" @click="cartStore.removeItem(item.productId)">
                    <span class="icon-[tabler--trash] size-5" />
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="mt-4 flex items-center justify-between">
          <NuxtLink to="/products" class="btn btn-ghost">
            <span class="icon-[tabler--arrow-left] size-4" />
            {{ t('cart.continueShopping') }}
          </NuxtLink>
          <button class="btn btn-ghost text-error" @click="cartStore.clearCart()">
            <span class="icon-[tabler--trash] size-4" />
            {{ t('cart.clearCart') }}
          </button>
        </div>
      </div>

      <div class="rounded-box border p-6">
        <h3 class="mb-4 text-lg font-semibold">{{ t('cart.summary') }}</h3>
        <div class="space-y-3">
          <div v-for="item in cartStore.items" :key="item.id" class="flex justify-between text-sm">
            <span class="truncate">{{ item.name }} x{{ item.quantity }}</span>
            <span>{{ formatNumberBR(item.price * item.quantity) }}</span>
          </div>
        </div>
        <div class="my-4 border-t pt-4">
          <div class="flex justify-between text-lg font-bold">
            <span>{{ t('cart.cartTotal') }}</span>
            <span class="text-primary">{{ formatNumberBR(cartStore.totalPrice) }}</span>
          </div>
        </div>
        <NuxtLink to="/checkout" class="btn btn-primary w-full">
          {{ t('cart.checkout') }}
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
const cartStore = useCartStore()
</script>

<style scoped>
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
input[type="number"] {
  -moz-appearance: textfield;
}
</style>
