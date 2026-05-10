<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? 'Editar Categoria' : 'Nova Categoria' }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">Salvando categoria...</span>
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
              <span class="label-text font-semibold">Nome da Categoria *</span>
            </label>
            <input
              v-model="form.name"
              type="text"
              placeholder="Nome da categoria"
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
              <span class="label-text font-semibold">Slug</span>
            </label>
            <input
              v-model="form.slug"
              type="text"
              placeholder="slug-da-categoria"
              class="input input-bordered w-full"
              :disabled="pending"
            />
            <label class="label">
              <span class="label-text-alt text-gray-500">Gerado automaticamente se deixado em branco</span>
            </label>
          </div>

          <!-- Description -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Descrição</span>
            </label>
            <textarea
              v-model="form.description"
              placeholder="Descrição da categoria"
              class="textarea textarea-bordered w-full"
              rows="3"
              :disabled="pending"
            ></textarea>
          </div>

          <!-- Parent Category -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Categoria Pai</span>
            </label>
            <select v-model="form.parent_id" class="select select-bordered w-full" :disabled="pending">
              <option :value="null">Nenhuma (categoria raiz)</option>
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
                <span class="label-text font-semibold">Posição</span>
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
                <span class="label-text font-semibold">Categoria Ativa</span>
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
              Cancelar
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? 'Atualizar' : 'Salvar' }} Categoria
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Category } from '~/types'

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

const config = useRuntimeConfig()

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
const { data: parentCategories } = useLazyFetch<Category[]>(
  `${config.public.baseURL}/api/categories`
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
    errors.name = 'O nome é obrigatório'
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
      ? `${config.public.baseURL}/api/categories/${props.category?.id}`
      : `${config.public.baseURL}/api/categories`

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await $fetch<Category>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? 'Categoria atualizada com sucesso!'
      : 'Categoria criada com sucesso!'

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || 'Erro ao salvar categoria. Tente novamente.'
  } finally {
    pending.value = false
  }
}
</script>

<style scoped></style>