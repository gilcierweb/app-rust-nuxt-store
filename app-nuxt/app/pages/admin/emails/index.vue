<template>
  <div>
    <!-- Title -->
    <div class="mb-6">
      <h1 class="h1">{{ $t('admin.emails.title') }}</h1>
      <p class="text-sm text-gray-500 mt-1">{{ $t('admin.emails.description') }}</p>
    </div>

    <!-- Tabs Container -->
    <div class="tabs tabs-lifted mb-6">
      <button 
        type="button" 
        class="tab tab-lg" 
        :class="{ 'tab-active font-bold': activeTab === 'logs' }" 
        @click="activeTab = 'logs'"
      >
        <span class="flex items-center gap-2">
          <i class="icon-[tabler--list-check] size-5"></i>
          {{ $t('admin.emails.tabs.logs') }}
        </span>
      </button>
      <button 
        type="button" 
        class="tab tab-lg" 
        :class="{ 'tab-active font-bold': activeTab === 'templates' }" 
        @click="activeTab = 'templates'"
      >
        <span class="flex items-center gap-2">
          <i class="icon-[tabler--template] size-5"></i>
          {{ $t('admin.emails.tabs.templates') }}
        </span>
      </button>
    </div>

    <!-- TAB 1: LOGS -->
    <div v-if="activeTab === 'logs'">
      <!-- Filter panel -->
      <div class="mb-6 flex flex-col md:flex-row gap-4 items-center justify-between">
        <div class="flex flex-col md:flex-row gap-4 w-full md:w-auto">
          <!-- Recipient search -->
          <input
            v-model="recipientSearch"
            type="text"
            :placeholder="$t('admin.emails.filters.searchPlaceholder')"
            class="input input-bordered w-full md:w-64"
            @input="handleFilterChange"
          />

          <!-- Status selector -->
          <select 
            v-model="statusFilter" 
            class="select select-bordered w-full md:w-48"
            @change="handleFilterChange"
          >
            <option value="">{{ $t('admin.emails.filters.allStatuses') }}</option>
            <option value="0">{{ $t('admin.emails.filters.pending') }}</option>
            <option value="1">{{ $t('admin.emails.filters.sent') }}</option>
            <option value="2">{{ $t('admin.emails.filters.failed') }}</option>
          </select>
        </div>

        <button type="button" class="btn btn-outline gap-2" @click="refreshLogs">
          <i class="icon-[tabler--refresh] size-4"></i>
          {{ $t('admin.emails.filters.refresh') }}
        </button>
      </div>

      <!-- Loading State -->
      <div v-if="logsPending" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary size-12"></span>
        <span class="ml-3">{{ $t('admin.emails.loadingLogs') }}</span>
      </div>

      <!-- Error State -->
      <div v-else-if="logsError" class="alert alert-error mb-6">
        <span>{{ $t('admin.emails.errorLogs', { message: logsError.message }) }}</span>
      </div>

      <!-- Empty State -->
      <div v-else-if="!logsData || !logsData.items || logsData.items.length === 0" class="text-center py-12 bg-base-100 rounded-box border border-base-200">
        <p class="text-gray-500 text-lg">{{ $t('admin.emails.notFoundLogs') }}</p>
      </div>

      <!-- Logs Table -->
      <div v-else class="rounded-box bg-base-100 border border-base-200 w-full overflow-hidden shadow-sm">
        <div class="overflow-x-auto">
          <table class="table w-full">
            <thead>
              <tr>
                <th>{{ $t('admin.emails.table.id') }}</th>
                <th>{{ $t('admin.emails.table.recipient') }}</th>
                <th>{{ $t('admin.emails.table.template') }}</th>
                <th>{{ $t('admin.emails.table.subject') }}</th>
                <th>{{ $t('admin.emails.table.status') }}</th>
                <th>{{ $t('admin.emails.table.date') }}</th>
                <th class="text-right">{{ $t('admin.emails.table.actions') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="log in logsData.items" :key="log.id" class="hover:bg-base-200/50 transition-colors">
                <td class="font-mono text-xs">{{ log.id }}</td>
                <td class="font-medium">{{ log.recipient }}</td>
                <td>
                  <span class="badge badge-outline text-xs capitalize">{{ log.template_name }}</span>
                </td>
                <td class="max-w-xs truncate">{{ log.subject }}</td>
                <td>
                  <span :class="['badge text-xs font-semibold px-2.5 py-1 rounded', getStatusBadgeClass(log.status)]">
                    {{ getStatusLabel(log.status) }}
                  </span>
                </td>
                <td>{{ formatDate(log.sent_at || log.created_at) }}</td>
                <td class="text-right flex items-center justify-end gap-2">
                  <button 
                    type="button"
                    class="btn btn-sm btn-ghost gap-1.5"
                    aria-haspopup="dialog" aria-expanded="false" aria-controls="log-details-modal"
                    data-overlay="#log-details-modal"
                    @click="selectedLog = log"
                  >
                    <i class="icon-[tabler--eye] size-4"></i>
                    {{ $t('admin.emails.actions.view') }}
                  </button>
                  <button 
                    type="button"
                    class="btn btn-sm btn-primary gap-1.5"
                    :disabled="resendingIds.includes(log.id)"
                    @click="resendLog(log.id)"
                  >
                    <span v-if="resendingIds.includes(log.id)" class="loading loading-spinner loading-xs"></span>
                    <i v-else class="icon-[tabler--send] size-4"></i>
                    {{ $t('admin.emails.actions.resend') }}
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <AdminPagination
          :current-page="currentPage"
          :page-size="pageSize"
          :current-count="logsData.items.length"
          :total="logsData.total"
          :pending="logsPending"
          :summary="$t('admin.emails.pagination.showing', { current: logsData.items.length, total: logsData.total })"
          :previous-label="$t('admin.emails.pagination.previous')"
          :next-label="$t('admin.emails.pagination.next')"
          @change="changePage"
        />
      </div>

      <!-- Modal: View Log Details (FlyonUI overlay) -->
      <div id="log-details-modal" class="overlay modal overlay-open:opacity-100 overlay-open:duration-300 modal-middle z-50 hidden" role="dialog" tabindex="-1">
        <div class="modal-dialog max-w-4xl">
          <div class="modal-content">
            <div class="modal-header">
              <h3 class="modal-title flex items-center gap-2">
                <i class="icon-[tabler--mail-opened] size-6 text-primary"></i>
                {{ $t('admin.emails.detailsModal.title', { id: selectedLog?.id }) }}
              </h3>
              <button type="button" class="btn btn-text btn-circle btn-sm absolute end-3 top-3" aria-label="Close" data-overlay="#log-details-modal">
                <span class="icon-[tabler--x] size-4"></span>
              </button>
            </div>
            
            <div v-if="selectedLog" class="modal-body">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <div class="space-y-3">
                  <div>
                    <span class="text-xs text-gray-400 block">{{ $t('admin.emails.detailsModal.recipient') }}</span>
                    <span class="font-medium text-base-content">{{ selectedLog.recipient }}</span>
                  </div>
                  <div>
                    <span class="text-xs text-gray-400 block">{{ $t('admin.emails.detailsModal.template') }}</span>
                    <span class="badge badge-neutral capitalize">{{ selectedLog.template_name }}</span>
                  </div>
                  <div>
                    <span class="text-xs text-gray-400 block">{{ $t('admin.emails.detailsModal.createdAt') }}</span>
                    <span>{{ formatDate(selectedLog.created_at) }}</span>
                  </div>
                  <div>
                    <span class="text-xs text-gray-400 block">{{ $t('admin.emails.detailsModal.status') }}</span>
                    <span :class="['badge font-semibold px-2 py-1 rounded text-xs mt-1', getStatusBadgeClass(selectedLog.status)]">
                      {{ getStatusLabel(selectedLog.status) }}
                    </span>
                  </div>
                </div>

                <div class="space-y-3">
                  <div>
                    <span class="text-xs text-gray-400 block">{{ $t('admin.emails.detailsModal.subject') }}</span>
                    <span class="font-medium text-base-content block mt-0.5">{{ selectedLog.subject }}</span>
                  </div>
                  <div v-if="selectedLog.sent_at">
                    <span class="text-xs text-gray-400 block">{{ $t('admin.emails.detailsModal.sentAt') }}</span>
                    <span>{{ formatDate(selectedLog.sent_at) }}</span>
                  </div>
                  <div v-if="selectedLog.error_message">
                    <span class="text-xs text-error font-semibold block">{{ $t('admin.emails.detailsModal.errorMessage') }}</span>
                    <pre class="bg-error/10 border border-error/20 text-error p-3 rounded-lg text-xs overflow-x-auto font-mono mt-1 whitespace-pre-wrap">{{ selectedLog.error_message }}</pre>
                  </div>
                </div>
              </div>

              <div v-if="selectedLog" class="mb-6">
                <span class="text-xs text-gray-400 block mb-2 font-semibold">{{ $t('admin.emails.detailsModal.variables') }}</span>
                <pre class="bg-base-200 border border-base-300 p-4 rounded-lg text-xs font-mono overflow-x-auto max-h-48 text-base-content">{{ formatJson(selectedLog.locals_json) }}</pre>
              </div>
            </div>

            <div class="modal-footer flex justify-between items-center">
              <button 
                type="button" 
                class="btn btn-primary gap-2"
                :disabled="resendingIds.includes(selectedLog?.id)"
                @click="resendLog(selectedLog?.id)"
              >
                <span v-if="resendingIds.includes(selectedLog?.id)" class="loading loading-spinner loading-xs"></span>
                <i v-else class="icon-[tabler--send] size-4"></i>
                {{ $t('admin.emails.actions.resendNow') }}
              </button>
              <button type="button" class="btn btn-soft btn-secondary" data-overlay="#log-details-modal">
                {{ $t('admin.emails.actions.close') }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- TAB 2: TEMPLATES -->
    <div v-else-if="activeTab === 'templates'">
      <!-- Loading State -->
      <div v-if="templatesPending" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary size-12"></span>
        <span class="ml-3">{{ $t('admin.emails.loadingTemplates') }}</span>
      </div>

      <!-- Error State -->
      <div v-else-if="templatesError" class="alert alert-error mb-6">
        <span>{{ $t('admin.emails.errorTemplates', { message: templatesError.message }) }}</span>
      </div>

      <!-- Templates Grid -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div 
          v-for="tmpl in templates" 
          :key="tmpl.name" 
          class="card bg-base-100 border border-base-200 shadow-sm hover:shadow-md transition-shadow"
        >
          <div class="card-body">
            <h2 class="card-title capitalize text-lg flex items-center justify-between">
              {{ (tmpl.name || '').replace('_', ' ') }}
              <span class="badge badge-soft text-xs badge-neutral">Loco Mailer</span>
            </h2>
            <p class="text-xs text-gray-500 font-mono mt-1">{{ $t('admin.emails.templates.defaultSubject') }}</p>
            <p class="font-medium text-sm text-base-content mt-0.5">{{ tmpl.subject }}</p>
            
            <div class="card-actions justify-end mt-6 pt-4 border-t border-base-100">
              <button 
                type="button" 
                class="btn btn-outline btn-sm gap-1.5"
                aria-haspopup="dialog" aria-expanded="false" aria-controls="template-preview-modal"
                data-overlay="#template-preview-modal"
                @click="selectedTemplate = tmpl"
              >
                <i class="icon-[tabler--eye] size-4"></i>
                {{ $t('admin.emails.templates.viewCode') }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal: Preview Template (FlyonUI overlay) -->
      <div id="template-preview-modal" class="overlay modal overlay-open:opacity-100 overlay-open:duration-300 modal-middle z-50 hidden" role="dialog" tabindex="-1">
        <div class="modal-dialog max-w-5xl">
          <div class="modal-content">
            <div class="modal-header">
              <h3 class="modal-title capitalize flex items-center gap-2">
                <i class="icon-[tabler--code] size-6 text-primary"></i>
                {{ $t('admin.emails.codeModal.title', { name: (selectedTemplate?.name || '').replace('_', ' ') }) }}
              </h3>
              <button type="button" class="btn btn-text btn-circle btn-sm absolute end-3 top-3" aria-label="Close" data-overlay="#template-preview-modal">
                <span class="icon-[tabler--x] size-4"></span>
              </button>
            </div>
            
            <div v-if="selectedTemplate" class="modal-body">
              <div class="mb-4">
                <span class="text-xs text-gray-400 block font-semibold mb-1">{{ $t('admin.emails.codeModal.defaultSubject') }}</span>
                <span class="font-medium text-base-content border border-base-200 p-2.5 rounded-lg block bg-base-200/50">{{ selectedTemplate.subject }}</span>
              </div>

              <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div>
                  <span class="text-xs text-gray-400 block font-semibold mb-2">{{ $t('admin.emails.codeModal.htmlTemplate') }}</span>
                  <pre class="bg-base-200 border border-base-300 p-4 rounded-lg text-xs font-mono overflow-auto h-96 text-base-content">{{ selectedTemplate.html }}</pre>
                </div>
                <div>
                  <span class="text-xs text-gray-400 block font-semibold mb-2">{{ $t('admin.emails.codeModal.textTemplate') }}</span>
                  <pre class="bg-base-200 border border-base-300 p-4 rounded-lg text-xs font-mono overflow-auto h-96 text-base-content">{{ selectedTemplate.text }}</pre>
                </div>
              </div>
            </div>

            <div class="modal-footer">
              <button type="button" class="btn btn-soft btn-secondary" data-overlay="#template-preview-modal">
                {{ $t('admin.emails.actions.close') }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiFetch } = useApi()
const { t } = useI18n()
const toast = useAppToast()

const activeTab = ref('logs')
const recipientSearch = ref('')
const statusFilter = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const resendingIds = ref<number[]>([])

const selectedLog = ref<any>(null)
const selectedTemplate = ref<any>(null)

// API fetch for templates
const templatesPromise = useApiFetch<any[]>('/api/admin/emails/templates', { key: 'admin-email-templates' })

// API fetch for logs (paginated & filterable)
const logsPromise = useApiFetch<any>('/api/admin/emails/logs', {
  key: 'admin-email-logs',
  query: computed(() => ({
    page: currentPage.value,
    page_size: pageSize.value,
    recipient: recipientSearch.value || undefined,
    status: statusFilter.value !== '' ? Number(statusFilter.value) : undefined
  }))
})

const [
  { pending: templatesPending, data: templatesRaw, error: templatesError },
  { pending: logsPending, data: logsData, error: logsError, refresh: refreshLogs }
] = await Promise.all([templatesPromise, logsPromise])

const templates = computed(() => {
  const raw = templatesRaw.value
  return Array.isArray(raw) ? raw : []
})

// Trigger query parameter refresh
const handleFilterChange = () => {
  currentPage.value = 1
  refreshLogs()
}

const changePage = (page: number) => {
  currentPage.value = page
  refreshLogs()
}

// Helpers
const getStatusLabel = (status: number) => {
  switch (status) {
    case 0: return t('admin.emails.filters.pending')
    case 1: return t('admin.emails.filters.sent')
    case 2: return t('admin.emails.filters.failed')
    default: return t('admin.statusLabels.unknown')
  }
}

const getStatusBadgeClass = (status: number) => {
  switch (status) {
    case 0: return 'badge-warning bg-warning/20 text-warning border-warning/30'
    case 1: return 'badge-success bg-success/20 text-success border-success/30'
    case 2: return 'badge-error bg-error/20 text-error border-error/30'
    default: return 'badge-neutral bg-base-300 text-gray-500 border-base-200'
  }
}

const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

const formatJson = (jsonStr: string) => {
  try {
    const parsed = JSON.parse(jsonStr)
    return JSON.stringify(parsed, null, 2)
  } catch {
    return jsonStr
  }
}

// Re-init FlyonUI when tab changes (new DOM elements with data-overlay need processing)
watch(activeTab, async () => {
  await nextTick()
  if (window.HSStaticMethods) {
    window.HSStaticMethods.autoInit()
  }
})

// Actions
const resendLog = async (logId: number) => {
  if (!logId) return
  resendingIds.value.push(logId)
  try {
    await apiFetch(`/api/admin/emails/logs/${logId}/resend`, {
      method: 'POST'
    })
    toast.success(t('admin.emails.actions.success'))
    await refreshLogs()
    if (selectedLog.value && selectedLog.value.id === logId) {
      const updatedLog = logsData.value?.items.find((item: any) => item.id === logId)
      if (updatedLog) {
        selectedLog.value = updatedLog
      }
    }
  } catch (err) {
    console.error('Erro ao reenviar e-mail:', err)
    toast.error(t('admin.emails.actions.error'))
  } finally {
    resendingIds.value = resendingIds.value.filter(id => id !== logId)
  }
}
</script>
