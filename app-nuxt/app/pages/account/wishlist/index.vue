<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">Account</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">{{ $t('account.wishlistTitle') }}</h1>
        <p class="text-base-content/60 mt-1">{{ $t('account.wishlistSubtitle') }}</p>
      </div>
      <NuxtLinkLocale to="/products" class="btn btn-primary">
        <span class="icon-[tabler--shopping-bag] size-5"></span>
        {{ $t('account.viewProducts') }}
      </NuxtLinkLocale>
    </div>

    <div v-if="pending" class="rounded-box border border-base-content/10 bg-base-100 p-8">
      <div class="flex items-center gap-3 text-base-content/60">
        <span class="loading loading-spinner loading-sm"></span>
        {{ $t('account.loadingWishlist') }}
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span class="icon-[tabler--alert-circle] size-6"></span>
      <span>{{ $t('account.errorLoadingWishlist') }} {{ error.message }}</span>
    </div>

    <div v-else-if="wishlist.length === 0" class="rounded-box border border-dashed border-base-content/20 bg-base-100 p-10 text-center">
      <span class="icon-[tabler--heart-off] text-base-content/30 mx-auto mb-4 size-12"></span>
      <h2 class="text-lg font-semibold">{{ $t('account.wishlistEmptyTitle') }}</h2>
      <p class="text-base-content/60 mt-1">{{ $t('account.wishlistEmptyMessage') }}</p>
      <NuxtLinkLocale to="/products" class="btn btn-primary mt-6">
        {{ $t('account.exploreProducts') }}
      </NuxtLinkLocale>
    </div>

    <div v-else class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
      <article v-for="item in wishlist" :key="item.id" class="rounded-box border border-base-content/10 bg-base-100 p-5 shadow-sm">
        <div class="mb-5 flex items-start justify-between gap-4">
          <div class="min-w-0">
            <h2 class="truncate font-semibold">{{ $t('account.productNumber') }}{{ item.product_id }}</h2>
            <p class="text-base-content/60 mt-1 text-sm">{{ $t('account.addedOn') }} {{ formatDate(item.created_at) }}</p>
          </div>
          <span class="icon-[tabler--heart-filled] text-error size-6"></span>
        </div>

        <div class="flex items-center gap-3">
          <NuxtLinkLocale :to="`/products/${item.product_id}`" class="btn btn-primary btn-sm flex-1">
            {{ $t('account.viewProduct') }}
          </NuxtLinkLocale>
          <button class="btn btn-error btn-soft btn-square btn-sm" :disabled="removingId === item.id" :aria-label="$t('account.remove')" @click="removeItem(item)">
            <span v-if="removingId === item.id" class="loading loading-spinner loading-xs"></span>
            <span v-else class="icon-[tabler--trash] size-4"></span>
          </button>
        </div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WishlistItem } from '~/types'

definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

const { locale, t } = useI18n()

useSeoMeta({
  title: t('account.wishlistTitle')
})
const { apiFetch, useApiFetch } = useApi()

const removingId = ref<number | null>(null)
const { data, pending, error, refresh } = await useApiFetch<WishlistItem[]>(
  '/api/account/wishlist',
  { key: 'account-wishlist' }
)

const wishlist = computed(() => data.value ?? [])

async function removeItem(item: WishlistItem) {
  removingId.value = item.id
  try {
    await apiFetch(`/api/account/wishlist/remove?id=${item.id}`)
    await refresh()
  } finally {
    removingId.value = null
  }
}

function formatDate(value?: string): string {
  if (!value) return '-'
  return new Intl.DateTimeFormat(locale.value || 'pt-BR', {
    dateStyle: 'short',
    timeStyle: 'short'
  }).format(new Date(value))
}
</script>
