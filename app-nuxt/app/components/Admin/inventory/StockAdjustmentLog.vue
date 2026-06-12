<template>
  <div class="card shadow-base-300/10 shadow-md">
    <div class="card-body p-0">
      <div class="p-4 border-b border-base-200 flex items-center justify-between">
        <h3 class="font-bold">{{ t('admin.inventory.logs.title') }}</h3>
        <button class="btn btn-ghost btn-sm" type="button" :disabled="loading" @click="loadLogs">
          <i class="icon-[tabler--refresh] size-4"></i>
        </button>
      </div>

      <div v-if="loading" class="flex items-center justify-center py-12">
        <span class="loading loading-spinner text-primary"></span>
      </div>

      <div v-else-if="logs.length === 0" class="py-12 text-center text-base-content/50">
        <i class="icon-[tabler--history] size-12 opacity-20 mb-2 block mx-auto"></i>
        <p>{{ t('admin.inventory.logs.empty') }}</p>
      </div>

      <div v-else class="overflow-x-auto">
        <table class="table table-zebra">
          <thead>
            <tr>
              <th>{{ t('admin.inventory.logs.date') }}</th>
              <th>{{ t('admin.inventory.logs.item') }}</th>
              <th>{{ t('admin.inventory.logs.type') }}</th>
              <th class="text-right">{{ t('admin.inventory.logs.quantity') }}</th>
              <th class="text-right">{{ t('admin.inventory.logs.before') }}</th>
              <th class="text-right">{{ t('admin.inventory.logs.after') }}</th>
              <th>{{ t('admin.inventory.logs.reason') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="log in logs" :key="log.id">
              <td class="text-sm">{{ formatDate(log.created_at) }}</td>
              <td>
                <div class="text-sm font-bold">{{ log.product_name || `#${log.variant_id}` }}</div>
                <div class="text-xs text-base-content/50">{{ log.variant_name || log.sku }}</div>
              </td>
              <td>
                <span :class="['badge badge-sm badge-soft', typeBadgeClass(log.type)]">
                  {{ t(`admin.inventory.logs.types.${log.type}`) }}
                </span>
              </td>
              <td class="text-right font-mono" :class="log.quantity >= 0 ? 'text-success' : 'text-error'">
                {{ log.quantity >= 0 ? '+' : '' }}{{ log.quantity }}
              </td>
              <td class="text-right font-mono">{{ log.quantity_before }}</td>
              <td class="text-right font-mono">{{ log.quantity_after }}</td>
              <td class="text-sm text-base-content/60 max-w-48 truncate">{{ log.reason || '-' }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { StockAdjustment } from '~/types'

defineProps<{
  variantId?: number
}>()

const { t } = useI18n()
const { getLogs } = useInventory()

const loading = ref(false)
const logs = ref<StockAdjustment[]>([])

function typeBadgeClass(type: string) {
  switch (type) {
    case 'restock': return 'badge-success'
    case 'sale': return 'badge-info'
    case 'return': return 'badge-warning'
    case 'damage': return 'badge-error'
    case 'correction': return 'badge-neutral'
    default: return 'badge-ghost'
  }
}

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString(getAppLocale(), {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

async function loadLogs() {
  loading.value = true
  try {
    const result = await getLogs({ variant_id: undefined, page: 1, page_size: 50 })
    logs.value = result.items || []
  } catch {
    logs.value = []
  } finally {
    loading.value = false
  }
}

onMounted(() => loadLogs())
</script>
