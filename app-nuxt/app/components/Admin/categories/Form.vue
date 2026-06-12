<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? t('admin.categories.form.titleEdit') : t('admin.categories.form.titleNew') }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">{{ t('admin.categories.form.saving') }}</span>
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
          <!-- Name -->
          <div class="form-control">
            <label class="label">
                <span class="label-text font-semibold">{{ t('admin.categories.form.name') }} *</span>
            </label>
            <input
              v-model="name"
              @blur="nameBlur"
              type="text"
              :placeholder="t('admin.categories.form.namePlaceholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': nameError }"
              required
              :disabled="pending"
            />
            <label v-if="nameError" class="label">
              <span class="label-text-alt text-error">{{ nameError }}</span>
            </label>
          </div>

          <!-- Slug -->
          <div class="form-control">
            <label class="label">
                <span class="label-text font-semibold">{{ t('admin.categories.form.slug') }}</span>
            </label>
            <input
              v-model="values.slug"
              type="text"
              :placeholder="t('admin.categories.form.slugPlaceholder')"
              class="input input-bordered w-full"
              :disabled="pending"
            />
            <label class="label">
              <span class="label-text-alt text-gray-500">{{ t('admin.categories.form.slugHint') }}</span>
            </label>
          </div>

          <!-- Description -->
          <div class="form-control">
            <label class="label">
                <span class="label-text font-semibold">{{ t('admin.categories.form.description') }}</span>
            </label>
            <textarea
              v-model="values.description"
              :placeholder="t('admin.categories.form.descriptionPlaceholder')"
              class="textarea textarea-bordered w-full"
              rows="3"
              :disabled="pending"
            ></textarea>
          </div>

          <!-- Parent Category -->
          <div class="form-control">
            <label class="label">
                <span class="label-text font-semibold">{{ t('admin.categories.form.parentCategory') }}</span>
            </label>
            <select v-model="values.parent_id" class="select select-bordered w-full" :disabled="pending">
              <option :value="null">{{ t('admin.categories.form.parentCategoryRoot') }}</option>
              <option
                v-for="cat in availableParentCategories"
                :key="cat.id"
                :value="cat.id"
              >
                {{ cat.name }}
              </option>
            </select>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Position -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('admin.categories.form.position') }}</span>
              </label>
              <input
                v-model.number="values.position"
                type="number"
                min="0"
                placeholder="0"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <!-- Active -->
            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text font-semibold">{{ t('admin.categories.form.active') }}</span>
                <input
                  v-model="values.active"
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  :disabled="pending"
                />
              </label>
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
              {{ t('admin.categories.form.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? t('admin.categories.form.submitUpdate') : t('admin.categories.form.submitSave') }} {{ t('admin.categories.form.submitCategory') }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types'
import { useForm, useField } from 'vee-validate'

const { t } = useI18n()

interface Props {
  category?: Partial<Category>
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', category: Category): void
  (e: 'cancel'): void
}>()

const { apiFetch, useApiLazyFetch } = useApi()

const { handleSubmit, values, setFieldValue } = useForm({
  initialValues: {
    name: '',
    slug: '',
    description: '',
    parent_id: null as number | null,
    position: 0,
    active: true
  }
})

const { value: name, errorMessage: nameError, handleBlur: nameBlur } = useField<string>('name', (v) => {
  if (!v?.trim()) return t('admin.categories.form.validation.nameRequired')
  return true
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

const { data: parentCategories } = useApiLazyFetch<Category[]>(
  '/api/admin/categories',
  { key: 'admin-categories-list' }
)

const availableParentCategories = computed(() => {
  if (!parentCategories.value) return []
  if (!props.isEditing || !props.category?.id) return parentCategories.value
  return parentCategories.value.filter(cat => cat.id !== props.category?.id)
})

const generateSlug = (text: string) => {
  return text
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/(^-|-$)/g, '')
}

watch(name, (newName) => {
  if (!values.slug || values.slug === generateSlug(props.category?.name || '')) {
    setFieldValue('slug', generateSlug(newName))
  }
})

onMounted(() => {
  if (props.category && props.isEditing) {
    setFieldValue('name', props.category.name || '')
    setFieldValue('slug', props.category.slug || '')
    setFieldValue('description', props.category.description || '')
    setFieldValue('parent_id', props.category.parent_id || null)
    setFieldValue('position', props.category.position || 0)
    setFieldValue('active', props.category.active ?? true)
  }
})

watch(() => props.category, (newCategory) => {
  if (newCategory && props.isEditing) {
    setFieldValue('name', newCategory.name || '')
    setFieldValue('slug', newCategory.slug || '')
    setFieldValue('description', newCategory.description || '')
    setFieldValue('parent_id', newCategory.parent_id || null)
    setFieldValue('position', newCategory.position || 0)
    setFieldValue('active', newCategory.active ?? true)
  }
}, { immediate: true })

const onSubmit = handleSubmit(async () => {
  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      name: values.name.trim(),
      slug: values.slug.trim() || generateSlug(values.name),
      description: values.description.trim() || null,
      parent_id: values.parent_id,
      position: values.position,
      active: values.active
    }

    const url = props.isEditing
      ? `/api/admin/categories/${props.category?.id}`
      : '/api/admin/categories'

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await apiFetch<Category>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? t('admin.categories.form.successUpdated')
      : t('admin.categories.form.successCreated')

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || t('admin.categories.form.error')
  } finally {
    pending.value = false
  }
})
</script>

<style scoped></style>
