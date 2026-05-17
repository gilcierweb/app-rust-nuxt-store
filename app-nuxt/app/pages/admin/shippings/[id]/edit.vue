<template>
  <div>
    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Carregando método de envio...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="max-w-2xl mx-auto">
      <div class="alert alert-error">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span>Erro ao carregar método de envio: {{ error.message }}</span>
        <NuxtLinkLocale to="/admin/shippings" class="btn btn-sm">Voltar</NuxtLinkLocale>
      </div>
    </div>

    <!-- Form -->
    <AdminShippingsForm
      v-else-if="shipping"
      :shipping="shipping"
      :is-editing="true"
      @saved="handleSaved"
      @cancel="navigateTo('/admin/shippings')"
    />
  </div>
</template>

<script setup lang="ts">
import type { ShippingMethod } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const { useApiFetch } = useApi()

const { data: shipping, pending, error } = await useApiFetch<ShippingMethod>(
  `/api/admin/shippings/${route.params.id}`
)

const handleSaved = (savedShipping: ShippingMethod) => {
  navigateTo('/admin/shippings')
}
</script>

<style scoped></style>
