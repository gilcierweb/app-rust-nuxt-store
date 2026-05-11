<template>
  <div>
    <div class="mb-6">
      <h1 class="h1">{{ $t('admin.categories.title') }}</h1>
    </div>

    <div class="mb-6 justify-between flex items-center">
      <form @submit.prevent="handleSearch">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="$t('admin.categories.searchPlaceholder')"
            class="input input-bordered w-full mb-4"
          />
          <button type="submit" class="btn btn-primary">{{ $t('common.search') }}</button>
        </div>
      </form>

      <NuxtLink to="/admin/categories/new" class="btn btn-success">{{ $t('admin.categories.add') }}</NuxtLink>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.categories.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>{{ $t('admin.categories.error', { message: error.message }) }}</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredCategories.length === 0" class="text-center py-12">
      <p class="text-gray-500 text-lg">{{ $t('admin.categories.notFound') }}</p>
      <NuxtLink to="/admin/categories/new" class="btn btn-primary mt-4">{{ $t('admin.categories.createFirst') }}</NuxtLink>
    </div>

    <!-- Categories Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.categories.table.name') }}</th>
              <th>{{ $t('admin.categories.table.slug') }}</th>
              <th>{{ $t('admin.categories.table.description') }}</th>
              <th>{{ $t('admin.categories.table.status') }}</th>
              <th>{{ $t('admin.categories.table.position') }}</th>
              <th>{{ $t('admin.categories.table.date') }}</th>
              <th>{{ $t('admin.categories.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="category in filteredCategories" :key="category.id" class="row-hover">
              <td class="font-medium">{{ category.name }}</td>
              <td class="font-mono text-sm text-gray-500">{{ category.slug }}</td>
              <td>{{ $truncate(category.description || '', 50, '...') }}</td>
              <td>
                <span :class="['badge badge-soft text-xs', category.active ? 'badge-success' : 'badge-error']">
                  {{ category.active ? $t('admin.categories.detail.active') : $t('admin.categories.detail.inactive') }}
                </span>
              </td>
              <td>{{ category.position ?? '-' }}</td>
              <td>{{ formatDate(category.created_at) }}</td>
              <td>
                <NuxtLink
                  :to="`/admin/categories/${category.id}`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.view')"
                >
                  <i class="icon-[tabler--eye] size-5"></i>
                </NuxtLink>
                <NuxtLink
                  :to="`/admin/categories/${category.id}/edit`"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.edit')"
                >
                  <i class="icon-[tabler--pencil] size-5"></i>
                </NuxtLink>
                <button
                  type="button"
                  class="btn btn-circle btn-text btn-sm"
                  :aria-label="$t('common.delete')"
                  @click="confirmDelete(category)"
                >
                  <span class="icon-[tabler--trash] size-5"></span>
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>