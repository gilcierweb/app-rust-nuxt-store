<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? 'Editar Post' : 'Novo Post' }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">Salvando post...</span>
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
              <span class="label-text font-semibold">Título *</span>
            </label>
            <input
              v-model="form.title"
              type="text"
              placeholder="Título do post"
              class="input input-bordered w-full"
              :class="{ 'input-error': errors.title }"
              required
              :disabled="pending"
            />
            <label v-if="errors.title" class="label">
              <span class="label-text-alt text-error">{{ errors.title }}</span>
            </label>
          </div>

          <!-- Content -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Conteúdo</span>
            </label>
            <textarea
              v-model="form.content"
              placeholder="Conteúdo do post"
              class="textarea textarea-bordered w-full"
              rows="6"
              :disabled="pending"
            ></textarea>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Status -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Status</span>
              </label>
              <select v-model.number="form.status" class="select select-bordered w-full" :disabled="pending">
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
              Cancelar
            </button>
            <button type="submit" class="btn btn-primary" :disabled="pending">
              <span v-if="pending" class="loading loading-spinner loading-sm"></span>
              {{ isEditing ? 'Atualizar' : 'Salvar' }} Post
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

// Form state
const form = reactive({
  title: '',
  content: '',
  status: 1
})

const errors = reactive({
  title: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Populate form when editing
onMounted(() => {
  if (props.post && props.isEditing) {
    form.title = props.post.title || ''
    form.content = props.post.content || ''
    form.status = props.post.status ?? 1
  }
})

// Watch for post prop changes (in case it loads async)
watch(() => props.post, (newPost) => {
  if (newPost && props.isEditing) {
    form.title = newPost.title || ''
    form.content = newPost.content || ''
    form.status = newPost.status ?? 1
  }
}, { immediate: true })

// Validation
const validate = () => {
  let isValid = true
  errors.title = ''

  if (!form.title.trim()) {
    errors.title = 'O título é obrigatório'
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
      title: form.title.trim() || null,
      content: form.content.trim() || null,
      status: form.status
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
      ? 'Post atualizado com sucesso!'
      : 'Post criado com sucesso!'

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || 'Erro ao salvar post. Tente novamente.'
  } finally {
    pending.value = false
  }
}
</script>

<style scoped></style>
