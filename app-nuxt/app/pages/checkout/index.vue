<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="mb-10 pt-10">
      <h1 class="h2 gradient-text">{{ t('pages.checkout.title') }}</h1>
      <p class="text-base-content/60 mt-1">Complete your purchase by providing shipping and payment details.</p>
    </div>

    <!-- Empty State -->
    <div v-if="cartStore.isEmpty" class="flex flex-col items-center justify-center py-24 bg-base-200/30 rounded-[3rem] border-2 border-dashed border-base-200">
      <div class="size-24 rounded-full bg-base-200 flex items-center justify-center mb-6">
        <span class="icon-[tabler--shopping-cart-off] size-12 opacity-20" />
      </div>
      <h2 class="h3 mb-2">{{ t('cart.empty') }}</h2>
      <NuxtLink to="/products" class="btn btn-primary btn-lg px-10 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-105">
        {{ t('cart.continueShopping') }}
      </NuxtLink>
    </div>

    <!-- Checkout Content -->
    <div v-else class="grid grid-cols-1 lg:grid-cols-12 gap-10">
      <!-- Forms Area -->
      <div class="lg:col-span-8 space-y-10">
        <!-- Shipping Address -->
        <div class="bg-base-100 p-8 md:p-10 rounded-[2.5rem] border border-base-200 shadow-sm">
          <div class="flex items-center gap-4 mb-8">
            <div class="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary font-bold">1</div>
            <h2 class="h4">{{ t('shipping.address') }}</h2>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="form-label">{{ t('shipping.firstName') }}</label>
              <input v-model="address.firstName" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="First Name" />
            </div>
            <div class="form-control">
              <label class="form-label">{{ t('shipping.lastName') }}</label>
              <input v-model="address.lastName" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="Last Name" />
            </div>
            <div class="form-control md:col-span-2">
              <label class="form-label">{{ t('shipping.address1') }}</label>
              <input v-model="address.address1" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="Street Address, P.O. box, etc." />
            </div>
            <div class="form-control md:col-span-2">
              <label class="form-label">{{ t('shipping.address2') }} (Optional)</label>
              <input v-model="address.address2" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="Apartment, suite, unit, building, floor, etc." />
            </div>
            <div class="form-control">
              <label class="form-label">{{ t('shipping.city') }}</label>
              <input v-model="address.city" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="City" />
            </div>
            <div class="form-control">
              <label class="form-label">{{ t('shipping.state') }}</label>
              <input v-model="address.state" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="State / Province" />
            </div>
            <div class="form-control">
              <label class="form-label">{{ t('shipping.zipCode') }}</label>
              <input v-model="address.zipCode" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="ZIP / Postal Code" />
            </div>
            <div class="form-control">
              <label class="form-label">{{ t('shipping.phone') }}</label>
              <input v-model="address.phone" type="text" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" placeholder="+00 00000-0000" />
            </div>
          </div>
        </div>

        <!-- Shipping Method -->
        <div class="bg-base-100 p-8 md:p-10 rounded-[2.5rem] border border-base-200 shadow-sm">
          <div class="flex items-center gap-4 mb-8">
            <div class="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary font-bold">2</div>
            <h2 class="h4">{{ t('shipping.method') }}</h2>
          </div>
          
          <div v-if="shippingMethods.length === 0" class="flex items-center justify-center py-10 opacity-40">
            <span class="loading loading-spinner loading-md"></span>
          </div>
          <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <label v-for="method in shippingMethods" :key="method.id"
              class="relative flex p-6 rounded-2xl border-2 cursor-pointer transition-all duration-300"
              :class="selectedShippingMethod === method.id ? 'border-primary bg-primary/5 ring-4 ring-primary/10' : 'border-base-200 hover:border-base-300'">
              <input v-model="selectedShippingMethod" type="radio" name="shipping_method" :value="method.id" class="hidden" />
              <div class="grow">
                <p class="font-bold mb-1">{{ method.name || method.code }}</p>
                <p class="text-xs text-base-content/50">Delivery in 3-5 business days</p>
              </div>
              <div class="text-right">
                <p class="font-black text-primary">{{ method.base_price ? formatNumberBR(method.base_price) : t('shipping.free') }}</p>
              </div>
              <span v-if="selectedShippingMethod === method.id" class="absolute top-2 right-2 icon-[tabler--circle-check-filled] text-primary size-5"></span>
            </label>
          </div>
        </div>

        <!-- Payment Method -->
        <div class="bg-base-100 p-8 md:p-10 rounded-[2.5rem] border border-base-200 shadow-sm">
          <div class="flex items-center gap-4 mb-8">
            <div class="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary font-bold">3</div>
            <h2 class="h4">{{ t('payment.select') }}</h2>
          </div>
          
          <div v-if="paymentMethods.length === 0" class="flex items-center justify-center py-10 opacity-40">
            <span class="loading loading-spinner loading-md"></span>
          </div>
          <div v-else class="space-y-4">
            <label v-for="method in paymentMethods" :key="method.id"
              class="relative flex items-center p-6 rounded-2xl border-2 cursor-pointer transition-all duration-300"
              :class="selectedPaymentMethod === method.id ? 'border-primary bg-primary/5 ring-4 ring-primary/10' : 'border-base-200 hover:border-base-300'">
              <input v-model="selectedPaymentMethod" type="radio" name="payment_method" :value="method.id" class="hidden" />
              <div class="size-12 rounded-xl bg-white shadow-sm flex items-center justify-center mr-5 shrink-0 border border-base-200">
                <span :class="[method.code.toLowerCase().includes('card') ? 'icon-[tabler--credit-card]' : method.code.toLowerCase().includes('pix') ? 'icon-[tabler--qrcode]' : 'icon-[tabler--wallet]', 'size-6 text-primary']"></span>
              </div>
              <div class="grow font-bold text-lg">{{ method.name || method.code }}</div>
              <span v-if="selectedPaymentMethod === method.id" class="icon-[tabler--circle-check-filled] text-primary size-6"></span>
            </label>
          </div>
        </div>
      </div>

      <!-- Order Summary -->
      <div class="lg:col-span-4">
        <div class="bg-base-100 p-8 rounded-[2rem] border border-base-200 shadow-xl shadow-primary/5 sticky top-24">
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
                <p class="text-xs text-base-content/40">Qtd: {{ item.quantity }}</p>
              </div>
              <div class="font-bold text-sm">{{ formatNumberBR(item.price * item.quantity) }}</div>
            </div>
          </div>

          <!-- Coupon -->
          <div class="mb-8">
            <div class="relative group">
              <input v-model="couponCode" type="text" placeholder="Promo Code" 
                class="input bg-base-200/50 border-none rounded-2xl h-14 w-full pr-24 uppercase font-bold tracking-widest placeholder:normal-case placeholder:font-normal placeholder:tracking-normal"
                :disabled="couponApplied || couponChecking" @keyup.enter="applyCoupon" />
              <button v-if="!couponApplied" @click="applyCoupon" :disabled="!couponCode.trim() || couponChecking"
                class="btn btn-primary btn-sm absolute right-2 top-2 h-10 px-4 rounded-xl">
                <span v-if="couponChecking" class="loading loading-spinner loading-xs"></span>
                <span v-else>Apply</span>
              </button>
              <button v-else @click="removeCoupon" class="btn btn-ghost btn-circle btn-sm absolute right-2 top-2 h-10 w-10 text-error">
                <span class="icon-[tabler--x] size-5"></span>
              </button>
            </div>
            <p v-if="couponMessage" :class="['mt-2 text-xs font-medium px-2', couponApplied ? 'text-success' : 'text-error']">
              <span :class="[couponApplied ? 'icon-[tabler--circle-check]' : 'icon-[tabler--alert-circle]', 'size-3.5 inline-block align-middle mr-1']"></span>
              {{ couponMessage }}
            </p>
          </div>
          
          <!-- Pricing -->
          <div class="space-y-4 mb-8">
            <div class="flex justify-between text-base-content/60">
              <span>Subtotal</span>
              <span>{{ formatNumberBR(cartStore.totalPrice) }}</span>
            </div>
            <div v-if="selectedShippingCost" class="flex justify-between text-base-content/60">
              <span>{{ t('shipping.shipping') }}</span>
              <span>{{ formatNumberBR(selectedShippingCost) }}</span>
            </div>
            <div v-if="couponDiscount" class="flex justify-between text-success font-medium">
              <span>Discount</span>
              <span>-{{ formatNumberBR(couponDiscount) }}</span>
            </div>
          </div>
          
          <div class="pt-6 border-t border-base-200 mb-8">
            <div class="flex justify-between items-end">
              <span class="font-bold text-lg text-base-content/60">Total</span>
              <div class="text-right">
                <span class="block text-3xl font-black text-primary">{{ formatNumberBR(totalAmount) }}</span>
                <span class="text-[10px] text-base-content/40 uppercase tracking-widest">Final Price</span>
              </div>
            </div>
          </div>
          
          <div class="space-y-4">
            <button class="btn btn-primary btn-lg w-full h-16 rounded-2xl shadow-xl shadow-primary/20 text-lg transition-transform hover:scale-[1.02]"
              :disabled="submitting || !selectedPaymentMethod" @click="placeOrder">
              <span v-if="submitting" class="loading loading-spinner mr-2" />
              <span v-else class="icon-[tabler--lock] size-5 mr-2"></span>
              {{ t('pages.checkout.placeOrder') }}
            </button>
            
            <p v-if="error" class="text-center text-xs text-error font-medium px-4">
              {{ error }}
            </p>

            <p class="text-center text-[10px] text-base-content/30 uppercase tracking-widest flex items-center justify-center gap-2 mt-6">
              <span class="icon-[tabler--shield-lock] size-4"></span>
              Secure 256-bit SSL Encrypted Payment
            </p>
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
    couponMessage.value = 'Erro ao validar cupom'
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
    error.value = err?.data?.message || err?.message || 'Error placing order'
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
