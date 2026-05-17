<template>
  <div>
    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Carregando endereço...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="max-w-2xl mx-auto">
      <div class="alert alert-error">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span>Erro ao carregar endereço: {{ error.message }}</span>
        <button class="btn btn-sm" @click="handleCancel">Voltar</button>
      </div>
    </div>

    <!-- Form -->
    <AdminAddressesForm
      v-else-if="address"
      :address="address"
      :is-editing="true"
      @saved="handleSaved"
      @cancel="handleCancel"
    />
  </div>
</template>

<script setup lang="ts">
import type { Address } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const { useApiFetch } = useApi()

const { data: address, pending, error } = await useApiFetch<Address>(
  `/api/admin/addresses/${route.params.id}`
)

const handleSaved = (savedAddress: Address) => {
  const returnTo = route.query.return_to as string
  navigateTo(returnTo || '/admin/addresses')
}

const handleCancel = () => {
  const returnTo = route.query.return_to as string
  navigateTo(returnTo || '/admin/addresses')
}
</script>

<style scoped></style>
