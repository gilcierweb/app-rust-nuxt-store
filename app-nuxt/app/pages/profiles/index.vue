<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12 pt-10">
      <div>
        <h1 class="h2 gradient-text mb-2">{{ t('pages.profiles.title') }}</h1>
        <p class="text-base-content/60">{{ t('pages.profiles.description') }}</p>
      </div>
      
      <div class="max-w-xl mx-auto w-full md:w-80">
        <div class="flex items-center gap-2 w-full">
          <input type="text" :placeholder="t('pages.profiles.search')" class="input grow" id="profileSearch" />
          <button class="btn btn-primary">
            <span class="icon-[tabler--search] size-5"></span>
          </button>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-24">
      <div class="alert alert-info max-w-md">
        <div class="flex items-center gap-4">
          <div class="loading loading-spinner loading-md"></div>
          <div>
            <p class="font-bold">Loading profiles</p>
            <p class="text-sm opacity-80">Please wait while we fetch user profiles...</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Profiles Grid -->
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
      <div v-for="profile in profiles" :key="profile.id" 
        class="card bg-base-100 shadow-soft border border-base-200 hover-lift group">
        
        <!-- Avatar/Cover Area -->
        <div class="relative h-32 bg-gradient-to-br from-primary/10 to-secondary/10">
          <div class="absolute -bottom-10 left-1/2 -translate-x-1/2 size-24 rounded-[1.5rem] border-4 border-base-100 overflow-hidden shadow-md bg-white">
            <NuxtImg :src="profile?.avatar || '/placeholder-avatar.png'" class="size-full object-cover group-hover:scale-110 transition-transform duration-700" :alt="profile?.nickname" />
          </div>
        </div>
        
        <!-- Content -->
        <div class="card-body pt-14 pb-8 px-6 text-center">
          <h3 class="font-black text-xl mb-1 group-hover:text-primary transition-colors">{{ profile?.nickname }}</h3>
          <p class="text-sm text-base-content/50 mb-6 line-clamp-1 px-4">{{ profile?.full_name }}</p>
          
          <div class="flex items-center justify-center gap-3 mb-8">
            <div class="badge badge-soft badge-primary rounded-lg px-3 py-2">
              <span class="flex items-center gap-1">
                <span class="icon-[tabler--compass] size-3"></span>
                {{ t('pages.profiles.explorer') }}
              </span>
            </div>
            <div class="badge badge-soft badge-secondary rounded-lg px-3 py-2">
              <span class="flex items-center gap-1">
                <span class="icon-[tabler--user-check] size-3"></span>
                {{ t('pages.profiles.member') }}
              </span>
            </div>
          </div>
          
          <NuxtLink :to="`/profiles/${profile.id}`" class="btn btn-primary w-full">
            {{ t('pages.profiles.readMore') }}
            <span class="icon-[tabler--arrow-right] size-4 ml-1"></span>
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="!pending && (!profiles || profiles.length === 0)" class="text-center py-20">
      <div class="alert alert-warning max-w-md mx-auto">
        <div class="flex items-center gap-4">
          <div class="size-16 rounded-full bg-warning/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--users-group] size-8 text-warning"></span>
          </div>
          <div>
            <h3 class="font-bold text-lg">{{ t('categories.notFound') }}</h3>
            <p class="text-sm opacity-80 mt-1">{{ t('pages.profiles.description') }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Profile } from '~/types';
const { t } = useI18n()
const config = useRuntimeConfig();

useSeoMeta({
  title: t('pages.profiles.title'),
  ogTitle: t('pages.profiles.title'),
  description: t('pages.profiles.description'),
  ogDescription: t('pages.profiles.description'),
})

const { pending, data: profiles } = await useLazyFetch<Profile[]>(`${config.public.baseURL}/api/profiles`)
</script>
