import { defineStore } from 'pinia'
import type { CartApiItem, CartItem } from '~/types'

export const useCartStore = defineStore('cart', {
  state: () => ({
    items: [] as CartItem[],
  }),
  getters: {
    totalItems: (state) => state.items.reduce((sum, item) => sum + item.quantity, 0),
    totalPrice: (state) => state.items.reduce((sum, item) => sum + item.price * item.quantity, 0),
    isEmpty: (state) => state.items.length === 0,
  },
  actions: {
    addItem(item: { productId: number; name: string; price: number; image?: string; slug?: string; quantity?: number; variantId?: number }) {
      const existing = this.items.find(i => i.productId === item.productId && i.variantId === item.variantId)
      if (existing) {
        existing.quantity += item.quantity || 1
      } else {
        this.items.push({
          id: '',
          productId: item.productId,
          name: item.name,
          price: Number(item.price),
          quantity: item.quantity || 1,
          image: item.image,
          slug: item.slug,
          variantId: item.variantId,
        })
      }
    },
    removeItem(productId: number) {
      this.items = this.items.filter(i => i.productId !== productId)
    },
    updateQuantity(productId: number, quantity: number) {
      const item = this.items.find(i => i.productId === productId)
      if (item) {
        if (quantity <= 0) {
          this.removeItem(productId)
        } else {
          item.quantity = quantity
        }
      }
    },
    clearCart() {
      this.items = []
    },
    setItems(items: CartItem[]) {
      this.items = items
    },
    hasItem(productId: number, variantId?: number): boolean {
      return this.items.some(i => i.productId === productId && i.variantId === (variantId ?? i.variantId))
    },
  },
  persist: true,
})

export function mapCartApiItem(item: CartApiItem): CartItem {
  return {
    id: String(item.id),
    productId: item.product_id,
    name: item.product_name ?? 'Product',
    price: Number(item.price),
    quantity: item.quantity,
    image: item.product_image ?? undefined,
    slug: item.product_slug ?? undefined,
    variantId: item.product_variant_id ?? undefined,
  }
}
