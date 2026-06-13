import { describe, it, expect, vi, beforeEach } from 'vitest'

const mockApiFetch = vi.fn()
let mockIsAuthenticated = { value: false }

let cartItems: any[] = []

const mockCartStore = {
  items: cartItems,
  addItem: vi.fn((item: any) => { cartItems.push({ id: '1', ...item, quantity: item.quantity || 1 }) }),
  removeItem: vi.fn((id: number) => { cartItems = cartItems.filter(i => i.productId !== id) }),
  updateQuantity: vi.fn((id: number, qty: number) => {
    const item = cartItems.find(i => i.productId === id)
    if (item) item.quantity = qty
  }),
  clearCart: vi.fn(() => { cartItems = [] }),
  setItems: vi.fn((items: any[]) => { cartItems = items }),
}

vi.mock('~/composables/useApi', () => ({
  useApi: () => ({ apiFetch: mockApiFetch }),
}))

vi.mock('~/composables/useAuth', () => ({
  useAuth: () => ({ isAuthenticated: mockIsAuthenticated }),
}))

vi.mock('~/stores/cart', () => ({
  useCartStore: () => mockCartStore,
  mapCartApiItem: (item: any) => ({
    id: String(item.id),
    productId: item.product_id,
    name: item.product_name ?? 'Product',
    price: Number(item.price),
    quantity: item.quantity,
    image: item.product_image ?? undefined,
    slug: item.product_slug ?? undefined,
    variantId: item.product_variant_id ?? undefined,
  }),
}))

describe('useCartSync', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    cartItems = []
    mockCartStore.items = cartItems
    mockIsAuthenticated.value = false
  })

  it('should initialize with syncing=false', async () => {
    const { useCartSync } = await import('~/composables/useCartSync')
    const { syncing } = useCartSync()
    expect(syncing.value).toBe(false)
  })

  it('fetchCart should not fetch when not authenticated', async () => {
    const { useCartSync } = await import('~/composables/useCartSync')
    const { fetchCart } = useCartSync()

    await fetchCart()
    expect(mockApiFetch).not.toHaveBeenCalled()
  })

  it('fetchCart should fetch and set items when authenticated', async () => {
    mockIsAuthenticated.value = true
    const serverItems = [
      { id: 1, product_id: 10, product_name: 'Shirt', price: 29.99, quantity: 2 },
    ]
    mockApiFetch.mockResolvedValueOnce({ items: serverItems })

    const { useCartSync } = await import('~/composables/useCartSync')
    const { fetchCart } = useCartSync()

    await fetchCart()
    expect(mockApiFetch).toHaveBeenCalledWith('/api/carts/')
    expect(mockCartStore.setItems).toHaveBeenCalled()
  })

  it('fetchCart should clear items on auth error', async () => {
    mockIsAuthenticated.value = true
    mockApiFetch.mockRejectedValueOnce({ statusCode: 401 })

    const { useCartSync } = await import('~/composables/useCartSync')
    const { fetchCart } = useCartSync()

    await fetchCart()
    expect(mockCartStore.setItems).toHaveBeenCalledWith([])
  })

  it('addItemSync should add to local store when not authenticated', async () => {
    const { useCartSync } = await import('~/composables/useCartSync')
    const { addItemSync } = useCartSync()

    await addItemSync({ productId: 10, name: 'Shirt', price: 29.99 })
    expect(mockCartStore.addItem).toHaveBeenCalled()
    expect(mockApiFetch).not.toHaveBeenCalled()
  })

  it('addItemSync should sync to server when authenticated', async () => {
    mockIsAuthenticated.value = true
    mockApiFetch.mockResolvedValueOnce({ items: [] })

    const { useCartSync } = await import('~/composables/useCartSync')
    const { addItemSync } = useCartSync()

    await addItemSync({ productId: 10, name: 'Shirt', price: 29.99 })
    expect(mockCartStore.addItem).toHaveBeenCalled()
    expect(mockApiFetch).toHaveBeenCalledWith('/api/carts/add_item', expect.objectContaining({ method: 'POST' }))
  })

  it('removeItemSync should remove from local store', async () => {
    cartItems.push({ id: '1', productId: 10, name: 'Shirt', price: 29.99, quantity: 1 })
    mockIsAuthenticated.value = false

    const { useCartSync } = await import('~/composables/useCartSync')
    const { removeItemSync } = useCartSync()

    await removeItemSync(10)
    expect(mockCartStore.removeItem).toHaveBeenCalledWith(10)
  })

  it('updateQuantitySync should update local store', async () => {
    cartItems.push({ id: '1', productId: 10, name: 'Shirt', price: 29.99, quantity: 1 })
    mockIsAuthenticated.value = false

    const { useCartSync } = await import('~/composables/useCartSync')
    const { updateQuantitySync } = useCartSync()

    await updateQuantitySync(10, 5)
    expect(mockCartStore.updateQuantity).toHaveBeenCalledWith(10, 5)
  })

  it('clearCartSync should clear local store', async () => {
    mockIsAuthenticated.value = false

    const { useCartSync } = await import('~/composables/useCartSync')
    const { clearCartSync } = useCartSync()

    await clearCartSync()
    expect(mockCartStore.clearCart).toHaveBeenCalled()
  })

  it('checkStock should return available=true when variants not found', async () => {
    mockApiFetch.mockResolvedValueOnce([])

    const { useCartSync } = await import('~/composables/useCartSync')
    const { checkStock } = useCartSync()

    const result = await checkStock(10)
    expect(result.available).toBe(true)
    expect(result.maxQuantity).toBe(999)
  })

  it('checkStock should return available=true when track_inventory is false', async () => {
    mockApiFetch.mockResolvedValueOnce([{ id: 1, track_inventory: false }])

    const { useCartSync } = await import('~/composables/useCartSync')
    const { checkStock } = useCartSync()

    const result = await checkStock(10, 1)
    expect(result.available).toBe(true)
  })

  it('checkStock should calculate available from inventory', async () => {
    mockApiFetch.mockResolvedValueOnce([{
      id: 1,
      track_inventory: true,
      allow_backorder: false,
      inventory_quantity: 10,
      reserved_quantity: 3,
    }])

    const { useCartSync } = await import('~/composables/useCartSync')
    const { checkStock } = useCartSync()

    const result = await checkStock(10, 1)
    expect(result.available).toBe(true)
    expect(result.maxQuantity).toBe(7)
  })

  it('checkStock should return not available when inventory is zero', async () => {
    mockApiFetch.mockResolvedValueOnce([{
      id: 1,
      track_inventory: true,
      allow_backorder: false,
      inventory_quantity: 0,
      reserved_quantity: 0,
    }])

    const { useCartSync } = await import('~/composables/useCartSync')
    const { checkStock } = useCartSync()

    const result = await checkStock(10, 1)
    expect(result.available).toBe(false)
    expect(result.maxQuantity).toBe(0)
  })

  it('checkStock should return available=true when allow_backorder is true', async () => {
    mockApiFetch.mockResolvedValueOnce([{
      id: 1,
      track_inventory: true,
      allow_backorder: true,
      inventory_quantity: 0,
      reserved_quantity: 0,
    }])

    const { useCartSync } = await import('~/composables/useCartSync')
    const { checkStock } = useCartSync()

    const result = await checkStock(10, 1)
    expect(result.available).toBe(true)
  })

  it('checkStock should fallback to available on error', async () => {
    mockApiFetch.mockRejectedValueOnce(new Error('Network error'))

    const { useCartSync } = await import('~/composables/useCartSync')
    const { checkStock } = useCartSync()

    const result = await checkStock(10)
    expect(result.available).toBe(true)
    expect(result.maxQuantity).toBe(999)
  })

  it('mergeCartOnLogin should not run when not authenticated', async () => {
    const { useCartSync } = await import('~/composables/useCartSync')
    const { mergeCartOnLogin } = useCartSync()

    await mergeCartOnLogin()
    expect(mockApiFetch).not.toHaveBeenCalled()
  })

  it('mergeCartOnLogin should fetch server cart when authenticated', async () => {
    mockIsAuthenticated.value = true
    mockApiFetch.mockResolvedValueOnce({ items: [] })

    const { useCartSync } = await import('~/composables/useCartSync')
    const { mergeCartOnLogin } = useCartSync()

    await mergeCartOnLogin()
    expect(mockApiFetch).toHaveBeenCalledWith('/api/carts/')
    expect(mockCartStore.setItems).toHaveBeenCalled()
  })
})
