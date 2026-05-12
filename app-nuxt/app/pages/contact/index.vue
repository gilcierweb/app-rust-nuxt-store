<template>
  <div class="pb-20">
    <div class="max-w-6xl mx-auto">
      <!-- Header -->
      <div class="text-center mb-16 pt-10">
        <h1 class="h1 mb-4 gradient-text">{{ t('pages.contact.title') }}</h1>
        <p class="text-xl text-base-content/60">{{ t('pages.contact.description') }}</p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-12 gap-12">
        <!-- Contact Info -->
        <div class="lg:col-span-5 space-y-8">
          <div class="card bg-base-100 shadow-soft border border-base-200">
            <div class="card-body p-8">
              <h2 class="card-title text-2xl mb-8">{{ t('pages.contact.info.title') }}</h2>
              <div class="space-y-6">
                <div class="flex items-start gap-4">
                  <div class="size-12 rounded-2xl bg-base-200 shadow-sm flex items-center justify-center shrink-0">
                     <span class="icon-[tabler--mail] size-6 text-primary"></span>
                  </div>
                  <div>
                    <h4 class="font-bold">{{ t('admin.account.fields.email') }}</h4>
                    <p class="text-base-content/60">{{ t('pages.contact.info.email') }}</p>
                  </div>
                </div>
                <div class="flex items-start gap-4">
                  <div class="size-12 rounded-2xl bg-base-200 shadow-sm flex items-center justify-center shrink-0">
                    <span class="icon-[tabler--phone] size-6 text-primary"></span>
                  </div>
                  <div>
                    <h4 class="font-bold">{{ t('profiles.detail.fields.phone') }}</h4>
                    <p class="text-base-content/60">{{ t('pages.contact.info.phone') }}</p>
                    <p class="text-sm text-base-content/40 mt-1">{{ t('pages.contact.info.hours') }}</p>
                  </div>
                </div>
                <div class="flex items-start gap-4">
                  <div class="size-12 rounded-2xl bg-base-200 shadow-sm flex items-center justify-center shrink-0">
                    <span class="icon-[tabler--map-pin] size-6 text-primary"></span>
                  </div>
                  <div>
                    <h4 class="font-bold">{{ t('stores.details') }}</h4>
                    <p class="text-base-content/60">{{ t('pages.contact.info.address') }}</p>
                  </div>
                </div>
              </div>

              <!-- Social Links -->
              <div class="mt-12 pt-8 border-t border-base-200">
                <h4 class="font-bold mb-4">{{ t('pages.contact.followUs') }}</h4>
                <div class="flex gap-4">
                  <a v-for="social in socials" :key="social.icon" href="#" 
                    class="btn btn-square btn-ghost hover:bg-primary/10 hover:text-primary transition-all rounded-xl">
                    <span :class="[social.icon, 'size-6']"></span>
                  </a>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Small Map Placeholder -->
          <div class="card bg-base-200 shadow-soft h-64 overflow-hidden relative">
            <div class="absolute inset-0 bg-gradient-to-br from-primary/10 to-secondary/10"></div>
            <div class="flex items-center justify-center h-full flex-col gap-2 text-base-content/20 relative z-10">
              <span class="icon-[tabler--map-2] size-12"></span>
              <span class="font-medium text-sm">{{ t('pages.contact.mapPlaceholder') }}</span>
            </div>
          </div>
        </div>

        <!-- Contact Form -->
        <div class="lg:col-span-7">
          <div class="card bg-base-100 shadow-xl border border-base-200">
            <div class="card-body p-8 md:p-12">
              <h2 class="card-title text-2xl mb-8">{{ t('pages.contact.form.submit') }}</h2>
              <form @submit.prevent class="space-y-6">
                <div class="w-full">
                  <label class="label-text" for="contactName">{{ t('pages.contact.form.name') }}</label>
                  <input type="text" :placeholder="t('pages.contact.form.placeholders.name')" class="input" id="contactName" />
                </div>

                <div class="w-full">
                  <label class="label-text" for="contactEmail">{{ t('pages.contact.form.email') }}</label>
                  <input type="email" :placeholder="t('pages.contact.form.placeholders.email')" class="input" id="contactEmail" />
                </div>
                
                <div class="w-full">
                  <label class="label-text" for="contactSubject">{{ t('pages.contact.form.subject') }}</label>
                  <select class="select" id="contactSubject">
                    <option disabled selected>{{ t('pages.contact.form.placeholders.subject') }}</option>
                    <option>{{ t('pages.contact.form.placeholders.subjects.general') }}</option>
                    <option>{{ t('pages.contact.form.placeholders.subjects.support') }}</option>
                    <option>{{ t('pages.contact.form.placeholders.subjects.sales') }}</option>
                    <option>{{ t('pages.contact.form.placeholders.subjects.other') }}</option>
                  </select>
                </div>

                <div class="w-full">
                  <label class="label-text" for="contactMessage">{{ t('pages.contact.form.message') }}</label>
                  <textarea class="textarea" :placeholder="t('pages.contact.form.placeholders.message')" id="contactMessage"></textarea>
                </div>

                <div class="card-actions mt-4">
                  <button type="submit" class="btn btn-primary w-full">
                    <span class="icon-[tabler--send] size-5 mr-2"></span>
                    {{ t('pages.contact.form.submit') }}
                  </button>
                </div>
                
                <p class="text-center text-xs text-base-content/40 mt-6">
                  {{ t('pages.contact.form.success') }}
                </p>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.contact.title'),
  ogTitle: t('pages.contact.title'),
  description: t('pages.contact.description'),
  ogDescription: t('pages.contact.description'),
})

const socials = [
  { icon: 'icon-[tabler--brand-instagram]' },
  { icon: 'icon-[tabler--brand-twitter]' },
  { icon: 'icon-[tabler--brand-facebook]' },
  { icon: 'icon-[tabler--brand-linkedin]' }
]
</script>