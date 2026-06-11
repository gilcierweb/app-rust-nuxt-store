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
              v-model="form.name"
              type="text"
              :placeholder="t('admin.categories.form.namePlaceholder')"
              class="input input-bordered w-full"
              :class="{ 'input-error': errors.name }"
              required
              :disabled="pending"
            />
            <label v-if="errors.name" class="label">
              <span class="label-text-alt text-error">{{ errors.name }}</span>
            </label>
          </div>

          <!-- Slug -->
          <div class="form-control">
            <label class="label">
                <span class="label-text font-semibold">{{ t('admin.categories.form.slug') }}</span>
            </label>
            <input
              v-model="form.slug"
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
              v-model="form.description"
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
            <select v-model="form.parent_id" class="select select-bordered w-full" :disabled="pending">
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
                v-model.number="form.position"
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
                  v-model="form.active"
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

// Form state
const form = reactive({
  name: '',
  slug: '',
  description: '',
  parent_id: null as number | null,
  position: 0,
  active: true
})

const errors = reactive({
  name: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Fetch parent categories
const { data: parentCategories } = useApiLazyFetch<Category[]>(
  '/api/admin/categories',
  { key: 'admin-categories-list' }
)

// Filter out current category from parent options (can't be parent of itself)
const availableParentCategories = computed(() => {
  if (!parentCategories.value) return []
  if (!props.isEditing || !props.category?.id) return parentCategories.value
  return parentCategories.value.filter(cat => cat.id !== props.category?.id)
})

// Generate slug from name
const generateSlug = (text: string) => {
  return text
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/(^-|-$)/g, '')
}

// Auto-generate slug when name changes
watch(() => form.name, (newName) => {
  // Only auto-generate if slug is empty or matches the previous auto-generated value
  if (!form.slug || form.slug === generateSlug(props.category?.name || '')) {
    form.slug = generateSlug(newName)
  }
})

// Populate form when editing
onMounted(() => {
  if (props.category && props.isEditing) {
    form.name = props.category.name || ''
    form.slug = props.category.slug || ''
    form.description = props.category.description || ''
    form.parent_id = props.category.parent_id || null
    form.position = props.category.position || 0
    form.active = props.category.active ?? true
  }
})

// Watch for category prop changes (in case it loads async)
watch(() => props.category, (newCategory) => {
  if (newCategory && props.isEditing) {
    form.name = newCategory.name || ''
    form.slug = newCategory.slug || ''
    form.description = newCategory.description || ''
    form.parent_id = newCategory.parent_id || null
    form.position = newCategory.position || 0
    form.active = newCategory.active ?? true
  }
}, { immediate: true })

// Validation
const validate = () => {
  let isValid = true
  errors.name = ''

  if (!form.name.trim()) {
    errors.name = t('admin.categories.form.validation.nameRequired')
    isValid = false
  }

  return isValid
}

// Submit
const onSubmit = async () => {
  if (!validate()) return

  pending.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = {
      name: form.name.trim(),
      slug: form.slug.trim() || generateSlug(form.name),
      description: form.description.trim() || null,
      parent_id: form.parent_id,
      position: form.position,
      active: form.active
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
}
</script>

<style scoped></style>
