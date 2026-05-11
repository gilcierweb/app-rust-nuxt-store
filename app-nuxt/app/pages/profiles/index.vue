<template>
  <div class="pb-20">
    <!-- Header -->
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12 pt-10">
      <div>
        <h1 class="h2 gradient-text mb-2">{{ t('pages.profiles.title') }}</h1>
        <p class="text-base-content/60">{{ t('pages.profiles.description') }}</p>
      </div>
      
      <div class="max-w-xl mx-auto w-full md:w-80">
        <div class="relative">
          <span class="icon-[tabler--search] size-5 absolute left-4 top-1/2 -translate-y-1/2 text-base-content/40"></span>
          <input type="text" :placeholder="t('pages.profiles.search')" 
            class="input input-lg bg-base-200/50 border-none rounded-2xl pl-12 w-full h-14" />
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-24">
      <span class="loading loading-spinner text-primary size-16 mb-4"></span>
      <p class="text-base-content/40 font-medium tracking-wider uppercase text-xs">{{ t('pages.profiles.loading') }}</p>
    </div>

    <!-- Profiles Grid -->
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
      <div v-for="profile in profiles" :key="profile.id" 
        class="group bg-base-100 rounded-[2.5rem] border border-base-200 overflow-hidden hover-lift shadow-sm hover:shadow-xl hover:shadow-primary/5 transition-all duration-500">
        
        <!-- Avatar/Cover Area -->
        <div class="relative h-32 bg-gradient-to-br from-primary/10 to-secondary/10">
          <div class="absolute -bottom-10 left-1/2 -translate-x-1/2 size-24 rounded-[1.5rem] border-4 border-base-100 overflow-hidden shadow-md bg-white">
            <NuxtImg :src="profile?.avatar || '/placeholder-avatar.png'" class="size-full object-cover group-hover:scale-110 transition-transform duration-700" :alt="profile?.nickname" />
          </div>
        </div>
        
        <!-- Content -->
        <div class="pt-14 pb-8 px-6 text-center">
          <h3 class="font-black text-xl mb-1 group-hover:text-primary transition-colors">{{ profile?.nickname }}</h3>
          <p class="text-sm text-base-content/50 mb-6 line-clamp-1 px-4">{{ profile?.full_name }}</p>
          
          <div class="flex items-center justify-center gap-3 mb-8">
            <div class="badge badge-soft badge-primary rounded-lg px-3 py-3">Explorer</div>
            <div class="badge badge-soft badge-secondary rounded-lg px-3 py-3">Member</div>
          </div>
          
          <NuxtLink :to="`/profiles/${profile.id}`" class="btn btn-primary btn-md w-full rounded-xl shadow-lg shadow-primary/10">
            {{ t('pages.profiles.readMore') }}
            <span class="icon-[tabler--arrow-right] size-4 ml-1"></span>
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="!pending && (!profiles || profiles.length === 0)" class="text-center py-20">
      <span class="icon-[tabler--users-group] size-16 text-base-content/20 mb-4"></span>
      <h3 class="text-xl font-bold">{{ t('categories.notFound') }}</h3>
      <p class="text-base-content/50 mt-2">{{ t('pages.profiles.description') }}</p>
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
