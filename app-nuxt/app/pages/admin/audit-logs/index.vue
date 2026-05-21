<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">{{ t('admin.audit.title') }}</h1>
        <p class="text-sm text-base-content/60">{{ t('admin.audit.description') }}</p>
      </div>
      <button class="btn btn-ghost btn-sm" type="button" @click="resetFilters">
        <i class="icon-[tabler--filter-off] size-4"></i>
        {{ t('admin.audit.actions.clear') }}
      </button>
    </div>

    <div class="card shadow-base-300/10 mb-6 shadow-md">
      <div class="card-body">
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-[minmax(220px,1fr)_220px_220px_auto] lg:items-end">
          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.audit.filters.search') }}</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-base-content/40 group-focus-within:text-primary"></span>
              <input
                v-model="searchQuery"
                class="input input-bordered w-full pl-10"
                :placeholder="t('admin.audit.filters.searchPlaceholder')"
                type="text"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.audit.filters.action') }}</span>
            </label>
            <select v-model="selectedAction" class="select select-bordered w-full">
              <option value="">{{ t('admin.audit.filters.allActions') }}</option>
              <option v-for="action in availableActions" :key="action" :value="action">
                {{ action }}
              </option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.audit.filters.resource') }}</span>
            </label>
            <select v-model="selectedResourceType" class="select select-bordered w-full">
              <option value="">{{ t('admin.audit.filters.allResources') }}</option>
              <option v-for="resourceType in availableResourceTypes" :key="resourceType" :value="resourceType">
                {{ resourceType }}
              </option>
            </select>
          </div>

          <button class="btn btn-ghost" type="button" @click="refresh()">
            <i class="icon-[tabler--refresh] size-4"></i>
            {{ t('admin.audit.actions.refresh') }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="pending" class="card shadow-base-300/10 shadow-md">
      <div class="card-body flex flex-col items-center justify-center py-20">
        <span class="loading loading-spinner text-primary size-12"></span>
        <p class="mt-4 text-base-content/60">{{ t('admin.audit.loading') }}</p>
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.audit.error', { message: error.message }) }}</span>
    </div>

    <div v-else class="card shadow-base-300/10 shadow-md overflow-hidden">
      <div class="card-header flex items-center justify-between gap-3">
        <div>
          <h2 class="card-title text-xl">{{ t('admin.audit.table.title') }}</h2>
          <p class="text-sm text-base-content/60">
            {{ t('admin.audit.table.summary', { count: filteredItems.length, total: data?.total || 0 }) }}
          </p>
        </div>
      </div>

      <div class="card-body p-0">
        <div class="overflow-x-auto">
          <table class="table table-lg">
            <thead class="bg-base-200/50">
              <tr>
                <th>{{ t('admin.audit.table.columns.when') }}</th>
                <th>{{ t('admin.audit.table.columns.actor') }}</th>
                <th>{{ t('admin.audit.table.columns.action') }}</th>
                <th>{{ t('admin.audit.table.columns.resource') }}</th>
                <th>{{ t('admin.audit.table.columns.details') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in filteredItems" :key="item.id" class="hover:bg-base-200/30 transition-colors">
                <td class="whitespace-nowrap">
                  <div class="text-sm font-medium">{{ formatDate(item.created_at) }}</div>
                  <div class="text-xs text-base-content/40">{{ formatTime(item.created_at) }}</div>
                </td>
                <td>
                  <div class="font-medium">{{ item.actor_name }}</div>
                  <div class="text-xs text-base-content/50">{{ item.actor_email }}</div>
                </td>
                <td>
                  <span class="badge badge-soft badge-info font-mono">{{ item.action }}</span>
                </td>
                <td>
                  <div class="font-medium">{{ item.resource_type }}</div>
                  <div class="text-xs text-base-content/50">
                    {{ item.resource_label || formatResourceId(item.resource_id) }}
                  </div>
                </td>
                <td class="max-w-md">
                  <div class="text-sm">{{ item.message || t('admin.audit.table.noDetails') }}</div>
                </td>
              </tr>
              <tr v-if="filteredItems.length === 0">
                <td colspan="5" class="py-20 text-center text-base-content/50 italic">
                  {{ t('admin.audit.empty') }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
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

const { t } = useI18n()
const { useApiFetch } = useApi()

const searchQuery = ref('')
const selectedAction = ref('')
const selectedResourceType = ref('')

const { pending, data, error, refresh } = await useApiFetch<AdminAuditLogResponse>(
  '/api/admin/audit-logs',
  { key: 'admin-audit-logs' }
)

const items = computed(() => data.value?.items || [])

const availableActions = computed(() => {
  return [...new Set(items.value.map(item => item.action))].sort()
})

const availableResourceTypes = computed(() => {
  return [...new Set(items.value.map(item => item.resource_type))].sort()
})

const filteredItems = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()

  return items.value.filter((item) => {
    const matchesSearch = !query || [
      item.actor_name,
      item.actor_email,
      item.action,
      item.resource_type,
      item.resource_label,
      item.message
    ]
      .filter(Boolean)
      .join(' ')
      .toLowerCase()
      .includes(query)

    const matchesAction = !selectedAction.value || item.action === selectedAction.value
    const matchesResource = !selectedResourceType.value || item.resource_type === selectedResourceType.value

    return matchesSearch && matchesAction && matchesResource
  })
})

function resetFilters() {
  searchQuery.value = ''
  selectedAction.value = ''
  selectedResourceType.value = ''
}

function formatResourceId(resourceId?: number | null) {
  return resourceId ? `#${resourceId}` : '-'
}

function formatDate(dateString: string) {
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric'
  }).format(new Date(dateString))
}

function formatTime(dateString: string) {
  return new Intl.DateTimeFormat('pt-BR', {
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}
</script>
