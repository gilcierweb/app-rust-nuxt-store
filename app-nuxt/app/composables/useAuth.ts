import type { LoginResponse, CurrentResponse } from '~/types/index'

export const useAuth = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.baseURL
  const { apiFetch } = useApi()

  const tokenCookie = useCookie<string | null>('auth_token', {
    default: () => null,
    sameSite: 'lax',
    secure: process.env.NODE_ENV === 'production',
    maxAge: 60 * 60 * 24 * 7
  })
  const user = useState<CurrentResponse | null>('auth_user', () => null)
  const isAuthenticated = computed(() => !!tokenCookie.value)
  const loading = useState<boolean>('auth_loading', () => false)
  const error = useState<string | null>('auth_error', () => null)

  async function login(email: string, password: string) {
    loading.value = true
    error.value = null
    try {
      const data = await $fetch<LoginResponse>(`${baseURL}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { email, password }
      })
      tokenCookie.value = data.token
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

  function logout() {
    tokenCookie.value = null
    user.value = null
    error.value = null
    navigateTo('/users/sessions')
  }

  async function fetchCurrentUser() {
    if (!tokenCookie.value) {
      user.value = null
      return null
    }
    try {
      const data = await apiFetch<CurrentResponse>('/api/auth/current', {
        headers: { 'Content-Type': 'application/json' }
      })
      user.value = data
      return data
    } catch {
      tokenCookie.value = null
      user.value = null
      return null
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
    if (tokenCookie.value) {
      await fetchCurrentUser()
    }
  }

  return {
    token: tokenCookie,
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
