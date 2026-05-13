<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="mb-10 pt-10">
      <h1 class="h2 gradient-text">{{ t('pages.checkout.title') }}</h1>
      <p class="text-base-content/60 mt-1">{{ t('pages.checkout.description') }}</p>
    </div>

    <!-- Empty State -->
    <div v-if="cartStore.isEmpty" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="alert alert-warning max-w-md">
        <div class="flex items-center gap-4">
          <div class="size-16 rounded-full bg-warning/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--shopping-cart-off] size-8 text-warning" />
          </div>
          <div>
            <h2 class="font-bold text-lg">{{ t('cart.empty') }}</h2>
            <p class="text-sm opacity-80 mt-1">{{ t('pages.checkout.emptyDescription') }}</p>
          </div>
        </div>
      </div>
      <NuxtLink to="/products" class="btn btn-primary btn-lg mt-8 shadow-md">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <!-- Checkout Content -->
    <div v-else class="grid grid-cols-1 lg:grid-cols-12 gap-10">
      <!-- Forms Area -->
      <div class="lg:col-span-8 space-y-10">
        <!-- Shipping Address -->
        <div class="card bg-base-100 shadow-soft border border-base-200">
          <div class="card-body p-8 md:p-10">
          <div class="flex items-center gap-4 mb-8">
            <div class="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary font-bold">1</div>
            <h2 class="h4">{{ t('pages.checkout.step1') }}</h2>
          </div>
          
          <div class="space-y-4 w-full">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="w-full">
                <label class="label-text" for="firstName">{{ t('shipping.firstName') }}</label>
                <input v-model="address.firstName" type="text" class="input" id="firstName" :placeholder="t('shipping.firstName')" />
              </div>
              <div class="w-full">
                <label class="label-text" for="lastName">{{ t('shipping.lastName') }}</label>
                <input v-model="address.lastName" type="text" class="input" id="lastName" :placeholder="t('shipping.lastName')" />
              </div>
            </div>
            <div class="w-full">
              <label class="label-text" for="address1">{{ t('shipping.address1') }}</label>
              <input v-model="address.address1" type="text" class="input" id="address1" :placeholder="t('shipping.address1')" />
            </div>
            <div class="w-full">
              <label class="label-text" for="address2">{{ t('shipping.address2') }} ({{ t('common.optional') }})</label>
              <input v-model="address.address2" type="text" class="input" id="address2" :placeholder="t('shipping.address2')" />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="w-full">
                <label class="label-text" for="city">{{ t('shipping.city') }}</label>
                <input v-model="address.city" type="text" class="input" id="city" :placeholder="t('shipping.city')" />
              </div>
              <div class="w-full">
                <label class="label-text" for="state">{{ t('shipping.state') }}</label>
                <input v-model="address.state" type="text" class="input" id="state" :placeholder="t('shipping.state')" />
              </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="w-full">
                <label class="label-text" for="zipCode">{{ t('shipping.zipCode') }}</label>
                <input v-model="address.zipCode" type="text" class="input" id="zipCode" :placeholder="t('shipping.zipCode')" />
              </div>
              <div class="w-full">
                <label class="label-text" for="phone">{{ t('shipping.phone') }}</label>
                <input v-model="address.phone" type="text" class="input" id="phone" :placeholder="t('shipping.phone')" />
              </div>
            </div>
          </div>
          </div>
        </div>

        <!-- Shipping Method -->
        <div class="card bg-base-100 shadow-soft border border-base-200">
          <div class="card-body p-8 md:p-10">
          <div class="flex items-center gap-4 mb-8">
            <div class="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary font-bold">2</div>
            <h2 class="h4">{{ t('pages.checkout.step2') }}</h2>
          </div>
          
          <div v-if="shippingMethods.length === 0" class="flex items-center justify-center py-10">
            <div class="loading loading-spinner loading-md"></div>
            <span class="ml-3 text-base-content/60">{{ t('pages.checkout.loadingShipping') }}</span>
          </div>
          <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div v-for="method in shippingMethods" :key="method.id"
              class="flex items-center gap-3 p-4 rounded-xl border-2 transition-all duration-300 cursor-pointer hover:border-primary/50"
              :class="selectedShippingMethod === method.id ? 'border-primary bg-primary/5 shadow-sm' : 'border-base-200'"
              @click="selectedShippingMethod = method.id">
              <input v-model="selectedShippingMethod" type="radio" name="shipping_method" :value="method.id" class="radio radio-primary" :id="`shipping-${method.id}`" />
              <label class="label-text text-base grow cursor-pointer" :for="`shipping-${method.id}`">
                <div class="font-bold">{{ method.name || method.code }}</div>
                <div class="text-xs text-base-content/50">{{ t('shipping.deliveryTime') }}</div>
              </label>
              <div class="font-black text-primary">{{ method.base_price ? formatNumberBR(method.base_price) : t('shipping.free') }}</div>
            </div>
          </div>
          </div>
        </div>

        <!-- Payment Method -->
        <div class="card bg-base-100 shadow-soft border border-base-200">
          <div class="card-body p-8 md:p-10">
          <div class="flex items-center gap-4 mb-8">
            <div class="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary font-bold">3</div>
            <h2 class="h4">{{ t('pages.checkout.step3') }}</h2>
          </div>
          
          <div v-if="paymentMethods.length === 0" class="flex items-center justify-center py-10">
            <div class="loading loading-spinner loading-md"></div>
            <span class="ml-3 text-base-content/60">{{ t('pages.checkout.loadingPayment') }}</span>
          </div>
          <div v-else class="space-y-4">
            <div v-for="method in paymentMethods" :key="method.id"
              class="flex items-center gap-3 p-4 rounded-xl border-2 transition-all duration-300 cursor-pointer hover:border-primary/50"
              :class="selectedPaymentMethod === method.id ? 'border-primary bg-primary/5 shadow-sm' : 'border-base-200'"
              @click="selectedPaymentMethod = method.id">
              <input v-model="selectedPaymentMethod" type="radio" name="payment_method" :value="method.id" class="radio radio-primary" :id="`payment-${method.id}`" />
              <label class="label-text text-base grow cursor-pointer flex items-center gap-4" :for="`payment-${method.id}`">
                <div class="size-10 rounded-xl bg-white shadow-sm flex items-center justify-center border border-base-200">
                  <span :class="[method.code.toLowerCase().includes('card') ? 'icon-[tabler--credit-card]' : method.code.toLowerCase().includes('pix') ? 'icon-[tabler--qrcode]' : 'icon-[tabler--wallet]', 'size-5 text-primary']"></span>
                </div>
                <div class="font-bold text-lg">{{ method.name || method.code }}</div>
              </label>
            </div>
          </div>
          </div>
        </div>
      </div>

      <!-- Order Summary -->
      <div class="lg:col-span-4 mt-8 lg:mt-0">
        <div class="card bg-base-100 shadow-xl border border-base-200 sticky top-24">
          <div class="card-body p-8">
          <h3 class="h4 mb-8">{{ t('pages.checkout.orderSummary') }}</h3>
          
          <!-- Items Mini-List -->
          <div class="space-y-4 mb-8 max-h-60 overflow-y-auto pr-2 custom-scrollbar">
            <div v-for="item in cartStore.items" :key="item.productId" class="flex items-center gap-4 p-3 rounded-2xl bg-base-200/30">
              <div class="size-14 rounded-xl overflow-hidden bg-white shrink-0 border border-base-200">
                <NuxtImg v-if="item.image" :src="item.image" class="size-full object-cover" :alt="item.name" />
                <div v-else class="flex items-center justify-center h-full text-base-content/20"><span class="icon-[tabler--photo] size-6"></span></div>
              </div>
              <div class="grow min-w-0">
                <p class="font-bold text-sm line-clamp-1">{{ item.name }}</p>
                <p class="text-xs text-base-content/40">{{ t('cart.quantity') }}: {{ item.quantity }}</p>
              </div>
              <div class="font-bold text-sm">{{ formatNumberBR(item.price * item.quantity) }}</div>
            </div>
          </div>

          <!-- Coupon -->
          <div class="mb-8">
            <div class="alert alert-info mb-4">
              <div class="flex items-center gap-2">
                <span class="icon-[tabler--discount-2] size-5"></span>
                <span class="text-sm">{{ t('pages.checkout.promoInfo') }}</span>
              </div>
            </div>
            <div class="w-full">
              <label class="label-text" for="couponCode">{{ t('pages.checkout.promoCode') }}</label>
              <div class="flex gap-2 mt-1">
                <input v-model="couponCode" type="text" :placeholder="t('pages.checkout.promoCode')" id="couponCode"
                  class="input grow uppercase font-bold tracking-widest placeholder:normal-case placeholder:font-normal placeholder:tracking-normal"
                  :disabled="couponApplied || couponChecking" @keyup.enter="applyCoupon" />
                <button v-if="!couponApplied" @click="applyCoupon" :disabled="!couponCode.trim() || couponChecking"
                  class="btn btn-primary px-6">
                  <span v-if="couponChecking" class="loading loading-spinner loading-xs"></span>
                  <span v-else>{{ t('pages.checkout.apply') }}</span>
                </button>
                <button v-else @click="removeCoupon" class="btn btn-outline btn-error hover:bg-error hover:text-error-content">
                  <span class="icon-[tabler--x] size-5"></span>
                </button>
              </div>
            </div>
            <p v-if="couponMessage" :class="['mt-2 text-xs font-medium px-2', couponApplied ? 'text-success' : 'text-error']">
              <span :class="[couponApplied ? 'icon-[tabler--circle-check]' : 'icon-[tabler--alert-circle]', 'size-3.5 inline-block align-middle mr-1']"></span>
              {{ couponMessage }}
            </p>
          </div>
          
          <!-- Pricing -->
          <div class="space-y-4 mb-8">
            <div class="flex justify-between text-base-content/60">
              <span>{{ t('cart.subtotal') }}</span>
              <span>{{ formatNumberBR(cartStore.totalPrice) }}</span>
            </div>
            <div v-if="selectedShippingCost" class="flex justify-between items-center gap-2">
              <span>{{ t('shipping.title') }}</span>
              <span class="badge badge-info badge-sm">{{ formatNumberBR(selectedShippingCost) }}</span>
            </div>
            <div v-if="couponDiscount" class="flex justify-between items-center gap-2">
              <span>{{ t('pages.checkout.discount') }}</span>
              <span class="badge badge-success badge-sm">-{{ formatNumberBR(couponDiscount) }}</span>
            </div>
          </div>
          
          <div class="pt-6 border-t border-base-200 mb-8">
            <div class="alert alert-success mb-4">
              <div class="flex items-center gap-2">
                <span class="icon-[tabler--shield-check] size-5"></span>
                <span class="text-sm">{{ t('pages.checkout.secureInfo') }}</span>
              </div>
            </div>
            <div class="flex justify-between items-end">
              <span class="font-bold text-lg text-base-content/60">{{ t('cart.total') }}</span>
              <div class="text-right">
                <span class="block text-3xl font-black text-primary">{{ formatNumberBR(totalAmount) }}</span>
                <span class="text-[10px] text-base-content/40 uppercase tracking-widest">{{ t('orders.finalPrice') }}</span>
              </div>
            </div>
          </div>
          
          <div class="space-y-4">
            <button class="btn btn-primary btn-lg w-full shadow-lg hover:shadow-primary/20 transition-all duration-300"
              :disabled="submitting || !selectedPaymentMethod" @click="placeOrder">
              <span v-if="submitting" class="loading loading-spinner mr-2" />
              <span v-else class="icon-[tabler--lock] size-6 mr-2"></span>
              {{ t('pages.checkout.placeOrder') }}
            </button>
            
            <p v-if="error" class="text-center text-xs text-error font-medium px-4">
              {{ error }}
            </p>

            <p class="text-center text-[10px] text-base-content/30 uppercase tracking-widest flex items-center justify-center gap-2 mt-6">
              <span class="icon-[tabler--shield-lock] size-4"></span>
              {{ t('pages.checkout.securePayment') }}
            </p>
          </div>
          </div>
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
const couponCode = ref('')
const couponDiscount = ref<number | null>(null)
const couponMessage = ref('')
const couponApplied = ref(false)
const couponChecking = ref(false)
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

const totalAmount = computed(() => {
  const subtotal = cartStore.totalPrice
  const shipping = selectedShippingCost.value || 0
  const discount = couponDiscount.value || 0
  return subtotal + shipping - discount
})

async function applyCoupon() {
  if (!couponCode.value.trim() || couponApplied.value) return
  couponChecking.value = true
  couponMessage.value = ''
  try {
    const result = await $fetch<any>(`${config.public.baseURL}/api/coupons/validate`, {
      method: 'POST',
      body: {
        code: couponCode.value.trim().toUpperCase(),
        total_amount: cartStore.totalPrice + (selectedShippingCost.value || 0)
      }
    })
    if (result.valid) {
      couponDiscount.value = result.discount
      couponApplied.value = true
      couponMessage.value = result.message
    } else {
      couponDiscount.value = null
      couponApplied.value = false
      couponMessage.value = result.message
    }
  } catch {
    couponMessage.value = t('product.reviewError')
  } finally {
    couponChecking.value = false
  }
}

function removeCoupon() {
  couponCode.value = ''
  couponDiscount.value = null
  couponApplied.value = false
  couponMessage.value = ''
}

onMounted(async () => {
  const [pm, sm] = await Promise.all([
    $fetch<PaymentMethod[]>(`${config.public.baseURL}/api/payments/methods`).catch(() => []),
    $fetch<ShippingMethod[]>(`${config.public.baseURL}/api/shippings`).catch(() => []),
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

  const shippingCost = selectedShippingCost.value || 0
  const discount = couponDiscount.value || 0

  try {
    const data = await $fetch<any>(`${config.public.baseURL}/api/orders/checkout`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: {
        items,
        subtotal: cartStore.totalPrice,
        total_amount: cartStore.totalPrice + shippingCost - discount,
        shipping_amount: shippingCost,
        discount_amount: discount || null,
        coupon_code: couponApplied.value ? couponCode.value.trim().toUpperCase() : null,
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
    error.value = err?.data?.message || err?.message || t('pages.products.edit.error', { message: '' })
  } finally {
    submitting.value = false
  }
}
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: oklch(var(--b3));
  border-radius: 10px;
}
</style>
