<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.reviews.title') }}</h1>
      <NuxtLink to="/admin/reviews/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.reviews.add') }}
      </NuxtLink>
    </div>

    <!-- Filters & Search -->
    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <form @submit.prevent="handleSearch" class="flex flex-wrap gap-4 items-end">
          <div class="form-control flex-1 min-w-[240px]">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">Buscar Avaliação</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 group-focus-within:text-primary transition-colors"></span>
              <input 
                v-model="searchQuery" 
                type="text" 
                :placeholder="$t('admin.reviews.searchPlaceholder')" 
                class="input input-bordered w-full pl-10" 
              />
            </div>
          </div>
          <button type="submit" class="btn btn-ghost">
            Limpar
          </button>
        </form>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="pending" class="flex flex-col items-center justify-center py-20 bg-base-100 rounded-box border shadow-sm">
      <span class="loading loading-spinner text-primary size-12"></span>
      <p class="mt-4 text-gray-500">{{ $t('admin.reviews.loading') }}</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error mb-6">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.reviews.error', { message: error.message }) }}</span>
    </div>

    <!-- Reviews Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.reviews.table.product') }}</th>
              <th>{{ $t('admin.reviews.table.user') }}</th>
              <th>{{ $t('admin.reviews.table.rating') }}</th>
              <th>{{ $t('admin.reviews.table.title') }}</th>
              <th>{{ $t('admin.reviews.table.status') }}</th>
              <th>{{ $t('admin.reviews.table.date') }}</th>
              <th class="text-right">{{ $t('admin.reviews.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="review in filteredReviews" :key="review.id" class="row-hover">
              <td><span class="font-mono text-xs">ID: {{ review.product_id }}</span></td>
              <td><span class="font-mono text-xs">ID: {{ review.user_id }}</span></td>
              <td>
                <div class="flex items-center gap-1">
                  <span class="font-bold text-warning">{{ review.rating }}</span>
                  <i class="icon-[tabler--star-filled] text-warning size-4"></i>
                </div>
              </td>
              <td>
                <div class="font-medium">{{ review.title || '-' }}</div>
                <div v-if="review.verified_purchase" class="badge badge-soft badge-success text-[10px] h-4">
                  {{ $t('admin.reviews.status.verified') }}
                </div>
              </td>
              <td>
                <span :class="['badge badge-soft text-xs', review.active ? 'badge-success' : 'badge-error']">
                  {{ review.active ? $t('admin.reviews.status.active') : $t('admin.reviews.status.inactive') }}
                </span>
              </td>
              <td><div class="text-sm">{{ formatDate(review.created_at) }}</div></td>
              <td class="text-right">
                <div class="flex justify-end gap-1">
                  <NuxtLink
                    :to="`/admin/reviews/${review.id}`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.view')"
                  >
                    <i class="icon-[tabler--eye] size-5"></i>
                  </NuxtLink>
                  <NuxtLink
                    :to="`/admin/reviews/${review.id}/edit`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.edit')"
                  >
                    <i class="icon-[tabler--pencil] size-5"></i>
                  </NuxtLink>
                  <button
                    type="button"
                    class="btn btn-circle btn-text btn-sm text-error"
                    :aria-label="$t('common.delete')"
                    @click="confirmDelete(review)"
                  >
                    <i class="icon-[tabler--trash] size-5"></i>
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="filteredReviews.length === 0">
              <td colspan="7" class="text-center py-20 text-gray-500 italic">
                {{ $t('admin.reviews.notFound') }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
