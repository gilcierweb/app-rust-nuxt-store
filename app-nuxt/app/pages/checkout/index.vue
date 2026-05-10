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
      <div class="lg:col-span-2 space-y-6">
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

        <div class="rounded-box border p-6">
          <h3 class="mb-4 text-lg font-semibold">{{ t('shipping.address') }}</h3>
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div class="form-control">
              <label class="label"><span class="label-text">{{ t('shipping.firstName') }}</span></label>
              <input v-model="address.firstName" type="text" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">{{ t('shipping.lastName') }}</span></label>
              <input v-model="address.lastName" type="text" class="input input-bordered" />
            </div>
            <div class="form-control md:col-span-2">
              <label class="label"><span class="label-text">{{ t('shipping.address1') }}</span></label>
              <input v-model="address.address1" type="text" class="input input-bordered" />
            </div>
            <div class="form-control md:col-span-2">
              <label class="label"><span class="label-text">{{ t('shipping.address2') }}</span></label>
              <input v-model="address.address2" type="text" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">{{ t('shipping.city') }}</span></label>
              <input v-model="address.city" type="text" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">{{ t('shipping.state') }}</span></label>
              <input v-model="address.state" type="text" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">{{ t('shipping.zipCode') }}</span></label>
              <input v-model="address.zipCode" type="text" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">{{ t('shipping.phone') }}</span></label>
              <input v-model="address.phone" type="text" class="input input-bordered" />
            </div>
          </div>
        </div>

        <div class="rounded-box border p-6">
          <h3 class="mb-4 text-lg font-semibold">{{ t('shipping.method') }}</h3>
          <div v-if="shippingMethods.length === 0" class="text-sm text-base-content/60">
            {{ t('common.loading') }}
          </div>
          <div v-else class="space-y-3">
            <label
              v-for="method in shippingMethods"
              :key="method.id"
              class="flex items-center gap-3 rounded-lg border p-4 cursor-pointer has-[:checked]:border-primary has-[:checked]:bg-primary/5"
            >
              <input
                v-model="selectedShippingMethod"
                type="radio"
                name="shipping_method"
                :value="method.id"
                class="radio radio-primary"
              />
              <div class="flex flex-1 items-center justify-between">
                <div>
                  <p class="font-medium">{{ method.name || method.code }}</p>
                </div>
                <p class="font-semibold text-primary">{{ method.base_price ? formatNumberBR(method.base_price) : t('shipping.free') }}</p>
              </div>
            </label>
          </div>
        </div>

        <div class="rounded-box border p-6">
          <h3 class="mb-4 text-lg font-semibold">{{ t('payment.select') }}</h3>
          <div v-if="paymentMethods.length === 0" class="text-sm text-base-content/60">
            {{ t('common.loading') }}
          </div>
          <div v-else class="space-y-3">
            <label
              v-for="method in paymentMethods"
              :key="method.id"
              class="flex items-center gap-3 rounded-lg border p-4 cursor-pointer has-[:checked]:border-primary has-[:checked]:bg-primary/5"
            >
              <input
                v-model="selectedPaymentMethod"
                type="radio"
                name="payment_method"
                :value="method.id"
                class="radio radio-primary"
              />
              <div>
                <p class="font-medium">{{ method.name || method.code }}</p>
              </div>
            </label>
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
            <div v-if="selectedShippingCost" class="flex justify-between">
              <span>{{ t('shipping.shipping') }}</span>
              <span>{{ formatNumberBR(selectedShippingCost) }}</span>
            </div>
            <div class="flex justify-between border-t pt-3 text-lg font-bold">
              <span>{{ t('cart.total') }}</span>
              <span class="text-primary">{{ formatNumberBR(cartStore.totalPrice + (selectedShippingCost || 0)) }}</span>
            </div>
          </div>
          <button class="btn btn-primary mt-6 w-full" :disabled="submitting || !selectedPaymentMethod" @click="placeOrder">
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
import type { PaymentMethod, ShippingMethod } from '~/types'

const submitting = ref(false)
const error = ref('')
const selectedPaymentMethod = ref<number | null>(null)
const paymentMethods = ref<PaymentMethod[]>([])
const selectedShippingMethod = ref<number | null>(null)
const shippingMethods = ref<ShippingMethod[]>([])
const address = reactive({
  firstName: '',
  lastName: '',
  address1: '',
  address2: '',
  city: '',
  state: '',
  zipCode: '',
  phone: '',
})

const selectedShippingCost = computed(() => {
  if (!selectedShippingMethod.value) return null
  const method = shippingMethods.value.find(m => m.id === selectedShippingMethod.value)
  return method?.base_price ?? null
})

onMounted(async () => {
  const [pm, sm] = await Promise.all([
    $fetch(`${config.public.baseURL}/api/payments/methods`).catch(() => []),
    $fetch(`${config.public.baseURL}/api/shippings`).catch(() => []),
  ])
  paymentMethods.value = pm
  shippingMethods.value = sm
  if (paymentMethods.value.length > 0) {
    selectedPaymentMethod.value = paymentMethods.value[0].id
  }
  if (shippingMethods.value.length > 0) {
    selectedShippingMethod.value = shippingMethods.value[0].id
  }
})

async function placeOrder() {
  if (cartStore.isEmpty || !selectedPaymentMethod.value) return
  submitting.value = true
  error.value = ''

  const items = cartStore.items.map(item => ({
    product_id: item.productId,
    quantity: item.quantity,
    price: item.price,
  }))

  const shippingCost = selectedShippingCost || 0

  try {
    const data = await $fetch(`${config.public.baseURL}/api/orders/checkout`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: {
        items,
        subtotal: cartStore.totalPrice,
        total_amount: cartStore.totalPrice + shippingCost,
        shipping_amount: shippingCost,
        discount_amount: 0,
        payment_method_id: selectedPaymentMethod.value,
        shipping_method_id: selectedShippingMethod.value,
        address_first_name: address.firstName || null,
        address_last_name: address.lastName || null,
        address1: address.address1 || null,
        address2: address.address2 || null,
        address_city: address.city || null,
        address_state: address.state || null,
        address_zip_code: address.zipCode || null,
        address_phone: address.phone || null,
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
