<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-base-100 shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          <span class="icon-[tabler--truck] size-6"></span>
          {{ isEditing ? t('shipping.editShipment') : t('shipping.createShipment') }}
        </h2>

        <AppAlert v-if="successMessage" type="success" :message="successMessage" :auto-close="3000" @close="successMessage = ''" />
        <AppAlert v-if="errorMessage" type="error" :message="errorMessage" :auto-close="5000" @close="errorMessage = ''" :dismissible="true" />

        <form @submit.prevent="onSubmit" class="space-y-6" novalidate>
          <!-- Order ID -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('shipping.order') }} *</span>
            </label>
            <input
              v-model="orderId"
              @blur="orderIdBlur"
              type="number"
              min="1"
              :placeholder="t('shipping.orderPlaceholder', 'Ex: 1')"
              class="input input-bordered w-full"
              :class="{ 'input-error': orderIdError }"
              :disabled="isEditing"
              required
            />
            <label v-if="orderIdError" class="label">
              <span class="label-text-alt text-error">{{ orderIdError }}</span>
            </label>
          </div>

          <!-- Shipping Method -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('shipping.method') }} *</span>
            </label>
            <select
              v-model="shippingMethodId"
              @blur="shippingMethodIdBlur"
              class="select select-bordered w-full"
              :class="{ 'select-error': shippingMethodIdError }"
              :disabled="isEditing"
              required
            >
              <option value="" disabled>{{ t('shipping.methodPlaceholder', 'Selecione o metodo de envio') }}</option>
              <option v-for="method in shippingMethods" :key="method.id" :value="method.id">
                {{ method.name || method.code }}
              </option>
            </select>
            <label v-if="shippingMethodIdError" class="label">
              <span class="label-text-alt text-error">{{ shippingMethodIdError }}</span>
            </label>
          </div>

          <!-- Carrier & Tracking -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('shipping.carrier') }}</span>
              </label>
              <input
                v-model="carrier"
                type="text"
                :placeholder="t('shipping.carrierPlaceholder', 'Ex: Correios, Jadlog')"
                class="input input-bordered w-full"
              />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('shipping.trackingNumber') }}</span>
              </label>
              <input
                v-model="trackingNumber"
                type="text"
                :placeholder="t('shipping.trackingPlaceholder', 'Ex: BR123456789')"
                class="input input-bordered w-full"
              />
            </div>
          </div>

          <!-- Status -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('pages.orders.status') }}</span>
            </label>
            <select v-model="status" class="select select-bordered w-full">
              <option :value="1">{{ t('shipping.status.pending') }}</option>
              <option :value="2">{{ t('shipping.status.shipped') }}</option>
              <option :value="3">{{ t('shipping.status.delivered') }}</option>
              <option :value="4">{{ t('shipping.status.cancelled') }}</option>
            </select>
          </div>

          <!-- Action Buttons -->
          <div class="flex justify-end gap-3 pt-4 border-t border-base-200">
            <button type="button" class="btn btn-ghost" @click="emit('cancel')" :disabled="pending">
              {{ t('common.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              <span v-else class="icon-[tabler--device-floppy] size-4"></span>
              {{ t('common.save') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useForm, useField } from 'vee-validate'
import type { Shipment, ShippingMethod } from '~/types'

const { t } = useI18n()
const { apiFetch, useApiLazyFetch } = useApi()

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

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Fetch shipping methods
const { data: shippingMethods } = useApiLazyFetch<ShippingMethod[]>(
  '/api/admin/shippings',
  { key: 'admin-shippings-list', default: () => [] }
)

// vee-validate form
const { handleSubmit, setFieldValue } = useForm({
  initialValues: {
    order_id: props.shipment?.order_id ?? 0,
    shipping_method_id: props.shipment?.shipping_method_id ?? ('' as any),
    tracking_number: props.shipment?.tracking_number ?? '',
    carrier: props.shipment?.carrier ?? '',
    status: props.shipment?.status ?? 1,
  }
})

const { value: orderId, errorMessage: orderIdError, handleBlur: orderIdBlur } = useField('order_id', 'required|min_value:1')
const { value: shippingMethodId, errorMessage: shippingMethodIdError, handleBlur: shippingMethodIdBlur } = useField('shipping_method_id', 'required')
const { value: trackingNumber } = useField('tracking_number')
const { value: carrier } = useField('carrier')
const { value: status } = useField('status')

const onSubmit = handleSubmit(async (vals) => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const body = {
      order_id: Number(vals.order_id),
      shipping_method_id: Number(vals.shipping_method_id),
      tracking_number: vals.tracking_number || null,
      carrier: vals.carrier || null,
      status: Number(vals.status),
    }

    let result: Shipment
    if (props.isEditing && props.shipment?.id) {
      result = await apiFetch<Shipment>(`/api/admin/shipments/${props.shipment.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body,
      })
    } else {
      result = await apiFetch<Shipment>('/api/admin/shipments', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body,
      })
    }
    successMessage.value = t('admin.statusLabels.completed')
    setTimeout(() => emit('saved', result), 500)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err?.message || t('common.error')
  } finally {
    pending.value = false
  }
})
</script>
