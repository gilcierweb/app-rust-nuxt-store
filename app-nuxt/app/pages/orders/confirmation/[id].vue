<template>
  <div>
    <div v-if="pending" class="flex items-center justify-center h-96">
      <span class="loading loading-spinner text-info size-40" />
    </div>

    <div v-else-if="!order" class="flex flex-col items-center justify-center py-20">
      <span class="icon-[tabler--alert-circle] mb-4 size-20 text-error opacity-60" />
      <p class="text-lg">{{ t('admin.statusLabels.unknown') }}</p>
      <NuxtLinkLocale to="/orders" class="btn btn-primary mt-4">
        {{ t('pages.orders.title') }}
      </NuxtLinkLocale>
    </div>

    <div v-else class="mx-auto max-w-2xl py-10 text-center">
      <span class="icon-[tabler--circle-check] mx-auto mb-6 size-20 text-success" />
      <h1 class="h1 mb-2">{{ t('pages.confirmation.title') }}</h1>
      <p class="mb-8 text-lg text-base-content/60">{{ t('pages.confirmation.success') }}</p>

      <div v-if="paymentActionRequired && (!payment || paymentPending)" class="alert alert-warning mb-6 text-left shadow-sm">
        <span class="icon-[tabler--alert-triangle] size-5" />
        <span>{{ t('order.paymentStatus.unpaid') }}</span>
      </div>

      <!-- Async Payment Details (Pix or Boleto) -->
      <div v-if="paymentActionRequired && payment && !paymentPending" class="mt-6 mb-8 p-6 md:p-8 rounded-[2.5rem] bg-base-100 border border-base-200 shadow-xl max-w-xl mx-auto transition-all duration-300">
        <!-- Pix Section -->
        <div v-if="isPix" class="space-y-6 flex flex-col items-center">
          <div class="size-16 rounded-2xl bg-primary/10 flex items-center justify-center text-primary mb-2">
            <span class="icon-[tabler--qrcode] size-9" />
          </div>
          <div>
            <h3 class="font-black text-xl text-base-content">{{ t('pages.confirmation.pixPaymentTitle', 'Pague com Pix') }}</h3>
            <p class="text-xs text-base-content/60 mt-1 max-w-md mx-auto">{{ t('pages.confirmation.pixPaymentDesc', 'Escaneie o código QR abaixo ou copie a chave Pix para concluir sua compra.') }}</p>
          </div>

          <!-- Pulsing QR Code Container -->
          <div class="relative group p-4 bg-white rounded-3xl border-2 border-primary/20 shadow-md transition-all duration-500 hover:border-primary/50">
            <img :src="`https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=${encodeURIComponent(pixCode)}`" alt="Pix QR Code" class="size-48 md:size-52 object-contain" />
            <div class="absolute inset-0 bg-primary/5 opacity-0 group-hover:opacity-100 rounded-3xl transition-opacity duration-300 pointer-events-none" />
          </div>

          <!-- Copy Pix Key -->
          <div class="w-full space-y-2">
            <label class="label-text text-xs font-bold uppercase tracking-wider text-base-content/50 block text-left" for="pix-key-input">{{ t('pages.confirmation.pixKey', 'Chave Pix Copia e Cola') }}</label>
            <div class="flex gap-2">
              <input id="pix-key-input" type="text" readonly :value="pixCode" class="input input-sm grow font-mono bg-base-200/50 border-base-300 text-xs py-5 px-4 rounded-xl" />
              <button @click="copyPixKey" class="btn btn-primary btn-sm px-5 py-5 flex items-center gap-2 rounded-xl transition-all duration-300 relative overflow-hidden shadow-sm" :class="{ 'btn-success': pixCopied }">
                <span :class="pixCopied ? 'icon-[tabler--circle-check]' : 'icon-[tabler--copy]'" class="size-4" />
                <span class="text-sm font-semibold">{{ pixCopied ? t('common.copied', 'Copiado!') : t('common.copy', 'Copiar') }}</span>
              </button>
            </div>
          </div>

          <!-- Pix Instructions -->
          <div class="w-full text-left bg-base-200/40 rounded-2xl p-4 space-y-2 border border-base-200/50">
            <p class="text-xs font-bold text-base-content/70 mb-1">{{ t('pages.confirmation.howToPay', 'Como pagar:') }}</p>
            <ol class="list-decimal list-inside text-[11px] text-base-content/60 space-y-1">
              <li>{{ t('pages.confirmation.step1', 'Abra o aplicativo do seu banco.') }}</li>
              <li>{{ t('pages.confirmation.step2', 'Escolha a opção de pagar via Pix (ou Pix Copia e Cola).') }}</li>
              <li>{{ t('pages.confirmation.step3', 'Escaneie o código acima ou cole o código copiado.') }}</li>
            </ol>
          </div>
        </div>

        <!-- Boleto Section -->
        <div v-else-if="isBoleto" class="space-y-6 flex flex-col items-center">
          <div class="size-16 rounded-2xl bg-warning/10 flex items-center justify-center text-warning mb-2">
            <span class="icon-[tabler--file-text] size-9" />
          </div>
          <div>
            <h3 class="font-black text-xl text-base-content">{{ t('pages.confirmation.boletoPaymentTitle', 'Pague com Boleto Bancário') }}</h3>
            <p class="text-xs text-base-content/60 mt-1 max-w-md mx-auto">{{ t('pages.confirmation.boletoPaymentDesc', 'Clique no botão abaixo para gerar seu boleto ou copie a linha digitável abaixo.') }}</p>
          </div>

          <!-- Print / View Boleto URL -->
          <div v-if="boletoUrl" class="w-full">
            <a :href="boletoUrl" target="_blank" class="btn btn-warning btn-lg w-full rounded-2xl shadow-md hover:shadow-warning/20 transition-all duration-300 flex items-center justify-center gap-3">
              <span class="icon-[tabler--printer] size-6" />
              <span class="font-bold text-base">{{ t('pages.confirmation.printBoleto', 'Visualizar / Imprimir Boleto') }}</span>
            </a>
          </div>

          <!-- Copy Barcode -->
          <div v-if="barcodeNumber" class="w-full space-y-2">
            <label class="label-text text-xs font-bold uppercase tracking-wider text-base-content/50 block text-left" for="barcode-input">{{ t('pages.confirmation.barcode', 'Linha Digitável / Código de Barras') }}</label>
            <div class="flex gap-2">
              <input id="barcode-input" type="text" readonly :value="barcodeNumber" class="input input-sm grow font-mono bg-base-200/50 border-base-300 text-xs py-5 px-4 rounded-xl" />
              <button @click="copyBarcode" class="btn btn-warning btn-sm px-5 py-5 flex items-center gap-2 rounded-xl transition-all duration-300 relative overflow-hidden shadow-sm" :class="{ 'btn-success text-success-content': barcodeCopied }">
                <span :class="barcodeCopied ? 'icon-[tabler--circle-check]' : 'icon-[tabler--copy]'" class="size-4" />
                <span class="text-sm font-semibold">{{ barcodeCopied ? t('common.copied', 'Copiado!') : t('common.copy', 'Copiar') }}</span>
              </button>
            </div>
          </div>
        </div>

        <!-- Generic Pending Redirect -->
        <div v-else-if="boletoUrl" class="space-y-6 flex flex-col items-center">
          <div class="size-16 rounded-2xl bg-info/10 flex items-center justify-center text-info mb-2">
            <span class="icon-[tabler--wallet] size-9" />
          </div>
          <div>
            <h3 class="font-black text-xl text-base-content">{{ t('pages.confirmation.pendingPayment', 'Concluir Pagamento') }}</h3>
            <p class="text-xs text-base-content/60 mt-1 max-w-md mx-auto">{{ t('pages.confirmation.pendingPaymentDesc', 'Clique no botão abaixo para concluir o pagamento em sua carteira digital.') }}</p>
          </div>

          <a :href="boletoUrl" target="_blank" class="btn btn-info btn-lg w-full rounded-2xl shadow-md hover:shadow-info/20 transition-all duration-300 flex items-center justify-center gap-3">
            <span class="icon-[tabler--external-link] size-6" />
            <span class="font-bold text-base">{{ t('pages.confirmation.completePayment', 'Ir para o Pagamento') }}</span>
          </a>
        </div>
      </div>

      <div class="rounded-box border p-6 text-left">
        <div class="flex justify-between py-2">
          <span class="text-base-content/60">{{ t('pages.confirmation.number') }}</span>
          <span class="font-semibold">{{ order.order_number || '-' }}</span>
        </div>
        <div class="flex justify-between py-2">
          <span class="text-base-content/60">{{ t('order.date') }}</span>
          <span>{{ formatDate(order.created_at) }}</span>
        </div>
        <div class="flex justify-between py-2">
          <span class="text-base-content/60">{{ t('pages.orders.status') }}</span>
          <span class="badge badge-soft badge-primary">{{ t('order.status.pending') }}</span>
        </div>
        <div class="flex justify-between py-2">
          <span class="text-base-content/60">{{ t('pages.orders.payment') }}</span>
          <span :class="paymentBadgeClass(order.payment_status)">
            {{ paymentLabel(order.payment_status) }}
          </span>
        </div>
        <div class="flex justify-between border-t pt-4 mt-2 text-lg font-bold">
          <span>{{ t('order.total') }}</span>
          <span class="text-primary">{{ formatNumberBR(order.total_amount) }}</span>
        </div>
      </div>

      <p class="mt-6 text-sm text-base-content/40">{{ t('pages.confirmation.emailSent') }}</p>

      <div class="mt-8 flex justify-center gap-4">
        <NuxtLinkLocale to="/orders" class="btn btn-primary">
          {{ t('pages.orders.title') }}
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/products" class="btn btn-ghost">
          {{ t('cart.continueShopping') }}
        </NuxtLinkLocale>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

const { t } = useI18n()
const route = useRoute()
import type { Order } from '~/types'
const { useApiLazyFetch } = useApi()

const id = route.params.id
const paymentActionRequired = computed(() => route.query.payment_action === 'required')
const { data: order, pending } = useApiLazyFetch<Order>(
  `/api/account/orders/${id}`,
  { key: `confirmation-${id}` }
)

const { data: payment, pending: paymentPending } = useApiLazyFetch<any>(
  `/api/payments/order/${id}`,
  { key: `confirmation-payment-${id}` }
)

const { data: paymentMethods } = useLazyFetch<any[]>(
  '/api/payments/methods',
  { key: 'checkout-payment-methods', default: () => [] }
)

const selectedMethod = computed(() => {
  if (!payment.value || !paymentMethods.value) return null
  return paymentMethods.value.find(m => m.id === payment.value.payment_method_id) ?? null
})

const isPix = computed(() => {
  const code = selectedMethod.value?.code?.toLowerCase() || ''
  return code.includes('pix')
})

const isBoleto = computed(() => {
  const code = selectedMethod.value?.code?.toLowerCase() || ''
  return code.includes('boleto') || code.includes('bank_slip') || code.includes('bill')
})

const pixCode = computed(() => {
  if (!payment.value?.payment_session) return ''
  return payment.value.payment_session.external_client_secret || ''
})

const boletoUrl = computed(() => {
  if (!payment.value?.payment_session) return ''
  const secret = payment.value.payment_session.external_client_secret || ''
  if (secret.startsWith('http://') || secret.startsWith('https://')) {
    return secret
  }
  return ''
})

const barcodeNumber = computed(() => {
  if (!payment.value?.payment_session) return ''
  const secret = payment.value.payment_session.external_client_secret || ''
  if (!secret.startsWith('http://') && !secret.startsWith('https://')) {
    return secret
  }
  return ''
})

const pixCopied = ref(false)
const barcodeCopied = ref(false)

function copyPixKey() {
  if (!pixCode.value) return
  navigator.clipboard.writeText(pixCode.value)
  pixCopied.value = true
  setTimeout(() => {
    pixCopied.value = false
  }, 2000)
}

function copyBarcode() {
  if (!barcodeNumber.value) return
  navigator.clipboard.writeText(barcodeNumber.value)
  barcodeCopied.value = true
  setTimeout(() => {
    barcodeCopied.value = false
  }, 2000)
}

const paymentMap: Record<number, { label: string; badge: string }> = {
  1: { label: t('order.paymentStatus.unpaid'), badge: 'badge-soft badge-error' },
  2: { label: t('order.paymentStatus.paid'), badge: 'badge-soft badge-success' },
  3: { label: t('order.paymentStatus.refunded'), badge: 'badge-soft badge-info' },
  4: { label: t('order.paymentStatus.partiallyRefunded'), badge: 'badge-soft badge-warning' },
}

function paymentLabel(status: unknown): string {
  if (status == null) return '-'
  return paymentMap[status as number]?.label ?? t('admin.statusLabels.unknown')
}

function paymentBadgeClass(status: unknown): string {
  if (status == null) return 'badge-soft'
  return paymentMap[status as number]?.badge ?? 'badge-soft'
}
</script>
