import type { LoginResponse, CurrentResponse, AdminSectionsResponse } from '~/types/index'

export const useAuth = () => {
  const { apiFetch } = useApi()

  const user = useState<CurrentResponse | null>('auth_user', () => null)
  const adminSections = useState<string[]>('auth_admin_sections', () => [])
  const isAuthenticated = computed(() => !!user.value)
  const loading = useState<boolean>('auth_loading', () => false)
  const error = useState<string | null>('auth_error', () => null)

  async function login(email: string, password: string) {
    loading.value = true
    error.value = null
    try {
      const data = await apiFetch<LoginResponse>('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { email, password }
      })
      
      user.value = {
        pid: data.pid,
        name: data.name,
        email,
        roles: data.roles,
        can_manage_admin: data.can_manage_admin,
        admin_sections: data.admin_sections
      }

      if (data.admin_sections) {
        adminSections.value = data.admin_sections
      }

      const { mergeCartOnLogin } = useCartSync()
      await mergeCartOnLogin()

      return data
    } catch (err: any) {
      const { $i18n } = useNuxtApp()
      const message = err?.data?.message || err?.message || ($i18n as any).t('auth.login.error.generic')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  async function register(name: string, email: string, password: string) {
    loading.value = true
    error.value = null
    try {
      await apiFetch('/api/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { name, email, password }
      })
    } catch (err: any) {
      const { $i18n } = useNuxtApp()
      const message = err?.data?.message || err?.message || ($i18n as any).t('auth.register.errors.generic')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    try {
      await apiFetch('/api/auth/logout', { method: 'POST' })
    } catch {
    } finally {
      user.value = null
      adminSections.value = []
      error.value = null
      navigateTo('/users/sessions')
    }
  }

  async function fetchCurrentUser() {
    if (loading.value) return user.value
    
    loading.value = true
    try {
      const data = await apiFetch<CurrentResponse>('/api/auth/current', {
        headers: { 'Content-Type': 'application/json' }
      })
      user.value = data

      if (data.admin_sections) {
        adminSections.value = data.admin_sections
      } else if (data.can_manage_admin) {
        await fetchAdminSections()
      }

      return data
    } catch {
      user.value = null
      adminSections.value = []
      return null
    } finally {
      loading.value = false
    }
  }

  async function fetchAdminSections() {
    try {
      const data = await apiFetch<AdminSectionsResponse>('/api/admin/rbac/sections')
      adminSections.value = data.sections
      if (user.value) {
        user.value.admin_sections = data.sections
      }
    } catch {
      adminSections.value = []
    }
  }

  function hasSectionAccess(section: string): boolean {
    if (!user.value?.can_manage_admin) return false
    if (user.value.roles?.includes('admin')) return true
    return adminSections.value.includes(section)
  }

  async function forgotPassword(email: string) {
    loading.value = true
    error.value = null
    try {
      await apiFetch('/api/auth/forgot', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { email }
      })
    } catch (err: any) {
      const { $i18n } = useNuxtApp()
      const message = err?.data?.message || err?.message || ($i18n as any).t('auth.forgotPassword.error.generic')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  async function resetPassword(token: string, password: string) {
    loading.value = true
    error.value = null
    try {
      await apiFetch('/api/auth/reset', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { token, password }
      })
    } catch (err: any) {
      const { $i18n } = useNuxtApp()
      const message = err?.data?.message || err?.message || ($i18n as any).t('auth.resetPassword.error.generic')
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  async function init() {
    if (user.value) return
    await callOnce('auth:current', () => fetchCurrentUser(), { mode: 'navigation' })
  }

  return {
    user,
    adminSections,
    isAuthenticated,
    loading,
    error,
    login,
    register,
    logout,
    fetchCurrentUser,
    fetchAdminSections,
    hasSectionAccess,
    forgotPassword,
    resetPassword,
    init
  }
}
