<template>
  <div>
    <div class="mb-6 flex items-center justify-between gap-4">
      <div class="flex items-center gap-4">
        <NuxtLinkLocale to="/admin/banners" class="btn btn-ghost btn-circle">
          <i class="icon-[tabler--arrow-left] size-6"></i>
        </NuxtLinkLocale>
        <div>
          <h1 class="h1">{{ t('admin.banners.detail.title') }}</h1>
          <p v-if="banner" class="text-sm text-base-content/60">ID: {{ banner.id }}</p>
        </div>
      </div>

      <div v-if="banner" class="flex gap-2">
        <button class="btn btn-error btn-outline" @click="deleteBanner">
          <i class="icon-[tabler--trash] size-5 mr-2"></i>
          {{ t('common.delete') }}
        </button>
        <NuxtLinkLocale :to="`/admin/banners/${route.params.id}/edit`" class="btn btn-primary">
          <i class="icon-[tabler--pencil] size-5 mr-2"></i>
          {{ t('common.edit') }}
        </NuxtLinkLocale>
      </div>
    </div>

    <div v-if="pending" class="flex flex-col items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="mt-4 text-base-content/60">{{ t('admin.banners.detail.loading') }}</span>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ t('admin.banners.error', { message: error.message }) }}</span>
      <button class="btn btn-sm btn-ghost" @click="() => refresh()">{{ t('common.actions.tryAgain') }}</button>
    </div>

    <div v-else-if="banner" class="grid grid-cols-1 xl:grid-cols-3 gap-6">
      <div class="card bg-base-100 shadow-sm xl:col-span-2">
        <figure class="bg-base-200 max-h-96">
          <img :src="banner.image_desktop_url" :alt="banner.alt_text || banner.title" class="w-full object-cover" />
        </figure>
        <div class="card-body">
          <div class="flex flex-wrap items-center gap-2">
            <span :class="['badge badge-soft', statusClass(banner.status)]">{{ statusLabel(banner.status) }}</span>
            <span class="badge badge-soft">{{ deviceLabel(banner.device) }}</span>
            <span class="badge badge-soft">{{ positionName(banner.position_id) }}</span>
          </div>

          <h2 class="card-title text-2xl mt-3">{{ banner.title }}</h2>
          <p v-if="banner.description" class="text-base-content/70">{{ banner.description }}</p>

          <div class="divider"></div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <div class="text-sm text-base-content/50">{{ t('admin.banners.form.campaignName') }}</div>
              <div class="font-medium">{{ banner.campaign_name || '-' }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/50">{{ t('admin.banners.form.priority') }}</div>
              <div class="font-medium">{{ banner.priority }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/50">{{ t('admin.banners.form.startAt') }}</div>
              <div class="font-medium">{{ formatDate(banner.start_at) }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/50">{{ t('admin.banners.form.endAt') }}</div>
              <div class="font-medium">{{ banner.end_at ? formatDate(banner.end_at) : t('admin.banners.detail.noEnd') }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/50">{{ t('admin.banners.form.linkTarget') }}</div>
              <div class="font-medium">{{ linkTargetLabel(banner.link_target) }}</div>
            </div>
            <div>
              <div class="text-sm text-base-content/50">{{ t('admin.banners.form.linkUrl') }}</div>
              <a v-if="banner.link_url" :href="banner.link_url" target="_blank" rel="noopener noreferrer" class="link link-primary break-all">
                {{ banner.link_url }}
              </a>
              <div v-else class="font-medium">-</div>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-6">
        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <h2 class="card-title">{{ t('admin.banners.detail.assets') }}</h2>
            <div class="space-y-4">
              <div>
                <div class="text-sm text-base-content/50">{{ t('admin.banners.form.imageDesktopUrl') }}</div>
                <div class="font-mono text-xs break-all">{{ banner.image_desktop_url }}</div>
              </div>
              <div>
                <div class="text-sm text-base-content/50">{{ t('admin.banners.form.imageMobileUrl') }}</div>
                <div class="font-mono text-xs break-all">{{ banner.image_mobile_url || '-' }}</div>
              </div>
              <div>
                <div class="text-sm text-base-content/50">{{ t('admin.banners.form.altText') }}</div>
                <div>{{ banner.alt_text || '-' }}</div>
              </div>
            </div>
          </div>
        </div>

        <div class="card bg-base-100 shadow-sm">
          <div class="card-body">
            <h2 class="card-title">{{ t('admin.banners.detail.audit') }}</h2>
            <div class="text-sm space-y-2">
              <div class="flex justify-between gap-4">
                <span class="text-base-content/50">{{ t('admin.banners.detail.createdAt') }}</span>
                <span class="text-right">{{ formatDate(banner.created_at) }}</span>
              </div>
              <div class="flex justify-between gap-4">
                <span class="text-base-content/50">{{ t('admin.banners.detail.updatedAt') }}</span>
                <span class="text-right">{{ formatDate(banner.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="alert alert-warning">
      <i class="icon-[tabler--alert-triangle] size-6"></i>
      <span>{{ t('admin.banners.notFound') }}</span>
      <NuxtLinkLocale to="/admin/banners" class="btn btn-sm">{{ t('common.back') }}</NuxtLinkLocale>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Banner, BannerPosition } from '~/types'

definePageMeta({
  layout: 'admin'
})

const route = useRoute()
const router = useRouter()
const { t } = useI18n()
const { apiFetch, useApiFetch } = useApi()

const [
  { pending, data: banner, error, refresh },
  { data: positions }
] = await Promise.all([
  useApiFetch<Banner>(`/api/admin/banners/${route.params.id}`, {
    key: `admin-banner-detail-${route.params.id}`
  }),
  useApiFetch<BannerPosition[]>('/api/admin/banners/positions', {
    key: 'admin-banner-positions-detail'
  })
])

function positionName(id: number) {
  return positions.value?.find(position => position.id === id)?.name || `#${id}`
}

function statusLabel(status: number) {
  const labels: Record<number, string> = {
    1: t('admin.banners.status.draft'),
    2: t('admin.banners.status.active'),
    3: t('admin.banners.status.paused'),
    4: t('admin.banners.status.expired')
  }
  return labels[status] || t('admin.statusLabels.unknown')
}

function statusClass(status: number) {
  const classes: Record<number, string> = {
    1: 'badge-neutral',
    2: 'badge-success',
    3: 'badge-warning',
    4: 'badge-error'
  }
  return classes[status] || 'badge-neutral'
}

function deviceLabel(device: number) {
  const labels: Record<number, string> = {
    1: t('admin.banners.devices.all'),
    2: t('admin.banners.devices.desktop'),
    3: t('admin.banners.devices.mobile')
  }
  return labels[device] || t('admin.statusLabels.unknown')
}

function linkTargetLabel(target: number) {
  return target === 2 ? t('admin.banners.linkTargets.newTab') : t('admin.banners.linkTargets.sameTab')
}

function formatDate(dateString?: string) {
  if (!dateString) return '-'
  return new Intl.DateTimeFormat('pt-BR', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(dateString))
}

async function deleteBanner() {
  if (!banner.value) return
  if (!confirm(t('admin.banners.detail.confirmDelete', { name: banner.value.title }))) return

  try {
    await apiFetch(`/api/admin/banners/${banner.value.id}`, { method: 'DELETE' })
    router.push('/admin/banners')
  } catch (err) {
    alert(t('admin.banners.detail.errorDelete'))
    console.error(err)
  }
}
</script>
