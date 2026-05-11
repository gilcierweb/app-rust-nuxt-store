<template>
  <div>
    <div class="mb-6 flex justify-between items-center">
      <h1 class="h1">{{ $t('admin.users.title') }}</h1>
      <button class="btn btn-primary">
        <i class="icon-[tabler--plus] size-5 mr-2"></i>
        {{ $t('admin.users.add') }}
      </button>
    </div>

    <!-- Users Table -->
    <div class="rounded-box shadow-base-300/10 bg-base-100 w-full pb-2 shadow-md overflow-hidden">
      <div class="overflow-x-auto">
        <table class="table">
          <thead>
            <tr>
              <th>{{ $t('admin.users.table.email') }}</th>
              <th>{{ $t('admin.users.table.role') }}</th>
              <th>{{ $t('admin.users.table.status') }}</th>
              <th>{{ $t('admin.users.table.date') }}</th>
              <th class="text-right">{{ $t('admin.users.table.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="user in mockUsers" :key="user.id" class="row-hover">
              <td>
                <div class="flex items-center gap-3">
                  <div class="avatar avatar-placeholder">
                    <div class="bg-neutral text-neutral-content rounded-full size-10">
                      <span class="text-lg">{{ user.email[0].toUpperCase() }}</span>
                    </div>
                  </div>
                  <span class="font-medium">{{ user.email }}</span>
                </div>
              </td>
              <td>
                <span class="badge badge-soft" :class="user.role === 'Admin' ? 'badge-primary' : 'badge-secondary'">
                  {{ user.role }}
                </span>
              </td>
              <td>
                <span class="badge badge-soft text-xs" :class="user.status === 'Active' ? 'badge-success' : 'badge-error'">
                  {{ user.status }}
                </span>
              </td>
              <td>{{ user.date }}</td>
              <td class="text-right">
                <button class="btn btn-circle btn-text btn-sm" :aria-label="$t('common.edit')">
                  <i class="icon-[tabler--pencil] size-5"></i>
                </button>
                <button class="btn btn-circle btn-text btn-sm text-error" :aria-label="$t('common.delete')">
                  <i class="icon-[tabler--trash] size-5"></i>
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

const mockUsers = [
  { id: 1, email: 'admin@example.com', role: 'Admin', status: 'Active', date: '2024-01-01' },
  { id: 2, email: 'manager@example.com', role: 'Manager', status: 'Active', date: '2024-02-15' },
  { id: 3, email: 'editor@example.com', role: 'Editor', status: 'Inactive', date: '2024-03-20' },
]
</script>
