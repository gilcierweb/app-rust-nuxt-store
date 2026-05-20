<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">{{ $t('admin.settings.title') }}</h1>
        <p class="text-sm text-base-content/60">Persist store, SEO, API, notification, and security settings.</p>
      </div>
      <button class="btn btn-primary" type="button" :disabled="pending || isSaving" @click="saveSettings">
        <span v-if="isSaving" class="loading loading-spinner loading-sm"></span>
        <i v-else class="icon-[tabler--device-floppy] size-5"></i>
        {{ $t('admin.settings.save') }}
      </button>
    </div>

    <div v-if="pending" class="card shadow-base-300/10 shadow-md">
      <div class="card-body flex flex-col items-center justify-center py-20">
        <span class="loading loading-spinner text-primary size-12"></span>
        <p class="mt-4 text-base-content/60">Loading settings...</p>
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>Failed to load settings: {{ error.message }}</span>
    </div>

    <div v-else class="grid grid-cols-1 gap-6 lg:grid-cols-[260px_1fr]">
      <div class="card shadow-base-300/10 shadow-md">
        <div class="card-body p-2">
          <button
            v-for="group in groups"
            :key="group.namespace"
            type="button"
            :class="[
              'flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left transition-colors',
              activeNamespace === group.namespace ? 'bg-primary/10 text-primary font-bold' : 'hover:bg-base-200'
            ]"
            @click="activeNamespace = group.namespace"
          >
            <i :class="[groupIcon(group.namespace), 'size-5']"></i>
            <span>{{ group.label }}</span>
          </button>
        </div>
      </div>

      <div class="card shadow-base-300/10 shadow-md">
        <div v-if="currentGroup">
          <div class="card-header">
            <h2 class="card-title text-xl">{{ currentGroup.label }}</h2>
          </div>

          <div class="card-body">
            <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
              <div v-for="setting in currentGroup.settings" :key="settingId(setting)" class="form-control">
                <label class="label">
                  <span class="label-text font-medium">{{ setting.label }}</span>
                </label>

                <input
                  v-if="setting.value_type === 1"
                  v-model="formValues[settingId(setting)]"
                  class="input input-bordered w-full"
                  :type="setting.key.includes('email') ? 'email' : 'text'"
                />

                <label v-else class="flex min-h-12 items-center justify-between rounded-lg border border-base-300 px-4">
                  <span class="text-sm text-base-content/70">{{ booleanLabel(formValues[settingId(setting)]) }}</span>
                  <input
                    type="checkbox"
                    class="toggle toggle-success"
                    :checked="formValues[settingId(setting)] === 'true'"
                    @change="setBooleanValue(setting, $event)"
                  />
                </label>
              </div>
            </div>

            <div v-if="saveMessage" class="alert alert-success mt-6">
              <i class="icon-[tabler--check] size-6"></i>
              <span>{{ saveMessage }}</span>
            </div>

            <div v-if="saveError" class="alert alert-error mt-6">
              <i class="icon-[tabler--alert-circle] size-6"></i>
              <span>{{ saveError }}</span>
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

interface AdminSetting {
  namespace: string
  key: string
  label: string
  value_type: number
  value: string
}

interface AdminSettingsGroup {
  namespace: string
  label: string
  settings: AdminSetting[]
}

interface AdminSettingsResponse {
  groups: AdminSettingsGroup[]
}

const { apiFetch, useApiFetch } = useApi()

const activeNamespace = ref('general')
const formValues = reactive<Record<string, string>>({})
const isSaving = ref(false)
const saveMessage = ref('')
const saveError = ref('')

const settingId = (setting: Pick<AdminSetting, 'namespace' | 'key'>) => {
  return `${setting.namespace}.${setting.key}`
}

const { pending, data, error, refresh } = await useApiFetch<AdminSettingsResponse>(
  '/api/admin/settings',
  { key: 'admin-settings' }
)

const groups = computed(() => data.value?.groups || [])
const currentGroup = computed(() => {
  return groups.value.find(group => group.namespace === activeNamespace.value) || groups.value[0]
})

watch(
  groups,
  (value) => {
    for (const group of value) {
      for (const setting of group.settings) {
        formValues[settingId(setting)] = setting.value
      }
    }

    if (value.length > 0 && !value.some(group => group.namespace === activeNamespace.value)) {
      activeNamespace.value = value[0].namespace
    }
  },
  { immediate: true }
)

const groupIcon = (namespace: string) => {
  switch (namespace) {
    case 'general': return 'icon-[tabler--settings]'
    case 'seo': return 'icon-[tabler--world]'
    case 'api': return 'icon-[tabler--code]'
    case 'notifications': return 'icon-[tabler--mail]'
    case 'security': return 'icon-[tabler--shield-lock]'
    default: return 'icon-[tabler--adjustments]'
  }
}

const booleanLabel = (value: string) => {
  return value === 'true' ? 'Enabled' : 'Disabled'
}

const setBooleanValue = (setting: AdminSetting, event: Event) => {
  const input = event.target as HTMLInputElement
  formValues[settingId(setting)] = input.checked ? 'true' : 'false'
}

const saveSettings = async () => {
  saveMessage.value = ''
  saveError.value = ''
  isSaving.value = true

  try {
    const settings = groups.value.flatMap(group =>
      group.settings.map(setting => ({
        namespace: setting.namespace,
        key: setting.key,
        value: formValues[settingId(setting)] ?? ''
      }))
    )

    await apiFetch('/api/admin/settings', {
      method: 'PUT',
      body: { settings }
    })
    await refresh()
    saveMessage.value = 'Settings saved.'
  } catch (err: any) {
    saveError.value = err?.message || 'Failed to save settings.'
  } finally {
    isSaving.value = false
  }
}
</script>
