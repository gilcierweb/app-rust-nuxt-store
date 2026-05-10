import { defineStore } from 'pinia'
import type { CartItem } from '~/types'

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
    addItem(item: { productId: number; name: string; price: number; image?: string; slug?: string; quantity?: number }) {
      const existing = this.items.find(i => i.productId === item.productId)
      if (existing) {
        existing.quantity += item.quantity || 1
      } else {
        this.items.push({
          id: Date.now().toString(),
          productId: item.productId,
          name: item.name,
          price: item.price,
          quantity: item.quantity || 1,
          image: item.image,
          slug: item.slug,
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
  },
  persist: true,
})
