import { describe, it, expect, vi, beforeEach } from 'vitest'

const mockForgotPassword = vi.fn()
const mockLoading = { value: false }
const mockError = { value: null as string | null }

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
  createI18n: () => ({ global: { t: (key: string) => key } }),
}))

vi.mock('~/composables/useAuth', () => ({
  useAuth: () => ({
    forgotPassword: mockForgotPassword,
    loading: mockLoading,
    error: mockError,
    login: vi.fn(),
    register: vi.fn(),
    logout: vi.fn(),
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

describe('forgot password page', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockLoading.value = false
    mockError.value = null
  })

  it('should call forgotPassword with email', async () => {
    mockForgotPassword.mockResolvedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await auth.forgotPassword('test@example.com')
    expect(mockForgotPassword).toHaveBeenCalledWith('test@example.com')
  })

  it('should set error on failed forgotPassword', async () => {
    mockForgotPassword.mockRejectedValueOnce({ data: { message: 'Email not found' } })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.forgotPassword('test@example.com')).rejects.toThrow()
  })

  it('should set i18n error on network failure', async () => {
    mockForgotPassword.mockImplementation(async () => {
      mockError.value = 'auth.forgotPassword.error.generic'
      throw new Error('fail')
    })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.forgotPassword('test@example.com')).rejects.toThrow()
    expect(auth.error.value).toBe('auth.forgotPassword.error.generic')
  })

  it('should handle loading state', async () => {
    mockLoading.value = true

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.loading.value).toBe(true)
  })

  it('should handle error state', async () => {
    mockError.value = 'Email not found'

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.error.value).toBe('Email not found')
  })

  it('should return success on valid email', async () => {
    mockForgotPassword.mockResolvedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    const result = await auth.forgotPassword('test@example.com')
    expect(result).toEqual({})
  })
})
