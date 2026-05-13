<template>
  <div>
    <div class="bg-base-100 flex h-auto min-h-screen items-center justify-center overflow-x-hidden py-10">
    <div class="relative flex items-center justify-center px-4 sm:px-6 lg:px-8">
      <div class="absolute">
        <svg width="200" height="200" viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="100" cy="100" r="80" stroke="var(--color-primary)" stroke-opacity="0.2" stroke-dasharray="8 8" fill="none" />
          <circle cx="100" cy="100" r="50" stroke="var(--color-primary)" stroke-opacity="0.1" fill="none" />
        </svg>
      </div>
      <div class="bg-base-100 shadow-base-300/20 z-1 w-full space-y-6 rounded-xl p-6 shadow-md sm:max-w-md lg:p-8">
        <div class="flex items-center gap-3">
          <img src="https://cdn.flyonui.com/fy-assets/logo/logo.png" class="size-8" alt="brand-logo" />
          <h2 class="text-base-content text-xl font-bold">{{ t('common.appName') }}</h2>
        </div>
        <div>
          <h3 class="text-base-content mb-1.5 text-2xl font-semibold">{{ t('auth.forgotPassword.title') }}</h3>
          <p class="text-base-content/80">{{ t('auth.forgotPassword.description') }}</p>
        </div>

        <div v-if="error" class="alert alert-error">
          <span>{{ error }}</span>
        </div>
        <div v-if="success" class="alert alert-success">
          <span>{{ success }}</span>
        </div>

        <form class="mb-4 space-y-4" @submit.prevent="handleForgot">
          <div>
            <label class="label-text" for="userEmail">{{ t('auth.forgotPassword.email') }}*</label>
            <input v-model="email" type="email" :placeholder="t('auth.forgotPassword.emailPlaceholder')" class="input" id="userEmail" required />
          </div>
          <button type="submit" class="btn btn-lg btn-primary btn-gradient btn-block" :disabled="loading">
            <span v-if="loading" class="loading loading-spinner"></span>
            <span v-else>{{ t('auth.forgotPassword.submit') }}</span>
          </button>
        </form>
        <div class="group flex items-center justify-center gap-2">
          <span
            class="icon-[tabler--chevron-left] text-primary size-5 shrink-0 transition-transform group-hover:-translate-x-1 rtl:rotate-180"
          ></span>
          <NuxtLinkLocale to="/users/sessions" class="link link-animated link-primary font-normal">{{ t('auth.forgotPassword.backToLogin') }}</NuxtLinkLocale>
        </div>
      </div>
    </div>
  </div>
  </div>
</template>

<script lang="ts" setup>
const { t } = useI18n()
const { forgotPassword, loading, error } = useAuth()

const email = ref('')
const success = ref('')

async function handleForgot() {
  success.value = ''
  try {
    await forgotPassword(email.value)
    success.value = t('auth.forgotPassword.success.message')
  } catch {
    // error is set by useAuth
  }
}
</script>

<style></style>
