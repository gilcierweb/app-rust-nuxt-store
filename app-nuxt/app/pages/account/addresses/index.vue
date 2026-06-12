<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">Account</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">{{ t('account.addressesTitle') }}</h1>
        <p class="text-base-content/60 mt-1">{{ t('account.addressesSubtitle') }}</p>
      </div>
      <button class="btn btn-primary" @click="openAddModal">
        <span class="icon-[tabler--plus] size-5"></span>
        {{ t('account.addAddress') }}
      </button>
    </div>

    <div v-if="pending" class="rounded-box border border-base-content/10 bg-base-100 p-8">
      <div class="flex items-center gap-3 text-base-content/60">
        <span class="loading loading-spinner loading-sm"></span>
        {{ t('account.loadingAddresses') }}
      </div>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <span class="icon-[tabler--alert-circle] size-6"></span>
      <span>{{ t('account.errorLoadingAddresses') }} {{ error.message }}</span>
    </div>

    <div v-else-if="addresses.length === 0" class="rounded-box border border-dashed border-base-content/20 bg-base-100 p-10 text-center">
      <span class="icon-[tabler--map-pin-off] text-base-content/30 mx-auto mb-4 size-12"></span>
      <h2 class="text-lg font-semibold">{{ t('account.noAddressesTitle') }}</h2>
      <p class="text-base-content/60 mt-1">{{ t('account.noAddressesMessage') }}</p>
      <button class="btn btn-primary mt-6" @click="openAddModal">
        {{ t('account.addFirstAddress') }}
      </button>
    </div>

    <div v-else class="grid gap-4 md:grid-cols-2">
      <article v-for="addr in addresses" :key="addr.id" class="rounded-box border border-base-content/10 bg-base-100 p-5 shadow-sm">
        <div class="mb-4 flex items-start justify-between gap-4">
          <div class="min-w-0">
            <div class="flex items-center gap-2">
              <span class="badge badge-sm" :class="addr.default ? 'badge-primary' : 'badge-soft'">
                {{ addr.default ? t('account.default') : t(`account.addressType.${addr.type || 'other'}`) }}
              </span>
              <span v-if="addr.default" class="icon-[tabler--star-filled] text-primary size-4"></span>
            </div>
            <h3 class="mt-2 font-semibold">{{ addr.first_name }} {{ addr.last_name }}</h3>
            <p v-if="addr.company" class="text-base-content/60 text-sm">{{ addr.company }}</p>
          </div>
        </div>

        <div class="text-base-content/70 mb-4 space-y-1 text-sm">
          <p>{{ addr.address1 }}</p>
          <p v-if="addr.address2">{{ addr.address2 }}</p>
          <p>{{ addr.city }}, {{ addr.state }} {{ addr.zip_code }}</p>
          <p>{{ addr.country }}</p>
          <p v-if="addr.phone" class="text-base-content/50">{{ addr.phone }}</p>
        </div>

        <div class="flex items-center gap-2">
          <button
            v-if="!addr.default"
            class="btn btn-soft btn-xs"
            @click="setDefault(addr)"
            :disabled="settingDefaultId === addr.id"
          >
            <span v-if="settingDefaultId === addr.id" class="loading loading-spinner loading-xs"></span>
            <span v-else class="icon-[tabler--star] size-3"></span>
            {{ t('account.setDefault') }}
          </button>
          <button class="btn btn-soft btn-xs" @click="openEditModal(addr)">
            <span class="icon-[tabler--pencil] size-3"></span>
            {{ t('account.edit') }}
          </button>
          <button class="btn btn-error btn-soft btn-xs" @click="confirmDelete(addr)" :disabled="deletingId === addr.id">
            <span v-if="deletingId === addr.id" class="loading loading-spinner loading-xs"></span>
            <span v-else class="icon-[tabler--trash] size-3"></span>
            {{ t('account.remove') }}
          </button>
        </div>
      </article>
    </div>

    <dialog ref="modalRef" class="modal" :class="{ 'modal-open': showModal }" @close="showModal = false">
      <div class="modal-box max-w-2xl">
        <h3 class="text-lg font-bold">
          {{ editingAddress ? t('account.editAddress') : t('account.addAddress') }}
        </h3>

        <form @submit.prevent="handleSubmit" class="mt-4 space-y-4">
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.type') }}</span>
              </label>
              <select v-model="form.type" class="select select-bordered w-full">
                <option value="">{{ t('account.addressForm.typePlaceholder') }}</option>
                <option value="home">{{ t('account.addressType.home') }}</option>
                <option value="work">{{ t('account.addressType.work') }}</option>
                <option value="other">{{ t('account.addressType.other') }}</option>
              </select>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.firstName') }} *</span>
              </label>
              <input
                v-model="form.first_name"
                type="text"
                :placeholder="t('account.addressForm.firstNamePlaceholder')"
                class="input input-bordered w-full"
                required
              />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.lastName') }} *</span>
              </label>
              <input
                v-model="form.last_name"
                type="text"
                :placeholder="t('account.addressForm.lastNamePlaceholder')"
                class="input input-bordered w-full"
                required
              />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.company') }}</span>
              </label>
              <input
                v-model="form.company"
                type="text"
                :placeholder="t('account.addressForm.companyPlaceholder')"
                class="input input-bordered w-full"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('account.addressForm.address1') }} *</span>
            </label>
            <input
              v-model="form.address1"
              type="text"
              :placeholder="t('account.addressForm.address1Placeholder')"
              class="input input-bordered w-full"
              required
            />
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text font-semibold">{{ t('account.addressForm.address2') }}</span>
            </label>
            <input
              v-model="form.address2"
              type="text"
              :placeholder="t('account.addressForm.address2Placeholder')"
              class="input input-bordered w-full"
            />
          </div>

          <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.city') }} *</span>
              </label>
              <input
                v-model="form.city"
                type="text"
                :placeholder="t('account.addressForm.cityPlaceholder')"
                class="input input-bordered w-full"
                required
              />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.state') }} *</span>
              </label>
              <input
                v-model="form.state"
                type="text"
                :placeholder="t('account.addressForm.statePlaceholder')"
                class="input input-bordered w-full"
                required
              />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.zipCode') }} *</span>
              </label>
              <input
                v-model="form.zip_code"
                type="text"
                :placeholder="t('account.addressForm.zipCodePlaceholder')"
                class="input input-bordered w-full"
                required
              />
            </div>
          </div>

          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.country') }} *</span>
              </label>
              <input
                v-model="form.country"
                type="text"
                :placeholder="t('account.addressForm.countryPlaceholder')"
                class="input input-bordered w-full"
                required
              />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">{{ t('account.addressForm.phone') }}</span>
              </label>
              <input
                v-model="form.phone"
                type="text"
                :placeholder="t('account.addressForm.phonePlaceholder')"
                class="input input-bordered w-full"
              />
            </div>
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input v-model="form.default" type="checkbox" class="checkbox checkbox-primary" />
              <span class="label-text">{{ t('account.addressForm.setDefault') }}</span>
            </label>
          </div>

          <div v-if="formError" class="alert alert-error">
            <span>{{ formError }}</span>
          </div>

          <div class="modal-action">
            <button type="button" class="btn btn-ghost" @click="closeModal" :disabled="saving">
              {{ t('account.addressForm.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="saving">
              <span v-if="saving" class="loading loading-spinner loading-xs"></span>
              {{ editingAddress ? t('account.addressForm.submitUpdate') : t('account.addressForm.submitSave') }}
            </button>
          </div>
        </form>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
  </div>
</template>

<script setup lang="ts">
import type { Address } from '~/types'

definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

const { t } = useI18n()
const { apiFetch, useApiLazyFetch } = useApi()
const toast = useAppToast()
const dialog = useAppDialog()

useSeoMeta({
  title: t('account.addressesTitle')
})

const showModal = ref(false)
const editingAddress = ref<Address | null>(null)
const saving = ref(false)
const formError = ref('')
const deletingId = ref<number | null>(null)
const settingDefaultId = ref<number | null>(null)
const modalRef = ref<HTMLDialogElement | null>(null)

const form = reactive({
  type: '',
  first_name: '',
  last_name: '',
  company: '',
  address1: '',
  address2: '',
  city: '',
  state: '',
  zip_code: '',
  country: 'Brasil',
  phone: '',
  default: false
})

const { data, pending, error, refresh } = useApiLazyFetch<Address[]>(
  '/api/account/addresses/',
  { key: 'account-addresses' }
)

const addresses = computed(() => data.value ?? [])

function resetForm() {
  form.type = ''
  form.first_name = ''
  form.last_name = ''
  form.company = ''
  form.address1 = ''
  form.address2 = ''
  form.city = ''
  form.state = ''
  form.zip_code = ''
  form.country = 'Brasil'
  form.phone = ''
  form.default = false
}

function openAddModal() {
  editingAddress.value = null
  resetForm()
  formError.value = ''
  showModal.value = true
}

function openEditModal(addr: Address) {
  editingAddress.value = addr
  form.type = addr.type || ''
  form.first_name = addr.first_name || ''
  form.last_name = addr.last_name || ''
  form.company = addr.company || ''
  form.address1 = addr.address1 || ''
  form.address2 = addr.address2 || ''
  form.city = addr.city || ''
  form.state = addr.state || ''
  form.zip_code = addr.zip_code || ''
  form.country = addr.country || 'Brasil'
  form.phone = addr.phone || ''
  form.default = addr.default || false
  formError.value = ''
  showModal.value = true
}

function closeModal() {
  showModal.value = false
  editingAddress.value = null
  formError.value = ''
}

async function handleSubmit() {
  if (!form.first_name || !form.last_name || !form.address1 || !form.city || !form.state || !form.zip_code || !form.country) {
    formError.value = t('account.addressForm.validation.required')
    return
  }

  saving.value = true
  formError.value = ''

  const payload: Record<string, unknown> = {
    first_name: form.first_name.trim(),
    last_name: form.last_name.trim(),
    address1: form.address1.trim(),
    city: form.city.trim(),
    state: form.state.trim(),
    zip_code: form.zip_code.trim(),
    country: form.country.trim(),
    default: form.default
  }
  if (form.type) payload.type = form.type
  if (form.company) payload.company = form.company.trim()
  if (form.address2) payload.address2 = form.address2.trim()
  if (form.phone) payload.phone = form.phone.trim()

  try {
    if (editingAddress.value) {
      await apiFetch(`/api/account/addresses/${editingAddress.value.id}`, {
        method: 'PUT',
        body: payload
      })
      toast.success(t('account.addressForm.successUpdated'))
    } else {
      await apiFetch('/api/account/addresses/', {
        method: 'POST',
        body: payload
      })
      toast.success(t('account.addressForm.successCreated'))
    }
    await refresh()
    closeModal()
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : String(err)
    formError.value = `${t('account.addressForm.error')} ${message}`
  } finally {
    saving.value = false
  }
}

async function confirmDelete(addr: Address) {
  const confirmed = await dialog.confirm({
    title: t('account.remove'),
    message: t('account.confirmDeleteAddress', { name: `${addr.first_name} ${addr.last_name}` }),
    tone: 'danger'
  })
  if (!confirmed) return

  deletingId.value = addr.id
  try {
    await apiFetch(`/api/account/addresses/${addr.id}`, { method: 'DELETE' })
    toast.success(t('account.addressDeleted'))
    await refresh()
  } catch {
    toast.error(t('account.errorDeletingAddress'))
  } finally {
    deletingId.value = null
  }
}

async function setDefault(addr: Address) {
  settingDefaultId.value = addr.id
  try {
    await apiFetch(`/api/account/addresses/${addr.id}`, {
      method: 'PUT',
      body: { default: true }
    })
    toast.success(t('account.addressSetDefault'))
    await refresh()
  } catch {
    toast.error(t('account.errorSettingDefault'))
  } finally {
    settingDefaultId.value = null
  }
}
</script>
