<template>
  <div>
    <h1 class="h1 my-4">{{ t('pages.checkout.title') }}</h1>

    <div v-if="cartStore.isEmpty" class="flex flex-col items-center justify-center py-20">
      <span class="icon-[tabler--shopping-cart-off] mb-4 size-20 opacity-40" />
      <p class="mb-6 text-lg text-base-content/60">{{ t('cart.empty') }}</p>
      <NuxtLink to="/products" class="btn btn-primary">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <div v-else class="grid grid-cols-1 gap-8 lg:grid-cols-3">
      <div class="lg:col-span-2">
        <div class="rounded-box border p-6">
          <h3 class="mb-4 text-lg font-semibold">{{ t('cart.summary') }}</h3>
          <div class="space-y-4">
            <div v-for="item in cartStore.items" :key="item.id" class="flex items-center gap-4">
              <NuxtImg v-if="item.image" :src="item.image" class="size-16 rounded object-cover" :alt="item.name" />
              <div class="flex-1 min-w-0">
                <p class="font-medium truncate">{{ item.name }}</p>
                <p class="text-sm text-base-content/60">{{ t('cart.quantity') }}: {{ item.quantity }}</p>
              </div>
              <p class="font-semibold">{{ formatNumberBR(item.price * item.quantity) }}</p>
            </div>
          </div>
        </div>
      </div>

      <div>
        <div class="rounded-box border p-6">
          <h3 class="mb-4 text-lg font-semibold">{{ t('pages.checkout.orderSummary') }}</h3>
          <div class="space-y-3">
            <div class="flex justify-between">
              <span>{{ t('cart.subtotal') }}</span>
              <span>{{ formatNumberBR(cartStore.totalPrice) }}</span>
            </div>
            <div class="flex justify-between border-t pt-3 text-lg font-bold">
              <span>{{ t('cart.total') }}</span>
              <span class="text-primary">{{ formatNumberBR(cartStore.totalPrice) }}</span>
            </div>
          </div>
          <button class="btn btn-primary mt-6 w-full" :disabled="submitting" @click="placeOrder">
            <span v-if="submitting" class="loading loading-spinner" />
            {{ t('pages.checkout.placeOrder') }}
          </button>
          <p v-if="error" class="mt-2 text-center text-sm text-error">{{ error }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
const cartStore = useCartStore()
const router = useRouter()
const config = useRuntimeConfig()
const submitting = ref(false)
const error = ref('')

async function placeOrder() {
  if (cartStore.isEmpty) return
  submitting.value = true
  error.value = ''

  const items = cartStore.items.map(item => ({
    product_id: item.productId,
    quantity: item.quantity,
    price: item.price,
  }))

  try {
    const data = await $fetch(`${config.public.baseURL}/api/orders/checkout`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: {
        items,
        subtotal: cartStore.totalPrice,
        total_amount: cartStore.totalPrice,
        shipping_amount: 0,
        discount_amount: 0,
      },
    })

    cartStore.clearCart()
    router.push(`/orders/confirmation/${data.id}`)
  } catch (err: any) {
    error.value = err?.data?.message || err?.message || 'Error placing order'
  } finally {
    submitting.value = false
  }
}
</script>
