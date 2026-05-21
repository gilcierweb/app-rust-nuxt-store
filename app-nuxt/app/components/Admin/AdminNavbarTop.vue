<template>
  <!-- ---------- HEADER ---------- -->
  <div class="bg-base-100 border-base-content/20 lg:ps-75 sticky top-0 z-50 flex border-b">
    <div class="mx-auto w-full max-w-7xl">
      <nav class="navbar py-2 px-4 md:px-6">
        <div class="navbar-start items-center gap-2">
          <!-- Mobile Toggle -->
          <button type="button" class="btn btn-soft btn-square btn-sm lg:hidden" aria-haspopup="dialog" aria-expanded="false" aria-controls="layout-sidebar" data-overlay="#layout-sidebar">
            <span class="icon-[tabler--menu-2] size-4.5"></span>
          </button>

          <!-- Search  -->
          <div class="input no-focus border-0 px-0 hidden md:flex items-center">
            <span class="icon-[tabler--search] text-base-content/80 my-auto me-2 size-4 shrink-0"></span>
            <input 
              type="search" 
              class="grow placeholder:text-sm bg-transparent border-none focus:ring-0" 
              :placeholder="$t('admin.navbar.search.placeholder')" 
              id="kbdInput" 
            />
            <label class="sr-only" for="kbdInput">{{ t('admin.navbar.search.ariaLabel') }}</label>
          </div>
        </div>

        <!-- Breadcrumbs (Localized) -->
        <div class="hidden xl:flex flex-1 justify-center">
          <div class="flex items-center gap-2 text-[10px] uppercase tracking-[0.2em] font-bold">
            <span class="text-base-content/40">{{ $t('admin.navbar.breadcrumb.admin') }}</span>
            <span class="icon-[tabler--chevron-right] text-base-content/20 size-3"></span>
            <span class="text-primary">{{ localizedRouteName }}</span>
          </div>
        </div>

        <div class="navbar-end items-center gap-4">
          <!-- Language Switcher -->
          <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
            <button id="lang-dropdown" type="button" class="dropdown-toggle btn btn-text btn-circle size-10" aria-haspopup="menu" aria-expanded="false">
              <span class="icon-[tabler--language] size-5.5"></span>
            </button>
            <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-32 shadow-xl border border-base-content/10 mt-2" role="menu">
              <li v-for="lang in locales" :key="lang.code">
                <button @click="setLocale(lang.code)" class="dropdown-item flex items-center gap-2" :class="{ 'active': locale === lang.code }">
                  <span class="text-lg">{{ (lang as any).flag }}</span>
                  <span>{{ lang.name }}</span>
                </button>
              </li>
            </ul>
          </div>

          <!-- Activity/Notifications Button -->
          <button type="button" class="btn btn-text btn-circle size-10" data-overlay="#activity-drawer">
            <div class="indicator">
              <span class="indicator-item bg-primary size-2 rounded-full border-2 border-base-100"></span>
              <span class="icon-[tabler--bell] size-5.5"></span>
            </div>
          </button>

          <!-- Profile Dropdown -->
          <div class="dropdown relative inline-flex [--offset:15] [--placement:bottom-end]">
            <button id="profile-dropdown" type="button" class="dropdown-toggle avatar" aria-haspopup="menu" aria-expanded="false" aria-label="Dropdown">
              <div class="size-9.5 rounded-full border border-primary/20 p-0.5">
                <img :src="user?.avatar || 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png'" class="rounded-full" :alt="$t('admin.navbar.profile.avatarAlt')" />
              </div>
            </button>
            <ul class="dropdown-menu dropdown-open:opacity-100 max-w-75 hidden w-full space-y-0.5 shadow-xl border border-base-content/10 mt-2" role="menu">
              <li class="dropdown-header pt-4.5 mb-1 gap-4 px-5 pb-3.5 border-b border-base-content/5">
                <div class="avatar avatar-online-top">
                  <div class="w-10 rounded-full border border-primary/10 p-0.5">
                    <img :src="user?.avatar || 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png'" class="rounded-full" :alt="$t('admin.sidebar.avatarAlt')" />
                  </div>
                </div>
                <div class="min-w-0">
                  <h6 class="text-base-content mb-0.5 font-semibold truncate">{{ user?.name || $t('admin.navbar.profile.defaultName') }}</h6>
                  <p class="text-base-content/60 text-xs font-medium truncate">{{ user?.email || $t('admin.navbar.profile.defaultEmail') }}</p>
                </div>
              </li>
               <li>
                <NuxtLinkLocale to="/account" class="dropdown-item px-4">
                  <span class="icon-[tabler--user] size-5"></span>
                  {{ $t('admin.navbar.profile.myAccount') }}
                </NuxtLinkLocale>
              </li>
              <li>
                <NuxtLinkLocale to="/admin/settings" class="dropdown-item px-4">
                  <span class="icon-[tabler--settings] size-5"></span>
                  {{ $t('admin.navbar.profile.settings') }}
                </NuxtLinkLocale>
              </li>
              <li>
                <hr class="border-base-content/10 -mx-2 my-1" />
              </li>
              <li class="dropdown-footer p-2">
                <button @click="handleLogout" class="btn btn-text btn-error btn-block h-11 justify-start px-3 font-normal">
                  <span class="icon-[tabler--logout] size-5"></span>
                  {{ $t('admin.navbar.profile.logout') }}
                </button>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    </div>
  </div>

  <!-- Activity Drawer Content  -->
  <div id="activity-drawer" class="overlay overlay-open:translate-x-0 drawer drawer-end sm:max-w-104 hidden" role="dialog" tabindex="-1">
    <div class="drawer-header border-base-content/20 border-b p-4">
      <h3 class="drawer-title text-base font-semibold">{{ $t('admin.navbar.activity.title') }}</h3>
      <button type="button" class="btn btn-text btn-circle btn-xs" :aria-label="$t('admin.navbar.close')" data-overlay="#activity-drawer">
        <span class="icon-[tabler--x] size-4"></span>
      </button>
    </div>
    <div class="drawer-body p-0 overflow-y-auto">
      <div v-if="auditPending" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary size-8"></span>
      </div>
      <ul v-else-if="activities.length > 0" class="divide-y divide-base-content/10">
        <li v-for="activity in activities" :key="activity.id" class="flex items-start gap-4 p-4 hover:bg-base-200/50 transition-colors">
          <div class="avatar">
            <div class="size-9 rounded-full border border-base-content/10">
              <img :src="activity.avatar" :alt="activity.user" />
            </div>
          </div>
          <div class="flex-1 min-w-0">
            <div class="mb-1 leading-snug">
              <span class="text-base-content font-bold text-sm mr-1">{{ activity.user }}</span>
              <span class="text-base-content/80 text-sm">{{ activity.message }}</span>
            </div>
            <p class="text-base-content/40 text-[11px]">{{ activity.time }}</p>
            
            <div v-if="activity.content" class="mt-3 bg-base-200/50 rounded-lg border border-base-content/10 p-3">
               <p class="text-base-content text-xs italic opacity-80">"{{ activity.content }}"</p>
            </div>
          </div>
        </li>
      </ul>
      <div v-else class="flex flex-col items-center justify-center py-12 text-center">
        <span class="icon-[tabler--activity] text-base-content/30 size-12 mb-3"></span>
        <p class="text-base-content/50 text-sm">{{ t('admin.audit.empty') }}</p>
      </div>
    </div>
  </div>
  <!-- ---------- END HEADER ---------- -->
</template>

<script lang="ts" setup>
const { user, logout } = useAuth()
const route = useRoute()
const { t, locale, locales, setLocale } = useI18n()
const { useApiFetch } = useApi()

const localizedRouteName = computed(() => {
  const name = route.name?.toString() || ''
  const baseName = name.split('___')[0]
  if (!baseName || baseName === 'admin') return t('admin.sidebar.dashboard')
  const module = baseName.replace('admin-', '').split('-')[0] || ''
  if (!module) return t('admin.sidebar.dashboard')
  const translationKey = `admin.${module}.title`
  const translated = t(translationKey)
  if (translated && translated !== translationKey) return translated
  return module.charAt(0).toUpperCase() + module.slice(1).replace(/-/g, ' ')
})

interface AdminAuditLog {
  id: number
  actor_user_id?: number | null
  actor_name: string
  actor_email: string
  action: string
  resource_type: string
  resource_id?: number | null
  resource_label?: string | null
  message?: string | null
  created_at: string
}

interface AdminAuditLogResponse {
  items: AdminAuditLog[]
  total: number
  page: number
  page_size: number
}

interface PaymentGatewayEvent {
  id: number
  payment_gateway_id: number
  event_type: string
  external_event_id: string
  status: number
  signature_valid: boolean
  failure_message?: string | null
  created_at?: string | null
}

const { data: auditData, pending: auditPending } = await useApiFetch<AdminAuditLogResponse>(
  '/api/admin/audit-logs',
  {
    query: { page: 1, page_size: 8 }
  }
)

const { data: eventsData } = await useApiFetch<PaymentGatewayEvent[]>(
  '/api/admin/payment-gateway-events',
  {
    query: { limit: 5 }
  }
)

const activities = computed(() => {
  const auditLogs = auditData.value?.items || []
  const paymentEvents = eventsData.value || []
  
  const auditActivities = auditLogs.map((log) => ({
    id: log.id,
    user: log.actor_name,
    message: log.message || `${log.action} ${log.resource_type}`,
    time: getTimeAgo(log.created_at),
    avatar: `https://cdn.flyonui.com/fy-assets/avatar/avatar-${(log.id % 3) + 1}.png`,
    content: log.resource_label ? `${log.resource_type}: ${log.resource_label}` : undefined,
    type: 'audit' as const
  }))

  const eventActivities = paymentEvents
    .filter(event => event.status === 4 || !event.signature_valid)
    .slice(0, 3)
    .map((event) => ({
      id: event.id + 100000,
      user: 'Payment Gateway',
      message: `Webhook ${event.event_type} - ${event.signature_valid ? 'Valid' : 'Invalid signature'}`,
      time: getTimeAgo(event.created_at || ''),
      avatar: 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png',
      content: event.failure_message || `Event ID: ${event.external_event_id}`,
      type: 'operational' as const
    }))

  return [...auditActivities, ...eventActivities]
    .sort((a, b) => {
      const dateA = new Date(a.time.includes('ago') ? new Date().toISOString() : a.time)
      const dateB = new Date(b.time.includes('ago') ? new Date().toISOString() : b.time)
      return dateB.getTime() - dateA.getTime()
    })
    .slice(0, 10)
})

function getTimeAgo(dateString: string): string {
  const now = new Date()
  const past = new Date(dateString)
  const diffMs = now.getTime() - past.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMs / 3600000)
  const diffDays = Math.floor(diffMs / 86400000)

  if (diffMins < 1) return t('admin.dashboard.activity.time.minsAgo', { n: 1 })
  if (diffMins < 60) return t('admin.dashboard.activity.time.minsAgo', { n: diffMins })
  if (diffHours === 1) return t('admin.dashboard.activity.time.hourAgo')
  if (diffHours < 24) return t('admin.dashboard.activity.time.hoursAgo', { n: diffHours })
  return t('admin.dashboard.activity.time.daysAgo', { n: diffDays })
}

function handleLogout() {
  logout()
}
</script>

<style scoped>
.navbar {
  backdrop-filter: blur(8px);
}
</style>
