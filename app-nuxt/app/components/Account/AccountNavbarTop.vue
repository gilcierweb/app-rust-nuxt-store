<template>
  <div class="bg-base-100 border-base-content/20 lg:ps-64 sticky top-0 z-50 flex border-b">
    <div class="mx-auto w-full max-w-7xl">
      <nav class="navbar px-4 py-2 md:px-6">
        <div class="navbar-start items-center gap-2">
          <button
            type="button"
            class="btn btn-soft btn-square btn-sm lg:hidden"
            aria-haspopup="dialog"
            aria-expanded="false"
            aria-controls="account-sidebar"
            data-overlay="#account-sidebar"
          >
            <span class="icon-[tabler--menu-2] size-4.5"></span>
          </button>

          <div class="hidden items-center gap-2 md:flex">
            <span class="icon-[tabler--user-circle] text-primary size-5"></span>
            <span class="text-sm font-semibold">Minha conta</span>
          </div>
        </div>

        <div class="hidden xl:flex flex-1 justify-center">
          <div class="flex items-center gap-2 text-[10px] font-bold uppercase tracking-[0.2em]">
            <span class="text-base-content/40">Account</span>
            <span class="icon-[tabler--chevron-right] text-base-content/20 size-3"></span>
            <span class="text-primary">{{ currentSection }}</span>
          </div>
        </div>

        <div class="navbar-end items-center gap-2 md:gap-4">
          <NuxtLinkLocale to="/products" class="btn btn-text btn-circle size-10" aria-label="Produtos">
            <span class="icon-[tabler--shopping-bag] size-5.5"></span>
          </NuxtLinkLocale>

          <NuxtLinkLocale to="/cart" class="btn btn-text btn-circle size-10" aria-label="Carrinho">
            <span class="icon-[tabler--shopping-cart] size-5.5"></span>
          </NuxtLinkLocale>

          <div class="dropdown relative inline-flex [--offset:15] [--placement:bottom-end]">
            <button
              id="account-profile-dropdown"
              type="button"
              class="dropdown-toggle avatar placeholder"
              aria-haspopup="menu"
              aria-expanded="false"
              aria-label="Conta"
            >
              <div class="bg-primary/10 text-primary size-9.5 rounded-full border border-primary/20">
                <span class="text-sm font-bold">{{ userInitials }}</span>
              </div>
            </button>

            <ul class="dropdown-menu dropdown-open:opacity-100 mt-2 hidden w-72 space-y-0.5 border border-base-content/10 shadow-xl" role="menu">
              <li class="dropdown-header gap-4 border-b border-base-content/5 px-5 pb-3.5 pt-4.5">
                <div class="avatar placeholder">
                  <div class="bg-primary/10 text-primary w-10 rounded-full">
                    <span class="text-sm font-bold">{{ userInitials }}</span>
                  </div>
                </div>
                <div class="min-w-0">
                  <h6 class="text-base-content mb-0.5 truncate font-semibold">{{ user?.name || 'Cliente' }}</h6>
                  <p class="text-base-content/60 truncate text-xs font-medium">{{ user?.email || '-' }}</p>
                </div>
              </li>
              <li>
                <NuxtLinkLocale to="/account" class="dropdown-item px-4">
                  <span class="icon-[tabler--layout-dashboard] size-5"></span>
                  Resumo
                </NuxtLinkLocale>
              </li>
              <li>
                <NuxtLinkLocale to="/account/settings" class="dropdown-item px-4">
                  <span class="icon-[tabler--settings] size-5"></span>
                  Configuracoes
                </NuxtLinkLocale>
              </li>
              <li v-if="user?.can_manage_admin">
                <NuxtLinkLocale to="/admin" class="dropdown-item px-4">
                  <span class="icon-[tabler--shield] size-5"></span>
                  Administracao
                </NuxtLinkLocale>
              </li>
              <li>
                <hr class="-mx-2 my-1 border-base-content/10" />
              </li>
              <li class="dropdown-footer p-2">
                <button class="btn btn-text btn-error btn-block h-11 justify-start px-3 font-normal" @click="handleLogout">
                  <span class="icon-[tabler--logout] size-5"></span>
                  Sair
                </button>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    </div>
  </div>
</template>

<script lang="ts" setup>
const route = useRoute()
const { user, logout } = useAuth()

const sectionLabels: Record<string, string> = {
  account: 'Resumo',
  orders: 'Pedidos',
  wishlist: 'Wishlist',
  addresses: 'Enderecos',
  settings: 'Configuracoes'
}

const currentSection = computed(() => {
  const parts = route.path.split('/').filter(Boolean)
  const accountIndex = parts.indexOf('account')
  const segment = accountIndex >= 0 ? parts[accountIndex + 1] || 'account' : 'account'
  return sectionLabels[segment] || segment.charAt(0).toUpperCase() + segment.slice(1)
})

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
.navbar {
  backdrop-filter: blur(8px);
}
</style>
