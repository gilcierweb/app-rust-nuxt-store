<template>
  <div>
    <!-- Title -->
    <div class="mb-6">
      <h1 class="h1">E-mails Transacionais</h1>
      <p class="text-sm text-gray-500 mt-1">Gerencie os modelos de e-mail e monitore as entregas e tentativas de envio.</p>
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
          Histórico de Envios
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
          Modelos (Templates)
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
            placeholder="Buscar por destinatário..."
            class="input input-bordered w-full md:w-64"
            @input="handleFilterChange"
          />

          <!-- Status selector -->
          <select 
            v-model="statusFilter" 
            class="select select-bordered w-full md:w-48"
            @change="handleFilterChange"
          >
            <option value="">Todos os status</option>
            <option value="0">Pendente</option>
            <option value="1">Enviado</option>
            <option value="2">Falhou</option>
          </select>
        </div>

        <button type="button" class="btn btn-outline gap-2" @click="refreshLogs">
          <i class="icon-[tabler--refresh] size-4"></i>
          Atualizar Logs
        </button>
      </div>

      <!-- Loading State -->
      <div v-if="logsPending" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary size-12"></span>
        <span class="ml-3">Carregando logs de envio...</span>
      </div>

      <!-- Error State -->
      <div v-else-if="logsError" class="alert alert-error mb-6">
        <span>Erro ao carregar logs: {{ logsError.message }}</span>
      </div>

      <!-- Empty State -->
      <div v-else-if="!logsData || !logsData.items || logsData.items.length === 0" class="text-center py-12 bg-base-100 rounded-box border border-base-200">
        <p class="text-gray-500 text-lg">Nenhum log de envio correspondente encontrado.</p>
      </div>

      <!-- Logs Table -->
      <div v-else class="rounded-box bg-base-100 border border-base-200 w-full overflow-hidden shadow-sm">
        <div class="overflow-x-auto">
          <table class="table w-full">
            <thead>
              <tr>
                <th>ID</th>
                <th>Destinatário</th>
                <th>Template</th>
                <th>Assunto</th>
                <th>Status</th>
                <th>Data de Envio</th>
                <th class="text-right">Ações</th>
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
                    @click="viewLogDetails(log)"
                  >
                    <i class="icon-[tabler--eye] size-4"></i>
                    Ver
                  </button>
                  <button 
                    type="button"
                    class="btn btn-sm btn-primary gap-1.5"
                    :disabled="resendingIds.includes(log.id)"
                    @click="resendLog(log.id)"
                  >
                    <span v-if="resendingIds.includes(log.id)" class="loading loading-spinner loading-xs"></span>
                    <i v-else class="icon-[tabler--send] size-4"></i>
                    Reenviar
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Pagination -->
        <div class="flex items-center justify-between px-6 py-4 border-t border-base-200">
          <span class="text-xs text-gray-500">
            Mostrando {{ logsData.items.length }} de {{ logsData.total }} logs
          </span>
          <div class="join">
            <button 
              type="button"
              class="join-item btn btn-sm btn-outline" 
              :disabled="currentPage <= 1"
              @click="changePage(currentPage - 1)"
            >
              Anterior
            </button>
            <button type="button" class="join-item btn btn-sm btn-active font-mono">
              {{ currentPage }}
            </button>
            <button 
              type="button"
              class="join-item btn btn-sm btn-outline" 
              :disabled="currentPage * pageSize >= logsData.total"
              @click="changePage(currentPage + 1)"
            >
              Próximo
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- TAB 2: TEMPLATES -->
    <div v-else-if="activeTab === 'templates'">
      <!-- Loading State -->
      <div v-if="templatesPending" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary size-12"></span>
        <span class="ml-3">Carregando modelos de e-mail...</span>
      </div>

      <!-- Error State -->
      <div v-else-if="templatesError" class="alert alert-error mb-6">
        <span>Erro ao carregar templates: {{ templatesError.message }}</span>
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
              {{ tmpl.name.replace('_', ' ') }}
              <span class="badge badge-soft text-xs badge-neutral">Loco Mailer</span>
            </h2>
            <p class="text-xs text-gray-500 font-mono mt-1">Assunto padrão:</p>
            <p class="font-medium text-sm text-base-content mt-0.5">{{ tmpl.subject }}</p>
            
            <div class="card-actions justify-end mt-6 pt-4 border-t border-base-100">
              <button 
                type="button" 
                class="btn btn-outline btn-sm gap-1.5"
                @click="previewTemplate(tmpl)"
              >
                <i class="icon-[tabler--eye] size-4"></i>
                Visualizar Código
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal: View Log Details -->
    <dialog ref="logDetailsModal" class="modal">
      <div class="modal-box w-11/12 max-w-4xl bg-base-100 border border-base-200">
        <form method="dialog">
          <button class="btn btn-sm btn-circle btn-ghost absolute right-4 top-4" aria-label="Fechar">✕</button>
        </form>
        
        <h3 class="font-bold text-lg mb-4 flex items-center gap-2">
          <i class="icon-[tabler--mail-opened] size-6 text-primary"></i>
          Detalhes do Log de Envio #{{ selectedLog?.id }}
        </h3>
        
        <div v-if="selectedLog" class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
          <div class="space-y-3">
            <div>
              <span class="text-xs text-gray-400 block">Destinatário</span>
              <span class="font-medium text-base-content">{{ selectedLog.recipient }}</span>
            </div>
            <div>
              <span class="text-xs text-gray-400 block">Template Utilizado</span>
              <span class="badge badge-neutral capitalize">{{ selectedLog.template_name }}</span>
            </div>
            <div>
              <span class="text-xs text-gray-400 block">Data de Criação</span>
              <span>{{ formatDate(selectedLog.created_at) }}</span>
            </div>
            <div>
              <span class="text-xs text-gray-400 block">Status de Entrega</span>
              <span :class="['badge font-semibold px-2 py-1 rounded text-xs mt-1', getStatusBadgeClass(selectedLog.status)]">
                {{ getStatusLabel(selectedLog.status) }}
              </span>
            </div>
          </div>

          <div class="space-y-3">
            <div>
              <span class="text-xs text-gray-400 block">Assunto</span>
              <span class="font-medium text-base-content block mt-0.5">{{ selectedLog.subject }}</span>
            </div>
            <div v-if="selectedLog.sent_at">
              <span class="text-xs text-gray-400 block">Data de Envio Realizado</span>
              <span>{{ formatDate(selectedLog.sent_at) }}</span>
            </div>
            <div v-if="selectedLog.error_message">
              <span class="text-xs text-error font-semibold block">Mensagem de Erro</span>
              <pre class="bg-error/10 border border-error/20 text-error p-3 rounded-lg text-xs overflow-x-auto font-mono mt-1 whitespace-pre-wrap">{{ selectedLog.error_message }}</pre>
            </div>
          </div>
        </div>

        <div v-if="selectedLog" class="mb-6">
          <span class="text-xs text-gray-400 block mb-2 font-semibold">Parâmetros das Variáveis do Template (JSON)</span>
          <pre class="bg-base-200 border border-base-300 p-4 rounded-lg text-xs font-mono overflow-x-auto max-h-48 text-base-content">{{ formatJson(selectedLog.locals_json) }}</pre>
        </div>

        <div class="modal-action flex justify-between items-center pt-4 border-t border-base-200">
          <button 
            type="button" 
            class="btn btn-primary gap-2"
            :disabled="resendingIds.includes(selectedLog?.id)"
            @click="resendLog(selectedLog?.id)"
          >
            <span v-if="resendingIds.includes(selectedLog?.id)" class="loading loading-spinner loading-xs"></span>
            <i v-else class="icon-[tabler--send] size-4"></i>
            Reenviar E-mail Agora
          </button>
          <form method="dialog">
            <button class="btn btn-outline">Fechar</button>
          </form>
        </div>
      </div>
    </dialog>

    <!-- Modal: Preview Template -->
    <dialog ref="templateModal" class="modal">
      <div class="modal-box w-11/12 max-w-5xl bg-base-100 border border-base-200">
        <form method="dialog">
          <button class="btn btn-sm btn-circle btn-ghost absolute right-4 top-4" aria-label="Fechar">✕</button>
        </form>
        
        <h3 class="font-bold text-lg mb-4 flex items-center gap-2 capitalize">
          <i class="icon-[tabler--code] size-6 text-primary"></i>
          Visualizar Código do Template: {{ selectedTemplate?.name.replace('_', ' ') }}
        </h3>
        
        <div v-if="selectedTemplate" class="space-y-4">
          <div>
            <span class="text-xs text-gray-400 block font-semibold mb-1">Assunto Padrão</span>
            <span class="font-medium text-base-content border border-base-200 p-2.5 rounded-lg block bg-base-200/50">{{ selectedTemplate.subject }}</span>
          </div>

          <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div>
              <span class="text-xs text-gray-400 block font-semibold mb-2">HTML Template (html.tera)</span>
              <pre class="bg-base-200 border border-base-300 p-4 rounded-lg text-xs font-mono overflow-auto h-96 text-base-content">{{ selectedTemplate.html }}</pre>
            </div>
            <div>
              <span class="text-xs text-gray-400 block font-semibold mb-2">Text Template (text.tera)</span>
              <pre class="bg-base-200 border border-base-300 p-4 rounded-lg text-xs font-mono overflow-auto h-96 text-base-content">{{ selectedTemplate.text }}</pre>
            </div>
          </div>
        </div>

        <div class="modal-action pt-4 border-t border-base-200">
          <form method="dialog">
            <button class="btn btn-outline">Fechar</button>
          </form>
        </div>
      </div>
    </dialog>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

const { apiFetch, useApiFetch } = useApi()

const activeTab = ref('logs')
const recipientSearch = ref('')
const statusFilter = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const resendingIds = ref<number[]>([])

// Modals
const logDetailsModal = ref<HTMLDialogElement | null>(null)
const templateModal = ref<HTMLDialogElement | null>(null)

const selectedLog = ref<any>(null)
const selectedTemplate = ref<any>(null)

// API fetch for templates
const { 
  pending: templatesPending, 
  data: templates, 
  error: templatesError 
} = await useApiFetch<any[]>('/api/admin/emails/templates', { key: 'admin-email-templates' })

// API fetch for logs (paginated & filterable)
const { 
  pending: logsPending, 
  data: logsData, 
  error: logsError, 
  refresh: refreshLogs 
} = await useApiFetch<any>('/api/admin/emails/logs', {
  key: 'admin-email-logs',
  query: computed(() => ({
    page: currentPage.value,
    page_size: pageSize.value,
    recipient: recipientSearch.value || undefined,
    status: statusFilter.value !== '' ? Number(statusFilter.value) : undefined
  }))
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
    case 0: return 'Pendente'
    case 1: return 'Enviado'
    case 2: return 'Falhou'
    default: return 'Desconhecido'
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

// Actions
const viewLogDetails = (log: any) => {
  selectedLog.value = log
  if (logDetailsModal.value) {
    logDetailsModal.value.showModal()
  }
}

const previewTemplate = (tmpl: any) => {
  selectedTemplate.value = tmpl
  if (templateModal.value) {
    templateModal.value.showModal()
  }
}

const resendLog = async (logId: number) => {
  if (!logId) return
  resendingIds.value.push(logId)
  try {
    await apiFetch(`/api/admin/emails/logs/${logId}/resend`, {
      method: 'POST'
    })
    // Quick notification or feedback
    alert('Email reenviado com sucesso!')
    await refreshLogs()
    // Update selected log reference if modal is open
    if (selectedLog.value && selectedLog.value.id === logId) {
      const updatedLog = logsData.value?.items.find((item: any) => item.id === logId)
      if (updatedLog) {
        selectedLog.value = updatedLog
      }
    }
  } catch (err) {
    console.error('Erro ao reenviar e-mail:', err)
    alert('Falha ao reenviar e-mail. Por favor, tente novamente.')
  } finally {
    resendingIds.value = resendingIds.value.filter(id => id !== logId)
  }
}
</script>
