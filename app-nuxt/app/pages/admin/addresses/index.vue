<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.addresses.title') }}</h1>
      <NuxtLink to="/admin/addresses/new" class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.addresses.add') }}
      </NuxtLink>
    </div>

    <!-- Filters & Search -->
    <div class="card bg-base-100 shadow-sm border mb-6">
      <div class="card-body p-4">
        <form @submit.prevent="handleSearch" class="flex flex-wrap gap-4 items-end">
          <div class="form-control flex-1 min-w-[240px]">
            <label class="label pt-0">
              <span class="label-text-alt text-gray-500">Buscar Endereço</span>
            </label>
            <div class="relative group">
              <span class="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 group-focus-within:text-primary transition-colors"></span>
              <input 
                v-model="searchQuery" 
                type="text" 
                :placeholder="$t('admin.addresses.searchPlaceholder')" 
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
      <p class="mt-4 text-gray-500">{{ $t('admin.addresses.loading') }}</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="alert alert-error mb-6">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.addresses.error', { message: error.message }) }}</span>
    </div>

    <!-- Addresses Table -->
    <div v-else class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.addresses.table.name') }}</th>
              <th>{{ $t('admin.addresses.table.type') }}</th>
              <th>{{ $t('admin.addresses.table.address') }}</th>
              <th>{{ $t('admin.addresses.table.cityState') }}</th>
              <th>{{ $t('admin.addresses.table.default') }}</th>
              <th class="text-right">{{ $t('common.actions.title') || $t('admin.addresses.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="address in filteredAddresses" :key="address.id" class="row-hover">
              <td class="font-medium text-primary">{{ address.first_name }} {{ address.last_name }}</td>
              <td>
                <span class="badge badge-soft badge-sm">
                  {{ typeLabel(address.type) }}
                </span>
              </td>
              <td>
                <div class="text-sm">
                  <div>{{ address.address1 }}</div>
                  <div v-if="address.address2" class="text-gray-400 text-xs">{{ address.address2 }}</div>
                </div>
              </td>
              <td>
                <div class="font-medium">{{ address.city }}/{{ address.state }}</div>
                <div class="text-xs text-gray-400 font-mono">{{ address.zip_code }}</div>
              </td>
              <td>
                <span v-if="address.default" class="badge badge-soft badge-primary text-[10px]">
                  <i class="icon-[tabler--check] size-3 mr-1"></i>
                  {{ $t('admin.addresses.status.default') }}
                </span>
                <span v-else class="text-gray-300">-</span>
              </td>
              <td class="text-right">
                <div class="flex justify-end gap-1">
                  <NuxtLink
                    :to="`/admin/addresses/${address.id}`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.view')"
                  >
                    <i class="icon-[tabler--eye] size-5"></i>
                  </NuxtLink>
                  <NuxtLink
                    :to="`/admin/addresses/${address.id}/edit`"
                    class="btn btn-circle btn-text btn-sm"
                    :aria-label="$t('common.edit')"
                  >
                    <i class="icon-[tabler--pencil] size-5"></i>
                  </NuxtLink>
                  <button
                    type="button"
                    class="btn btn-circle btn-text btn-sm text-error"
                    :aria-label="$t('common.delete')"
                    @click="confirmDelete(address)"
                  >
                    <i class="icon-[tabler--trash] size-5"></i>
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="filteredAddresses.length === 0">
              <td colspan="6" class="text-center py-20 text-gray-500 italic">
                {{ $t('admin.addresses.notFound') }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
