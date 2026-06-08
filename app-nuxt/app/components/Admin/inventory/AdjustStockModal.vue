<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="fixed inset-0 z-[9998] flex items-center justify-center bg-base-content/45 p-4"
      role="dialog"
      aria-modal="true"
      @click.self="handleClose"
    >
      <div class="w-full max-w-md rounded-box bg-base-100 shadow-xl">
        <div class="border-base-content/10 flex items-start justify-between gap-4 border-b px-6 py-4">
          <div>
            <h3 class="text-lg font-semibold">{{ t('admin.inventory.adjust.title') }}</h3>
            <p v-if="item" class="mt-1 text-sm text-base-content/70">
              {{ item.product_name }} — {{ item.variant_name || `#${item.variant_id}` }}
            </p>
          </div>
          <button class="btn btn-text btn-circle btn-sm" type="button" @click="handleClose">
            <span class="icon-[tabler--x] size-4"></span>
          </button>
        </div>

        <div class="space-y-4 px-6 py-5">
          <div v-if="error" class="alert alert-error">
            <span class="icon-[tabler--alert-circle] size-5"></span>
            <span>{{ error }}</span>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.inventory.adjust.type') }}</span>
            </label>
            <select v-model="form.type" class="select select-bordered w-full">
              <option value="restock">{{ t('admin.inventory.adjust.types.restock') }}</option>
              <option value="sale">{{ t('admin.inventory.adjust.types.sale') }}</option>
              <option value="return">{{ t('admin.inventory.adjust.types.return') }}</option>
              <option value="damage">{{ t('admin.inventory.adjust.types.damage') }}</option>
              <option value="correction">{{ t('admin.inventory.adjust.types.correction') }}</option>
            </select>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.inventory.adjust.quantity') }}</span>
            </label>
            <input
              v-model.number="form.quantity"
              type="number"
              class="input input-bordered w-full"
              :placeholder="t('admin.inventory.adjust.quantityPlaceholder')"
            />
            <p class="text-xs text-base-content/50 mt-1">
              {{ t('admin.inventory.adjust.quantityHelp') }}
            </p>
          </div>

          <div class="form-control">
            <label class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ t('admin.inventory.adjust.reason') }}</span>
            </label>
            <textarea
              v-model="form.reason"
              class="textarea textarea-bordered w-full min-h-20"
              :placeholder="t('admin.inventory.adjust.reasonPlaceholder')"
            ></textarea>
          </div>

          <div v-if="item" class="rounded-box bg-base-200/50 p-4 text-sm space-y-1">
            <div class="flex justify-between">
              <span class="text-base-content/60">{{ t('admin.inventory.adjust.currentStock') }}</span>
              <span class="font-bold">{{ item.inventory_quantity ?? 0 }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-base-content/60">{{ t('admin.inventory.adjust.reserved') }}</span>
              <span>{{ item.reserved_quantity ?? 0 }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-base-content/60">{{ t('admin.inventory.adjust.newStock') }}</span>
              <span class="font-bold text-primary">{{ newStock }}</span>
            </div>
          </div>
        </div>

        <div class="border-base-content/10 flex items-center justify-end gap-3 border-t px-6 py-4">
          <button class="btn btn-ghost" type="button" :disabled="saving" @click="handleClose">
            {{ t('common.cancel') }}
          </button>
          <button
            class="btn btn-primary"
            type="button"
            :disabled="saving || !isValid"
            @click="handleSave"
          >
            <span v-if="saving" class="loading loading-spinner loading-xs mr-2"></span>
            {{ t('admin.inventory.adjust.save') }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import type { InventoryItem } from '~/types'

const props = defineProps<{
  visible: boolean
  item: InventoryItem | null
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const { t } = useI18n()
const { adjustStock } = useInventory()

const saving = ref(false)
const error = ref('')

const form = reactive({
  type: 'restock',
  quantity: 0,
  reason: '',
})

const newStock = computed(() => {
  if (!props.item) return 0
  const current = Number(props.item.inventory_quantity || 0)
  return current + form.quantity
})

const isValid = computed(() => {
  return form.quantity !== 0 && form.reason.trim().length > 0
})

function handleClose() {
  error.value = ''
  form.type = 'restock'
  form.quantity = 0
  form.reason = ''
  emit('close')
}

async function handleSave() {
  if (!props.item || !isValid.value) return

  saving.value = true
  error.value = ''

  try {
    await adjustStock({
      variant_id: props.item.variant_id,
      quantity: form.quantity,
      type: form.type,
      reason: form.reason.trim(),
    })
    emit('saved')
    handleClose()
  } catch (err: any) {
    error.value = err?.data?.message || err?.message || t('admin.inventory.adjust.error')
  } finally {
    saving.value = false
  }
}
</script>
