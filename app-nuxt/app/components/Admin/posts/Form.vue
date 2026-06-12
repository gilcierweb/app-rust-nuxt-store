<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? t('admin.posts.form.titleEdit') : t('admin.posts.form.titleNew') }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">{{ t('admin.posts.form.saving') }}</span>
        </div>

        <!-- Alerts -->
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
          @close="errorMessage = ''"
          :dismissible="true"
        />

        <!-- Form -->
        <form @submit.prevent="onSubmit" class="space-y-6" novalidate>
          <!-- Title -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.posts.form.title') }} *</span>
            </label>
            <input
              v-model="title"
              @blur="titleBlur"
              type="text"
              :placeholder="t('admin.posts.form.titlePlaceholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': titleError }"
              required
              :disabled="pending"
            />
            <label v-if="titleError" class="label">
              <span class="label-text-alt text-error">{{ titleError }}</span>
            </label>
          </div>

          <!-- Content -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('admin.posts.form.content') }}</span>
            </label>
            <textarea
              v-model="values.content"
              :placeholder="t('admin.posts.form.contentPlaceholder')"
              class="textarea textarea-bordered w-full"
              rows="6"
              :disabled="pending"
            ></textarea>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Status -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.posts.form.status') }}</span>
              </label>
              <select               v-model.number="values.status" class="select select-bordered w-full" :disabled="pending">
                <option v-for="(label, value) in PostStatusLabels" :key="value" :value="Number(value)">
                  {{ label }}
                </option>
              </select>
            </div>
          </div>

          <!-- Action Buttons -->
          <div class="flex justify-end space-x-4 pt-6">
            <button
              type="button"
              class="btn btn-outline"
              :disabled="pending"
              @click="emit('cancel')"
            >
              {{ t('admin.posts.form.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? t('admin.posts.form.submitUpdate') : t('admin.posts.form.submitSave') }} {{ t('admin.posts.form.submitPost') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types'
import { PostStatusLabels } from '~/types'
import { useForm, useField } from 'vee-validate'

interface Props {
  post?: Partial<Post>
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', post: Post): void
  (e: 'cancel'): void
}>()

const { apiFetch } = useApi()
const { t } = useI18n()

const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
    title: '',
    content: '',
    status: 1
  }
})

const { value: title, errorMessage: titleError, handleBlur: titleBlur } = useField<string>('title', (v) => {
  if (!v?.trim()) return t('admin.posts.form.validation.titleRequired')
  return true
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

onMounted(() => {
  if (props.post && props.isEditing) {
    setFieldValue('title', props.post.title || '')
    setFieldValue('content', props.post.content || '')
    setFieldValue('status', props.post.status ?? 1)
  }
})

watch(() => props.post, (newPost) => {
  if (newPost && props.isEditing) {
    setFieldValue('title', newPost.title || '')
    setFieldValue('content', newPost.content || '')
    setFieldValue('status', newPost.status ?? 1)
  }
}, { immediate: true })

const onSubmit = handleSubmit(async () => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      title: values.title.trim() || null,
      content: values.content.trim() || null,
      status: values.status
    }

    const url = props.isEditing
      ? `/api/admin/posts/${props.post?.id}`
      : '/api/admin/posts'

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await apiFetch<Post>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? t('admin.posts.form.successUpdated')
      : t('admin.posts.form.successCreated')

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || t('admin.posts.form.error')
  } finally {
    pending.value = false
  }
})
</script>

<style scoped></style>
