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
        <div class="flex items-center gap-3">
          <img src="https://cdn.flyonui.com/fy-assets/logo/logo.png" class="size-8" alt="brand-logo" />
          <h2 class="text-base-content text-xl font-bold">{{ t('common.appName') }}</h2>
        </div>
        <div>
          <h3 class="text-base-content mb-1.5 text-2xl font-semibold">{{ t('auth.register.title') }}</h3>
          <p class="text-base-content/80">{{ t('auth.register.title') }}</p>
        </div>

        <div v-if="error" class="alert alert-error">
          <span>{{ error }}</span>
        </div>
        <div v-if="success" class="alert alert-success">
          <span>{{ success }}</span>
        </div>

        <div class="space-y-4">
          <form class="mb-4 space-y-4" @submit.prevent="handleRegister">
            <div>
              <label class="label-text" for="userName">{{ t('contact.form.name') }}*</label>
              <input v-model="name" type="text" :placeholder="t('contact.form.name')" class="input" id="userName" required />
            </div>
            <div>
              <label class="label-text" for="userEmail">{{ t('auth.register.email') }}*</label>
              <input v-model="email" type="email" :placeholder="t('auth.register.emailPlaceholder')" class="input" id="userEmail" required />
            </div>
            <div>
              <label class="label-text" for="userPassword">{{ t('auth.register.password') }}*</label>
              <div class="input">
                <input v-model="password" id="userPassword" type="password" :placeholder="t('auth.register.passwordPlaceholder')" required />
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
            <div>
              <label class="label-text" for="userConfrimPassword">{{ t('auth.register.confirmPassword') }}*</label>
              <div class="input">
                <input
                  v-model="confirmPassword"
                  id="userConfrimPassword"
                  type="password"
                  :placeholder="t('auth.register.confirmPasswordPlaceholder')"
                  required
                />
                <button
                  type="button"
                  data-toggle-password='{ "target": "#userConfrimPassword" }'
                  class="block cursor-pointer"
                  aria-label="userConfrimPassword"
                >
                  <span class="icon-[tabler--eye] password-active:block hidden size-5 shrink-0"></span>
                  <span class="icon-[tabler--eye-off] password-active:hidden block size-5 shrink-0"></span>
                </button>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <input type="checkbox" class="checkbox checkbox-primary" id="policyagreement" required />
              <label class="label-text text-base-content/80 p-0 text-base" for="policyagreement">
                {{ t('auth.register.terms.consent') }}
                <a href="/terms" class="link link-animated link-primary font-normal">{{ t('auth.register.terms.termsOfUse') }}</a>
                {{ ` ${t('auth.register.terms.and')} ` }}
                <a href="/privacy" class="link link-animated link-primary font-normal">{{ t('auth.register.terms.privacyPolicy') }}</a>
              </label>
            </div>
            <button type="submit" class="btn btn-lg btn-primary btn-gradient btn-block" :disabled="loading">
              <span v-if="loading" class="loading loading-spinner"></span>
              <span v-else>{{ t('auth.register.submit') }}</span>
            </button>
          </form>
          <p class="text-base-content/80 mb-4 text-center">
            {{ t('auth.register.hasAccount') }}
            <NuxtLink to="/users/sessions" class="link link-animated link-primary font-normal">{{ t('auth.register.loginLink') }}</NuxtLink>
          </p>
        </div>
      </div>
    </div>
  </div>
  </div>
</template>

<script lang="ts" setup>
const { t } = useI18n()
const { register, loading, error } = useAuth()

const name = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const success = ref('')

async function handleRegister() {
  success.value = ''
  if (password.value !== confirmPassword.value) {
    error.value = t('auth.register.errors.passwordMismatch')
    return
  }
  try {
    await register(name.value, email.value, password.value)
    success.value = t('auth.register.success.message', { email: email.value })
    name.value = ''
    email.value = ''
    password.value = ''
    confirmPassword.value = ''
  } catch {
    // error is set by useAuth
  }
}
</script>

<style></style>
