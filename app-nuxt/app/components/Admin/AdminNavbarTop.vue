<template>
  <div class="bg-base-100 border-b border-base-content/10 lg:ps-75 sticky top-0 z-50 flex">
    <div class="mx-auto w-full max-w-7xl">
      <nav class="navbar py-2 px-4 md:px-6">
        <div class="navbar-start flex items-center gap-4">
          <!-- Mobile Toggle -->
          <button type="button" class="btn btn-text btn-square lg:hidden" aria-haspopup="dialog"
            aria-expanded="false" aria-controls="with-navbar-sidebar" data-overlay="#with-navbar-sidebar">
            <span class="icon-[tabler--menu-2] size-5.5"></span>
          </button>

          <!-- Search (Integrated Pattern) -->
          <div class="hidden md:flex items-center gap-2 bg-base-200/50 px-3 py-1.5 rounded-lg border border-transparent focus-within:border-primary/30 focus-within:bg-base-100 transition-all">
            <span class="icon-[tabler--search] text-base-content/40 size-4.5"></span>
            <input 
              type="text" 
              :placeholder="$t('admin.navbar.search.placeholder')" 
              class="bg-transparent border-none focus:ring-0 text-sm w-48 lg:w-64"
            />
          </div>
        </div>

        <!-- Breadcrumbs (Legacy from my previous improvement, adapted) -->
        <div class="hidden xl:flex flex-1 justify-center">
           <div class="flex items-center gap-2 text-xs text-base-content/40 uppercase tracking-widest font-bold">
              <span>Admin</span>
              <span class="icon-[tabler--chevron-right] size-3"></span>
              <span class="text-primary">{{ currentRouteName }}</span>
           </div>
        </div>

        <div class="navbar-end flex items-center gap-2 md:gap-4">
          <!-- Language Switcher -->
          <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
            <button id="lang-dropdown" type="button" class="dropdown-toggle btn btn-text btn-circle size-10" aria-haspopup="menu" aria-expanded="false">
              <span class="icon-[tabler--language] size-5.5"></span>
            </button>
            <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-32 shadow-xl border border-base-content/10 mt-2" role="menu">
              <li v-for="lang in availableLocales" :key="lang.code">
                <button @click="setLocale(lang.code)" class="dropdown-item flex items-center gap-2" :class="{ 'active': locale === lang.code }">
                  <span class="text-lg">{{ lang.flag }}</span>
                  <span>{{ lang.name }}</span>
                </button>
              </li>
            </ul>
          </div>

          <!-- Notifications -->
          <button class="btn btn-text btn-circle size-10">
            <div class="indicator">
              <span class="indicator-item bg-primary size-2 rounded-full border-2 border-base-100"></span>
              <span class="icon-[tabler--bell] size-5.5"></span>
            </div>
          </button>

          <!-- Profile Dropdown -->
          <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
            <button id="profile-dropdown" type="button" class="dropdown-toggle avatar" aria-haspopup="menu" aria-expanded="false">
              <div class="size-9.5 rounded-full border border-primary/20 p-0.5">
                <img :src="user?.avatar || 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png'" class="rounded-full" alt="avatar" />
              </div>
            </button>
            <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-60 shadow-xl border border-base-content/10 mt-2" role="menu">
              <li class="dropdown-header gap-3 border-b border-base-content/5 pb-3 mb-2 px-5 pt-4">
                <div class="avatar avatar-online-top">
                  <div class="w-10 rounded-full border border-primary/10 p-0.5">
                    <img :src="user?.avatar || 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png'" class="rounded-full" alt="avatar" />
                  </div>
                </div>
                <div class="min-w-0">
                  <h6 class="text-sm font-bold truncate">{{ user?.name || 'Administrator' }}</h6>
                  <p class="text-xs text-base-content/50 truncate">{{ user?.email || 'admin@store.com' }}</p>
                </div>
              </li>
              <li>
                <NuxtLink to="/admin/settings" class="dropdown-item px-4">
                  <span class="icon-[tabler--user] size-5"></span>
                  {{ $t('admin.navbar.profile.myAccount') || 'My Account' }}
                </NuxtLink>
              </li>
              <li>
                <NuxtLink to="/admin/settings" class="dropdown-item px-4">
                  <span class="icon-[tabler--settings] size-5"></span>
                  {{ $t('admin.navbar.profile.settings') }}
                </NuxtLink>
              </li>
              <div class="divider my-1 border-base-content/5"></div>
              <li class="p-2">
                <button class="btn btn-error btn-soft btn-block btn-sm" @click="handleLogout">
                  <span class="icon-[tabler--logout] size-4"></span>
                  {{ $t('admin.navbar.profile.logout') }}
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
const { user, logout } = useAuth()
const route = useRoute()
const { locale, setLocale } = useI18n()

const availableLocales = [
  { code: 'pt-BR', name: 'Português', flag: '🇧🇷' },
  { code: 'en', name: 'English', flag: '🇺🇸' },
  { code: 'es', name: 'Español', flag: '🇪🇸' }
]

const currentRouteName = computed(() => {
  const name = route.name?.toString() || ''
  return name.replace('admin-', '').replace(/-/g, ' ') || 'Dashboard'
})

function handleLogout() {
  logout()
}
</script>

<style scoped>
.navbar {
  backdrop-filter: blur(8px);
  background-color: rgba(var(--b1), 0.8);
}
</style>
