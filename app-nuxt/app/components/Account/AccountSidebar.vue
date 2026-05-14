<template>
  <aside
    id="account-sidebar"
    class="overlay [--auto-close:lg] lg:z-50 lg:block lg:translate-x-0 lg:shadow-none overlay-open:translate-x-0 drawer drawer-start hidden max-w-75 w-75 bg-base-100 border-e border-base-content/20 transition-transform duration-300 sm:fixed sm:inset-y-0 sm:start-0"
    role="dialog"
    tabindex="-1"
  >
    <div class="flex h-full flex-col">
      <div class="text-base-content border-base-content/10 flex flex-col items-center gap-4 border-b px-4 py-8">
        <div class="avatar placeholder">
          <div class="bg-primary/10 text-primary size-20 rounded-full border-2 border-primary/20">
            <span class="text-2xl font-bold">{{ userInitials }}</span>
          </div>
        </div>
        <div class="text-center">
          <h3 class="text-base-content text-lg font-bold">{{ user?.name || 'Cliente' }}</h3>
          <p class="text-base-content/60 max-w-[200px] truncate text-xs">{{ user?.email || '-' }}</p>
        </div>
        <div class="flex flex-wrap justify-center gap-2">
          <span v-for="role in user?.roles || []" :key="role" class="badge badge-soft badge-primary badge-sm">
            {{ role }}
          </span>
        </div>
      </div>

      <div class="h-full overflow-y-auto px-4 py-4">
        <ul class="menu gap-1 p-0">
          <li>
            <NuxtLinkLocale to="/account" class="flex items-center gap-3 rounded-lg px-4 py-3 transition-colors hover:bg-base-200" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--layout-dashboard] size-5"></i>
              Resumo
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/account/orders" class="flex items-center gap-3 rounded-lg px-4 py-2.5 transition-colors hover:bg-base-200" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--package] size-5"></i>
              Pedidos
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/account/wishlist" class="flex items-center gap-3 rounded-lg px-4 py-2.5 transition-colors hover:bg-base-200" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--heart] size-5"></i>
              Wishlist
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/account/settings" class="flex items-center gap-3 rounded-lg px-4 py-2.5 transition-colors hover:bg-base-200" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--settings] size-5"></i>
              Configuracoes
            </NuxtLinkLocale>
          </li>

          <li class="menu-title mt-6 mb-2 px-4 text-xs font-bold uppercase tracking-wider text-base-content/40">
            Loja
          </li>
          <li>
            <NuxtLinkLocale to="/products" class="flex items-center gap-3 rounded-lg px-4 py-2.5 transition-colors hover:bg-base-200">
              <i class="icon-[tabler--shopping-bag] size-5"></i>
              Produtos
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/cart" class="flex items-center gap-3 rounded-lg px-4 py-2.5 transition-colors hover:bg-base-200">
              <i class="icon-[tabler--shopping-cart] size-5"></i>
              Carrinho
            </NuxtLinkLocale>
          </li>

          <li v-if="user?.can_manage_admin" class="menu-title mt-6 mb-2 px-4 text-xs font-bold uppercase tracking-wider text-base-content/40">
            Admin
          </li>
          <li v-if="user?.can_manage_admin">
            <NuxtLinkLocale to="/admin" class="flex items-center gap-3 rounded-lg px-4 py-2.5 transition-colors hover:bg-base-200">
              <i class="icon-[tabler--shield] size-5"></i>
              Administracao
            </NuxtLinkLocale>
          </li>
        </ul>
      </div>

      <div class="mt-auto border-t border-base-content/10 p-4">
        <button v-if="isAuthenticated" class="btn btn-error btn-soft btn-block gap-2" @click="handleLogout">
          <i class="icon-[tabler--logout] size-5"></i>
          Sair
        </button>
      </div>
    </div>
  </aside>
</template>

<script lang="ts" setup>
const { isAuthenticated, user, logout } = useAuth()

const userInitials = computed(() => {
  const source = user.value?.name || user.value?.email || 'Cliente'
  return source
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part.charAt(0).toUpperCase())
    .join('')
})

function handleLogout() {
  logout()
}
</script>

<style scoped>
.overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: rgb(0 0 0 / 10%);
  border-radius: 10px;
}
</style>
