import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

describe('cart store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('should initialize with empty items', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    expect(store.items).toEqual([])
    expect(store.totalItems).toBe(0)
    expect(store.totalPrice).toBe(0)
    expect(store.isEmpty).toBe(true)
  })

  it('should add new item', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, quantity: 2 })

    expect(store.items).toHaveLength(1)
    expect(store.items[0].productId).toBe(1)
    expect(store.items[0].quantity).toBe(2)
    expect(store.items[0].price).toBe(50)
    expect(store.totalItems).toBe(2)
    expect(store.totalPrice).toBe(100)
    expect(store.isEmpty).toBe(false)
  })

  it('should increment quantity for existing item', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, quantity: 2 })
    store.addItem({ productId: 1, name: 'Shirt', price: 50, quantity: 3 })

    expect(store.items).toHaveLength(1)
    expect(store.items[0].quantity).toBe(5)
    expect(store.totalItems).toBe(5)
  })

  it('should add different items separately', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50 })
    store.addItem({ productId: 2, name: 'Pants', price: 100 })

    expect(store.items).toHaveLength(2)
    expect(store.totalPrice).toBe(150)
  })

  it('should add different variants of same product separately', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, variantId: 10 })
    store.addItem({ productId: 1, name: 'Shirt', price: 60, variantId: 20 })

    expect(store.items).toHaveLength(2)
    expect(store.items[0].variantId).toBe(10)
    expect(store.items[1].variantId).toBe(20)
  })

  it('should remove item by productId', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50 })
    store.addItem({ productId: 2, name: 'Pants', price: 100 })
    store.removeItem(1)

    expect(store.items).toHaveLength(1)
    expect(store.items[0].productId).toBe(2)
  })

  it('should update quantity', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, quantity: 1 })
    store.updateQuantity(1, 5)

    expect(store.items[0].quantity).toBe(5)
    expect(store.totalItems).toBe(5)
  })

  it('should remove item when quantity is updated to 0', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, quantity: 1 })
    store.updateQuantity(1, 0)

    expect(store.items).toHaveLength(0)
  })

  it('should remove item when quantity is negative', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, quantity: 1 })
    store.updateQuantity(1, -1)

    expect(store.items).toHaveLength(0)
  })

  it('should clear all items', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50 })
    store.addItem({ productId: 2, name: 'Pants', price: 100 })
    store.clearCart()

    expect(store.items).toHaveLength(0)
    expect(store.isEmpty).toBe(true)
    expect(store.totalPrice).toBe(0)
  })

  it('should set items directly', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.setItems([
      { id: '1', productId: 1, name: 'Shirt', price: 50, quantity: 2 },
      { id: '2', productId: 2, name: 'Pants', price: 100, quantity: 1 },
    ])

    expect(store.items).toHaveLength(2)
    expect(store.totalPrice).toBe(200)
  })

  it('hasItem should return true for existing item', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50, variantId: 10 })

    expect(store.hasItem(1, 10)).toBe(true)
  })

  it('hasItem should return false for non-existing item', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50 })

    expect(store.hasItem(99)).toBe(false)
  })

  it('should compute totalPrice correctly with multiple items', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 29.99, quantity: 2 })
    store.addItem({ productId: 2, name: 'Pants', price: 49.99, quantity: 1 })

    expect(store.totalPrice).toBeCloseTo(109.97)
    expect(store.totalItems).toBe(3)
  })

  it('should handle default quantity of 1', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 50 })

    expect(store.items[0].quantity).toBe(1)
    expect(store.totalItems).toBe(1)
  })

  it('mapCartApiItem should map API response correctly', async () => {
    const { mapCartApiItem } = await import('~/stores/cart')

    const result = mapCartApiItem({
      id: 42,
      product_id: 10,
      product_name: 'Shirt',
      price: 50,
      quantity: 3,
      product_image: 'https://example.com/img.jpg',
      product_slug: 'shirt',
      product_variant_id: 5,
    })

    expect(result).toEqual({
      id: '42',
      productId: 10,
      name: 'Shirt',
      price: 50,
      quantity: 3,
      image: 'https://example.com/img.jpg',
      slug: 'shirt',
      variantId: 5,
    })
  })

  it('mapCartApiItem should handle missing optional fields', async () => {
    const { mapCartApiItem } = await import('~/stores/cart')

    const result = mapCartApiItem({
      id: 1,
      product_id: 10,
      product_name: null,
      price: 25,
      quantity: 1,
    })

    expect(result.name).toBe('Product')
    expect(result.image).toBeUndefined()
    expect(result.slug).toBeUndefined()
    expect(result.variantId).toBeUndefined()
  })
})
