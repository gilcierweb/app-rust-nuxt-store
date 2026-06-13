import { describe, it, expect, vi, beforeEach } from 'vitest'

const mockApiFetch = vi.fn()
const mockNavigateTo = vi.fn()
const mockCallOnce = vi.fn((_key: string, fn: () => Promise<any>) => fn())
const mockMergeCartOnLogin = vi.fn()
const mockT = vi.fn((key: string) => key)

vi.mock('~/composables/useApi', () => ({
  useApi: () => ({ apiFetch: mockApiFetch }),
}))

vi.mock('~/composables/useCartSync', () => ({
  useCartSync: () => ({ mergeCartOnLogin: mockMergeCartOnLogin }),
}))

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: mockT }),
}))

describe('useAuth', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should initialize with default state', async () => {
    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.isAuthenticated.value).toBe(false)
    expect(auth.loading.value).toBe(false)
    expect(auth.error.value).toBeNull()
  })

  it('should set error on failed login', async () => {
    mockApiFetch.mockRejectedValueOnce({ data: { message: 'Invalid credentials' } })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.login('test@example.com', 'wrong')).rejects.toThrow()
    expect(auth.error.value).toBe('Invalid credentials')
  })

  it('should set error with i18n fallback on login failure', async () => {
    mockApiFetch.mockRejectedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.login('test@example.com', 'pass')).rejects.toThrow()
    expect(auth.error.value).toBe('auth.login.error.generic')
    expect(mockT).toHaveBeenCalledWith('auth.login.error.generic')
  })

  it('should set error on failed register', async () => {
    mockApiFetch.mockRejectedValueOnce({ data: { message: 'Email already exists' } })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.register('John', 'test@example.com', 'pass')).rejects.toThrow()
    expect(auth.error.value).toBe('Email already exists')
  })

  it('should set error with i18n fallback on register failure', async () => {
    mockApiFetch.mockRejectedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.register('John', 'test@example.com', 'pass')).rejects.toThrow()
    expect(auth.error.value).toBe('auth.register.errors.generic')
  })

  it('should set error on failed forgotPassword', async () => {
    mockApiFetch.mockRejectedValueOnce({ data: { message: 'Email not found' } })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.forgotPassword('test@example.com')).rejects.toThrow()
    expect(auth.error.value).toBe('Email not found')
  })

  it('should set error with i18n fallback on forgotPassword failure', async () => {
    mockApiFetch.mockRejectedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.forgotPassword('test@example.com')).rejects.toThrow()
    expect(auth.error.value).toBe('auth.forgotPassword.error.generic')
  })

  it('should set error on failed resetPassword', async () => {
    mockApiFetch.mockRejectedValueOnce({ data: { message: 'Invalid token' } })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.resetPassword('token123', 'newpass')).rejects.toThrow()
    expect(auth.error.value).toBe('Invalid token')
  })

  it('should set error with i18n fallback on resetPassword failure', async () => {
    mockApiFetch.mockRejectedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await expect(auth.resetPassword('token123', 'newpass')).rejects.toThrow()
    expect(auth.error.value).toBe('auth.resetPassword.error.generic')
  })

  it('hasSectionAccess should return false when no user', async () => {
    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    expect(auth.hasSectionAccess('orders')).toBe(false)
  })

  it('hasSectionAccess should return true for admin role', async () => {
    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    auth.user.value = { roles: ['admin'], can_manage_admin: true } as any
    expect(auth.hasSectionAccess('orders')).toBe(true)
  })

  it('hasSectionAccess should check sections for non-admin users', async () => {
    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    auth.user.value = { roles: ['user'], can_manage_admin: true } as any
    auth.adminSections.value = ['orders', 'products']

    expect(auth.hasSectionAccess('orders')).toBe(true)
    expect(auth.hasSectionAccess('refunds')).toBe(false)
  })

  it('hasSectionAccess should return false when can_manage_admin is false', async () => {
    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    auth.user.value = { roles: ['user'], can_manage_admin: false } as any
    expect(auth.hasSectionAccess('orders')).toBe(false)
  })

  it('login should set user on success', async () => {
    mockApiFetch.mockResolvedValueOnce({
      pid: 'user-1',
      name: 'John',
      roles: ['user'],
      can_manage_admin: false,
    })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    const result = await auth.login('john@example.com', 'pass123')
    expect(result.pid).toBe('user-1')
    expect(auth.user.value).toBeTruthy()
    expect(auth.user.value?.name).toBe('John')
  })

  it('login should set admin sections when present', async () => {
    mockApiFetch.mockResolvedValueOnce({
      pid: 'admin-1',
      name: 'Admin',
      roles: ['admin'],
      can_manage_admin: true,
      admin_sections: ['orders', 'products'],
    })

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    await auth.login('admin@example.com', 'pass123')
    expect(auth.adminSections.value).toEqual(['orders', 'products'])
  })

  it('logout should clear user state', async () => {
    mockApiFetch.mockResolvedValueOnce({})

    const { useAuth } = await import('~/composables/useAuth')
    const auth = useAuth()

    auth.user.value = { pid: 'user-1', name: 'John' } as any
    await auth.logout()
    expect(auth.user.value).toBeNull()
    expect(auth.adminSections.value).toEqual([])
  })
})
