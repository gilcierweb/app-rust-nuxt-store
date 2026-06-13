import { describe, it, expect, vi, beforeEach } from 'vitest'

const mockApiFetch = vi.fn()
let mockIsAuthenticated = { value: false }
let wishlistItems: any[] = []

vi.mock('~/composables/useApi', () => ({
  useApi: () => ({ apiFetch: mockApiFetch }),
}))

vi.mock('~/composables/useAuth', () => ({
  useAuth: () => ({ isAuthenticated: mockIsAuthenticated }),
}))

vi.mock('#imports', () => ({
  useState: vi.fn((_key: string, init?: () => any) => {
    return { value: init ? init() : [] }
  }),
}))

describe('useWishlist', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    wishlistItems = []
    mockIsAuthenticated.value = false
  })

  it('should initialize with loading=false', async () => {
    const { useWishlist } = await import('~/composables/useWishlist')
    const { loading } = useWishlist()
    expect(loading.value).toBe(false)
  })

  it('fetchWishlist should clear items when not authenticated', async () => {
    const { useWishlist } = await import('~/composables/useWishlist')
    const { fetchWishlist, wishlist } = useWishlist()

    await fetchWishlist()
    expect(wishlist.value).toEqual([])
    expect(mockApiFetch).not.toHaveBeenCalled()
  })

  it('fetchWishlist should fetch items when authenticated', async () => {
    mockIsAuthenticated.value = true
    const items = [{ id: 1, product_id: 10, product_name: 'Shirt' }]
    mockApiFetch.mockResolvedValueOnce(items)

    const { useWishlist } = await import('~/composables/useWishlist')
    const { fetchWishlist, wishlist } = useWishlist()

    await fetchWishlist()
    expect(mockApiFetch).toHaveBeenCalledWith('/api/account/wishlist')
    expect(wishlist.value).toEqual(items)
  })

  it('fetchWishlist should clear items on auth error', async () => {
    mockIsAuthenticated.value = true
    mockApiFetch.mockRejectedValueOnce({ statusCode: 401 })

    const { useWishlist } = await import('~/composables/useWishlist')
    const { fetchWishlist, wishlist } = useWishlist()

    await fetchWishlist()
    expect(wishlist.value).toEqual([])
  })

  it('fetchWishlist should clear items on other errors', async () => {
    mockIsAuthenticated.value = true
    mockApiFetch.mockRejectedValueOnce({ statusCode: 500 })

    const { useWishlist } = await import('~/composables/useWishlist')
    const { fetchWishlist, wishlist } = useWishlist()

    await fetchWishlist()
    expect(wishlist.value).toEqual([])
  })

  it('toggleWishlist should not call API when not authenticated', async () => {
    const { useWishlist } = await import('~/composables/useWishlist')
    const { toggleWishlist } = useWishlist()

    await toggleWishlist(10)
    expect(mockApiFetch).not.toHaveBeenCalled()
  })

  it('toggleWishlist should add item when not in wishlist', async () => {
    mockIsAuthenticated.value = true
    const newItem = { id: 1, product_id: 10, product_name: 'Shirt' }
    mockApiFetch.mockResolvedValueOnce(newItem)

    const { useWishlist } = await import('~/composables/useWishlist')
    const { toggleWishlist, wishlist } = useWishlist()

    await toggleWishlist(10)
    expect(mockApiFetch).toHaveBeenCalledWith('/api/account/wishlist/add', expect.objectContaining({ method: 'POST' }))
    expect(wishlist.value).toContainEqual(newItem)
  })

  it('toggleWishlist should remove item when in wishlist', async () => {
    mockIsAuthenticated.value = true
    wishlistItems = [{ id: 5, product_id: 10, product_name: 'Shirt' }]

    vi.doMock('#imports', () => ({
      useState: vi.fn(() => ({ value: wishlistItems })),
    }))

    mockApiFetch.mockResolvedValueOnce({})

    const { useWishlist } = await import('~/composables/useWishlist')
    const { toggleWishlist, wishlist } = useWishlist()

    wishlist.value = [...wishlistItems]

    await toggleWishlist(10)
    expect(mockApiFetch).toHaveBeenCalledWith('/api/account/wishlist/remove/5', expect.objectContaining({ method: 'DELETE' }))
  })

  it('isInWishlist should return false when item not present', async () => {
    const { useWishlist } = await import('~/composables/useWishlist')
    const { isInWishlist, wishlist } = useWishlist()

    wishlist.value = [{ id: 1, product_id: 10, product_name: 'Shirt' }]
    expect(isInWishlist(20)).toBe(false)
  })

  it('isInWishlist should return true when item is present', async () => {
    const { useWishlist } = await import('~/composables/useWishlist')
    const { isInWishlist, wishlist } = useWishlist()

    wishlist.value = [{ id: 1, product_id: 10, product_name: 'Shirt' }]
    expect(isInWishlist(10)).toBe(true)
  })

  it('isInWishlist should return false on empty wishlist', async () => {
    const { useWishlist } = await import('~/composables/useWishlist')
    const { isInWishlist, wishlist } = useWishlist()

    wishlist.value = []
    expect(isInWishlist(10)).toBe(false)
  })
})
