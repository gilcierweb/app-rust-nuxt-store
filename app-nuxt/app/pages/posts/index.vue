<template>
  <div class="pb-20">
    <!-- Hero / Featured Section -->
    <section class="pt-10 mb-16">
      <div class="bg-base-200/50 rounded-[3rem] p-8 md:p-16 relative overflow-hidden group">
        <div class="absolute top-0 right-0 -mt-20 -mr-20 size-80 bg-primary/10 rounded-full blur-3xl group-hover:bg-primary/20 transition-all duration-700"></div>
        <div class="relative z-10 grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div class="space-y-6">
            <span class="badge badge-primary badge-soft px-4 py-1.5 rounded-lg">{{ t('pages.posts.featured') }}</span>
            <h1 class="h1 leading-tight gradient-text">{{ t('pages.posts.featuredTitle') }}</h1>
            <p class="text-xl text-base-content/60 leading-relaxed">
              {{ t('pages.posts.featuredDescription') }}
            </p>
            <div class="flex items-center gap-4 pt-4">
              <div class="size-12 rounded-2xl bg-white shadow-sm border border-base-200 flex items-center justify-center">
                <span class="icon-[tabler--user] size-6 text-primary"></span>
              </div>
              <div>
                <p class="font-bold">Gabriel Rocha</p>
                <p class="text-xs text-base-content/40">{{ t('pages.posts.authorRole') }} • {{ t('pages.posts.readTime', { n: 5 }) }}</p>
              </div>
            </div>
            <button class="btn btn-primary btn-lg">{{ t('pages.posts.readMore') }}</button>
          </div>
          <div class="hidden lg:block relative">
            <div class="aspect-video rounded-[2.5rem] overflow-hidden shadow-2xl border-4 border-white/50 bg-base-300">
               <div class="flex items-center justify-center h-full bg-gradient-to-br from-primary/20 to-secondary/20">
                 <span class="icon-[tabler--news] size-32 text-base-content/10"></span>
               </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Categories & Search -->
    <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-12">
      <div class="flex items-center gap-2 overflow-x-auto no-scrollbar pb-2 w-full md:w-auto">
        <button v-for="cat in blogCategories" :key="cat.key" 
          class="btn btn-soft btn-sm whitespace-nowrap"
          :class="cat.key === 'all' ? 'btn-primary' : ''">
          {{ t(cat.label) }}
        </button>
      </div>
      
      <div class="relative w-full md:w-80">
        <div class="join w-full">
          <input type="text" :placeholder="t('pages.posts.hero.cta')" 
            class="input bg-base-200/50 border-none rounded-l-2xl pl-12 grow join-item h-12" />
          <button class="btn btn-primary join-item">
            <span class="icon-[tabler--search] size-5"></span>
          </button>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      <div v-for="i in 3" :key="i" class="space-y-4">
        <div class="aspect-video bg-base-200 animate-pulse rounded-[2rem]"></div>
        <div class="h-6 bg-base-200 animate-pulse rounded-lg w-3/4"></div>
        <div class="h-4 bg-base-200 animate-pulse rounded-lg w-full"></div>
      </div>
    </div>

    <!-- Posts Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      <article v-for="post in posts" :key="post.id" 
        class="card bg-base-100 shadow-soft border border-base-200 hover-lift group">
        
        <div class="relative h-56 bg-base-200">
          <NuxtImg v-if="post.image" :src="post.image" class="size-full object-cover group-hover:scale-110 transition-transform duration-700" :alt="post.title" />
          <div v-else class="flex items-center justify-center h-full bg-gradient-to-br from-base-200 to-base-300">
            <span class="icon-[tabler--article] size-16 text-base-content/10 group-hover:scale-110 transition-transform duration-700"></span>
          </div>
          <div class="absolute top-4 left-4">
            <span class="badge badge-primary backdrop-blur-md font-bold px-3 py-2 rounded-lg flex items-center gap-1">
              <span class="icon-[tabler--tag] size-3"></span>
              {{ t('pages.posts.categories.technology') }}
            </span>
          </div>
        </div>
        
        <div class="card-body p-8">
          <div class="flex items-center gap-3 text-xs text-base-content/40 mb-4">
            <span class="flex items-center gap-1"><span class="icon-[tabler--calendar] size-3.5"></span> {{ new Date('2025-07-15').toLocaleDateString(locale) }}</span>
            <span class="flex items-center gap-1"><span class="icon-[tabler--clock] size-3.5"></span> {{ t('pages.posts.readTime', { n: 5 }) }}</span>
          </div>
          
          <h3 class="h4 mb-4 group-hover:text-primary transition-colors line-clamp-2 leading-tight">{{ post.title }}</h3>
          <p class="text-sm text-base-content/50 mb-8 line-clamp-3 leading-relaxed">
            {{ $truncate(post.content, 120, '...') }}
          </p>
          
          <div class="flex items-center justify-between pt-6 border-t border-base-200">
            <div class="flex items-center gap-2">
              <div class="size-8 rounded-full bg-primary/10 flex items-center justify-center text-primary">
                <span class="icon-[tabler--user] size-4"></span>
              </div>
              <span class="text-xs font-bold">Admin</span>
            </div>
            <NuxtLinkLocale :to="`/posts/${post.id}`" class="btn btn-ghost btn-sm gap-1">
              {{ t('pages.posts.readMore') }}
              <span class="icon-[tabler--arrow-right] size-4"></span>
            </NuxtLinkLocale>
          </div>
        </div>
      </article>
    </div>

    <!-- Empty State -->
    <div v-if="!pending && (!posts || posts.length === 0)" class="text-center py-20">
      <div class="alert alert-warning max-w-md mx-auto">
        <div class="flex items-center gap-4">
          <div class="size-16 rounded-full bg-warning/20 flex items-center justify-center shrink-0">
            <span class="icon-[tabler--news-off] size-8 text-warning"></span>
          </div>
          <div>
            <h3 class="font-bold text-lg">{{ t('pages.posts.detail.sidebar.newsletterButton') }}</h3>
            <p class="text-sm opacity-80 mt-1">{{ t('pages.posts.detail.sidebar.newsletterText') }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Load More -->
    <div v-if="!pending && posts && posts.length > 0" class="text-center mt-16">
      <button class="btn btn-outline btn-lg">
        <span class="flex items-center gap-2">
          {{ t('pages.posts.loadMore') }}
          <span class="icon-[tabler--arrow-down] size-5"></span>
        </span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Post } from '~/types';
const { t, locale } = useI18n()
const { $truncate } = useNuxtApp();

useSeoMeta({
  title: t('pages.posts.title'),
  ogTitle: t('pages.posts.title'),
  description: t('pages.posts.description'),
  ogDescription: t('pages.posts.description'),
})

const { useApiLazyFetch } = useApi()
const { pending, data: posts } = useApiLazyFetch<Post[]>('/api/posts')

const blogCategories = [
  { key: 'all', label: 'pages.posts.categories.all' },
  { key: 'technology', label: 'pages.posts.categories.technology' },
  { key: 'marketing', label: 'pages.posts.categories.marketing' },
  { key: 'innovation', label: 'pages.posts.categories.innovation' },
  { key: 'security', label: 'pages.posts.categories.security' },
  { key: 'lifestyle', label: 'pages.posts.categories.lifestyle' }
]
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
