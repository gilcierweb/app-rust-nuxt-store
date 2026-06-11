<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ t('admin.payments.methods.title') }}</h1>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ t('admin.payments.methods.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <span>{{ t('admin.payments.methods.error') }} {{ error.message }}</span>
    </div>

    <!-- Methods Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ t('admin.payments.methods.id') }}</th>
              <th>{{ t('admin.payments.methods.name') }}</th>
              <th>{{ t('admin.payments.methods.code') }}</th>
              <th>{{ t('admin.payments.methods.gatewayId') }}</th>
              <th>{{ t('admin.payments.methods.type') }}</th>
              <th>{{ t('admin.payments.methods.active') }}</th>
              <th>{{ t('admin.payments.methods.autoCapture') }}</th>
              <th>{{ t('admin.payments.methods.position') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="method in methods" :key="method.id" class="row-hover">
              <td>{{ method.id }}</td>
              <td class="font-medium">{{ method.name }}</td>
              <td class="font-mono text-sm text-gray-500">{{ method.code }}</td>
              <td>{{ method.payment_gateway_id }}</td>
              <td>
                <span class="badge badge-soft badge-info">{{ methodTypeLabel(method.method_type) }}</span>
              </td>
              <td>
                <input 
                  type="checkbox" 
                  class="toggle toggle-success toggle-sm" 
                  :checked="method.active"
                  @change="toggleActive(method)"
                />
              </td>
              <td>
                <input 
                  type="checkbox" 
                  class="toggle toggle-warning toggle-sm" 
                  :checked="method.auto_capture"
                  @change="toggleAutoCapture(method)"
                />
              </td>
              <td>
                <input 
                  type="number" 
                  v-model.number="method.position"
                  @change="updateMethod(method)"
                  class="input input-sm input-bordered w-20"
                />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiFetch } = useApi()
const toast = useAppToast()
const { t } = useI18n()

const { pending, data: methods, error, refresh } = await useApiFetch<any[]>(
  '/api/admin/payment-methods',
  { key: 'admin-methods-list' }
)

const methodTypeLabel = (type: number) => {
  switch (type) {
    case 1: return t('admin.payments.methods.creditCard')
    case 2: return t('admin.payments.methods.boleto')
    case 3: return t('admin.payments.methods.pix')
    default: return 'Unknown'
  }
}

const toggleActive = async (method: any) => {
  const newStatus = !method.active
  try {
    await apiFetch(`/api/admin/payment-methods/${method.id}`, {
      method: 'PUT',
      body: { active: newStatus }
    })
    method.active = newStatus
  } catch (err) {
    toast.error(t('admin.payments.methods.updateStatusFailed'))
    method.active = !newStatus
  }
}

const toggleAutoCapture = async (method: any) => {
  const newValue = !method.auto_capture
  try {
    await apiFetch(`/api/admin/payment-methods/${method.id}`, {
      method: 'PUT',
      body: { auto_capture: newValue }
    })
    method.auto_capture = newValue
  } catch (err) {
    toast.error(t('admin.payments.methods.updateAutoCaptureFailed'))
    method.auto_capture = !newValue
  }
}

const updateMethod = async (method: any) => {
  try {
    await apiFetch(`/api/admin/payment-methods/${method.id}`, {
      method: 'PUT',
      body: { position: method.position }
    })
  } catch (err) {
    toast.error(t('admin.payments.methods.updatePositionFailed'))
    await refresh()
  }
}
</script>
