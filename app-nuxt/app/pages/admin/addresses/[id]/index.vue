<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLink to="/admin/addresses" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLink>
        <div>
          <h1 class="h1">Detalhes do Endereço</h1>
          <p class="text-sm text-gray-500" v-if="address">ID: {{ address.id }}</p>
        </div>
      </div>

      <div v-if="address" class="flex gap-2">
        <button @click="deleteAddress" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          Excluir
        </button>
        <NuxtLink :to="`/admin/addresses/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          Editar
        </NuxtLink>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">Carregando detalhes do endereço...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Erro ao carregar endereço: {{ error.message }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">Tentar novamente</button>
    </div>

    <!-- Content -->
    <div v-else-if="address" class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Main Info Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <div class="flex items-center gap-3 mb-4">
            <h2 class="card-title">{{ address.first_name }} {{ address.last_name }}</h2>
            <span class="badge badge-soft badge-sm">{{ typeLabel(address.type) }}</span>
            <span v-if="address.default" class="badge badge-primary badge-sm">Padrão</span>
          </div>

          <div class="space-y-4">
            <div v-if="address.company" class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Empresa</span>
              </label>
              <div class="font-medium">{{ address.company }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Endereço</span>
              </label>
              <div class="font-medium">{{ address.address1 }}</div>
              <div v-if="address.address2" class="text-gray-600">{{ address.address2 }}</div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Cidade</span>
                </label>
                <div class="font-medium">{{ address.city }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Estado</span>
                </label>
                <div class="font-medium">{{ address.state }}</div>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">CEP</span>
                </label>
                <div class="font-medium">{{ address.zip_code }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">País</span>
                </label>
                <div class="font-medium">{{ address.country }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Meta Info Card -->
      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title mb-4">Detalhes</h2>

          <div class="flex flex-col gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Telefone</span>
              </label>
              <div class="font-medium">{{ address.phone || '-' }}</div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Usuário ID</span>
              </label>
              <div class="font-medium">{{ address.user_id }}</div>
            </div>

            <div class="divider my-2"></div>

            <div class="text-xs text-gray-500 space-y-2">
              <div class="flex justify-between">
                <span>Criado em:</span>
                <span class="font-medium">{{ formatDate(address.created_at) }}</span>
              </div>
              <div class="flex justify-between">
                <span>Atualizado em:</span>
                <span class="font-medium">{{ formatDate(address.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Not Found State -->
    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>Endereço não encontrado.</span>
      <NuxtLink to="/admin/addresses" class="btn btn-sm">Voltar para lista</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Address } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()
const router = useRouter()

const { pending, data: address, error, refresh } = useFetch<Address>(
  `${config.public.baseURL}/api/addresses/${route.params.id}`
)

const typeLabel = (type?: string) => {
  const labels: Record<string, string> = {
    home: 'Casa',
    work: 'Trabalho',
    other: 'Outro'
  }
  return labels[type || ''] || 'Outro'
}

const formatDate = (dateString?: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

const deleteAddress = async () => {
  if (!address.value) return

  const name = `${address.value.first_name} ${address.value.last_name}`
  if (confirm(`Tem certeza que deseja excluir o endereço de "${name}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/addresses/${address.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/addresses')
    } catch (err) {
      alert('Erro ao excluir endereço')
      console.error(err)
    }
  }
}
</script>
