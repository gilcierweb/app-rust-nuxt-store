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
      <NuxtLinkLocale to="/products" class="btn btn-primary btn-lg mt-8 shadow-md">
        {{ t('cart.continueShopping') }}
      </NuxtLinkLocale>
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
                <input v-model="firstName" @blur="firstNameBlur" type="text" class="input" :class="{ 'input-error': firstNameError }" id="firstName" :placeholder="t('shipping.firstName')" />
                <p v-if="firstNameError" class="text-xs text-error font-semibold mt-1">{{ firstNameError }}</p>
              </div>
              <div class="w-full">
                <label class="label-text" for="lastName">{{ t('shipping.lastName') }}</label>
                <input v-model="lastName" @blur="lastNameBlur" type="text" class="input" :class="{ 'input-error': lastNameError }" id="lastName" :placeholder="t('shipping.lastName')" />
                <p v-if="lastNameError" class="text-xs text-error font-semibold mt-1">{{ lastNameError }}</p>
              </div>
            </div>
            <div class="w-full">
              <label class="label-text" for="address1">{{ t('shipping.address1') }}</label>
              <input v-model="address1" @blur="address1Blur" type="text" class="input" :class="{ 'input-error': address1Error }" id="address1" :placeholder="t('shipping.address1')" />
              <p v-if="address1Error" class="text-xs text-error font-semibold mt-1">{{ address1Error }}</p>
            </div>
            <div class="w-full">
              <label class="label-text" for="address2">{{ t('shipping.address2') }} ({{ t('common.optional') }})</label>
              <input v-model="address2" type="text" class="input" id="address2" :placeholder="t('shipping.address2')" />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="w-full">
                <label class="label-text" for="city">{{ t('shipping.city') }}</label>
                <input v-model="city" @blur="cityBlur" type="text" class="input" :class="{ 'input-error': cityError }" id="city" :placeholder="t('shipping.city')" />
                <p v-if="cityError" class="text-xs text-error font-semibold mt-1">{{ cityError }}</p>
              </div>
              <div class="w-full">
                <label class="label-text" for="state">{{ t('shipping.state') }}</label>
                <input v-model="state" @blur="stateBlur" type="text" class="input" :class="{ 'input-error': stateError }" id="state" :placeholder="t('shipping.state')" />
                <p v-if="stateError" class="text-xs text-error font-semibold mt-1">{{ stateError }}</p>
              </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="w-full">
                <label class="label-text" for="zipCode">{{ t('shipping.zipCode') }}</label>
                <input v-model="zipCode" @blur="zipCodeBlur" type="text" class="input" :class="{ 'input-error': zipCodeError }" id="zipCode" :placeholder="t('shipping.zipCode')" />
                <p v-if="zipCodeError" class="text-xs text-error font-semibold mt-1">{{ zipCodeError }}</p>
              </div>
              <div class="w-full">
                <label class="label-text" for="phone">{{ t('shipping.phone') }}</label>
                <input v-model="phone" @blur="phoneBlur" type="text" class="input" :class="{ 'input-error': phoneError }" id="phone" :placeholder="t('shipping.phone')" />
                <p v-if="phoneError" class="text-xs text-error font-semibold mt-1">{{ phoneError }}</p>
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
                  <span :class="[paymentMethodIcon(method), 'size-5 text-primary']"></span>
                </div>
                <div class="font-bold text-lg">{{ method.name || method.code }}</div>
              </label>
            </div>
            <div v-if="selectedGatewayDriver === 'braintree'" class="mt-4">
              <div id="braintree-dropin" class="rounded-box border border-base-200 bg-base-100 p-4" />
              <div v-if="braintreeLoading" class="mt-3 flex items-center gap-2 text-sm text-base-content/60">
                <span class="loading loading-spinner loading-xs" />
                <span>{{ t('pages.checkout.loadingPayment') }}</span>
              </div>
            </div>
            <div v-else-if="selectedGatewayDriver === 'getnet'" class="mt-4">
              <div class="rounded-box border border-base-200 bg-base-100 p-4 space-y-4">
                <div class="alert alert-info shadow-sm">
                  <div class="flex items-center gap-2">
                    <span class="icon-[tabler--external-link] size-5"></span>
                    <span class="text-sm text-left">{{ t('pages.checkout.getnetRedirectInfo', 'You will be securely redirected to Getnet Web Checkout to complete your payment.') }}</span>
                  </div>
                </div>
                <div>
                  <label class="label-text" for="getnetDocument">{{ t('shipping.document', 'Document (CPF/CNPJ)') }}</label>
                  <input v-model="getnetDocument" @blur="getnetDocumentBlur" type="text" class="input mt-1 w-full" :class="{ 'input-error': getnetDocumentError }" id="getnetDocument" placeholder="000.000.000-00" />
                  <p v-if="getnetDocumentError" class="text-xs text-error font-semibold mt-1">{{ getnetDocumentError }}</p>
                </div>
              </div>
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
            <div v-if="stripeClientSecret" class="space-y-4 rounded-box border border-base-200 p-4">
              <div id="stripe-payment-element" class="min-h-24" />
              <button class="btn btn-primary btn-lg w-full shadow-lg hover:shadow-primary/20 transition-all duration-300"
                :disabled="confirmingStripe || !stripeReady" @click="confirmStripePayment">
                <span v-if="confirmingStripe" class="loading loading-spinner mr-2" />
                <span v-else class="icon-[tabler--credit-card-pay] size-6 mr-2"></span>
                {{ t('pages.checkout.placeOrder') }}
              </button>
            </div>

            <button v-else class="btn btn-primary btn-lg w-full shadow-lg hover:shadow-primary/20 transition-all duration-300"
              :disabled="submitting || !canPlaceOrder" @click="placeOrder">
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
import { useForm, useField } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/valibot'
import * as v from 'valibot'

definePageMeta({
  middleware: 'auth'
})

const { t } = useI18n()
const config = useRuntimeConfig()
const cartStore = useCartStore()
const router = useRouter()
const { apiFetch } = useApi()
import type { PaymentMethod, PaymentSetupSession, ShippingMethod } from '~/types'

declare global {
  interface Window {
    Stripe?: (publishableKey: string) => any
    braintree?: {
      dropin: {
        create: (options: Record<string, unknown>, callback: (error: Error | null, instance?: any) => void) => void
      }
    }
  }
}

const submitting = ref(false)
const confirmingStripe = ref(false)
const stripeReady = ref(false)
const error = ref('')
const selectedPaymentMethod = ref<number | null>(null)
const selectedShippingMethod = ref<number | null>(null)
const pendingStripeOrderId = ref<number | null>(null)
const stripeClientSecret = ref('')
let stripeClient: any = null
let stripeElements: any = null
const braintreeLoading = ref(false)
const braintreeReady = ref(false)
const braintreePaymentMethodId = ref('')
let braintreeDropin: any = null
const couponCode = ref('')
const couponDiscount = ref<number | null>(null)
const couponMessage = ref('')
const couponApplied = ref(false)
const couponChecking = ref(false)
function generateUUID(): string {
  if (typeof crypto !== 'undefined' && crypto.randomUUID) {
    return crypto.randomUUID()
  }
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0
    const v = c === 'x' ? r : (r & 0x3) | 0x8
    return v.toString(16)
  })
}

const idempotencyKey = ref(generateUUID())

const schema = computed(() => {
  return toTypedSchema(
    v.object({
      firstName: v.pipe(v.string(), v.nonEmpty(t('shipping.validation.firstNameRequired', 'Primeiro nome é obrigatório'))),
      lastName: v.pipe(v.string(), v.nonEmpty(t('shipping.validation.lastNameRequired', 'Sobrenome é obrigatório'))),
      address1: v.pipe(v.string(), v.nonEmpty(t('shipping.validation.address1Required', 'Endereço é obrigatório'))),
      address2: v.optional(v.string()),
      city: v.pipe(v.string(), v.nonEmpty(t('shipping.validation.cityRequired', 'Cidade é obrigatória'))),
      state: v.pipe(v.string(), v.nonEmpty(t('shipping.validation.stateRequired', 'Estado é obrigatório'))),
      zipCode: v.pipe(v.string(), v.nonEmpty(t('shipping.validation.zipCodeRequired', 'CEP é obrigatório'))),
      phone: v.pipe(v.string(), v.nonEmpty(t('shipping.validation.phoneRequired', 'Telefone é obrigatório'))),
      getnetDocument: selectedGatewayDriver.value === 'getnet'
        ? v.pipe(v.string(), v.nonEmpty(t('shipping.validation.documentRequired', 'CPF/CNPJ é obrigatório')))
        : v.optional(v.string()),
    })
  )
})

const { handleSubmit, errors, setFieldValue } = useForm({
  validationSchema: schema,
  initialValues: {
    firstName: '',
    lastName: '',
    address1: '',
    address2: '',
    city: '',
    state: '',
    zipCode: '',
    phone: '',
    getnetDocument: '',
  }
})

const { value: firstName, errorMessage: firstNameError, handleBlur: firstNameBlur } = useField<string>('firstName')
const { value: lastName, errorMessage: lastNameError, handleBlur: lastNameBlur } = useField<string>('lastName')
const { value: address1, errorMessage: address1Error, handleBlur: address1Blur } = useField<string>('address1')
const { value: address2 } = useField<string>('address2')
const { value: city, errorMessage: cityError, handleBlur: cityBlur } = useField<string>('city')
const { value: state, errorMessage: stateError, handleBlur: stateBlur } = useField<string>('state')
const { value: zipCode, errorMessage: zipCodeError, handleBlur: zipCodeBlur } = useField<string>('zipCode')
const { value: phone, errorMessage: phoneError, handleBlur: phoneBlur } = useField<string>('phone')
const { value: getnetDocument, errorMessage: getnetDocumentError, handleBlur: getnetDocumentBlur } = useField<string>('getnetDocument')

const address = computed(() => ({
  firstName: firstName.value,
  lastName: lastName.value,
  address1: address1.value,
  address2: address2.value,
  city: city.value,
  state: state.value,
  zipCode: zipCode.value,
  phone: phone.value,
}))

// SSR-friendly data fetching with useLazyFetch (cached, deduplicated, JWT via global plugin)
const { data: paymentMethods } = useLazyFetch<PaymentMethod[]>(
  '/api/payments/methods',
  { key: 'checkout-payment-methods', default: () => [] }
)
const { data: shippingMethods } = useLazyFetch<ShippingMethod[]>(
  '/api/shippings',
  { key: 'checkout-shipping-methods', default: () => [] }
)

// Auto-select first method when data arrives
watch(paymentMethods, (methods) => {
  if (methods.length > 0 && !selectedPaymentMethod.value) {
    selectedPaymentMethod.value = methods[0]?.id ?? null
  }
}, { immediate: true })

watch(shippingMethods, (methods) => {
  if (methods.length > 0 && !selectedShippingMethod.value) {
    selectedShippingMethod.value = methods[0]?.id ?? null
  }
}, { immediate: true })

const selectedShippingCost = computed(() => {
  if (!selectedShippingMethod.value) return null
  const method = shippingMethods.value.find(m => m.id === selectedShippingMethod.value)
  return method?.base_price ?? null
})

const selectedPaymentMethodRecord = computed(() => {
  if (!selectedPaymentMethod.value) return null
  return paymentMethods.value.find(method => method.id === selectedPaymentMethod.value) ?? null
})

const selectedGatewayDriver = computed(() => selectedPaymentMethodRecord.value?.gateway_driver || null)

watch(selectedGatewayDriver, async (driver) => {
  if (driver === 'braintree') {
    await mountBraintreeDropin()
    return
  }

  teardownBraintreeDropin()
})

function paymentMethodIcon(method: PaymentMethod) {
  const code = method.code?.toLowerCase() || ''
  if (code.includes('card')) return 'icon-[tabler--credit-card]'
  if (code.includes('pix')) return 'icon-[tabler--qrcode]'
  return 'icon-[tabler--wallet]'
}

const canPlaceOrder = computed(() => {
  if (!selectedPaymentMethod.value) return false
  if (selectedGatewayDriver.value === 'braintree') return braintreeReady.value
  if (selectedGatewayDriver.value === 'getnet') return getnetDocument.value.trim().length > 0
  return true
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
    const result = await apiFetch<any>('/api/coupons/validate', {
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

const placeOrder = handleSubmit(async () => {
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
    let paymentGatewayPayload: Record<string, unknown> | null = null

    if (selectedGatewayDriver.value === 'braintree') {
      braintreePaymentMethodId.value = await requestBraintreePaymentMethodId()
      paymentGatewayPayload = {
        payment_method_id: braintreePaymentMethodId.value.trim(),
      }
    } else if (selectedGatewayDriver.value === 'getnet' && getnetDocument.value.trim()) {
      paymentGatewayPayload = {
        customer: {
          name: `${address.value.firstName} ${address.value.lastName}`.trim(),
          document: {
            type: getnetDocument.value.replace(/\D/g, '').length > 11 ? 'CNPJ' : 'CPF',
            number: getnetDocument.value.replace(/\D/g, '')
          }
        }
      }
    }

    // Embed the client-side idempotency key to prevent double charging
    paymentGatewayPayload = {
      ...(paymentGatewayPayload || {}),
      idempotency_key: idempotencyKey.value,
    }

    const data = await apiFetch<any>('/api/account/orders/checkout', {
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
        payment_gateway_payload: paymentGatewayPayload,
        shipping_method_id: selectedShippingMethod.value,
        address_first_name: address.value.firstName || null,
        address_last_name: address.value.lastName || null,
        address1: address.value.address1 || null,
        address2: address.value.address2 || null,
        address_city: address.value.city || null,
        address_state: address.value.state || null,
        address_zip_code: address.value.zipCode || null,
        address_phone: address.value.phone || null,
      },
    })

    if (data.payment_session?.action_url) {
      cartStore.clearCart()
      await navigateTo(data.payment_session.action_url, { external: true })
      return
    }

    if (
      selectedGatewayDriver.value === 'stripe' &&
      data.payment_session?.external_client_secret &&
      !data.payment_session?.action_url
    ) {
      pendingStripeOrderId.value = data.id
      stripeClientSecret.value = data.payment_session.external_client_secret
      cartStore.clearCart()
      await mountStripePaymentElement(stripeClientSecret.value)
      return
    }

    const confirmationPath = data.payment_session?.requires_action
      ? `/orders/confirmation/${data.id}?payment_action=required`
      : `/orders/confirmation/${data.id}`
    cartStore.clearCart()
    router.push(confirmationPath)
  } catch (err: any) {
    error.value = err?.data?.message || err?.message || t('pages.products.edit.error', { message: '' })
    // Refresh idempotency key to allow retry with new payment attempt
    idempotencyKey.value = generateUUID()
  } finally {
    submitting.value = false
  }
})

async function mountStripePaymentElement(clientSecret: string) {
  stripeReady.value = false
  stripeClient = await loadStripeClient()
  stripeElements = stripeClient.elements({ clientSecret })
  await nextTick()
  const paymentElement = stripeElements.create('payment')
  paymentElement.mount('#stripe-payment-element')
  stripeReady.value = true
}

async function confirmStripePayment() {
  if (!stripeClient || !stripeElements || !pendingStripeOrderId.value) return
  confirmingStripe.value = true
  error.value = ''

  try {
    const result = await stripeClient.confirmPayment({
      elements: stripeElements,
      confirmParams: {
        return_url: `${window.location.origin}/orders/confirmation/${pendingStripeOrderId.value}`,
      },
      redirect: 'if_required',
    })

    if (result.error) {
      error.value = result.error.message || t('pages.products.edit.error', { message: '' })
      return
    }

    router.push(`/orders/confirmation/${pendingStripeOrderId.value}`)
  } finally {
    confirmingStripe.value = false
  }
}

async function loadStripeClient() {
  if (!config.public.stripePublishableKey) {
    throw new Error('Missing Stripe publishable key')
  }
  if (!window.Stripe) {
    await loadScript('stripe-js', 'https://js.stripe.com/v3/')
  }
  if (!window.Stripe) {
    throw new Error('Stripe.js failed to load')
  }
  return window.Stripe(config.public.stripePublishableKey as string)
}

async function mountBraintreeDropin() {
  if (!selectedPaymentMethod.value || braintreeDropin || braintreeLoading.value) return
  braintreeLoading.value = true
  braintreeReady.value = false
  error.value = ''

  try {
    const setupSession = await apiFetch<PaymentSetupSession>('/api/account/payment-setup-sessions', {
      method: 'POST',
      body: {
        payment_method_id: selectedPaymentMethod.value,
      },
    })
    if (!setupSession.external_client_secret) {
      throw new Error('Braintree setup session did not return a client token')
    }

    await loadBraintreeDropinClient()
    await nextTick()
    braintreeDropin = await createBraintreeDropin(setupSession.external_client_secret)
    braintreeReady.value = true
  } catch (err: any) {
    error.value = err?.data?.message || err?.message || t('pages.products.edit.error', { message: '' })
  } finally {
    braintreeLoading.value = false
  }
}

async function requestBraintreePaymentMethodId() {
  if (!braintreeDropin) {
    throw new Error('Braintree checkout is not ready')
  }

  const payload = await braintreeDropin.requestPaymentMethod()
  const nonce = payload?.nonce
  if (!nonce) {
    throw new Error('Braintree did not return a payment method nonce')
  }
  return nonce
}

function createBraintreeDropin(authorization: string) {
  return new Promise<any>((resolve, reject) => {
    window.braintree?.dropin.create(
      {
        authorization,
        container: '#braintree-dropin',
      },
      (error, instance) => {
        if (error) {
          reject(error)
          return
        }
        resolve(instance)
      }
    )
  })
}

async function loadBraintreeDropinClient() {
  if (!window.braintree?.dropin) {
    await loadScript('braintree-dropin-js', 'https://js.braintreegateway.com/web/dropin/1.44.1/js/dropin.min.js')
  }
  if (!window.braintree?.dropin) {
    throw new Error('Braintree Drop-in failed to load')
  }
}

function teardownBraintreeDropin() {
  braintreeReady.value = false
  braintreePaymentMethodId.value = ''
  const instance = braintreeDropin
  braintreeDropin = null

  if (instance?.teardown) {
    instance.teardown(() => undefined)
  }
}

function loadScript(id: string, src: string) {
  return new Promise<void>((resolve, reject) => {
    const existing = document.getElementById(id) as HTMLScriptElement | null
    if (existing) {
      existing.addEventListener('load', () => resolve(), { once: true })
      existing.addEventListener('error', () => reject(new Error(`${id} failed to load`)), { once: true })
      if ((existing as any).dataset.loaded === 'true') resolve()
      return
    }

    const script = document.createElement('script')
    script.id = id
    script.src = src
    script.async = true
    script.onload = () => {
      script.dataset.loaded = 'true'
      resolve()
    }
    script.onerror = () => reject(new Error(`${id} failed to load`))
    document.head.appendChild(script)
  })
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
