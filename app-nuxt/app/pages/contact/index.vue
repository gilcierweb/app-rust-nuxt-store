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
          <div class="bg-base-200/50 p-8 rounded-[2.5rem] border border-base-200">
            <h2 class="h3 mb-8">Get in Touch</h2>
            <div class="space-y-6">
              <div class="flex items-start gap-4">
                <div class="size-12 rounded-2xl bg-white shadow-sm flex items-center justify-center shrink-0">
                  <span class="icon-[tabler--mail] size-6 text-primary"></span>
                </div>
                <div>
                  <h4 class="font-bold">Email Us</h4>
                  <p class="text-base-content/60">support@apprustnuxt.store</p>
                  <p class="text-base-content/60">sales@apprustnuxt.store</p>
                </div>
              </div>
              <div class="flex items-start gap-4">
                <div class="size-12 rounded-2xl bg-white shadow-sm flex items-center justify-center shrink-0">
                  <span class="icon-[tabler--phone] size-6 text-primary"></span>
                </div>
                <div>
                  <h4 class="font-bold">Call Us</h4>
                  <p class="text-base-content/60">+55 (11) 9999-9999</p>
                  <p class="text-sm text-base-content/40 mt-1">Mon-Fri from 9am to 6pm</p>
                </div>
              </div>
              <div class="flex items-start gap-4">
                <div class="size-12 rounded-2xl bg-white shadow-sm flex items-center justify-center shrink-0">
                  <span class="icon-[tabler--map-pin] size-6 text-primary"></span>
                </div>
                <div>
                  <h4 class="font-bold">Our Office</h4>
                  <p class="text-base-content/60">Av. Paulista, 1000 - Bela Vista</p>
                  <p class="text-base-content/60">São Paulo - SP, 01310-100</p>
                </div>
              </div>
            </div>

            <!-- Social Links -->
            <div class="mt-12 pt-8 border-t border-base-200">
              <h4 class="font-bold mb-4">Follow Us</h4>
              <div class="flex gap-4">
                <a v-for="social in socials" :key="social.icon" href="#" 
                  class="btn btn-square btn-ghost hover:bg-primary/10 hover:text-primary transition-all rounded-xl">
                  <span :class="[social.icon, 'size-6']"></span>
                </a>
              </div>
            </div>
          </div>
          
          <!-- Small Map Placeholder -->
          <div class="h-64 rounded-[2.5rem] bg-base-300 overflow-hidden relative">
            <div class="absolute inset-0 bg-gradient-to-br from-primary/10 to-secondary/10"></div>
            <div class="flex items-center justify-center h-full flex-col gap-2 text-base-content/20">
              <span class="icon-[tabler--map-2] size-12"></span>
              <span class="font-medium text-sm">Interactive Map</span>
            </div>
          </div>
        </div>

        <!-- Contact Form -->
        <div class="lg:col-span-7">
          <div class="bg-base-100 p-8 md:p-12 rounded-[2.5rem] shadow-2xl shadow-primary/5 border border-base-200">
            <h2 class="h3 mb-8">Send us a message</h2>
            <form @submit.prevent class="space-y-6">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="form-control">
                  <label class="form-label">Full Name</label>
                  <input type="text" placeholder="John Doe" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" />
                </div>
                <div class="form-control">
                  <label class="form-label">Email Address</label>
                  <input type="email" placeholder="john@example.com" class="input input-lg bg-base-200/50 border-none rounded-2xl h-14" />
                </div>
              </div>
              
              <div class="form-control">
                <label class="form-label">Subject</label>
                <select class="select select-lg bg-base-200/50 border-none rounded-2xl h-14 w-full">
                  <option disabled selected>How can we help you?</option>
                  <option>General Inquiry</option>
                  <option>Technical Support</option>
                  <option>Sales & Partnerships</option>
                  <option>Other</option>
                </select>
              </div>

              <div class="form-control">
                <label class="form-label">Message</label>
                <textarea class="textarea textarea-lg bg-base-200/50 border-none rounded-2xl min-h-40" placeholder="Tell us more about your needs..."></textarea>
              </div>

              <button type="submit" class="btn btn-primary btn-lg w-full h-14 rounded-2xl shadow-xl shadow-primary/20 transition-transform hover:scale-[1.02]">
                <span class="icon-[tabler--send] size-5 mr-2"></span>
                Send Message
              </button>
              
              <p class="text-center text-xs text-base-content/40 mt-6">
                We usually respond within 24 business hours.
              </p>
            </form>
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