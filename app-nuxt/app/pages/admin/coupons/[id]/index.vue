<template>
  <div>
    <!-- Header with Back Button -->
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <NuxtLink to="/admin/coupons" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLink>
        <div>
          <h1 class="h1">Detalhes do Cupom</h1>
          <p class="text-sm text-gray-500" v-if="coupon">ID: {{ coupon.id }}</p>
        </div>
      </div>

      <div v-if="coupon" class="flex gap-2">
        <button @click="deleteCoupon" class="btn btn-error btn-outline">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          Excluir
        </button>
        <NuxtLink :to="`/admin/coupons/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          Editar
        </NuxtLink>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-gray-500">Carregando detalhes do cupom...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Erro ao carregar cupom: {{ error.message }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">Tentar novamente</button>
    </div>

    <!-- Content -->
    <div v-else-if="coupon" class="grid grid-cols-1 lg:grid-cols-3 gap-6">

      <!-- Main Info Card -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <h2 class="card-title mb-4">Informações do Cupom</h2>

          <div class="space-y-4">
            <div class="flex items-center gap-4 p-4 bg-primary/10 rounded-lg">
              <span class="badge badge-primary badge-lg font-mono text-xl">{{ coupon.code }}</span>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Tipo de Desconto</span>
                </label>
                <div class="font-medium">{{ discountTypeLabel(coupon.discount_type) }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Valor do Desconto</span>
                </label>
                <div class="font-medium">{{ formatDiscountValue(coupon) }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Valor Mínimo</span>
                </label>
                <div class="font-medium">{{ coupon.minimum_amount ? `R$ ${coupon.minimum_amount.toFixed(2)}` : '-' }}</div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text text-gray-500">Desconto Máximo</span>
                </label>
                <div class="font-medium">{{ coupon.maximum_discount ? `R$ ${coupon.maximum_discount.toFixed(2)}` : '-' }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Usage Card -->
      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title mb-4">Uso do Cupom</h2>

          <div class="flex flex-col gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Usado / Limite</span>
              </label>
              <div class="font-medium text-lg">{{ coupon.used_count || 0 }} / {{ coupon.usage_limit || '∞' }}</div>
              <div class="w-full h-2 bg-base-200 rounded mt-1">
                <div 
                  class="h-full bg-primary rounded" 
                  :style="{ width: usagePercentage + '%' }"
                ></div>
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Status</span>
              </label>
              <div>
                <span :class="['badge badge-lg', coupon.active ? 'badge-success' : 'badge-error']">
                  {{ coupon.active ? 'Ativo' : 'Inativo' }}
                </span>
              </div>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text text-gray-500">Expiração</span>
              </label>
              <div class="font-medium" :class="isExpired ? 'text-error' : ''">
                {{ coupon.expires_at ? formatDate(coupon.expires_at) : 'Não expira' }}
              </div>
            </div>

            <div class="divider my-2"></div>

            <div class="text-xs text-gray-500 space-y-2">
              <div class="flex justify-between">
                <span>Criado em:</span>
                <span class="font-medium">{{ formatDate(coupon.created_at) }}</span>
              </div>
              <div class="flex justify-between">
                <span>Atualizado em:</span>
                <span class="font-medium">{{ formatDate(coupon.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Not Found State -->
    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>Cupom não encontrado.</span>
      <NuxtLink to="/admin/coupons" class="btn btn-sm">Voltar para lista</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Coupon } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const config = useRuntimeConfig()
const router = useRouter()

const { pending, data: coupon, error, refresh } = useFetch<Coupon>(
  `${config.public.baseURL}/api/coupons/${route.params.id}`
)

const discountTypeLabel = (type?: number) => {
  switch (type) {
    case 1: return 'Porcentagem'
    case 2: return 'Valor Fixo'
    case 3: return 'Frete Grátis'
    default: return 'Desconhecido'
  }
}

const formatDiscountValue = (coupon: Coupon) => {
  if (coupon.discount_type === 1) {
    return `${coupon.discount_value}%`
  } else if (coupon.discount_type === 2) {
    return `R$ ${coupon.discount_value?.toFixed(2) || '0.00'}`
  }
  return '-'
}

const usagePercentage = computed(() => {
  if (!coupon.value?.usage_limit || coupon.value.usage_limit === 0) return 0
  const used = coupon.value.used_count || 0
  return Math.min((used / coupon.value.usage_limit) * 100, 100)
})

const isExpired = computed(() => {
  if (!coupon.value?.expires_at) return false
  return new Date(coupon.value.expires_at) < new Date()
})

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

const deleteCoupon = async () => {
  if (!coupon.value) return

  if (confirm(`Tem certeza que deseja excluir o cupom "${coupon.value.code}"?`)) {
    try {
      await $fetch(`${config.public.baseURL}/api/coupons/${coupon.value.id}`, {
        method: 'DELETE'
      })
      router.push('/admin/coupons')
    } catch (err) {
      alert('Erro ao excluir cupom')
      console.error(err)
    }
  }
}
</script>
