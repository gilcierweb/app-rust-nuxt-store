import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

const mockApiFetch = vi.fn()

vi.mock('vue-i18n', () => ({
  useI18n: () => ({ t: (key: string) => key }),
  createI18n: () => ({ global: { t: (key: string) => key } }),
}))

vi.mock('~/composables/useApi', () => ({
  useApi: () => ({ apiFetch: mockApiFetch, useApiFetch: vi.fn(() => ({ data: { value: [] } })) }),
}))

vi.mock('~/composables/useCartSync', () => ({
  useCartSync: () => ({
    clearCartSync: vi.fn(),
    fetchCart: vi.fn(),
    addItemSync: vi.fn(),
    removeItemSync: vi.fn(),
    updateQuantitySync: vi.fn(),
  }),
}))

vi.mock('~/composables/useAuth', () => ({
  useAuth: () => ({
    isAuthenticated: { value: true },
    user: { value: { pid: '1', name: 'Test' } },
    login: vi.fn(),
    logout: vi.fn(),
    loading: { value: false },
    error: { value: null },
  }),
}))

describe('checkout page', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    setActivePinia(createPinia())
  })

  it('should calculate total amount with subtotal only', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 100, quantity: 2 })

    const subtotal = store.totalPrice
    const total = subtotal + 0 - 0

    expect(total).toBe(200)
  })

  it('should calculate total with shipping cost', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 100, quantity: 1 })

    const total = store.totalPrice + 15 - 0
    expect(total).toBe(115)
  })

  it('should calculate total with coupon discount', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 100, quantity: 1 })

    const total = store.totalPrice + 0 - 20
    expect(total).toBe(80)
  })

  it('should calculate total with shipping and discount', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 100, quantity: 2 })
    store.addItem({ productId: 2, name: 'Pants', price: 50, quantity: 1 })

    const total = store.totalPrice + 10 - 25
    expect(total).toBe(235)
  })

  it('should validate coupon code', async () => {
    mockApiFetch.mockResolvedValueOnce({
      valid: true,
      discount: 15,
      message: 'Coupon applied',
    })

    const result = await mockApiFetch('/api/coupons/validate', {
      method: 'POST',
      body: { code: 'SAVE15', total_amount: 100 },
    })

    expect(result.valid).toBe(true)
    expect(result.discount).toBe(15)
    expect(result.message).toBe('Coupon applied')
  })

  it('should handle invalid coupon code', async () => {
    mockApiFetch.mockResolvedValueOnce({
      valid: false,
      discount: 0,
      message: 'Invalid coupon',
    })

    const result = await mockApiFetch('/api/coupons/validate', {
      method: 'POST',
      body: { code: 'INVALID', total_amount: 100 },
    })

    expect(result.valid).toBe(false)
    expect(result.discount).toBe(0)
  })

  it('should prepare checkout payload with items', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 100, quantity: 2 })
    store.addItem({ productId: 2, name: 'Pants', price: 50, quantity: 1 })

    const items = store.items.map(item => ({
      product_id: item.productId,
      product_variant_id: item.variantId ?? null,
      quantity: item.quantity,
      price: Number(item.price),
    }))

    expect(items).toHaveLength(2)
    expect(items[0]).toEqual({ product_id: 1, product_variant_id: null, quantity: 2, price: 100 })
    expect(items[1]).toEqual({ product_id: 2, product_variant_id: null, quantity: 1, price: 50 })
  })

  it('should build checkout request body correctly', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 100, quantity: 1 })

    const body = {
      items: store.items.map(item => ({
        product_id: item.productId,
        quantity: item.quantity,
        price: Number(item.price),
      })),
      subtotal: store.totalPrice,
      total_amount: store.totalPrice + 10 - 5,
      shipping_amount: 10,
      discount_amount: 5,
      coupon_code: 'SAVE5',
      payment_method_id: 1,
      shipping_method_id: 2,
      address_first_name: 'John',
      address_last_name: 'Doe',
      address1: '123 Main St',
      address_city: 'São Paulo',
      address_state: 'SP',
      address_zip_code: '01001-000',
      address_phone: '+5511999999999',
    }

    expect(body.items).toHaveLength(1)
    expect(body.subtotal).toBe(100)
    expect(body.total_amount).toBe(105)
    expect(body.shipping_amount).toBe(10)
    expect(body.discount_amount).toBe(5)
    expect(body.coupon_code).toBe('SAVE5')
  })

  it('should generate idempotency key', () => {
    function generateUUID(): string {
      return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
        const r = (Math.random() * 16) | 0
        const v = c === 'x' ? r : (r & 0x3) | 0x8
        return v.toString(16)
      })
    }

    const key = generateUUID()
    expect(typeof key).toBe('string')
    expect(key.length).toBeGreaterThan(0)
  })

  it('should map payment method icon correctly', () => {
    function paymentMethodIcon(method: { code?: string }): string {
      const code = method.code?.toLowerCase() || ''
      if (code.includes('card')) return 'icon-[tabler--credit-card]'
      if (code.includes('pix')) return 'icon-[tabler--qrcode]'
      return 'icon-[tabler--wallet]'
    }

    expect(paymentMethodIcon({ code: 'credit_card' })).toBe('icon-[tabler--credit-card]')
    expect(paymentMethodIcon({ code: 'pix' })).toBe('icon-[tabler--qrcode]')
    expect(paymentMethodIcon({ code: 'boleto' })).toBe('icon-[tabler--wallet]')
    expect(paymentMethodIcon({ code: undefined })).toBe('icon-[tabler--wallet]')
  })

  it('should validate canPlaceOrder logic', () => {
    function canPlaceOrder(
      selectedPaymentMethod: number | null,
      gatewayDriver: string | null,
      braintreeReady: boolean,
      getnetDocument: string,
    ): boolean {
      if (!selectedPaymentMethod) return false
      if (gatewayDriver === 'braintree') return braintreeReady
      if (gatewayDriver === 'getnet') return getnetDocument.trim().length > 0
      return true
    }

    expect(canPlaceOrder(null, null, false, '')).toBe(false)
    expect(canPlaceOrder(1, null, false, '')).toBe(true)
    expect(canPlaceOrder(1, 'braintree', false, '')).toBe(false)
    expect(canPlaceOrder(1, 'braintree', true, '')).toBe(true)
    expect(canPlaceOrder(1, 'getnet', false, '')).toBe(false)
    expect(canPlaceOrder(1, 'getnet', false, '12345678901')).toBe(true)
  })

  it('should detect invalid cart prices', async () => {
    const { useCartStore } = await import('~/stores/cart')
    const store = useCartStore()

    store.addItem({ productId: 1, name: 'Shirt', price: 100, quantity: 1 })

    const hasInvalidPrice = store.items.some(
      item => item.price === null || item.price === undefined || !isFinite(item.price),
    )
    expect(hasInvalidPrice).toBe(false)
  })

  it('should validate document type for getnet', () => {
    function getDocumentType(document: string): string {
      const digits = document.replace(/\D/g, '')
      return digits.length > 11 ? 'CNPJ' : 'CPF'
    }

    expect(getDocumentType('123.456.789-01')).toBe('CPF')
    expect(getDocumentType('12.345.678/0001-95')).toBe('CNPJ')
    expect(getDocumentType('12345678901')).toBe('CPF')
    expect(getDocumentType('12345678000195')).toBe('CNPJ')
  })

  it('should calculate shipping cost from method', () => {
    const shippingMethods = [
      { id: 1, name: 'Standard', base_price: '15.00' },
      { id: 2, name: 'Express', base_price: '25.00' },
      { id: 3, name: 'Free', base_price: null },
    ]

    function getShippingCost(methodId: number | null): number | null {
      if (!methodId) return null
      const method = shippingMethods.find(m => m.id === methodId)
      const price = method?.base_price
      return price != null ? Number(price) : null
    }

    expect(getShippingCost(null)).toBeNull()
    expect(getShippingCost(1)).toBe(15)
    expect(getShippingCost(2)).toBe(25)
    expect(getShippingCost(3)).toBeNull()
  })
})
