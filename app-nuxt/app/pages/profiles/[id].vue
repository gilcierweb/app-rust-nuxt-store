<template>
  <div class="pb-20">
    <div v-if="pending" class="flex flex-col items-center justify-center py-24">
      <div class="alert alert-info max-w-md">
        <div class="flex items-center gap-4">
          <div class="loading loading-spinner loading-md"></div>
          <div>
            <p class="font-bold">{{ t('pages.profiles.detail.loading') }}</p>
          </div>
        </div>
      </div>
    </div>
         
    <div v-else>
      <!-- Hero Header -->
      <section class="relative bg-gradient-to-br from-primary/80 to-secondary/80 rounded-b-[3rem] pt-20 pb-24 overflow-hidden mb-10">
        <!-- Background accents -->
        <div class="absolute inset-0 bg-black/10"></div>
        <div class="absolute top-10 left-10 size-20 bg-white/10 rounded-full animate-pulse"></div>
        <div class="absolute bottom-10 right-10 size-16 bg-white/10 rounded-full animate-pulse"></div>
        
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10 text-center text-primary-content">
          <div class="mb-6 relative inline-block">
            <div class="size-32 rounded-[2rem] border-4 border-white overflow-hidden shadow-2xl mx-auto bg-base-200">
              <NuxtImg :src="profile?.avatar || 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-4.0.3&auto=format&fit=crop&w=200&h=200&q=80'" 
                       :alt="t('pages.profiles.detail.photoAlt')" class="size-full object-cover" />
            </div>
            <button class="btn btn-circle btn-sm btn-primary absolute -bottom-2 -right-2 border-2 border-white shadow-lg">
              <span class="icon-[tabler--camera] size-4"></span>
            </button>
          </div>
          
          <h1 class="text-4xl font-black mb-2">{{ profile?.full_name || t('pages.profiles.detail.mocks.user') }}</h1>
          <p class="text-lg opacity-90 mb-8">{{ t('pages.profiles.detail.premiumSince') }}</p>
          
          <div class="flex flex-wrap justify-center gap-4">
            <div class="card bg-white/20 backdrop-blur-md text-primary-content shadow-none border-none">
              <div class="card-body p-4 text-center">
                <div class="text-2xl font-black">156</div>
                <div class="text-sm opacity-80 uppercase tracking-wider font-bold">{{ t('pages.profiles.detail.stats.orders') }}</div>
              </div>
            </div>
            <div class="card bg-white/20 backdrop-blur-md text-primary-content shadow-none border-none">
              <div class="card-body p-4 text-center">
                <div class="text-2xl font-black">{{ formatNumberBR(25680) }}</div>
                <div class="text-sm opacity-80 uppercase tracking-wider font-bold">{{ t('pages.profiles.detail.stats.totalSpent') }}</div>
              </div>
            </div>
            <div class="card bg-white/20 backdrop-blur-md text-primary-content shadow-none border-none">
              <div class="card-body p-4 text-center">
                <div class="text-2xl font-black">4.8</div>
                <div class="text-sm opacity-80 uppercase tracking-wider font-bold">{{ t('pages.profiles.detail.stats.rating') }}</div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Main Content -->
      <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Left Column -->
          <div class="lg:col-span-2 space-y-8">
            <!-- Personal Info -->
            <div class="card bg-base-100 shadow-soft border border-base-200">
              <div class="card-body">
                <div class="flex items-center justify-between mb-6">
                  <h2 class="card-title text-2xl">{{ t('pages.profiles.detail.personalInfo') }}</h2>
                  <button class="btn btn-primary btn-sm gap-2">
                    <span class="icon-[tabler--edit] size-4"></span>
                    {{ t('pages.profiles.detail.edit') }}
                  </button>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="w-full">
                    <label class="label-text" for="fullName">{{ t('pages.profiles.detail.fields.fullName') }}</label>
                    <input type="text" class="input w-full mt-1" id="fullName" :value="profile?.full_name || t('pages.profiles.detail.mocks.user')" readonly>
                  </div>
                  <div class="w-full">
                    <label class="label-text" for="emailField">{{ t('pages.profiles.detail.fields.email') }}</label>
                    <input type="email" class="input w-full mt-1" id="emailField" :value="profile?.email || t('pages.profiles.detail.mocks.email')" readonly>
                  </div>
                  <div class="w-full">
                    <label class="label-text" for="phoneField">{{ t('pages.profiles.detail.fields.phone') }}</label>
                    <input type="tel" class="input w-full mt-1" id="phoneField" :value="profile?.phone || t('pages.profiles.detail.mocks.phone')" readonly>
                  </div>
                  <div class="w-full">
                    <label class="label-text" for="birthDateField">{{ t('pages.profiles.detail.fields.birthDate') }}</label>
                    <input type="date" class="input w-full mt-1" id="birthDateField" :value="profile?.birth_date || t('pages.profiles.detail.mocks.date')" readonly>
                  </div>
                </div>
              </div>
            </div>

            <!-- Last Orders -->
            <div class="card bg-base-100 shadow-soft border border-base-200">
              <div class="card-body">
                <h2 class="card-title text-2xl mb-6">{{ t('pages.profiles.detail.lastOrders') }}</h2>
                
                <div class="space-y-4">
                  <div class="flex items-center justify-between p-4 bg-base-200/50 rounded-2xl">
                    <div class="flex items-center gap-4">
                      <div class="size-12 bg-primary/10 rounded-xl flex items-center justify-center shrink-0">
                        <span class="icon-[tabler--package] size-6 text-primary"></span>
                      </div>
                      <div>
                        <div class="font-bold">{{ t('pages.profiles.detail.mocks.order1') }}</div>
                        <div class="text-sm text-base-content/60">{{ t('pages.profiles.detail.mocks.date1') }}</div>
                      </div>
                    </div>
                    <div class="text-right">
                      <div class="font-bold mb-1">{{ t('pages.profiles.detail.mocks.price1') }}</div>
                      <div class="badge badge-success badge-sm">{{ t('pages.profiles.detail.orders.status.delivered') }}</div>
                    </div>
                  </div>
                  
                  <div class="flex items-center justify-between p-4 bg-base-200/50 rounded-2xl">
                    <div class="flex items-center gap-4">
                      <div class="size-12 bg-warning/10 rounded-xl flex items-center justify-center shrink-0">
                        <span class="icon-[tabler--truck] size-6 text-warning"></span>
                      </div>
                      <div>
                        <div class="font-bold">{{ t('pages.profiles.detail.mocks.order2') }}</div>
                        <div class="text-sm text-base-content/60">{{ t('pages.profiles.detail.mocks.date2') }}</div>
                      </div>
                    </div>
                    <div class="text-right">
                      <div class="font-bold mb-1">{{ t('pages.profiles.detail.mocks.price2') }}</div>
                      <div class="badge badge-warning badge-sm">{{ t('pages.profiles.detail.orders.status.inTransit') }}</div>
                    </div>
                  </div>
                </div>
                
                <div class="card-actions justify-center mt-6">
                  <button class="btn btn-outline btn-primary">{{ t('pages.profiles.detail.viewAllOrders') }}</button>
                </div>
              </div>
            </div>
          </div>

          <!-- Right Column -->
          <div class="space-y-8">
            <!-- Addresses -->
            <div class="card bg-base-100 shadow-soft border border-base-200">
              <div class="card-body">
                <h3 class="card-title text-xl mb-4">{{ t('pages.profiles.detail.savedAddresses') }}</h3>
                
                <div class="space-y-4">
                  <div class="p-4 bg-base-200/50 rounded-2xl border border-base-200">
                    <div class="flex items-center justify-between mb-2">
                      <div class="font-bold">{{ t('pages.profiles.detail.addresses.home') }}</div>
                      <div class="badge badge-primary badge-sm">{{ t('pages.profiles.detail.addresses.main') }}</div>
                    </div>
                    <div class="text-sm text-base-content/60 leading-relaxed">{{ t('pages.profiles.detail.mocks.addressHome') }}</div>
                  </div>
                  
                  <div class="p-4 bg-base-200/50 rounded-2xl border border-base-200">
                    <div class="font-bold mb-2">{{ t('pages.profiles.detail.addresses.work') }}</div>
                    <div class="text-sm text-base-content/60 leading-relaxed">{{ t('pages.profiles.detail.mocks.addressWork') }}</div>
                  </div>
                </div>
                
                <div class="card-actions mt-6">
                  <button class="btn btn-soft btn-primary w-full gap-2">
                    <span class="icon-[tabler--plus] size-4"></span>
                    {{ t('pages.profiles.detail.addresses.add') }}
                  </button>
                </div>
              </div>
            </div>

            <!-- Coupons -->
            <div class="card bg-base-100 shadow-soft border border-base-200">
              <div class="card-body">
                <h3 class="card-title text-xl mb-4">{{ t('pages.profiles.detail.coupons') }}</h3>
                
                <div class="space-y-4">
                  <div class="p-4 bg-gradient-to-r from-warning/20 to-error/20 border border-warning/30 rounded-2xl">
                    <div class="font-black text-lg mb-1 text-warning-content">{{ t('pages.profiles.detail.mocks.coupon1.code') }}</div>
                    <div class="text-sm text-base-content/80 font-medium">{{ t('pages.profiles.detail.mocks.coupon1.desc') }}</div>
                    <div class="text-xs text-base-content/50 mt-2">{{ t('pages.profiles.detail.mocks.coupon1.validity') }}</div>
                  </div>
                  
                  <div class="p-4 bg-gradient-to-r from-success/20 to-info/20 border border-success/30 rounded-2xl">
                    <div class="font-black text-lg mb-1 text-success-content">{{ t('pages.profiles.detail.mocks.coupon2.code') }}</div>
                    <div class="text-sm text-base-content/80 font-medium">{{ t('pages.profiles.detail.mocks.coupon2.desc') }}</div>
                    <div class="text-xs text-base-content/50 mt-2">{{ t('pages.profiles.detail.mocks.coupon2.validity') }}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Profile } from '~/types';
const { t } = useI18n()

const route = useRoute();
const { useApiLazyFetch } = useApi()
const id = route.params.id;

useSeoMeta({
  title: t('pages.profiles.detail.title', { name: 'Profile' }),
})

const { pending, data: profile } = useApiLazyFetch<Profile>(() => `/api/profiles/${id}`);
</script>
