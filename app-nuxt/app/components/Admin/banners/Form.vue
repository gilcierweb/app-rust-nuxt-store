<template>
  <div class="max-w-4xl mx-auto">
    <div class="card bg-base-100 shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? t('admin.banners.form.titleEdit') : t('admin.banners.form.titleNew') }}
        </h2>

        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">{{ t('admin.banners.form.saving') }}</span>
        </div>

        <AppAlert
          v-if="successMessage"
          type="success"
          :message="successMessage"
          :auto-close="3000"
          @close="successMessage = ''"
        />
        <AppAlert
          v-if="errorMessage"
          type="error"
          :message="errorMessage"
          :auto-close="5000"
          :dismissible="true"
          @close="errorMessage = ''"
        />

        <form class="space-y-6" novalidate @submit.prevent="onSubmit">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="form-control md:col-span-2">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.title') }} *</span>
              </label>
              <input
                v-model="form.title"
                type="text"
                maxlength="150"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.title }"
                :disabled="pending"
              />
              <label v-if="errors.title" class="label">
                <span class="label-text-alt text-error">{{ errors.title }}</span>
              </label>
            </div>

            <div class="form-control md:col-span-2">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.description') }}</span>
              </label>
              <textarea
                v-model="form.description"
                class="textarea textarea-bordered min-h-24"
                :disabled="pending"
              ></textarea>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.position') }} *</span>
              </label>
              <select
                v-model.number="form.position_id"
                class="select select-bordered w-full"
                :class="{ 'select-error': errors.position_id }"
                :disabled="pending"
              >
                <option :value="0">{{ t('admin.banners.form.selectPosition') }}</option>
                <option v-for="position in activePositions" :key="position.id" :value="position.id">
                  {{ position.name }} ({{ position.code }})
                </option>
              </select>
              <label v-if="errors.position_id" class="label">
                <span class="label-text-alt text-error">{{ errors.position_id }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.status') }}</span>
              </label>
              <select v-model.number="form.status" class="select select-bordered w-full" :disabled="pending">
                <option :value="1">{{ t('admin.banners.status.draft') }}</option>
                <option :value="2">{{ t('admin.banners.status.active') }}</option>
                <option :value="3">{{ t('admin.banners.status.paused') }}</option>
                <option :value="4">{{ t('admin.banners.status.expired') }}</option>
              </select>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.device') }}</span>
              </label>
              <select v-model.number="form.device" class="select select-bordered w-full" :disabled="pending">
                <option :value="1">{{ t('admin.banners.devices.all') }}</option>
                <option :value="2">{{ t('admin.banners.devices.desktop') }}</option>
                <option :value="3">{{ t('admin.banners.devices.mobile') }}</option>
              </select>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.priority') }}</span>
              </label>
              <input
                v-model.number="form.priority"
                type="number"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <div class="form-control md:col-span-2">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.imageDesktopUrl') }} *</span>
              </label>
              <input
                v-model="form.image_desktop_url"
                type="url"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.image_desktop_url }"
                :disabled="pending"
              />
              <label v-if="errors.image_desktop_url" class="label">
                <span class="label-text-alt text-error">{{ errors.image_desktop_url }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.imageMobileUrl') }}</span>
              </label>
              <input v-model="form.image_mobile_url" type="url" class="input input-bordered w-full" :disabled="pending" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.altText') }}</span>
              </label>
              <input v-model="form.alt_text" type="text" maxlength="150" class="input input-bordered w-full" :disabled="pending" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.linkUrl') }}</span>
              </label>
              <input v-model="form.link_url" type="url" class="input input-bordered w-full" :disabled="pending" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.linkTarget') }}</span>
              </label>
              <select v-model.number="form.link_target" class="select select-bordered w-full" :disabled="pending">
                <option :value="1">{{ t('admin.banners.linkTargets.sameTab') }}</option>
                <option :value="2">{{ t('admin.banners.linkTargets.newTab') }}</option>
              </select>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.startAt') }} *</span>
              </label>
              <input
                v-model="form.start_at"
                type="datetime-local"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.start_at }"
                :disabled="pending"
              />
              <label v-if="errors.start_at" class="label">
                <span class="label-text-alt text-error">{{ errors.start_at }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.endAt') }}</span>
              </label>
              <input
                v-model="form.end_at"
                type="datetime-local"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.end_at }"
                :disabled="pending"
              />
              <label v-if="errors.end_at" class="label">
                <span class="label-text-alt text-error">{{ errors.end_at }}</span>
              </label>
            </div>

            <div class="form-control md:col-span-2">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.campaignName') }}</span>
              </label>
              <input v-model="form.campaign_name" type="text" maxlength="150" class="input input-bordered w-full" :disabled="pending" />
            </div>
          </div>

          <div v-if="form.image_desktop_url" class="rounded-box border overflow-hidden bg-base-200">
            <img :src="form.image_desktop_url" :alt="form.alt_text || form.title" class="w-full max-h-72 object-cover" />
          </div>

          <div class="flex justify-end gap-4 pt-6">
            <button type="button" class="btn btn-outline" :disabled="pending" @click="emit('cancel')">
              {{ t('common.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? t('admin.banners.form.submitUpdate') : t('admin.banners.form.submitSave') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Banner, BannerPosition } from '~/types'

interface Props {
  banner?: Partial<Banner>
  positions?: BannerPosition[]
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  positions: () => [],
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', banner: Banner): void
  (e: 'cancel'): void
}>()

const { t } = useI18n()
const { apiFetch } = useApi()

const form = reactive({
  title: '',
  description: '',
  image_desktop_url: '',
  image_mobile_url: '',
  alt_text: '',
  link_url: '',
  link_target: 1,
  position_id: 0,
  device: 1,
  start_at: '',
  end_at: '',
  priority: 0,
  status: 1,
  campaign_name: ''
})

const errors = reactive({
  title: '',
  image_desktop_url: '',
  position_id: '',
  start_at: '',
  end_at: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

const activePositions = computed(() => props.positions.filter(position => position.is_active || position.id === form.position_id))

watch(() => props.banner, (banner) => {
  if (!banner || !props.isEditing) return

  form.title = banner.title || ''
  form.description = banner.description || ''
  form.image_desktop_url = banner.image_desktop_url || ''
  form.image_mobile_url = banner.image_mobile_url || ''
  form.alt_text = banner.alt_text || ''
  form.link_url = banner.link_url || ''
  form.link_target = banner.link_target ?? 1
  form.position_id = banner.position_id ?? 0
  form.device = banner.device ?? 1
  form.start_at = banner.start_at ? formatDateTimeLocal(banner.start_at) : ''
  form.end_at = banner.end_at ? formatDateTimeLocal(banner.end_at) : ''
  form.priority = banner.priority ?? 0
  form.status = banner.status ?? 1
  form.campaign_name = banner.campaign_name || ''
}, { immediate: true })

if (!props.isEditing && !form.start_at) {
  form.start_at = formatDateTimeLocal(new Date().toISOString())
}

function formatDateTimeLocal(dateString: string) {
  const date = new Date(dateString)
  const offsetMs = date.getTimezoneOffset() * 60_000
  return new Date(date.getTime() - offsetMs).toISOString().slice(0, 16)
}

function toIsoDateTime(value: string) {
  return value ? new Date(value).toISOString() : null
}

function validate() {
  let isValid = true
  errors.title = ''
  errors.image_desktop_url = ''
  errors.position_id = ''
  errors.start_at = ''
  errors.end_at = ''

  if (!form.title.trim()) {
    errors.title = t('admin.banners.form.validation.titleRequired')
    isValid = false
  }

  if (!form.image_desktop_url.trim()) {
    errors.image_desktop_url = t('admin.banners.form.validation.imageRequired')
    isValid = false
  }

  if (!form.position_id) {
    errors.position_id = t('admin.banners.form.validation.positionRequired')
    isValid = false
  }

  if (!form.start_at) {
    errors.start_at = t('admin.banners.form.validation.startRequired')
    isValid = false
  }

  if (form.end_at && form.start_at && new Date(form.end_at) < new Date(form.start_at)) {
    errors.end_at = t('admin.banners.form.validation.endAfterStart')
    isValid = false
  }

  return isValid
}

async function onSubmit() {
  if (!validate()) return

  pending.value = true
  successMessage.value = ''
  errorMessage.value = ''

  try {
    const payload = {
      title: form.title.trim(),
      description: form.description.trim() || null,
      image_desktop_url: form.image_desktop_url.trim(),
      image_mobile_url: form.image_mobile_url.trim() || null,
      alt_text: form.alt_text.trim() || null,
      link_url: form.link_url.trim() || null,
      link_target: form.link_target,
      position_id: form.position_id,
      device: form.device,
      start_at: toIsoDateTime(form.start_at),
      end_at: toIsoDateTime(form.end_at),
      priority: form.priority || 0,
      status: form.status,
      campaign_name: form.campaign_name.trim() || null
    }

    const url = props.isEditing ? `/api/admin/banners/${props.banner?.id}` : '/api/admin/banners'
    const method = props.isEditing ? 'PUT' : 'POST'
    const response = await apiFetch<Banner>(url, { method, body: payload })

    successMessage.value = props.isEditing
      ? t('admin.banners.form.success.updated')
      : t('admin.banners.form.success.created')

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.description || err?.data?.message || err.message || t('admin.banners.form.error')
  } finally {
    pending.value = false
  }
}
</script>
