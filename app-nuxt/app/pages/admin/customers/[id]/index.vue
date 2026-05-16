<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/customers" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">Detalhes do Cliente</h1>
          <p class="text-sm text-gray-500" v-if="profile">ID: {{ profile.id }} (User ID: {{ profile.user_id }})</p>
        </div>
      </div>

      <div v-if="profile" class="flex gap-2">
        <button @click="deleteProfile" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          Excluir
        </button>
        <NuxtLinkLocale :to="`/admin/customers/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          Editar
        </NuxtLinkLocale>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending || pendingAddresses" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">Carregando detalhes do cliente...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error || errorAddresses" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Erro ao carregar dados do cliente: {{ error?.message || errorAddresses?.message }}</span>
      <button class="btn btn-sm btn-ghost" @click="refreshAll">Tentar novamente</button>
    </div>

    <!-- Content -->
    <div v-else-if="profile" class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Profile Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-1">
        <div class="card-body items-center text-center">
          <div class="avatar mb-4">
            <div v-if="profile.avatar" class="w-32 rounded-full">
              <img :src="profile.avatar" :alt="profileName" />
            </div>
            <div v-else class="bg-neutral text-neutral-content w-32 h-32 rounded-full flex items-center justify-center">
              <span class="text-4xl">{{ getInitials(profileName) }}</span>
            </div>
          </div>
          <h2 class="card-title text-2xl">{{ profileName }}</h2>
          <p v-if="profile.username" class="text-gray-500">@{{ profile.username }}</p>
          <p v-if="profile.bio" class="text-sm mt-2">{{ profile.bio }}</p>

          <div class="divider my-4"></div>

          <div class="text-xs text-gray-500 space-y-2 w-full">
            <div class="flex justify-between">
              <span>Criado em:</span>
              <span class="font-medium">{{ formatDate(profile.created_at) }}</span>
            </div>
            <div class="flex justify-between">
              <span>Atualizado em:</span>
              <span class="font-medium">{{ formatDate(profile.updated_at) }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Details & Addresses Column -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Details Card -->
        <div class="card bg-base-100 shadow-sm border">
          <div class="card-body">
            <h2 class="card-title mb-4">Informações de Contato</h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Primeiro Nome</span>
                </label>
                <div class="font-medium">{{ profile.first_name || '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Sobrenome</span>
                </label>
                <div class="font-medium">{{ profile.last_name || '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Nome Completo</span>
                </label>
                <div class="font-medium">{{ profile.full_name || '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Username</span>
                </label>
                <div class="font-medium">{{ profile.username || '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Apelido</span>
                </label>
                <div class="font-medium">{{ profile.nickname || '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Data de Nascimento</span>
                </label>
                <div class="font-medium">{{ profile.birth_date || '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Telefone</span>
                </label>
                <div class="font-medium">{{ profile.phone || '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">WhatsApp</span>
                </label>
                <div class="font-medium">{{ profile.whatsapp || '-' }}</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Addresses Card -->
        <div class="card bg-base-100 shadow-sm border">
          <div class="card-body">
            <div class="flex justify-between items-center mb-4">
              <h2 class="card-title">Endereços</h2>
              <NuxtLinkLocale 
                :to="{ path: '/admin/addresses/new', query: { user_id: profile.user_id, return_to: route.path } }" 
                class="btn btn-sm btn-ghost"
              >
                <i class="icon-[tabler--plus] size-4 mr-1"></i>
                Adicionar
              </NuxtLinkLocale>
            </div>

            <div v-if="customerAddresses.length === 0" class="text-center py-6 text-gray-500">
              Nenhum endereço cadastrado para este cliente.
            </div>

            <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div v-for="address in customerAddresses" :key="address.id" class="border rounded-lg p-4 relative group">
                <div class="absolute top-2 right-2 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                   <NuxtLinkLocale 
                     :to="{ path: `/admin/addresses/${address.id}/edit`, query: { return_to: route.path } }" 
                     class="btn btn-xs btn-circle btn-ghost"
                   >
                     <i class="icon-[tabler--pencil] size-3"></i>
                   </NuxtLinkLocale>
                </div>
                <div class="flex items-center gap-2 mb-2">
                  <span class="badge badge-sm badge-soft" :class="address.default ? 'badge-primary' : ''">
                    {{ address.type || 'Principal' }}
                  </span>
                  <span v-if="address.default" class="text-xs text-primary font-bold">Padrão</span>
                </div>
                <div class="text-sm font-medium">{{ address.first_name }} {{ address.last_name }}</div>
                <div class="text-sm text-gray-600">{{ address.address1 }}</div>
                <div v-if="address.address2" class="text-sm text-gray-600">{{ address.address2 }}</div>
                <div class="text-sm text-gray-600">{{ address.city }}, {{ address.state }} - {{ address.zip_code }}</div>
                <div class="text-sm text-gray-600">{{ address.country }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Not Found State -->
    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>Cliente não encontrado.</span>
      <NuxtLinkLocale to="/admin/customers" class="btn btn-sm">Voltar para lista</NuxtLinkLocale>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Profile, Address } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const { apiFetch, useApiFetch } = useApi()
const router = useRouter()

// Fetch Profile
const { pending, data: profile, error, refresh: refreshProfile } = useApiFetch<Profile>(
  `/api/admin/profiles/${route.params.id}`
)

// Fetch All Addresses and filter by user_id
const { pending: pendingAddresses, data: allAddresses, error: errorAddresses, refresh: refreshAddresses } = useApiFetch<Address[]>(
  '/api/admin/addresses'
)

const customerAddresses = computed(() => {
  if (!allAddresses.value || !profile.value) return []
  return allAddresses.value.filter(addr => addr.user_id === profile.value?.user_id)
})

const refreshAll = () => {
  refreshProfile()
  refreshAddresses()
}

const profileName = computed(() => {
  if (!profile.value) return ''
  return profile.value.full_name ||
    `${profile.value.first_name || ''} ${profile.value.last_name || ''}`.trim() ||
    '(Sem nome)'
})

const getInitials = (name: string) => {
  if (!name || name === '(Sem nome)') return '?'
  return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2)
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

const deleteProfile = async () => {
  if (!profile.value) return

  if (confirm(`Tem certeza que deseja excluir o perfil de "${profileName.value}"?`)) {
    try {
      await apiFetch(`/api/admin/profiles/${profile.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/customers')
    } catch (err) {
      alert('Erro ao excluir perfil')
      console.error(err)
    }
  }
}
</script>

<style scoped></style>
