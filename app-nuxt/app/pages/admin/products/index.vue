<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.products.title') }}</h1>
      <NuxtLinkLocale to="/admin/products/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.products.add') }}
      </NuxtLinkLocale>
    </div>

    <!-- Filters & Search -->
    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <div class="flex flex-wrap gap-4 items-end">
          <div class="form-control flex-1 min-w-[240px]">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">Buscar Produto</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 group-focus-within:text-primary transition-colors"></span>
              <input 
                v-model="searchQuery" 
                type="text" 
                :placeholder="$t('admin.products.searchPlaceholder')" 
                class="input input-bordered w-full pl-10" 
              />
            </div>
          </div>

          <div class="form-control w-48">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">Categoria</span>
            </label>
            <select v-model="selectedCategory" class="select select-bordered w-full">
              <option value="">Todas</option>
              <option v-for="cat in categories" :key="cat.id" :value="cat.id">
                {{ cat.name }}
              </option>
            </select>
          </div>

          <div class="form-control w-48">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">Status</span>
            </label>
            <select v-model="selectedStatus" class="select select-bordered w-full">
              <option value="">Todos</option>
              <option :value="1">Ativo</option>
              <option :value="0">Inativo</option>
            </select>
          </div>

          <button class="btn btn-ghost" @click="resetFilters">
            Limpar
          </button>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-gray-500">Carregando catálogo de produtos...</p>
    </div>

    <!-- Products Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table table-lg">
          <thead class="bg-base-200/50">
            <tr>
              <th class="w-16">Foto</th>
              <th>{{ $t('admin.products.table.name') }}</th>
              <th>SKU</th>
              <th>{{ $t('admin.products.table.category') }}</th>
              <th>{{ $t('admin.products.table.price') }}</th>
              <th>{{ $t('admin.products.table.status') }}</th>
              <th class="text-right">{{ $t('admin.products.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="product in filteredProducts" :key="product.id" class="hover:bg-base-200/30 transition-colors">
              <td>
                <div class="avatar">
                  <div class="mask mask-squircle w-12 h-12 bg-base-200">
                    <img v-if="product.images?.length" :src="product.images[0].image" :alt="product.name" />
                    <div v-else class="flex items-center justify-center h-full text-gray-400">
                      <i class="icon-[tabler--photo] size-6"></i>
                    </div>
                  </div>
                </div>
              </td>
              <td>
                <div class="font-bold text-base">{{ product.name }}</div>
                <div class="text-xs text-gray-500 line-clamp-1 max-w-xs">{{ product.shortDescription || product.description }}</div>
              </td>
              <td class="font-mono text-sm text-gray-600">{{ product.sku || '-' }}</td>
              <td>
                <span v-if="product.category" class="badge badge-soft badge-sm">
                  {{ product.category.name }}
                </span>
                <span v-else class="text-gray-400">-</span>
              </td>
              <td class="font-semibold">{{ formatNumberBR(product.price) }}</td>
              <td>
                <div class="flex items-center gap-2">
                  <span :class="product.active ? 'bg-success' : 'bg-error'" class="size-2 rounded-full"></span>
                  <span class="text-sm">{{ product.active ? 'Ativo' : 'Inativo' }}</span>
                </div>
              </td>
              <td class="text-right">
                <div class="flex justify-end gap-1">
                  <NuxtLinkLocale :to="`/admin/products/${product.id}`" class="btn btn-circle btn-text btn-sm" :aria-label="$t('common.view')">
                    <i class="icon-[tabler--eye] size-5"></i>
                  </NuxtLinkLocale>
                  <NuxtLinkLocale :to="`/admin/products/${product.id}/edit`" class="btn btn-circle btn-text btn-sm" :aria-label="$t('common.edit')">
                    <i class="icon-[tabler--pencil] size-5"></i>
                  </NuxtLinkLocale>
                  <button class="btn btn-circle btn-text btn-sm text-error" @click="confirmDelete(product)">
                    <i class="icon-[tabler--trash] size-5"></i>
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="filteredProducts.length === 0">
              <td colspan="7" class="text-center py-20 text-gray-500 italic">
                Nenhum produto encontrado com os filtros selecionados.
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProductApi, Category } from '~/types'

definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiFetch } = useApi()
const { t } = useI18n()
const toast = useAppToast()
const dialog = useAppDialog()

// Filters state
const searchQuery = ref('')
const selectedCategory = ref('')
const selectedStatus = ref('')

const [
  { pending, data: productsData, refresh },
  { data: categories }
] = await Promise.all([
  useApiFetch<ProductApi[]>('/api/admin/products', { key: 'admin-products-list' }),
  useApiFetch<Category[]>('/api/admin/categories', { key: 'admin-categories-list' })
])

const products = computed(() => productsData.value || [])

// Filtered data
const filteredProducts = computed(() => {
  return products.value.filter(p => {
    const matchesSearch = p.name.toLowerCase().includes(searchQuery.value.toLowerCase()) || 
                         p.sku?.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesCategory = !selectedCategory.value || p.categoryId === Number(selectedCategory.value)
    const matchesStatus = selectedStatus.value === '' || p.active === (selectedStatus.value === '1')
    
    return matchesSearch && matchesCategory && matchesStatus
  })
})

const resetFilters = () => {
  searchQuery.value = ''
  selectedCategory.value = ''
  selectedStatus.value = ''
}

const formatNumberBR = (num: number | undefined) => {
  return new Intl.NumberFormat(getAppLocale(), { style: 'currency', currency: 'BRL' }).format(num || 0)
}

const confirmDelete = async (product: ProductApi) => {
  const confirmed = await dialog.confirm({
    message: t('admin.products.confirmDelete', { name: product.name }),
    confirmLabel: t('common.delete'),
    tone: 'danger'
  })
  if (!confirmed) return

  try {
    await apiFetch(`/api/admin/products/${product.id}`, { method: 'DELETE' })
    await refresh()
  } catch (err) {
    toast.error(t('admin.products.errorDelete'))
  }
}
</script>
