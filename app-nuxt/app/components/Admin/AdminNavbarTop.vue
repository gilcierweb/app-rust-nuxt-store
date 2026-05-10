<template>
  <nav class="navbar bg-base-100 border-b border-base-content/10 px-4 py-2 sticky top-0 z-50">
    <div class="flex items-center gap-4 w-full">
      <!-- Sidebar Toggle (Mobile) -->
      <button type="button" class="btn btn-text btn-square sm:hidden" aria-haspopup="dialog"
        aria-expanded="false" aria-controls="with-navbar-sidebar" data-overlay="#with-navbar-sidebar">
        <span class="icon-[tabler--menu-2] size-6"></span>
      </button>

      <!-- Breadcrumbs / Title -->
      <div class="flex-1 flex items-center gap-4">
        <div class="hidden md:flex items-center gap-2 text-sm text-base-content/60">
          <NuxtLink to="/admin" class="hover:text-primary transition-colors">Admin</NuxtLink>
          <span class="icon-[tabler--chevron-right] size-4"></span>
          <span class="text-base-content font-medium capitalize">{{ currentRouteName }}</span>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex items-center gap-2 md:gap-4">
        
        <!-- Search Bar (Desktop) -->
        <div class="hidden lg:flex relative group">
          <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary transition-colors"></span>
          <input 
            type="text" 
            :placeholder="$t('admin.navbar.search.placeholder')" 
            class="input input-sm input-bordered pl-10 w-64 focus:border-primary focus:ring-1 focus:ring-primary transition-all bg-base-200/50"
          />
        </div>

        <!-- Language Switcher -->
        <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
          <button id="lang-dropdown" type="button" class="dropdown-toggle btn btn-text btn-circle size-10" aria-haspopup="menu" aria-expanded="false">
            <span class="icon-[tabler--language] size-5.5"></span>
          </button>
          <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-32 shadow-lg border border-base-content/10" role="menu">
            <li v-for="lang in availableLocales" :key="lang.code">
              <button @click="setLocale(lang.code)" class="dropdown-item flex items-center gap-2" :class="{ 'active': locale === lang.code }">
                <span class="text-lg">{{ lang.flag }}</span>
                <span>{{ lang.name }}</span>
              </button>
            </li>
          </ul>
        </div>

        <!-- Notifications -->
        <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
          <button id="notify-dropdown" type="button" class="dropdown-toggle btn btn-text btn-circle size-10" aria-haspopup="menu" aria-expanded="false">
            <div class="indicator">
              <span class="indicator-item bg-primary size-2 rounded-full border-2 border-base-100"></span>
              <span class="icon-[tabler--bell] size-5.5"></span>
            </div>
          </button>
          <div class="dropdown-menu dropdown-open:opacity-100 hidden w-80 shadow-xl border border-base-content/10" role="menu">
            <div class="dropdown-header flex justify-between items-center border-b border-base-content/5 pb-3">
              <h6 class="text-base font-bold">{{ $t('admin.navbar.activity.title') }}</h6>
              <span class="badge badge-primary badge-sm">4 New</span>
            </div>
            <div class="max-h-80 overflow-y-auto py-2">
              <div v-for="n in 3" :key="n" class="dropdown-item flex items-start gap-3 p-3 hover:bg-base-200/50 transition-colors cursor-pointer border-b border-base-content/5 last:border-0">
                <div class="avatar avatar-sm placeholder">
                  <div class="bg-primary/10 text-primary rounded-full size-10">
                    <span class="icon-[tabler--user] size-5"></span>
                  </div>
                </div>
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-medium truncate">New order #{{ 1024 + n }}</p>
                  <p class="text-xs text-base-content/60 truncate">Received 2 minutes ago</p>
                </div>
              </div>
            </div>
            <div class="dropdown-footer p-2 text-center border-t border-base-content/5">
              <button class="btn btn-ghost btn-xs btn-block text-primary">View all notifications</button>
            </div>
          </div>
        </div>

        <div class="divider divider-horizontal mx-0 hidden md:flex"></div>

        <!-- User Profile -->
        <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
          <button id="profile-dropdown" type="button" class="dropdown-toggle flex items-center gap-2 hover:bg-base-200 p-1 pr-2 rounded-full transition-colors" aria-haspopup="menu" aria-expanded="false">
            <div class="avatar">
              <div class="size-8 rounded-full border border-primary/20 p-0.5">
                <img :src="user?.avatar || 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png'" class="rounded-full" alt="avatar" />
              </div>
            </div>
            <div class="hidden md:block text-left">
              <p class="text-xs font-bold truncate max-w-24">{{ user?.name || 'Admin' }}</p>
              <p class="text-[10px] text-base-content/50 uppercase tracking-wider">Super Admin</p>
            </div>
            <span class="icon-[tabler--chevron-down] size-4 text-base-content/40 hidden md:block"></span>
          </button>
          <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-56 shadow-xl border border-base-content/10 mt-2" role="menu">
            <li class="dropdown-header gap-3 border-b border-base-content/5 pb-3 mb-2">
              <div class="avatar">
                <div class="size-10 rounded-full border border-primary/20 p-0.5">
                  <img :src="user?.avatar || 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png'" class="rounded-full" alt="avatar" />
                </div>
              </div>
              <div class="min-w-0">
                <h6 class="text-sm font-bold truncate">{{ user?.name || 'Administrator' }}</h6>
                <p class="text-xs text-base-content/50 truncate">{{ user?.email || 'admin@store.com' }}</p>
              </div>
            </li>
            <li>
              <NuxtLink to="/admin/settings" class="dropdown-item">
                <span class="icon-[tabler--settings] size-5"></span>
                {{ $t('admin.navbar.profile.settings') }}
              </NuxtLink>
            </li>
            <li>
              <a href="#" class="dropdown-item">
                <span class="icon-[tabler--help-triangle] size-5"></span>
                {{ $t('common.help') || 'Help Center' }}
              </a>
            </li>
            <div class="divider my-1 border-base-content/5"></div>
            <li>
              <button class="dropdown-item text-error hover:bg-error/10" @click="handleLogout">
                <span class="icon-[tabler--logout] size-5"></span>
                {{ $t('admin.navbar.profile.logout') }}
              </button>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </nav>
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
