<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">Endereços</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Buscar endereços"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">Buscar</button>
        </div>
      </form>

      <NuxtLink to="/admin/addresses/new" class="btn btn-success">Adicionar</NuxtLink>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">Carregando endereços...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>Erro ao carregar endereços: {{ error.message }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredAddresses.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">Nenhum endereço encontrado.</p>
      <NuxtLink to="/admin/addresses/new" class="btn btn-primary mt-4">Criar primeiro endereço</NuxtLink>
    </div>

    <!-- Addresses Table -->
    <div v-else class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <tr>
            <th>Nome</th>
            <th>Tipo</th>
            <th>Endereço</th>
            <th>Cidade/Estado</th>
            <th>Padrão</th>
            <th>Usuário ID</th>
            <th>Ações</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="address in filteredAddresses" :key="address.id" class="row-hover">
            <td>{{ address.first_name }} {{ address.last_name }}</td>
            <td>
              <span class="badge badge-soft badge-sm">
                {{ typeLabel(address.type) }}
              </span>
            </td>
            <td>
              <div class="text-sm">
                <div>{{ address.address1 }}</div>
                <div v-if="address.address2" class="text-gray-500">{{ address.address2 }}</div>
              </div>
            </td>
            <td>
              {{ address.city }}/{{ address.state }}
              <div class="text-xs text-gray-500">{{ address.zip_code }}</div>
            </td>
            <td>
              <span v-if="address.default" class="badge badge-primary badge-sm">
                <i class="icon-[tabler--check] size-3 mr-1"></i>
                Padrão
              </span>
              <span v-else class="text-gray-400">-</span>
            </td>
            <td>{{ address.user_id }}</td>
            <td>
              <NuxtLink
                :to="`/admin/addresses/${address.id}`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Ver detalhes"
              >
                <i class="icon-[tabler--eye] size-5"></i>
              </NuxtLink>
              <NuxtLink
                :to="`/admin/addresses/${address.id}/edit`"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Editar"
              >
                <i class="icon-[tabler--pencil] size-5"></i>
              </NuxtLink>
              <button
                type="button"
                class="btn btn-circle btn-text btn-sm"
                aria-label="Excluir"
                @click="confirmDelete(address)"
              >
                <span class="icon-[tabler--trash] size-5"></span>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Address } from '~/types'

definePageMeta({
  layout: 'admin'
})

const config = useRuntimeConfig()

const searchQuery = ref('')

const { pending, data: addresses, error, refresh } = useLazyFetch<Address[]>(
  `${config.public.baseURL}/api/addresses`
)

// Filtered addresses based on search
const filteredAddresses = computed(() => {
  if (!addresses.value) return []
  if (!searchQuery.value.trim()) return addresses.value

  const query = searchQuery.value.toLowerCase()
  return addresses.value.filter(address =>
    address.first_name?.toLowerCase().includes(query) ||
    address.last_name?.toLowerCase().includes(query) ||
    address.city?.toLowerCase().includes(query) ||
    address.address1?.toLowerCase().includes(query)
  )
})

// Type label
const typeLabel = (type?: string) => {
  const labels: Record<string, string> = {
    home: 'Casa',
    work: 'Trabalho',
    other: 'Outro'
  }
  return labels[type || ''] || 'Outro'
}

// Search handler
const handleSearch = () => {
  // Search is handled reactively via computed
}

// Delete confirmation
const confirmDelete = async (address: Address) => {
  const name = `${address.first_name} ${address.last_name}`
  if (confirm(`Tem certeza que deseja excluir o endereço de "${name}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/addresses/${address.id}`, {
        method: 'DELETE'
      })
      await refresh()
    } catch (err) {
      alert('Erro ao excluir endereço')
      console.error(err)
    }
  }
}
</script>

<style scoped></style>
