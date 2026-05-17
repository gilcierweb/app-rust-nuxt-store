<template>
  <div>
    <div class="mb-6 flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <h1 class="h1">{{ t('admin.banners.title') }}</h1>
        <p class="text-sm text-base-content/60">{{ t('admin.banners.description') }}</p>
      </div>
      <NuxtLinkLocale to="/admin/banners/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ t('admin.banners.add') }}
      </NuxtLinkLocale>
    </div>

    <div class="tabs tabs-bordered mb-6">
      <button class="tab" :class="{ 'tab-active': activeTab === 'banners' }" @click="activeTab = 'banners'">
        <i class="icon-[tabler--photo] size-4 mr-2"></i>
        {{ t('admin.banners.tabs.banners') }}
      </button>
      <button class="tab" :class="{ 'tab-active': activeTab === 'positions' }" @click="activeTab = 'positions'">
        <i class="icon-[tabler--layout-list] size-4 mr-2"></i>
        {{ t('admin.banners.tabs.positions') }}
      </button>
      <button class="tab" :class="{ 'tab-active': activeTab === 'analytics' }" @click="activeTab = 'analytics'">
        <i class="icon-[tabler--chart-bar] size-4 mr-2"></i>
        {{ t('admin.banners.tabs.analytics') }}
      </button>
    </div>

    <section v-if="activeTab === 'banners'">
      <div class="card bg-base-100 shadow-sm border mb-6">
        <div class="card-body p-4">
          <div class="flex flex-wrap gap-4 items-end">
            <div class="form-control flex-1 min-w-60">
              <label class="label pt-0">
                <span class="label-text-alt text-base-content/60">{{ t('admin.banners.search') }}</span>
              </label>
              <div class="relative group">
                <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
                <input v-model="searchQuery" type="text" class="input input-bordered w-full pl-10" :placeholder="t('admin.banners.searchPlaceholder')" />
              </div>
            </div>

            <div class="form-control w-52">
              <label class="label pt-0">
                <span class="label-text-alt text-base-content/60">{{ t('admin.banners.form.position') }}</span>
              </label>
              <select v-model="selectedPosition" class="select select-bordered w-full">
                <option value="">{{ t('admin.banners.filters.allPositions') }}</option>
                <option v-for="position in positionsList" :key="position.id" :value="String(position.id)">
                  {{ position.name }}
                </option>
              </select>
            </div>

            <div class="form-control w-44">
              <label class="label pt-0">
                <span class="label-text-alt text-base-content/60">{{ t('admin.banners.form.status') }}</span>
              </label>
              <select v-model="selectedStatus" class="select select-bordered w-full">
                <option value="">{{ t('admin.banners.filters.allStatuses') }}</option>
                <option value="1">{{ statusLabel(1) }}</option>
                <option value="2">{{ statusLabel(2) }}</option>
                <option value="3">{{ statusLabel(3) }}</option>
                <option value="4">{{ statusLabel(4) }}</option>
              </select>
            </div>

            <button class="btn btn-ghost" @click="resetFilters">{{ t('admin.banners.filters.clear') }}</button>
          </div>
        </div>
      </div>

      <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
        <span class="loading loading-spinner text-primary size-12"></span>
        <p class="mt-4 text-base-content/60">{{ t('admin.banners.loading') }}</p>
      </div>

      <div v-else-if="error" class="alert alert-error">
        <i class="icon-[tabler--alert-circle] size-6"></i>
        <span>{{ t('admin.banners.error', { message: error.message }) }}</span>
        <button class="btn btn-sm btn-ghost" @click="() => refresh()">{{ t('common.actions.tryAgain') }}</button>
      </div>

      <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
        <div class="overflow-x-auto">
          <table class="table table-lg">
            <thead class="bg-base-200/50">
              <tr>
                <th class="w-24">{{ t('admin.banners.table.image') }}</th>
                <th>{{ t('admin.banners.table.title') }}</th>
                <th>{{ t('admin.banners.table.position') }}</th>
                <th>{{ t('admin.banners.table.device') }}</th>
                <th>{{ t('admin.banners.table.period') }}</th>
                <th>{{ t('admin.banners.table.priority') }}</th>
                <th>{{ t('admin.banners.table.status') }}</th>
                <th class="text-right">{{ t('admin.banners.table.actions') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="banner in filteredBanners" :key="banner.id" class="hover:bg-base-200/30 transition-colors">
                <td>
                  <div class="avatar">
                    <div class="w-20 h-12 rounded bg-base-200">
                      <img :src="banner.image_desktop_url" :alt="banner.alt_text || banner.title" class="object-cover" />
                    </div>
                  </div>
                </td>
                <td>
                  <div class="font-bold">{{ banner.title }}</div>
                  <div class="text-xs text-base-content/60 line-clamp-1 max-w-xs">
                    {{ banner.campaign_name || banner.description || '-' }}
                  </div>
                </td>
                <td>
                  <span class="badge badge-soft badge-sm">{{ positionName(banner.position_id) }}</span>
                </td>
                <td>{{ deviceLabel(banner.device) }}</td>
                <td class="text-sm">
                  <div>{{ formatDate(banner.start_at) }}</div>
                  <div class="text-base-content/50">{{ banner.end_at ? formatDate(banner.end_at) : t('admin.banners.detail.noEnd') }}</div>
                </td>
                <td class="font-mono">{{ banner.priority }}</td>
                <td>
                  <span :class="['badge badge-soft text-xs', statusClass(banner.status)]">{{ statusLabel(banner.status) }}</span>
                </td>
                <td class="text-right">
                  <div class="flex justify-end gap-1">
                    <NuxtLinkLocale :to="`/admin/banners/${banner.id}`" class="btn btn-circle btn-text btn-sm" :aria-label="t('common.view')">
                      <i class="icon-[tabler--eye] size-5"></i>
                    </NuxtLinkLocale>
                    <NuxtLinkLocale :to="`/admin/banners/${banner.id}/edit`" class="btn btn-circle btn-text btn-sm" :aria-label="t('common.edit')">
                      <i class="icon-[tabler--pencil] size-5"></i>
                    </NuxtLinkLocale>
                    <button type="button" class="btn btn-circle btn-text btn-sm text-error" :aria-label="t('common.delete')" @click="confirmDelete(banner)">
                      <i class="icon-[tabler--trash] size-5"></i>
                    </button>
                  </div>
                </td>
              </tr>
              <tr v-if="filteredBanners.length === 0">
                <td colspan="8" class="text-center py-20 text-base-content/60 italic">
                  {{ t('admin.banners.notFound') }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </section>

    <section v-else-if="activeTab === 'positions'" class="grid grid-cols-1 xl:grid-cols-3 gap-6">
      <div class="card bg-base-100 shadow-sm xl:col-span-2">
        <div class="card-body">
          <h2 class="card-title">{{ t('admin.banners.positions.title') }}</h2>
          <div class="overflow-x-auto">
            <table class="table">
              <thead>
                <tr>
                  <th>{{ t('admin.banners.positions.code') }}</th>
                  <th>{{ t('admin.banners.positions.name') }}</th>
                  <th>{{ t('admin.banners.positions.status') }}</th>
                  <th class="text-right">{{ t('admin.banners.table.actions') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="position in positionsList" :key="position.id">
                  <td class="font-mono text-sm">{{ position.code }}</td>
                  <td>
                    <div class="font-medium">{{ position.name }}</div>
                    <div class="text-xs text-base-content/60">{{ position.description || '-' }}</div>
                  </td>
                  <td>
                    <span :class="['badge badge-soft', position.is_active ? 'badge-success' : 'badge-error']">
                      {{ position.is_active ? t('admin.banners.positions.active') : t('admin.banners.positions.inactive') }}
                    </span>
                  </td>
                  <td class="text-right">
                    <button type="button" class="btn btn-circle btn-text btn-sm" @click="editPosition(position)">
                      <i class="icon-[tabler--pencil] size-5"></i>
                    </button>
                    <button type="button" class="btn btn-circle btn-text btn-sm text-error" @click="deletePosition(position)">
                      <i class="icon-[tabler--trash] size-5"></i>
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-sm h-fit">
        <div class="card-body">
          <h2 class="card-title">{{ editingPositionId ? t('admin.banners.positions.edit') : t('admin.banners.positions.add') }}</h2>
          <form class="space-y-4" @submit.prevent="savePosition">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.positions.code') }}</span>
              </label>
              <input v-model="positionForm.code" type="text" class="input input-bordered w-full" required />
            </div>
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.positions.name') }}</span>
              </label>
              <input v-model="positionForm.name" type="text" class="input input-bordered w-full" required />
            </div>
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.positions.description') }}</span>
              </label>
              <textarea v-model="positionForm.description" class="textarea textarea-bordered"></textarea>
            </div>
            <label class="label cursor-pointer justify-start gap-3">
              <input v-model="positionForm.is_active" type="checkbox" class="checkbox checkbox-primary" />
              <span class="label-text font-semibold">{{ t('admin.banners.positions.active') }}</span>
            </label>
            <div class="flex justify-end gap-3">
              <button v-if="editingPositionId" type="button" class="btn btn-ghost" @click="resetPositionForm">{{ t('common.cancel') }}</button>
              <button type="submit" class="btn btn-primary" :disabled="positionSaving">
                <span v-if="positionSaving" class="loading loading-spinner loading-sm"></span>
                {{ t('common.save') }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </section>

    <section v-else class="space-y-6">
      <div class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4">
          <div class="flex flex-wrap gap-4 items-end">
            <div class="form-control">
              <label class="label pt-0">
                <span class="label-text-alt text-base-content/60">{{ t('admin.banners.analytics.from') }}</span>
              </label>
              <input v-model="analyticsFrom" type="date" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label pt-0">
                <span class="label-text-alt text-base-content/60">{{ t('admin.banners.analytics.to') }}</span>
              </label>
              <input v-model="analyticsTo" type="date" class="input input-bordered" />
            </div>
            <button class="btn btn-primary" @click="() => refreshAnalytics()">{{ t('admin.banners.analytics.refresh') }}</button>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="stats bg-base-100 shadow-sm">
          <div class="stat">
            <div class="stat-title">{{ t('admin.banners.analytics.impressions') }}</div>
            <div class="stat-value text-primary">{{ totalImpressions }}</div>
          </div>
        </div>
        <div class="stats bg-base-100 shadow-sm">
          <div class="stat">
            <div class="stat-title">{{ t('admin.banners.analytics.clicks') }}</div>
            <div class="stat-value text-secondary">{{ totalClicks }}</div>
          </div>
        </div>
        <div class="stats bg-base-100 shadow-sm">
          <div class="stat">
            <div class="stat-title">{{ t('admin.banners.analytics.ctr') }}</div>
            <div class="stat-value">{{ averageCtr.toFixed(2) }}%</div>
          </div>
        </div>
      </div>

      <div class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
        <div class="overflow-x-auto">
          <table class="table">
            <thead>
              <tr>
                <th>{{ t('admin.banners.table.title') }}</th>
                <th>{{ t('admin.banners.analytics.impressions') }}</th>
                <th>{{ t('admin.banners.analytics.clicks') }}</th>
                <th>{{ t('admin.banners.analytics.ctr') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="row in analyticsRows" :key="row.id">
                <td class="font-medium">{{ row.title }}</td>
                <td>{{ row.impressions }}</td>
                <td>{{ row.clicks }}</td>
                <td>{{ row.ctr_percent.toFixed(2) }}%</td>
              </tr>
              <tr v-if="analyticsRows.length === 0">
                <td colspan="4" class="text-center py-12 text-base-content/60">{{ t('admin.banners.analytics.empty') }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import type { Banner, BannerAnalytics, BannerPosition } from '~/types'

definePageMeta({
  layout: 'admin'
})

const { t } = useI18n()
const { apiFetch, useApiFetch } = useApi()

const activeTab = ref<'banners' | 'positions' | 'analytics'>('banners')
const searchQuery = ref('')
const selectedPosition = ref('')
const selectedStatus = ref('')

const today = new Date()
const thirtyDaysAgo = new Date(today)
thirtyDaysAgo.setDate(today.getDate() - 30)
const analyticsFrom = ref(toDateInput(thirtyDaysAgo))
const analyticsTo = ref(toDateInput(today))

const { pending, data: banners, error, refresh } = await useApiFetch<Banner[]>('/api/admin/banners', {
  key: 'admin-banners-list'
})
const { data: positions, refresh: refreshPositions } = await useApiFetch<BannerPosition[]>('/api/admin/banners/positions', {
  key: 'admin-banner-positions-list'
})

const analyticsUrl = computed(() => {
  const from = new Date(`${analyticsFrom.value}T00:00:00`).toISOString()
  const to = new Date(`${analyticsTo.value}T23:59:59`).toISOString()
  return `/api/admin/banners/analytics?from=${encodeURIComponent(from)}&to=${encodeURIComponent(to)}`
})
const { data: analytics, refresh: refreshAnalytics } = await useApiFetch<BannerAnalytics[]>(analyticsUrl, {
  key: 'admin-banner-analytics'
})

const positionsList = computed(() => positions.value || [])
const bannersList = computed(() => banners.value || [])
const analyticsRows = computed(() => analytics.value || [])

const positionForm = reactive({
  code: '',
  name: '',
  description: '',
  is_active: true
})
const editingPositionId = ref<number | null>(null)
const positionSaving = ref(false)

const filteredBanners = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  return bannersList.value.filter((banner) => {
    const matchesSearch = !query ||
      banner.title.toLowerCase().includes(query) ||
      banner.campaign_name?.toLowerCase().includes(query) ||
      banner.description?.toLowerCase().includes(query)
    const matchesPosition = !selectedPosition.value || banner.position_id === Number(selectedPosition.value)
    const matchesStatus = !selectedStatus.value || banner.status === Number(selectedStatus.value)
    return matchesSearch && matchesPosition && matchesStatus
  })
})

const totalImpressions = computed(() => analyticsRows.value.reduce((sum, row) => sum + row.impressions, 0))
const totalClicks = computed(() => analyticsRows.value.reduce((sum, row) => sum + row.clicks, 0))
const averageCtr = computed(() => {
  if (totalImpressions.value === 0) return 0
  return (totalClicks.value * 100) / totalImpressions.value
})

function toDateInput(date: Date) {
  return date.toISOString().slice(0, 10)
}

function resetFilters() {
  searchQuery.value = ''
  selectedPosition.value = ''
  selectedStatus.value = ''
}

function positionName(id: number) {
  return positionsList.value.find(position => position.id === id)?.name || `#${id}`
}

function statusLabel(status: number) {
  const labels: Record<number, string> = {
    1: t('admin.banners.status.draft'),
    2: t('admin.banners.status.active'),
    3: t('admin.banners.status.paused'),
    4: t('admin.banners.status.expired')
  }
  return labels[status] || t('admin.statusLabels.unknown')
}

function statusClass(status: number) {
  const classes: Record<number, string> = {
    1: 'badge-neutral',
    2: 'badge-success',
    3: 'badge-warning',
    4: 'badge-error'
  }
  return classes[status] || 'badge-neutral'
}

function deviceLabel(device: number) {
  const labels: Record<number, string> = {
    1: t('admin.banners.devices.all'),
    2: t('admin.banners.devices.desktop'),
    3: t('admin.banners.devices.mobile')
  }
  return labels[device] || t('admin.statusLabels.unknown')
}

function formatDate(dateString?: string) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

async function confirmDelete(banner: Banner) {
  if (!confirm(t('admin.banners.detail.confirmDelete', { name: banner.title }))) return

  try {
    await apiFetch(`/api/admin/banners/${banner.id}`, { method: 'DELETE' })
    await refresh()
    await refreshAnalytics()
  } catch (err) {
    alert(t('admin.banners.detail.errorDelete'))
    console.error(err)
  }
}

function editPosition(position: BannerPosition) {
  editingPositionId.value = position.id
  positionForm.code = position.code
  positionForm.name = position.name
  positionForm.description = position.description || ''
  positionForm.is_active = position.is_active
}

function resetPositionForm() {
  editingPositionId.value = null
  positionForm.code = ''
  positionForm.name = ''
  positionForm.description = ''
  positionForm.is_active = true
}

async function savePosition() {
  positionSaving.value = true
  try {
    const payload = {
      code: positionForm.code.trim(),
      name: positionForm.name.trim(),
      description: positionForm.description.trim() || null,
      is_active: positionForm.is_active
    }
    const url = editingPositionId.value
      ? `/api/admin/banners/positions/${editingPositionId.value}`
      : '/api/admin/banners/positions'
    const method = editingPositionId.value ? 'PUT' : 'POST'
    await apiFetch(url, { method, body: payload })
    resetPositionForm()
    await refreshPositions()
  } catch (err) {
    alert(t('admin.banners.positions.errorSave'))
    console.error(err)
  } finally {
    positionSaving.value = false
  }
}

async function deletePosition(position: BannerPosition) {
  if (!confirm(t('admin.banners.positions.confirmDelete', { name: position.name }))) return

  try {
    await apiFetch(`/api/admin/banners/positions/${position.id}`, { method: 'DELETE' })
    await refreshPositions()
  } catch (err) {
    alert(t('admin.banners.positions.errorDelete'))
    console.error(err)
  }
}
</script>
