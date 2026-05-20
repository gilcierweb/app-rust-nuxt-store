<template>
  <div>
    <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h1 class="h1">{{ $t('admin.rbac.title') }}</h1>
        <p class="text-sm text-base-content/60">{{ $t('admin.rbac.description') }}</p>
      </div>
      <button type="button" class="btn btn-outline gap-2" :disabled="pending" @click="refresh">
        <i class="icon-[tabler--refresh] size-5"></i>
        {{ $t('common.actions.refresh') }}
      </button>
    </div>

    <div class="tabs tabs-lifted mb-6">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        type="button"
        class="tab tab-lg"
        :class="{ 'tab-active font-bold': activeTab === tab.key }"
        @click="activeTab = tab.key"
      >
        <span class="flex items-center gap-2">
          <i :class="[tab.icon, 'size-5']"></i>
          {{ tab.label }}
        </span>
      </button>
    </div>

    <div v-if="pending" class="flex items-center justify-center py-12">
      <span class="loading loading-spinner text-primary size-12"></span>
      <span class="ml-3">{{ $t('admin.rbac.loading') }}</span>
    </div>

    <div v-else-if="error" class="alert alert-error">
      <i class="icon-[tabler--alert-circle] size-6"></i>
      <span>{{ $t('admin.rbac.error', { message: error.message }) }}</span>
    </div>

    <div v-else>
      <AppAlert
        v-if="actionError"
        type="error"
        :message="actionError"
        :auto-close="5000"
        dismissible
        @close="actionError = ''"
      />

      <section v-if="activeTab === 'roles'" class="grid grid-cols-1 gap-6 xl:grid-cols-[360px_1fr]">
        <div class="rounded-box bg-base-100 border shadow-sm p-5">
          <h2 class="text-lg font-bold mb-4">{{ $t('admin.rbac.roles.create') }}</h2>
          <form class="space-y-4" @submit.prevent="createRole">
            <div class="form-control">
              <label class="label" for="roleName">
                <span class="label-text font-semibold">{{ $t('admin.rbac.roles.name') }}</span>
              </label>
              <input id="roleName" v-model="roleForm.name" class="input input-bordered w-full" type="text" required />
            </div>
            <div class="form-control">
              <label class="label" for="roleResourceType">
                <span class="label-text font-semibold">{{ $t('admin.rbac.roles.resourceType') }}</span>
              </label>
              <input id="roleResourceType" v-model="roleForm.resource_type" class="input input-bordered w-full" type="text" />
            </div>
            <div class="form-control">
              <label class="label" for="roleResourceId">
                <span class="label-text font-semibold">{{ $t('admin.rbac.roles.resourceId') }}</span>
              </label>
              <input id="roleResourceId" v-model.number="roleForm.resource_id" class="input input-bordered w-full" type="number" min="1" />
            </div>
            <button type="submit" class="btn btn-primary w-full" :disabled="isSavingRole">
              <span v-if="isSavingRole" class="loading loading-spinner loading-sm"></span>
              <i v-else class="icon-[tabler--plus] size-5"></i>
              {{ $t('admin.rbac.roles.create') }}
            </button>
          </form>
        </div>

        <div class="rounded-box bg-base-100 border shadow-sm overflow-hidden">
          <div class="overflow-x-auto">
            <table class="table">
              <thead>
                <tr>
                  <th>{{ $t('admin.rbac.roles.name') }}</th>
                  <th>{{ $t('admin.rbac.roles.scope') }}</th>
                  <th>{{ $t('admin.rbac.roles.assignments') }}</th>
                  <th class="text-right">{{ $t('common.table.actions') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="role in roles" :key="role.id">
                  <td class="font-medium">{{ role.name }}</td>
                  <td>{{ roleScope(role) }}</td>
                  <td>{{ role.assignment_count }}</td>
                  <td class="text-right">
                    <button
                      type="button"
                      class="btn btn-circle btn-text btn-sm text-error"
                      :disabled="role.protected || role.assignment_count > 0"
                      :aria-label="$t('common.delete')"
                      @click="deleteRole(role)"
                    >
                      <i class="icon-[tabler--trash] size-5"></i>
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <section v-else-if="activeTab === 'assignments'" class="space-y-6">
        <div class="rounded-box bg-base-100 border shadow-sm p-5">
          <form class="grid grid-cols-1 gap-4 md:grid-cols-[1fr_1fr_auto]" @submit.prevent="assignRole">
            <select v-model.number="assignmentForm.user_id" class="select select-bordered w-full" required>
              <option :value="0">{{ $t('admin.rbac.assignments.selectUser') }}</option>
              <option v-for="user in users" :key="user.id" :value="user.id">
                {{ user.email }}
              </option>
            </select>
            <select v-model.number="assignmentForm.role_id" class="select select-bordered w-full" required>
              <option :value="0">{{ $t('admin.rbac.assignments.selectRole') }}</option>
              <option v-for="role in roles" :key="role.id" :value="role.id">
                {{ role.name }} - {{ roleScope(role) }}
              </option>
            </select>
            <button type="submit" class="btn btn-primary" :disabled="isAssigning">
              <span v-if="isAssigning" class="loading loading-spinner loading-sm"></span>
              <i v-else class="icon-[tabler--user-plus] size-5"></i>
              {{ $t('admin.rbac.assignments.assign') }}
            </button>
          </form>
        </div>

        <div class="rounded-box bg-base-100 border shadow-sm overflow-hidden">
          <div class="overflow-x-auto">
            <table class="table">
              <thead>
                <tr>
                  <th>{{ $t('admin.users.table.email') }}</th>
                  <th>{{ $t('admin.rbac.assignments.roles') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="user in users" :key="user.id">
                  <td>
                    <div class="font-medium">{{ user.email }}</div>
                    <div class="text-xs text-base-content/60">{{ user.name }}</div>
                  </td>
                  <td>
                    <div class="flex flex-wrap gap-2">
                      <span v-for="role in user.roles" :key="role.id" class="badge badge-soft badge-primary gap-1">
                        {{ role.name }}
                        <button type="button" class="btn btn-xs btn-circle btn-text" @click="removeAssignment(user, role)">
                          <i class="icon-[tabler--x] size-3"></i>
                        </button>
                      </span>
                      <span v-if="user.roles.length === 0" class="text-sm text-base-content/50">
                        {{ $t('admin.rbac.assignments.noRoles') }}
                      </span>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <section v-else class="rounded-box bg-base-100 border shadow-sm overflow-hidden">
        <div class="overflow-x-auto">
          <table class="table">
            <thead>
              <tr>
                <th>{{ $t('admin.rbac.permissions.subject') }}</th>
                <th>{{ $t('admin.rbac.permissions.actions') }}</th>
                <th>{{ $t('admin.rbac.permissions.source') }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="group in permissions?.groups || []" :key="group.subject">
                <td class="font-medium">{{ group.subject }}</td>
                <td>
                  <div class="flex flex-wrap gap-2">
                    <span v-for="action in group.actions" :key="action" class="badge badge-outline">
                      {{ action }}
                    </span>
                  </div>
                </td>
                <td class="font-mono text-xs text-base-content/60">{{ group.source }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin'
})

interface RoleItem {
  id: number
  name: string
  resource_type?: string | null
  resource_id?: number | null
  assignment_count: number
  protected: boolean
}

interface RbacUser {
  id: number
  email: string
  name: string
  roles: RoleItem[]
}

interface RbacSummary {
  roles: RoleItem[]
  users: RbacUser[]
  total_users: number
}

interface PermissionsResponse {
  groups: Array<{
    subject: string
    actions: string[]
    source: string
  }>
}

const { t } = useI18n()
const { apiFetch, useApiFetch } = useApi()

const activeTab = ref<'roles' | 'assignments' | 'permissions'>('roles')
const actionError = ref('')
const isSavingRole = ref(false)
const isAssigning = ref(false)

const roleForm = reactive({
  name: '',
  resource_type: '',
  resource_id: undefined as number | undefined
})

const assignmentForm = reactive({
  user_id: 0,
  role_id: 0
})

const tabs = computed(() => [
  { key: 'roles', label: t('admin.rbac.tabs.roles'), icon: 'icon-[tabler--shield]' },
  { key: 'assignments', label: t('admin.rbac.tabs.assignments'), icon: 'icon-[tabler--users-group]' },
  { key: 'permissions', label: t('admin.rbac.tabs.permissions'), icon: 'icon-[tabler--list-check]' }
])

const { pending, data, error, refresh } = await useApiFetch<RbacSummary>(
  '/api/admin/rbac/summary',
  {
    key: 'admin-rbac-summary',
    query: { page: 1, per_page: 100 }
  }
)

const { data: permissions } = await useApiFetch<PermissionsResponse>(
  '/api/admin/rbac/permissions',
  { key: 'admin-rbac-permissions' }
)

const roles = computed(() => data.value?.roles || [])
const users = computed(() => data.value?.users || [])

function roleScope(role: RoleItem) {
  if (!role.resource_type) return t('admin.rbac.roles.global')
  return role.resource_id ? `${role.resource_type} #${role.resource_id}` : role.resource_type
}

async function createRole() {
  actionError.value = ''
  isSavingRole.value = true
  try {
    await apiFetch('/api/admin/rbac/roles', {
      method: 'POST',
      body: {
        name: roleForm.name,
        resource_type: roleForm.resource_type || undefined,
        resource_id: roleForm.resource_id || undefined
      }
    })
    roleForm.name = ''
    roleForm.resource_type = ''
    roleForm.resource_id = undefined
    await refresh()
  } catch (error: any) {
    actionError.value = error?.data?.message || error?.message || t('admin.rbac.errors.action')
  } finally {
    isSavingRole.value = false
  }
}

async function deleteRole(role: RoleItem) {
  if (!confirm(t('common.confirmDelete', { name: role.name }))) return

  actionError.value = ''
  try {
    await apiFetch(`/api/admin/rbac/roles/${role.id}`, { method: 'DELETE' })
    await refresh()
  } catch (error: any) {
    actionError.value = error?.data?.message || error?.message || t('admin.rbac.errors.action')
  }
}

async function assignRole() {
  if (!assignmentForm.user_id || !assignmentForm.role_id) return

  actionError.value = ''
  isAssigning.value = true
  try {
    await apiFetch('/api/admin/rbac/assignments', {
      method: 'POST',
      body: assignmentForm
    })
    assignmentForm.user_id = 0
    assignmentForm.role_id = 0
    await refresh()
  } catch (error: any) {
    actionError.value = error?.data?.message || error?.message || t('admin.rbac.errors.action')
  } finally {
    isAssigning.value = false
  }
}

async function removeAssignment(user: RbacUser, role: RoleItem) {
  actionError.value = ''
  try {
    await apiFetch(`/api/admin/rbac/assignments/${user.id}/${role.id}`, { method: 'DELETE' })
    await refresh()
  } catch (error: any) {
    actionError.value = error?.data?.message || error?.message || t('admin.rbac.errors.action')
  }
}
</script>
