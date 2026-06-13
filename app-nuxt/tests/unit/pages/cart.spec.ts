import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

const mockRemoveItemSync = vi.fn()
const mockUpdateQuantitySync = vi.fn()
const mockClearCartSync = vi.fn()

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
  createI18n: () => ({ global: { t: (key: string) => key } }),
}))

vi.mock('~/composables/useCartSync', () => ({
  useCartSync: () => ({
    removeItemSync: mockRemoveItemSync,
    updateQuantitySync: mockUpdateQuantitySync,
    clearCartSync: mockClearCartSync,
  }),
}))

vi.mock('~/composables/useApi', () => ({
  useApi: () => ({ apiFetch: vi.fn() }),
}))

vi.mock('~/composables/useAuth', () => ({
  useAuth: () => ({
    isAuthenticated: { value: false },
    user: { value: null },
    login: vi.fn(),
    logout: vi.fn(),
    loading: { value: false },
    error: { value: null },
  }),
}))

describe('cart page', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    setActivePinia(createPinia())
  })

  it('should show empty cart message when cart is empty', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    expect(store.isEmpty).toBe(true)
  })

  it('should calculate total items correctly', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, quantity: 2 })
    store.addItem({ productId: 2, name: 'Pants', price: 100, quantity: 1 })

    expect(store.totalItems).toBe(3)
    expect(store.totalPrice).toBe(200)
  })

  it('should call removeItemSync when removing an item', async () => {
    mockRemoveItemSync(1)
    expect(mockRemoveItemSync).toHaveBeenCalledWith(1)
  })

  it('should call updateQuantitySync when changing quantity', async () => {
    mockUpdateQuantitySync(1, 3)
    expect(mockUpdateQuantitySync).toHaveBeenCalledWith(1, 3)
  })

  it('should call clearCartSync when clearing cart', async () => {
    mockClearCartSync()
    expect(mockClearCartSync).toHaveBeenCalled()
  })

  it('should display correct price for items', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 29.99, quantity: 2 })

    expect(store.items[0].price * store.items[0].quantity).toBeCloseTo(59.98)
    expect(store.totalPrice).toBeCloseTo(59.98)
  })

  it('should have empty total when all items removed', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50 })
    store.removeItem(1)

    expect(store.totalItems).toBe(0)
    expect(store.totalPrice).toBe(0)
    expect(store.isEmpty).toBe(true)
  })
})
