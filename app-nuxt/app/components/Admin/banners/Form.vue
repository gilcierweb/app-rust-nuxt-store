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
                v-model="title"
                @blur="titleBlur"
                type="text"
                maxlength="150"
                class="input input-bordered w-full"
                :class="{ 'input-error': titleError }"
                :disabled="pending"
              />
              <label v-if="titleError" class="label">
                <span class="label-text-alt text-error">{{ titleError }}</span>
              </label>
            </div>

            <div class="form-control md:col-span-2">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.description') }}</span>
              </label>
              <textarea
                v-model="values.description"
                class="textarea textarea-bordered min-h-24"
                :disabled="pending"
              ></textarea>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.position') }} *</span>
              </label>
              <select
                v-model.number="position_id"
                @blur="position_idBlur"
                class="select select-bordered w-full"
                :class="{ 'select-error': position_idError }"
                :disabled="pending"
              >
                <option :value="0">{{ t('admin.banners.form.selectPosition') }}</option>
                <option v-for="position in activePositions" :key="position.id" :value="position.id">
                  {{ position.name }} ({{ position.code }})
                </option>
              </select>
              <label v-if="position_idError" class="label">
                <span class="label-text-alt text-error">{{ position_idError }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.status') }}</span>
              </label>
              <select                 v-model.number="values.status" class="select select-bordered w-full" :disabled="pending">
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
              <select                 v-model.number="values.device" class="select select-bordered w-full" :disabled="pending">
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
                v-model.number="values.priority"
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
                v-model="image_desktop_url"
                @blur="image_desktop_urlBlur"
                type="url"
                class="input input-bordered w-full"
                :class="{ 'input-error': image_desktop_urlError }"
                :disabled="pending"
              />
              <label v-if="image_desktop_urlError" class="label">
                <span class="label-text-alt text-error">{{ image_desktop_urlError }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.imageMobileUrl') }}</span>
              </label>
              <input v-model="values.image_mobile_url" type="url" class="input input-bordered w-full" :disabled="pending" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.altText') }}</span>
              </label>
              <input v-model="values.alt_text" type="text" maxlength="150" class="input input-bordered w-full" :disabled="pending" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.linkUrl') }}</span>
              </label>
              <input v-model="values.link_url" type="url" class="input input-bordered w-full" :disabled="pending" />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.linkTarget') }}</span>
              </label>
              <select v-model.number="values.link_target" class="select select-bordered w-full" :disabled="pending">
                <option :value="1">{{ t('admin.banners.linkTargets.sameTab') }}</option>
                <option :value="2">{{ t('admin.banners.linkTargets.newTab') }}</option>
              </select>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.startAt') }} *</span>
              </label>
              <input
                v-model="start_at"
                @blur="start_atBlur"
                type="datetime-local"
                class="input input-bordered w-full"
                :class="{ 'input-error': start_atError }"
                :disabled="pending"
              />
              <label v-if="start_atError" class="label">
                <span class="label-text-alt text-error">{{ start_atError }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.endAt') }}</span>
              </label>
              <input
                v-model="end_at"
                @blur="end_atBlur"
                type="datetime-local"
                class="input input-bordered w-full"
                :class="{ 'input-error': end_atError }"
                :disabled="pending"
              />
              <label v-if="end_atError" class="label">
                <span class="label-text-alt text-error">{{ end_atError }}</span>
              </label>
            </div>

            <div class="form-control md:col-span-2">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.banners.form.campaignName') }}</span>
              </label>
              <input v-model="values.campaign_name" type="text" maxlength="150" class="input input-bordered w-full" :disabled="pending" />
            </div>
          </div>

          <div v-if="values.image_desktop_url" class="rounded-box border overflow-hidden bg-base-200">
            <img :src="values.image_desktop_url" :alt="values.alt_text || values.title" class="w-full max-h-72 object-cover" />
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
import { useForm, useField } from 'vee-validate'

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

const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
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
  }
})

const { value: title, errorMessage: titleError, handleBlur: titleBlur } = useField<string>('title', (v) => {
  if (!v?.trim()) return t('admin.banners.form.validation.titleRequired')
  return true
})
const { value: image_desktop_url, errorMessage: image_desktop_urlError, handleBlur: image_desktop_urlBlur } = useField<string>('image_desktop_url', (v) => {
  if (!v?.trim()) return t('admin.banners.form.validation.imageRequired')
  return true
})
const { value: position_id, errorMessage: position_idError, handleBlur: position_idBlur } = useField<number>('position_id', (v) => {
  if (!v) return t('admin.banners.form.validation.positionRequired')
  return true
})
const { value: start_at, errorMessage: start_atError, handleBlur: start_atBlur } = useField<string>('start_at', (v) => {
  if (!v) return t('admin.banners.form.validation.startRequired')
  return true
})
const { value: end_at, errorMessage: end_atError, handleBlur: end_atBlur } = useField<string>('end_at', (v) => {
  if (v && values.start_at && new Date(v) < new Date(values.start_at)) {
    return t('admin.banners.form.validation.endAfterStart')
  }
  return true
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

const activePositions = computed(() => props.positions.filter(position => position.is_active || position.id === values.position_id))

function formatDateTimeLocal(dateString: string) {
  const date = new Date(dateString)
  const offsetMs = date.getTimezoneOffset() * 60_000
  return new Date(date.getTime() - offsetMs).toISOString().slice(0, 16)
}

function toIsoDateTime(value: string) {
  return value ? new Date(value).toISOString() : null
}

watch(() => props.banner, (banner) => {
  if (!banner || !props.isEditing) return

  setFieldValue('title', banner.title || '')
  setFieldValue('description', banner.description || '')
  setFieldValue('image_desktop_url', banner.image_desktop_url || '')
  setFieldValue('image_mobile_url', banner.image_mobile_url || '')
  setFieldValue('alt_text', banner.alt_text || '')
  setFieldValue('link_url', banner.link_url || '')
  setFieldValue('link_target', banner.link_target ?? 1)
  setFieldValue('position_id', banner.position_id ?? 0)
  setFieldValue('device', banner.device ?? 1)
  setFieldValue('start_at', banner.start_at ? formatDateTimeLocal(banner.start_at) : '')
  setFieldValue('end_at', banner.end_at ? formatDateTimeLocal(banner.end_at) : '')
  setFieldValue('priority', banner.priority ?? 0)
  setFieldValue('status', banner.status ?? 1)
  setFieldValue('campaign_name', banner.campaign_name || '')
}, { immediate: true })

if (!props.isEditing && !values.start_at) {
  setFieldValue('start_at', formatDateTimeLocal(new Date().toISOString()))
}

const onSubmit = handleSubmit(async () => {
  pending.value = true
  successMessage.value = ''
  errorMessage.value = ''

  try {
    const payload = {
      title: values.title.trim(),
      description: values.description.trim() || null,
      image_desktop_url: values.image_desktop_url.trim(),
      image_mobile_url: values.image_mobile_url.trim() || null,
      alt_text: values.alt_text.trim() || null,
      link_url: values.link_url.trim() || null,
      link_target: values.link_target,
      position_id: values.position_id,
      device: values.device,
      start_at: toIsoDateTime(values.start_at),
      end_at: toIsoDateTime(values.end_at),
      priority: values.priority || 0,
      status: values.status,
      campaign_name: values.campaign_name.trim() || null
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
})
</script>
