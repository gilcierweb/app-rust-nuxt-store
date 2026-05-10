<template>
  <div class="max-w-2xl mx-auto">
    <h1 class="h1 mb-6">{{ isEditing ? t('shipping.editShipment') : t('shipping.createShipment') }}</h1>

    <div class="rounded-box border p-6">
      <AppAlert v-if="submitError" type="error" :message="submitError" class="mb-4" />
      <AppAlert v-if="submitSuccess" type="success" :message="submitSuccess" class="mb-4" />

      <form @submit.prevent="handleSubmit">
        <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
          <div class="form-control md:col-span-2">
            <label class="label">
              <span class="label-text">{{ t('shipping.order') }}</span>
            </label>
            <input
              v-model="form.order_id"
              type="number"
              class="input input-bordered"
              :disabled="isEditing"
              required
            />
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">{{ t('shipping.carrier') }}</span>
            </label>
            <input v-model="form.carrier" type="text" class="input input-bordered" />
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">{{ t('shipping.trackingNumber') }}</span>
            </label>
            <input v-model="form.tracking_number" type="text" class="input input-bordered" />
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">{{ t('shipping.method') }}</span>
            </label>
            <select v-model="form.shipping_method_id" class="select select-bordered" :disabled="isEditing" required>
              <option value="" disabled>{{ t('shipping.method') }}</option>
              <option v-for="method in shippingMethods" :key="method.id" :value="method.id">
                {{ method.name || method.code }}
              </option>
            </select>
          </div>
          <div class="form-control">
            <label class="label">
              <span class="label-text">{{ t('pages.orders.status') }}</span>
            </label>
            <select v-model="form.status" class="select select-bordered">
              <option :value="1">{{ t('shipping.status.pending') }}</option>
              <option :value="2">{{ t('shipping.status.shipped') }}</option>
              <option :value="3">{{ t('shipping.status.delivered') }}</option>
              <option :value="4">{{ t('shipping.status.cancelled') }}</option>
            </select>
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-6">
          <button type="button" class="btn btn-ghost" @click="emit('cancel')">
            {{ t('common.cancel') }}
          </button>
          <button type="submit" class="btn btn-primary" :disabled="saving">
            <span v-if="saving" class="loading loading-spinner" />
            {{ t('common.save') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Shipment, ShippingMethod } from '~/types'

const { t } = useI18n()
const config = useRuntimeConfig()

const props = withDefaults(defineProps<{
  shipment?: Partial<Shipment>
  isEditing?: boolean
}>(), {
  isEditing: false,
})

const emit = defineEmits<{
  saved: [shipment: Shipment]
  cancel: []
}>()

const saving = ref(false)
const submitError = ref('')
const submitSuccess = ref('')
const shippingMethods = ref<ShippingMethod[]>([])

const form = reactive({
  order_id: props.shipment?.order_id ?? 0,
  shipping_method_id: props.shipment?.shipping_method_id ?? ('' as any),
  tracking_number: props.shipment?.tracking_number ?? '',
  carrier: props.shipment?.carrier ?? '',
  status: props.shipment?.status ?? 1,
})

onMounted(async () => {
  try {
    shippingMethods.value = await $fetch(`${config.public.baseURL}/api/shippings`)
  } catch {
    shippingMethods.value = []
  }
})

async function handleSubmit() {
  saving.value = true
  submitError.value = ''
  submitSuccess.value = ''

  try {
    const body = {
      order_id: Number(form.order_id),
      shipping_method_id: Number(form.shipping_method_id),
      tracking_number: form.tracking_number || null,
      carrier: form.carrier || null,
      status: Number(form.status),
    }

    let result: Shipment
    if (props.isEditing && props.shipment?.id) {
      result = await $fetch(`${config.public.baseURL}/api/shipments/${props.shipment.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body,
      })
    } else {
      result = await $fetch(`${config.public.baseURL}/api/shipments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body,
      })
    }
    submitSuccess.value = t('admin.statusLabels.completed')
    setTimeout(() => emit('saved', result), 500)
  } catch (err: any) {
    submitError.value = err?.data?.message || err?.message || t('common.error')
  } finally {
    saving.value = false
  }
}
</script>
