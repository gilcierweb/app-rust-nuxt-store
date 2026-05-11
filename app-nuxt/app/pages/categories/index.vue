<template>
  <div class="min-h-screen bg-base-50/30 pb-20">
    <!-- Hero Section -->
    <section class="relative bg-neutral-900 overflow-hidden mb-12">
      <!-- Gradient Background -->
      <div class="absolute inset-0 opacity-40">
        <div class="absolute -top-[10%] -left-[10%] w-[40%] h-[40%] rounded-full bg-primary/30 blur-[120px]"></div>
        <div class="absolute top-[20%] -right-[5%] w-[30%] h-[30%] rounded-full bg-secondary/20 blur-[100px]"></div>
        <div class="absolute -bottom-[10%] left-[20%] w-[35%] h-[35%] rounded-full bg-accent/20 blur-[110px]"></div>
      </div>

      <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24 lg:py-32">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div class="text-left">
            <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white/10 backdrop-blur-md border border-white/10 mb-6">
              <span class="size-2 rounded-full bg-primary animate-pulse"></span>
              <span class="text-xs font-medium text-white/80 uppercase tracking-wider">{{ t('pages.categories.title') }}</span>
            </div>
            
            <h1 class="text-5xl lg:text-7xl font-bold text-white mb-6 leading-tight">
              Explore por <span class="text-transparent bg-clip-text bg-gradient-to-r from-primary to-secondary">Categorias</span>
            </h1>
            <p class="text-xl text-white/60 max-w-lg mb-10 leading-relaxed">{{ t('pages.categories.description') }}</p>
            
            <!-- Search Input -->
            <div class="relative max-w-md group">
              <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                <span class="icon-[tabler--search] size-5 text-white/40 group-focus-within:text-primary transition-colors"></span>
              </div>
              <input type="text" 
                :placeholder="t('pages.categories.search.placeholder')" 
                class="w-full bg-white/5 border border-white/10 rounded-2xl py-4 pl-12 pr-4 text-white placeholder:text-white/30 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary transition-all backdrop-blur-sm" />
            </div>
          </div>
          
          <!-- Decorative Stats Display -->
          <div class="hidden lg:grid grid-cols-2 gap-6">
            <div class="p-8 rounded-[2.5rem] bg-white/5 backdrop-blur-xl border border-white/10 transform hover:-translate-y-2 transition-all duration-500">
              <div class="size-14 rounded-2xl bg-primary/20 flex items-center justify-center mb-6">
                <span class="icon-[tabler--category] size-8 text-primary"></span>
              </div>
              <div class="text-4xl font-bold text-white mb-2">{{ allCategories.length }}</div>
              <div class="text-white/40 font-medium">Categorias Ativas</div>
            </div>
            <div class="p-8 rounded-[2.5rem] bg-white/5 backdrop-blur-xl border border-white/10 mt-12 transform hover:-translate-y-2 transition-all duration-500">
              <div class="size-14 rounded-2xl bg-secondary/20 flex items-center justify-center mb-6">
                <span class="icon-[tabler--box] size-8 text-secondary"></span>
              </div>
              <div class="text-4xl font-bold text-white mb-2">{{ totalProducts }}</div>
              <div class="text-white/40 font-medium">Produtos Totais</div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Section Header for Mobile/Small Screens -->
      <div class="lg:hidden grid grid-cols-2 gap-4 mb-12">
        <div class="p-6 rounded-3xl bg-base-100 border border-base-200 shadow-sm">
          <div class="text-2xl font-bold text-base-content mb-1">{{ allCategories.length }}</div>
          <div class="text-xs text-base-content/50 uppercase font-semibold">Categorias</div>
        </div>
        <div class="p-6 rounded-3xl bg-base-100 border border-base-200 shadow-sm">
          <div class="text-2xl font-bold text-base-content mb-1">{{ totalProducts }}</div>
          <div class="text-xs text-base-content/50 uppercase font-semibold">Produtos</div>
        </div>
      </div>

      <!-- Categories Grid -->
      <div v-if="pending" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        <div v-for="i in 6" :key="i" class="h-[400px] rounded-[3rem] bg-base-200/50 animate-pulse"></div>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        <div v-for="cat in allCategories" :key="cat.id" 
          class="group relative h-[420px] rounded-[3rem] overflow-hidden bg-base-100 border border-base-200 transition-all duration-500 hover:shadow-2xl hover:shadow-primary/10">
          
          <!-- Background Pattern -->
          <div class="absolute top-0 right-0 p-8 opacity-[0.03] group-hover:opacity-[0.08] transition-opacity duration-500">
            <span :class="[cat.icon || 'icon-[tabler--category]', 'size-48']"></span>
          </div>

          <div class="h-full flex flex-col p-10">
            <!-- Header -->
            <div class="flex justify-between items-start mb-8">
              <div class="size-16 rounded-3xl bg-primary/10 flex items-center justify-center group-hover:bg-primary transition-colors duration-500">
                <span :class="[cat.icon || 'icon-[tabler--category]', 'size-8 text-primary group-hover:text-white transition-colors duration-500']"></span>
              </div>
              
              <!-- Refined Dropdown Menu -->
              <div class="dropdown dropdown-end">
                <button tabindex="0" class="btn btn-ghost btn-sm btn-circle opacity-40 group-hover:opacity-100 transition-opacity">
                  <span class="icon-[tabler--dots-vertical] size-5"></span>
                </button>
                <ul tabindex="0" class="dropdown-content menu p-2 shadow-xl bg-base-100 rounded-2xl w-48 border border-base-200 z-[100] mt-2">
                  <li><a @click="viewCategory(cat.id)" class="py-3 px-4 rounded-xl flex items-center gap-3">
                    <span class="icon-[tabler--eye] size-4"></span>Ver Detalhes
                  </a></li>
                  <li><a @click="editCategory(cat.id)" class="py-3 px-4 rounded-xl flex items-center gap-3 text-info">
                    <span class="icon-[tabler--edit] size-4"></span>Editar
                  </a></li>
                  <li><a @click="deleteCategory(cat.id)" class="py-3 px-4 rounded-xl flex items-center gap-3 text-error">
                    <span class="icon-[tabler--trash] size-4"></span>Excluir
                  </a></li>
                </ul>
              </div>
            </div>

            <!-- Content -->
            <div class="flex-grow">
              <div class="flex items-center gap-3 mb-3">
                <h3 class="text-2xl font-bold text-base-content">{{ cat.name }}</h3>
                <span v-if="cat.featured" class="badge badge-warning badge-sm rounded-full">Destaque</span>
              </div>
              <p class="text-base-content/50 leading-relaxed mb-8 line-clamp-2">{{ cat.description }}</p>
              
              <div class="grid grid-cols-2 gap-6 p-6 rounded-3xl bg-base-50/50 border border-base-200/50">
                <div>
                  <div class="text-2xl font-bold text-base-content">{{ cat.productsCount }}</div>
                  <div class="text-xs font-medium text-base-content/40 uppercase">Produtos</div>
                </div>
                <div>
                  <div class="text-2xl font-bold text-base-content">{{ cat.subcategories }}</div>
                  <div class="text-xs font-medium text-base-content/40 uppercase">Subcategorias</div>
                </div>
              </div>
            </div>

            <!-- Footer Action -->
            <div class="mt-8 flex items-center justify-between">
              <NuxtLink :to="`/categories/${cat.id}`" class="flex items-center gap-2 text-primary font-bold hover:gap-4 transition-all">
                Explorar Agora
                <span class="icon-[tabler--arrow-right] size-5"></span>
              </NuxtLink>
              
              <button @click="toggleFavorite(cat.id)" class="btn btn-ghost btn-sm btn-circle hover:bg-error/10 transition-colors">
                <span :class="[cat.isFavorite ? 'icon-[tabler--heart-filled] text-error' : 'icon-[tabler--heart] text-base-content/20', 'size-5']"></span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="!pending && allCategories.length === 0" class="text-center py-20">
        <div class="size-24 rounded-full bg-base-200 flex items-center justify-center mx-auto mb-6">
          <span class="icon-[tabler--category-off] size-12 text-base-content/20"></span>
        </div>
        <h3 class="text-2xl font-bold mb-2">Nenhuma categoria encontrada</h3>
        <p class="text-base-content/50 mb-8">Tente ajustar sua busca ou explore todos os produtos.</p>
        <NuxtLink to="/products" class="btn btn-primary px-8 rounded-2xl">Ver Todos os Produtos</NuxtLink>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig()
const { t } = useI18n()

// Add pending state for loading
const pending = ref(false)

// Mock data for demonstration
const mockCategories = [
  { id: 'f', name: 'Fashion', icon: 'icon-[tabler--shirt-filled]', description: 'Latest trends and styles for everyone in our curated selection.', productsCount: 124, featured: true, subcategories: 12, updatedAt: '2 days ago', isFavorite: false },
  { id: 't', name: 'Technology', icon: 'icon-[tabler--device-laptop]', description: 'Innovative gadgets and devices for modern living.', productsCount: 86, featured: true, subcategories: 8, updatedAt: '1 week ago', isFavorite: true },
  { id: 'b', name: 'Books', icon: 'icon-[tabler--book]', description: 'A world of knowledge and imagination in every page.', productsCount: 210, featured: false, subcategories: 15, updatedAt: '3 days ago', isFavorite: false },
  { id: 'h', name: 'Home', icon: 'icon-[tabler--home-dot]', description: 'Everything for a cozy and modern home environment.', productsCount: 54, featured: false, subcategories: 6, updatedAt: '5 days ago', isFavorite: false },
  { id: 'v', name: 'Vehicles', icon: 'icon-[tabler--car]', description: 'Automotive accessories and parts for enthusiasts.', productsCount: 32, featured: false, subcategories: 4, updatedAt: '1 week ago', isFavorite: false },
  { id: 'a', name: 'Art', icon: 'icon-[tabler--palette]', description: 'Supplies and inspiration for creatives and artists.', productsCount: 45, featured: true, subcategories: 9, updatedAt: '2 weeks ago', isFavorite: true }
]

const allCategories = ref(mockCategories)
const totalProducts = computed(() => allCategories.value.reduce((sum, cat) => sum + (cat.productsCount || 0), 0))

// SEO
useSeoMeta({
  title: t('pages.categories.title'),
  ogTitle: t('pages.categories.title'),
  description: t('pages.categories.description'),
  ogDescription: t('pages.categories.description'),
})

// Methods
function viewCategory(id: string) {
  console.log('View category:', id)
}

function editCategory(id: string) {
  console.log('Edit category:', id)
}

function deleteCategory(id: string) {
  console.log('Delete category:', id)
}

function toggleFavorite(id: string) {
  const cat = allCategories.value.find(c => c.id === id)
  if (cat) cat.isFavorite = !cat.isFavorite
}
</script>

<style scoped>
/* Avoiding @apply as much as possible to prevent build errors in current environment */
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>