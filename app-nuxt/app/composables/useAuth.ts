import type { LoginResponse, CurrentResponse } from '~/types/index'

export const useAuth = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.baseURL
  const { apiFetch } = useApi()

  const user = useState<CurrentResponse | null>('auth_user', () => null)
  const isAuthenticated = computed(() => !!user.value)
  const loading = useState<boolean>('auth_loading', () => false)
  const error = useState<string | null>('auth_error', () => null)

  async function login(email: string, password: string) {
    loading.value = true
    error.value = null
    try {
      const data = await $fetch<LoginResponse>(`${baseURL}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { email, password },
        credentials: 'include'
      })
      
      user.value = {
        pid: data.pid,
        name: data.name,
        email,
        roles: data.roles,
        can_manage_admin: data.can_manage_admin
      }
      return data
    } catch (err: any) {
      const message = err?.data?.message || err?.message || 'Erro ao fazer login'
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
      await $fetch(`${baseURL}/api/auth/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { name, email, password }
      })
    } catch (err: any) {
      const message = err?.data?.message || err?.message || 'Erro ao registrar'
      error.value = message
      throw err
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    try {
      await $fetch(`${baseURL}/api/auth/logout`, {
        method: 'POST',
        credentials: 'include'
      })
    } catch (err) {
      console.error('Erro ao fazer logout no servidor', err)
    } finally {
      user.value = null
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
      return data
    } catch {
      user.value = null
      return null
    } finally {
      loading.value = false
    }
  }

  async function forgotPassword(email: string) {
    loading.value = true
    error.value = null
    try {
      await $fetch(`${baseURL}/api/auth/forgot`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { email }
      })
    } catch (err: any) {
      const message = err?.data?.message || err?.message || 'Erro ao enviar email'
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
      await $fetch(`${baseURL}/api/auth/reset`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { token, password }
      })
    } catch (err: any) {
      const message = err?.data?.message || err?.message || 'Erro ao redefinir senha'
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
    isAuthenticated,
    loading,
    error,
    login,
    register,
    logout,
    fetchCurrentUser,
    forgotPassword,
    resetPassword,
    init
  }
}
