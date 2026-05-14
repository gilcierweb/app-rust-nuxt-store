<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-primary text-sm font-semibold uppercase tracking-wide">Account</p>
        <h1 class="text-base-content text-2xl font-bold md:text-3xl">{{ $t('account.settingsTitle') }}</h1>
        <p class="text-base-content/60 mt-1">{{ $t('account.settingsSubtitle') }}</p>
      </div>
      <button class="btn btn-error btn-soft" @click="logout">
        <span class="icon-[tabler--logout] size-5"></span>
        {{ $t('account.logout') }}
      </button>
    </div>

    <div class="grid gap-6 lg:grid-cols-3">
      <section class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm lg:col-span-2">
        <h2 class="mb-5 text-lg font-semibold">{{ $t('account.profile') }}</h2>
        <dl class="grid gap-5 sm:grid-cols-2">
          <div>
            <dt class="text-base-content/60 text-sm">{{ $t('account.name') }}</dt>
            <dd class="mt-1 font-medium">{{ user?.name || '-' }}</dd>
          </div>
          <div>
            <dt class="text-base-content/60 text-sm">{{ $t('account.email') }}</dt>
            <dd class="mt-1 break-all font-medium">{{ user?.email || '-' }}</dd>
          </div>
          <div>
            <dt class="text-base-content/60 text-sm">{{ $t('account.pid') }}</dt>
            <dd class="mt-1 break-all font-mono text-sm">{{ user?.pid || '-' }}</dd>
          </div>
          <div>
            <dt class="text-base-content/60 text-sm">{{ $t('account.admin') }}</dt>
            <dd class="mt-1">
              <span :class="['badge', user?.can_manage_admin ? 'badge-soft badge-success' : 'badge-soft']">
                {{ user?.can_manage_admin ? $t('account.yes') : $t('account.no') }}
              </span>
            </dd>
          </div>
        </dl>
      </section>

      <aside class="rounded-box border border-base-content/10 bg-base-100 p-6 shadow-sm">
        <h2 class="mb-4 text-lg font-semibold">{{ $t('account.roles') }}</h2>
        <div class="flex flex-wrap gap-2">
          <span v-for="role in user?.roles || []" :key="role" class="badge badge-soft badge-primary">
            {{ role }}
          </span>
          <span v-if="!user?.roles?.length" class="text-base-content/60 text-sm">{{ $t('account.noRolesLinked') }}</span>
        </div>

        <NuxtLinkLocale v-if="user?.can_manage_admin" to="/admin" class="btn btn-outline btn-block mt-6">
          <span class="icon-[tabler--shield] size-5"></span>
          {{ $t('account.openAdmin') }}
        </NuxtLinkLocale>
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'account',
  middleware: 'auth'
})

const { t } = useI18n()

useSeoMeta({
  title: t('account.settingsTitle')
})

const { user, fetchCurrentUser, logout } = useAuth()

if (!user.value) {
  await fetchCurrentUser()
}
</script>
