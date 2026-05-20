<template>
  <div>
    <div class="mb-6 flex items-center gap-4">
      <NuxtLinkLocale :to="`/admin/users/${route.params.id}`" class="btn btn-ghost btn-circle" :aria-label="$t('common.back')">
        <i class="icon-[tabler--arrow-left] size-6"></i>
      </NuxtLinkLocale>
      <h1 class="h1">{{ $t('admin.users.form.editTitle') }}</h1>
    </div>

    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.users.loading') }}</span>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.users.error', { message: error.message }) }}</span>
      <button type="button" class="btn btn-sm btn-ghost" @click="() => refresh()">
        {{ $t('common.actions.tryAgain') }}
      </button>
    </div>

    <AdminUsersForm
      v-else-if="user"
      :initial-user="user"
      @saved="handleSaved"
      @cancel="navigateTo(`/admin/users/${route.params.id}`)"
    />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

interface AdminUserDetail {
  id: number
  email: string
  name: string
  role: string
  active: boolean
}

const route = useRoute()
const { useApiFetch } = useApi()

const { pending, data: user, error, refresh } = await useApiFetch<AdminUserDetail>(
  `/api/admin/users/${route.params.id}`,
  { key: `admin-user-edit-${route.params.id}` }
)

function handleSaved(savedUser: AdminUserDetail) {
  navigateTo(`/admin/users/${savedUser.id}`)
}
</script>
