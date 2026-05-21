<template>
  <Teleport to="body">
    <div
      v-if="dialog"
      class="fixed inset-0 z-[9998] flex items-center justify-center bg-base-content/45 p-4"
      role="dialog"
      aria-modal="true"
      @click.self="handleCancel"
    >
      <div class="w-full max-w-lg rounded-box bg-base-100 shadow-xl">
        <div class="border-base-content/10 flex items-start justify-between gap-4 border-b px-6 py-4">
          <div>
            <h3 class="text-lg font-semibold">{{ dialog.title || t('common.confirm') }}</h3>
            <p class="mt-1 text-sm text-base-content/70">{{ dialog.message }}</p>
          </div>
          <button class="btn btn-text btn-circle btn-sm" type="button" :aria-label="t('common.close')" @click="handleCancel">
            <span class="icon-[tabler--x] size-4"></span>
          </button>
        </div>

        <div class="space-y-4 px-6 py-5">
          <AppAlert
            v-if="validationMessage"
            :message="validationMessage"
            tone="error"
            variant="soft"
          />

          <div v-if="dialog.input" class="form-control">
            <label v-if="dialog.input.label" class="label pt-0">
              <span class="label-text-alt text-base-content/60">{{ dialog.input.label }}</span>
            </label>
            <input
              ref="inputRef"
              v-model="inputValue"
              :type="dialog.input.type || 'text'"
              :min="dialog.input.min"
              :step="dialog.input.step"
              :placeholder="dialog.input.placeholder"
              class="input input-bordered w-full"
              @keyup.enter="handleConfirm"
              @keyup.esc="handleCancel"
            />
          </div>
        </div>

        <div class="border-base-content/10 flex items-center justify-end gap-3 border-t px-6 py-4">
          <button class="btn btn-ghost" type="button" @click="handleCancel">
            {{ dialog.cancelLabel || t('common.cancel') }}
          </button>
          <button
            :class="dialog.tone === 'danger' ? 'btn btn-error' : 'btn btn-primary'"
            type="button"
            @click="handleConfirm"
          >
            {{ dialog.confirmLabel || t('common.confirm') }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import AppAlert from '~/components/AppAlert.vue'

const { t } = useI18n()
const dialogApi = useAppDialog()
const dialog = computed(() => dialogApi.state.value)
const inputValue = ref('')
const inputRef = ref<HTMLInputElement | null>(null)
const validationMessage = ref('')
const handleWindowKeydown = (event: KeyboardEvent) => {
  if (!dialog.value) return
  if (event.key === 'Escape') handleCancel()
}

watch(dialog, async (value) => {
  validationMessage.value = ''
  inputValue.value = value?.input?.initialValue || ''

  if (value?.input) {
    await nextTick()
    inputRef.value?.focus()
    inputRef.value?.select()
  }
})

onMounted(() => window.addEventListener('keydown', handleWindowKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', handleWindowKeydown))

function handleCancel() {
  validationMessage.value = ''
  dialogApi.cancel()
}

function handleConfirm() {
  if (!dialog.value?.input) {
    dialogApi.resolveConfirm()
    return
  }

  const currentValue = inputValue.value.trim()
  const inputConfig = dialog.value.input

  if (inputConfig.required && !currentValue) {
    validationMessage.value = t('common.validation.required')
    return
  }

  if (currentValue && inputConfig.type === 'number') {
    const parsed = Number(currentValue)
    if (Number.isNaN(parsed)) {
      validationMessage.value = t('common.validation.invalidNumber')
      return
    }

    if (typeof inputConfig.min === 'number' && parsed < inputConfig.min) {
      validationMessage.value = t('common.validation.minNumber', { min: inputConfig.min })
      return
    }
  }

  validationMessage.value = ''
  dialogApi.resolvePrompt(currentValue)
}
</script>
