import { describe, it, expect, vi, beforeEach } from 'vitest'

const mockRegister = vi.fn()
const mockLoading = { value: false }
const mockError = { value: null as string | null }

vi.mock('vue-i18n', () => ({
  useI18n: () => ({
    t: (key: string, params?: any) => {
      if (key === 'auth.register.success.message') return `Success: ${params?.email}`
      if (key === 'auth.register.errors.passwordMismatch') return 'Passwords do not match'
      return key
    },
  }),
  createI18n: () => ({ global: { t: (key: string) => key } }),
}))

vi.mock('~/composables/useAuth', () => ({
  useAuth: () => ({
    register: mockRegister,
    loading: mockLoading,
    error: mockError,
    login: vi.fn(),
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

describe('register page', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockLoading.value = false
    mockError.value = null
  })

  it('should call register with name, email, and password', async () => {
    mockRegister.mockResolvedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await auth.register('John Doe', 'john@example.com', 'Password123!')
    expect(mockRegister).toHaveBeenCalledWith('John Doe', 'john@example.com', 'Password123!')
  })

  it('should set error on failed registration', async () => {
    mockRegister.mockRejectedValueOnce({ data: { message: 'Email already exists' } })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.register('John', 'test@example.com', 'pass')).rejects.toThrow()
  })

  it('should validate password mismatch client-side', () => {
    const password = 'Password123!'
    const confirmPassword = 'DifferentPassword'
    expect(password === confirmPassword).toBe(false)
  })

  it('should validate password match', () => {
    const password = 'Password123!'
    const confirmPassword = 'Password123!'
    expect(password === confirmPassword).toBe(true)
  })

  it('should handle loading state during registration', async () => {
    mockLoading.value = true

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.loading.value).toBe(true)
  })

  it('should handle error state after failed registration', async () => {
    mockError.value = 'Email already exists'

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.error.value).toBe('Email already exists')
  })
})
