<template>
  <div>
    <div class="mb-6 flex items-center gap-4">
      <NuxtLinkLocale to="/admin/users" class="btn btn-ghost btn-circle" :aria-label="$t('common.back')">
        <i class="icon-[tabler--arrow-left] size-6"></i>
      </NuxtLinkLocale>
      <h1 class="h1">{{ $t('admin.users.form.newTitle') }}</h1>
    </div>

    <AdminUsersForm @saved="handleSaved" @cancel="navigateTo('/admin/users')" />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

function handleSaved(user: { id: number }) {
  navigateTo(`/admin/users/${user.id}`)
}
</script>
