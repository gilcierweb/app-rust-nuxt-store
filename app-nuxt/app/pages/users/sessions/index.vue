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
      <div class="bg-base-100 shadow-base-300/20 z-1 w-full space-y-6 rounded-xl p-6 shadow-md sm:min-w-md lg:p-8">
        <NuxtLinkLocale to="/">
          <div class="flex items-center gap-3">
            <img src="https://cdn.flyonui.com/fy-assets/logo/logo.png" class="size-8" alt="brand-logo" />
            <h2 class="text-base-content text-xl font-bold">{{ t('common.appName') }}</h2>
          </div>
        </NuxtLinkLocale>
        <div>
          <h3 class="text-base-content mb-1.5 text-2xl font-semibold">{{ t('auth.login.title') }}</h3>
          <p class="text-base-content/80">{{ t('auth.login.title') }}</p>
        </div>

        <div v-if="error" class="alert alert-error">
          <span>{{ error }}</span>
        </div>

        <div class="space-y-4">
          <form class="mb-4 space-y-4" @submit.prevent="handleLogin">
            <div>
              <label class="label-text" for="userEmail">{{ t('auth.login.email') }}*</label>
              <input v-model="email" type="email" :placeholder="t('auth.login.emailPlaceholder')" class="input" id="userEmail" required />
            </div>
            <div>
              <label class="label-text" for="userPassword">{{ t('auth.login.password') }}*</label>
              <div class="input">
                <input v-model="password" id="userPassword" type="password" :placeholder="t('auth.login.passwordPlaceholder')" required />
                <button
                  type="button"
                  data-toggle-password='{ "target": "#userPassword" }'
                  class="block cursor-pointer"
                  aria-label="userPassword"
                >
                  <span class="icon-[tabler--eye] password-active:block hidden size-5 shrink-0"></span>
                  <span class="icon-[tabler--eye-off] password-active:hidden block size-5 shrink-0"></span>
                </button>
              </div>
            </div>
            <div class="flex items-center justify-between gap-y-2">
              <div class="flex items-center gap-2">
                <input type="checkbox" class="checkbox checkbox-primary" id="rememberMe" />
                <label class="label-text text-base-content/80 p-0 text-base" for="rememberMe">{{ t('auth.login.rememberMe') }}</label>
              </div>
              <NuxtLinkLocale to="/users/passwords" class="link link-animated link-primary font-normal">{{ t('auth.login.forgotPassword') }}</NuxtLinkLocale>
            </div>
            <button type="submit" class="btn btn-lg btn-primary btn-gradient btn-block" :disabled="loading">
              <span v-if="loading" class="loading loading-spinner"></span>
              <span v-else>{{ t('auth.login.submit') }}</span>
            </button>
          </form>
          <p class="text-base-content/80 mb-4 text-center">
            {{ t('auth.login.noAccount') }}
            <NuxtLinkLocale to="/users/registrations" class="link link-animated link-primary font-normal">{{ t('auth.login.createAccount') }}</NuxtLinkLocale>
          </p>
        </div>
      </div>
    </div>
  </div>
  </div>
</template>

<script lang="ts" setup>
const { t } = useI18n()
const { login, loading, error } = useAuth()

const email = ref('')
const password = ref('')

async function handleLogin() {
  try {
    await login(email.value, password.value)
    await navigateTo('/')
  } catch {
    // error is set by useAuth
  }
}
</script>

<style></style>
