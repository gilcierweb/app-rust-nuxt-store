<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">Account</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">Minha conta</h1>
        <p class="text-base-content/60 mt-1">Acompanhe seus pedidos, wishlist e dados de acesso.</p>
      </div>
      <div class="flex flex-wrap gap-3">
        <NuxtLinkLocale to="/products" class="btn btn-primary">
          <span class="icon-[tabler--shopping-bag] size-5"></span>
          Comprar
        </NuxtLinkLocale>
        <NuxtLinkLocale to="/account/settings" class="btn btn-soft">
          <span class="icon-[tabler--settings] size-5"></span>
          Configuracoes
        </NuxtLinkLocale>
      </div>
    </div>

    <div class="grid gap-4 md:grid-cols-3">
      <section class="rounded-box border border-base-content/10 bg-base-100 p-5 shadow-sm">
        <div class="flex items-center justify-between gap-4">
          <div>
            <p class="text-base-content/50 text-sm">Pedidos</p>
            <p class="text-base-content mt-1 text-3xl font-bold">{{ orders.length }}</p>
          </div>
          <span class="icon-[tabler--package] text-primary size-9"></span>
        </div>
      </section>

      <section class="rounded-box border border-base-content/10 bg-base-100 p-5 shadow-sm">
        <div class="flex items-center justify-between gap-4">
          <div>
            <p class="text-base-content/50 text-sm">Wishlist</p>
            <p class="text-base-content mt-1 text-3xl font-bold">{{ wishlist.length }}</p>
          </div>
          <span class="icon-[tabler--heart] text-error size-9"></span>
        </div>
      </section>

      <section class="rounded-box border border-base-content/10 bg-base-100 p-5 shadow-sm">
        <div class="flex items-center justify-between gap-4">
          <div>
            <p class="text-base-content/50 text-sm">Perfis de acesso</p>
            <p class="text-base-content mt-1 text-3xl font-bold">{{ user?.roles?.length || 0 }}</p>
          </div>
          <span class="icon-[tabler--shield-check] text-success size-9"></span>
        </div>
      </section>
    </div>

    <div class="grid gap-6 lg:grid-cols-3">
      <section class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm lg:col-span-2">
        <div class="mb-5 flex items-center justify-between gap-4">
          <div>
            <h2 class="text-lg font-semibold">Pedidos recentes</h2>
            <p class="text-base-content/60 text-sm">Ultimas compras feitas na sua conta.</p>
          </div>
          <NuxtLinkLocale to="/account/orders" class="btn btn-soft btn-sm">
            Ver todos
            <span class="icon-[tabler--arrow-right] size-4"></span>
          </NuxtLinkLocale>
        </div>

        <div v-if="ordersPending" class="flex items-center gap-3 py-8 text-base-content/60">
          <span class="loading loading-spinner loading-sm"></span>
          Carregando pedidos...
        </div>

        <div v-else-if="orders.length === 0" class="rounded-box border border-dashed border-base-content/20 p-8 text-center">
          <span class="icon-[tabler--package-off] text-base-content/30 mx-auto mb-3 size-10"></span>
          <p class="font-semibold">Nenhum pedido encontrado</p>
          <p class="text-base-content/60 mt-1 text-sm">Quando voce comprar, seus pedidos aparecem aqui.</p>
        </div>

        <div v-else class="divide-y divide-base-content/10">
          <div v-for="order in recentOrders" :key="order.id" class="flex flex-col gap-4 py-4 md:flex-row md:items-center md:justify-between">
            <div class="min-w-0">
              <p class="font-semibold">{{ order.order_number || `Pedido #${order.id}` }}</p>
              <p class="text-base-content/60 text-sm">{{ formatDate(order.created_at) }} · {{ order.items?.length || 0 }} itens</p>
            </div>
            <div class="flex items-center justify-between gap-4 md:justify-end">
              <span :class="['badge', statusBadgeClass(order.status)]">{{ statusLabel(order.status) }}</span>
              <span class="font-bold text-primary">{{ formatCurrency(order.total_amount, order.currency) }}</span>
              <NuxtLinkLocale :to="`/account/orders/${order.id}`" class="btn btn-square btn-soft btn-sm" aria-label="Ver pedido">
                <span class="icon-[tabler--eye] size-4"></span>
              </NuxtLinkLocale>
            </div>
          </div>
        </div>
      </section>

      <aside class="space-y-6">
        <section class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h2 class="mb-4 text-lg font-semibold">Perfil</h2>
          <dl class="space-y-4">
            <div>
              <dt class="text-base-content/60 text-sm">Nome</dt>
              <dd class="font-medium">{{ user?.name || '-' }}</dd>
            </div>
            <div>
              <dt class="text-base-content/60 text-sm">E-mail</dt>
              <dd class="break-all font-medium">{{ user?.email || '-' }}</dd>
            </div>
          </dl>
        </section>

        <section class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
          <h2 class="mb-4 text-lg font-semibold">Wishlist</h2>
          <div v-if="wishlistPending" class="flex items-center gap-3 py-4 text-base-content/60">
            <span class="loading loading-spinner loading-sm"></span>
            Carregando...
          </div>
          <div v-else-if="wishlist.length === 0" class="text-base-content/60 text-sm">
            Sua wishlist esta vazia.
          </div>
          <div v-else class="space-y-3">
            <NuxtLinkLocale
              v-for="item in wishlist.slice(0, 4)"
              :key="item.id"
              :to="`/products/${item.product_id}`"
              class="flex items-center justify-between rounded-lg border border-base-content/10 px-3 py-2 hover:bg-base-200"
            >
              <span class="font-medium">Produto #{{ item.product_id }}</span>
              <span class="icon-[tabler--arrow-right] size-4"></span>
            </NuxtLinkLocale>
          </div>
        </section>
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Order, WishlistItem } from '~/types'

definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

useSeoMeta({
  title: 'Minha conta'
})

const { locale } = useI18n()
const { user, fetchCurrentUser } = useAuth()
const { useApiFetch } = useApi()

if (!user.value) {
  await fetchCurrentUser()
}

const { data: ordersData, pending: ordersPending } = await useApiFetch<Order[]>(
  '/api/account/orders',
  { key: 'account-overview-orders' }
)
const { data: wishlistData, pending: wishlistPending } = await useApiFetch<WishlistItem[]>(
  '/api/account/wishlist',
  { key: 'account-overview-wishlist' }
)

const orders = computed(() => ordersData.value ?? [])
const wishlist = computed(() => wishlistData.value ?? [])
const recentOrders = computed(() => orders.value.slice(0, 5))

const statusMap: Record<number, { label: string; badge: string }> = {
  1: { label: 'Pendente', badge: 'badge-soft badge-warning' },
  2: { label: 'Confirmado', badge: 'badge-soft badge-info' },
  3: { label: 'Processando', badge: 'badge-soft badge-info' },
  4: { label: 'Enviado', badge: 'badge-soft badge-primary' },
  5: { label: 'Entregue', badge: 'badge-soft badge-success' },
  6: { label: 'Cancelado', badge: 'badge-soft badge-error' }
}

function statusLabel(status: unknown): string {
  return statusMap[Number(status)]?.label ?? 'Desconhecido'
}

function statusBadgeClass(status: unknown): string {
  return statusMap[Number(status)]?.badge ?? 'badge-soft'
}

function formatCurrency(value: number, currency = 'BRL'): string {
  return new Intl.NumberFormat(locale.value || 'pt-BR', {
    style: 'currency',
    currency: currency || 'BRL'
  }).format(Number(value || 0))
}

function formatDate(value?: string): string {
  if (!value) return '-'
  return new Intl.DateTimeFormat(locale.value || 'pt-BR', {
    dateStyle: 'short',
    timeStyle: 'short'
  }).format(new Date(value))
}
</script>
