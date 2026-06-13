import { describe, it, expect, vi, beforeEach } from 'vitest'

const mockLogin = vi.fn()
const mockLoading = { value: false }
const mockError = { value: null as string | null }

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
  createI18n: () => ({ global: { t: (key: string) => key } }),
}))

vi.mock('~/composables/useAuth', () => ({
  useAuth: () => ({
    login: mockLogin,
    loading: mockLoading,
    error: mockError,
    register: vi.fn(),
    logout: vi.fn(),
    forgotPassword: vi.fn(),
    resetPassword: vi.fn(),
    user: { value: null },
    isAuthenticated: { value: false },
    fetchCurrentUser: vi.fn(),
    hasSectionAccess: vi.fn().mockReturnValue(false),
  }),
}))

vi.mock('~/composables/useApi', () => ({
  useApi: () => ({ apiFetch: vi.fn() }),
}))

describe('login page', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockLoading.value = false
    mockError.value = null
  })

  it('should call login with email and password', async () => {
    mockLogin.mockResolvedValueOnce({ pid: 'user-1' })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await auth.login('test@example.com', 'password123')
    expect(mockLogin).toHaveBeenCalledWith('test@example.com', 'password123')
  })

  it('should return user data on successful login', async () => {
    mockLogin.mockResolvedValueOnce({ pid: 'user-1', name: 'John' })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    const result = await auth.login('test@example.com', 'password123')
    expect(result.pid).toBe('user-1')
  })

  it('should set error on failed login', async () => {
    mockLogin.mockRejectedValueOnce({ data: { message: 'Invalid credentials' } })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.login('test@example.com', 'wrong')).rejects.toThrow()
  })

  it('should handle loading state', async () => {
    mockLoading.value = true

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.loading.value).toBe(true)
  })

  it('should handle error state', async () => {
    mockError.value = 'Invalid credentials'

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.error.value).toBe('Invalid credentials')
  })
})
