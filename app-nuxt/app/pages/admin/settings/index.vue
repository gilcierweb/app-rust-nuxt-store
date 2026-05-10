<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">{{ $t('admin.settings.title') }}</h1>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <!-- Sidebar Tabs -->
      <div class="col-span-1">
        <div class="card bg-white shadow-sm border border-gray-100">
          <div class="card-body p-0">
            <ul class="menu w-full p-2">
              <li>
                <a @click="activeTab = 'general'" :class="{ 'active': activeTab === 'general' }">
                  <i class="icon-[tabler--settings] size-5"></i>
                  {{ $t('admin.settings.general') }}
                </a>
              </li>
              <li>
                <a @click="activeTab = 'seo'" :class="{ 'active': activeTab === 'seo' }">
                  <i class="icon-[tabler--world] size-5"></i>
                  {{ $t('admin.settings.seo') }}
                </a>
              </li>
              <li>
                <a @click="activeTab = 'api'" :class="{ 'active': activeTab === 'api' }">
                  <i class="icon-[tabler--code] size-5"></i>
                  {{ $t('admin.settings.api') }}
                </a>
              </li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Tab Content -->
      <div class="col-span-2">
        <div class="card bg-white shadow-sm border border-gray-100">
          <div class="card-body">
            <div v-if="activeTab === 'general'">
              <h2 class="text-xl font-bold mb-4">{{ $t('admin.settings.general') }}</h2>
              <div class="space-y-4">
                <div class="form-control">
                  <label class="label">Store Name</label>
                  <input type="text" class="input input-bordered" value="App Rust Nuxt Store" />
                </div>
                <div class="form-control">
                  <label class="label">Support Email</label>
                  <input type="email" class="input input-bordered" value="support@example.com" />
                </div>
              </div>
            </div>

            <div v-if="activeTab === 'seo'">
              <h2 class="text-xl font-bold mb-4">{{ $t('admin.settings.seo') }}</h2>
              <div class="space-y-4">
                <div class="form-control">
                  <label class="label">Meta Title</label>
                  <input type="text" class="input input-bordered" placeholder="Site title for search engines" />
                </div>
                <div class="form-control">
                  <label class="label">Meta Description</label>
                  <textarea class="textarea textarea-bordered" placeholder="Site description"></textarea>
                </div>
              </div>
            </div>

            <div v-if="activeTab === 'api'">
              <h2 class="text-xl font-bold mb-4">{{ $t('admin.settings.api') }}</h2>
              <div class="space-y-4">
                <div class="form-control">
                  <label class="label">API Base URL</label>
                  <input type="text" class="input input-bordered" value="http://localhost:5150" readonly />
                </div>
              </div>
            </div>

            <div class="card-actions justify-end mt-6">
              <button class="btn btn-primary" @click="saveSettings">
                {{ $t('admin.settings.save') }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

const activeTab = ref('general')

const saveSettings = () => {
  alert('Settings saved (mock)')
}
</script>
