<template>
  <div class="max-w-2xl mx-auto">
    <div class="card bg-white shadow-lg">
      <div class="card-body">
        <h2 class="card-title text-2xl font-bold mb-6">
          {{ isEditing ? 'Editar Perfil' : 'Novo Perfil' }}
        </h2>

        <!-- Loading State -->
        <div v-if="pending" class="flex items-center justify-center py-8">
          <span class="loading loading-spinner text-primary size-12"></span>
          <span class="ml-3">Salvando perfil...</span>
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
          <!-- Avatar Preview -->
          <div v-if="form.avatar" class="flex justify-center mb-4">
            <div class="avatar">
              <div class="w-24 rounded-full">
                <img :src="form.avatar" alt="Avatar preview" />
              </div>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- First Name -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Nome *</span>
              </label>
              <input
                v-model="form.first_name"
                type="text"
                placeholder="Primeiro nome"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.first_name }"
                required
                :disabled="pending"
              />
              <label v-if="errors.first_name" class="label">
                <span class="label-text-alt text-error">{{ errors.first_name }}</span>
              </label>
            </div>

            <!-- Last Name -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Sobrenome *</span>
              </label>
              <input
                v-model="form.last_name"
                type="text"
                placeholder="Sobrenome"
                class="input input-bordered w-full"
                :class="{ 'input-error': errors.last_name }"
                required
                :disabled="pending"
              />
              <label v-if="errors.last_name" class="label">
                <span class="label-text-alt text-error">{{ errors.last_name }}</span>
              </label>
            </div>
          </div>

          <!-- Full Name (Auto-generated) -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Nome Completo</span>
            </label>
            <input
              v-model="form.full_name"
              type="text"
              placeholder="Nome completo"
              class="input input-bordered w-full"
              :disabled="pending"
            />
            <label class="label">
              <span class="label-text-alt text-gray-500">Gerado automaticamente se deixado em branco</span>
            </label>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Username -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Username</span>
              </label>
              <input
                v-model="form.username"
                type="text"
                placeholder="nome_usuario"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <!-- Nickname -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Apelido</span>
              </label>
              <input
                v-model="form.nickname"
                type="text"
                placeholder="Apelido"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Phone -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Telefone</span>
              </label>
              <input
                v-model.number="form.phone"
                type="tel"
                placeholder="5511999999999"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>

            <!-- WhatsApp -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">WhatsApp</span>
              </label>
              <input
                v-model.number="form.whatsapp"
                type="tel"
                placeholder="5511999999999"
                class="input input-bordered w-full"
                :disabled="pending"
              />
            </div>
          </div>

          <!-- Birth Date -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Data de Nascimento</span>
            </label>
            <input
              v-model="form.birth_date"
              type="date"
              class="input input-bordered w-full"
              :disabled="pending"
            />
          </div>

          <!-- Avatar URL -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">URL do Avatar</span>
            </label>
            <input
              v-model="form.avatar"
              type="url"
              placeholder="https://exemplo.com/avatar.jpg"
              class="input input-bordered w-full"
              :disabled="pending"
            />
          </div>

          <!-- Bio -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Biografia</span>
            </label>
            <textarea
              v-model="form.bio"
              placeholder="Sobre você..."
              class="textarea textarea-bordered w-full"
              rows="3"
              :disabled="pending"
            ></textarea>
          </div>

          <!-- User ID -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">Usuário ID *</span>
            </label>
            <input
              v-model.number="form.user_id"
              type="number"
              placeholder="ID do usuário"
              class="input input-bordered w-full"
              :class="{ 'input-error': errors.user_id }"
              required
              :disabled="pending || isEditing"
            />
            <label v-if="errors.user_id" class="label">
              <span class="label-text-alt text-error">{{ errors.user_id }}</span>
            </label>
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
              {{ isEditing ? 'Atualizar' : 'Salvar' }} Perfil
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Profile } from '~/types'

interface Props {
  profile?: Partial<Profile>
  isEditing?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEditing: false
})

const emit = defineEmits<{
  (e: 'saved', profile: Profile): void
  (e: 'cancel'): void
}>()

const config = useRuntimeConfig()

// Form state
const form = reactive({
  first_name: '',
  last_name: '',
  full_name: '',
  username: '',
  nickname: '',
  phone: null as number | null,
  birth_date: '',
  avatar: '',
  bio: '',
  whatsapp: null as number | null,
  user_id: null as number | null
})

const errors = reactive({
  first_name: '',
  last_name: '',
  user_id: ''
})

const pending = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Auto-generate full name
watch([() => form.first_name, () => form.last_name], ([first, last]) => {
  if (!form.full_name || form.full_name === `${props.profile?.first_name || ''} ${props.profile?.last_name || ''}`.trim()) {
    form.full_name = `${first} ${last}`.trim()
  }
})

// Populate form when editing
onMounted(() => {
  if (props.profile && props.isEditing) {
    form.first_name = props.profile.first_name || ''
    form.last_name = props.profile.last_name || ''
    form.full_name = props.profile.full_name || ''
    form.username = props.profile.username || ''
    form.nickname = props.profile.nickname || ''
    form.phone = props.profile.phone ?? null
    form.birth_date = props.profile.birth_date || ''
    form.avatar = props.profile.avatar || ''
    form.bio = props.profile.bio || ''
    form.whatsapp = props.profile.whatsapp ?? null
    form.user_id = props.profile.user_id ?? null
  }
})

// Watch for profile prop changes (in case it loads async)
watch(() => props.profile, (newProfile) => {
  if (newProfile && props.isEditing) {
    form.first_name = newProfile.first_name || ''
    form.last_name = newProfile.last_name || ''
    form.full_name = newProfile.full_name || ''
    form.username = newProfile.username || ''
    form.nickname = newProfile.nickname || ''
    form.phone = newProfile.phone ?? null
    form.birth_date = newProfile.birth_date || ''
    form.avatar = newProfile.avatar || ''
    form.bio = newProfile.bio || ''
    form.whatsapp = newProfile.whatsapp ?? null
    form.user_id = newProfile.user_id ?? null
  }
}, { immediate: true })

// Validation
const validate = () => {
  let isValid = true
  errors.first_name = ''
  errors.last_name = ''
  errors.user_id = ''

  if (!form.first_name?.trim()) {
    errors.first_name = 'O nome é obrigatório'
    isValid = false
  }

  if (!form.last_name?.trim()) {
    errors.last_name = 'O sobrenome é obrigatório'
    isValid = false
  }

  if (!form.user_id) {
    errors.user_id = 'O ID do usuário é obrigatório'
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
      first_name: form.first_name.trim() || null,
      last_name: form.last_name.trim() || null,
      full_name: form.full_name.trim() || `${form.first_name} ${form.last_name}`.trim(),
      username: form.username?.trim() || null,
      nickname: form.nickname?.trim() || null,
      phone: form.phone,
      birth_date: form.birth_date || null,
      avatar: form.avatar?.trim() || null,
      bio: form.bio?.trim() || null,
      whatsapp: form.whatsapp,
      user_id: form.user_id
    }

    const url = props.isEditing
      ? `${config.public.baseURL}/api/admin/profiles/${props.profile?.id}`
      : `${config.public.baseURL}/api/admin/profiles`

    const method = props.isEditing ? 'PUT' : 'POST'

    const response = await $fetch<Profile>(url, {
      method,
      body: payload
    })

    successMessage.value = props.isEditing
      ? 'Perfil atualizado com sucesso!'
      : 'Perfil criado com sucesso!'

    emit('saved', response)
  } catch (err: any) {
    errorMessage.value = err?.data?.message || err.message || 'Erro ao salvar perfil. Tente novamente.'
  } finally {
    pending.value = false
  }
}
</script>

<style scoped></style>
