<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">Payment Gateways</h1>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Loading...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <span>Failed to load gateways: {{ error.message }}</span>
    </div>

    <!-- Gateways Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Code</th>
              <th>Environment</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="gateway in gateways" :key="gateway.id" class="row-hover">
              <td>{{ gateway.id }}</td>
              <td class="font-medium">{{ gateway.name }}</td>
              <td class="font-mono text-sm text-gray-500">{{ gateway.code }}</td>
              <td>
                <select 
                  v-model="gateway.environment" 
                  @change="updateGateway(gateway)"
                  class="select select-sm select-bordered"
                >
                  <option :value="1">Sandbox</option>
                  <option :value="2">Production</option>
                </select>
              </td>
              <td>
                <input 
                  type="checkbox" 
                  class="toggle toggle-success toggle-sm" 
                  :checked="gateway.status === 1"
                  @change="toggleStatus(gateway)"
                />
              </td>
              <td>
                <!-- Action buttons can be added later -->
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

const { pending, data: gateways, error, refresh } = await useApiFetch<any[]>(
  '/api/admin/payment-gateways',
  { key: 'admin-gateways-list' }
)

const toggleStatus = async (gateway: any) => {
  const newStatus = gateway.status === 1 ? 0 : 1
  try {
    await apiFetch(`/api/admin/payment-gateways/${gateway.id}`, {
      method: 'PUT',
      body: { status: newStatus }
    })
    gateway.status = newStatus
  } catch (err) {
    toast.error('Failed to update status')
    gateway.status = gateway.status === 1 ? 1 : 0 // revert
  }
}

const updateGateway = async (gateway: any) => {
  try {
    await apiFetch(`/api/admin/payment-gateways/${gateway.id}`, {
      method: 'PUT',
      body: { environment: gateway.environment }
    })
  } catch (err) {
    toast.error('Failed to update environment')
    await refresh()
  }
}
</script>
