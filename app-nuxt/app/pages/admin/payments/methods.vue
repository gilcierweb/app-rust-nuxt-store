<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">Payment Methods</h1>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Loading...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <span>Failed to load methods: {{ error.message }}</span>
    </div>

    <!-- Methods Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Code</th>
              <th>Gateway ID</th>
              <th>Type</th>
              <th>Active</th>
              <th>Auto Capture</th>
              <th>Position</th>
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

const { pending, data: methods, error, refresh } = await useApiFetch<any[]>(
  '/api/admin/payment-methods',
  { key: 'admin-methods-list' }
)

const methodTypeLabel = (type: number) => {
  switch (type) {
    case 1: return 'Credit Card'
    case 2: return 'Boleto'
    case 3: return 'Pix'
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
    toast.error('Failed to update status')
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
    toast.error('Failed to update auto capture')
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
    toast.error('Failed to update position')
    await refresh()
  }
}
</script>
