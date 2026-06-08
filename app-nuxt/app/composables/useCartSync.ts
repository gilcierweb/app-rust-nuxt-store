import type { CartApiResponse, CartItem, ProductVariant } from '~/types'
import { mapCartApiItem } from '~/stores/cart'

function findKey(item: CartItem): string {
  return `${item.productId}-${item.variantId ?? ''}`
}

export function useCartSync() {
  const cartStore = useCartStore()
  const { apiFetch } = useApi()
  const { isAuthenticated } = useAuth()
  const syncing = ref(false)

  function isAuthError(err: any): boolean {
    const statusCode = err?.statusCode ?? err?.response?.status ?? err?.data?.statusCode
    return statusCode === 401 || statusCode === 403
  }

  async function fetchCart() {
    if (!isAuthenticated.value) return

    syncing.value = true
    try {
      const data = await apiFetch<CartApiResponse>('/api/carts/')
      cartStore.setItems(data.items.map(mapCartApiItem))
    } catch (err: any) {
      if (isAuthError(err)) {
        cartStore.setItems([])
      }
    } finally {
      syncing.value = false
    }
  }

  async function checkStock(productId: number, variantId?: number): Promise<{ available: boolean; maxQuantity: number }> {
    try {
      const variants = await apiFetch<ProductVariant[]>(`/api/variants/list?product_id=${productId}`)
      const variant = variantId
        ? variants.find(v => v.id === variantId)
        : variants[0]
      if (!variant) return { available: true, maxQuantity: 999 }
      if (variant.track_inventory === false) return { available: true, maxQuantity: 999 }
      if (variant.allow_backorder) return { available: true, maxQuantity: 999 }
      const qty = variant.inventory_quantity ?? 0
      const reserved = variant.reserved_quantity ?? 0
      const available = qty - reserved
      return { available: available > 0, maxQuantity: Math.max(0, available) }
    } catch {
      return { available: true, maxQuantity: 999 }
    }
  }

  async function addItemSync(item: { productId: number; name: string; price: number; image?: string; slug?: string; quantity?: number; variantId?: number }) {
    const quantity = item.quantity || 1

    cartStore.addItem(item)

    if (!isAuthenticated.value) return

    syncing.value = true
    try {
      const data = await apiFetch<CartApiResponse>('/api/carts/add_item', {
        method: 'POST',
        body: {
          product_id: item.productId,
          product_variant_id: item.variantId ?? null,
          quantity: quantity,
          price: String(Number(item.price)),
        },
      })
      cartStore.setItems(data.items.map(mapCartApiItem))
    } catch {
      // Local state remains; API sync failure is non-blocking
    } finally {
      syncing.value = false
    }
  }

  async function removeItemSync(productId: number) {
    const item = cartStore.items.find(i => i.productId === productId)
    if (!item) return

    cartStore.removeItem(productId)

    if (!isAuthenticated.value || !item.id) return

    syncing.value = true
    try {
      await apiFetch<CartApiResponse>(`/api/carts/remove_item/${item.id}`, {
        method: 'DELETE',
      })
    } catch {
      // non-blocking
    } finally {
      syncing.value = false
    }
  }

  async function updateQuantitySync(productId: number, quantity: number) {
    const item = cartStore.items.find(i => i.productId === productId)
    if (!item) return

    cartStore.updateQuantity(productId, quantity)

    if (!isAuthenticated.value || !item.id) return

    syncing.value = true
    try {
      await apiFetch<CartApiResponse>('/api/carts/update_item', {
        method: 'PUT',
        body: {
          item_id: Number(item.id),
          quantity,
        },
      })
    } catch {
      // non-blocking
    } finally {
      syncing.value = false
    }
  }

  async function clearCartSync() {
    cartStore.clearCart()

    if (!isAuthenticated.value) return

    syncing.value = true
    try {
      await apiFetch('/api/carts/clear', { method: 'DELETE' })
    } catch {
      // non-blocking
    } finally {
      syncing.value = false
    }
  }

  async function mergeCartOnLogin() {
    if (!isAuthenticated.value) return

    const localItems = [...cartStore.items]

    syncing.value = true
    try {
      let serverData = await apiFetch<CartApiResponse>('/api/carts/')

      if (localItems.length > 0) {
        const serverMap = new Map<string, CartItem>()
        for (const si of serverData.items.map(mapCartApiItem)) {
          serverMap.set(findKey(si), si)
        }

        for (const li of localItems) {
          const key = findKey(li)
          const existing = serverMap.get(key)
          if (existing) {
            if (li.quantity > existing.quantity) {
              await apiFetch('/api/carts/update_item', {
                method: 'PUT',
                body: { item_id: Number(existing.id), quantity: li.quantity },
              })
            }
          } else {
            await apiFetch('/api/carts/add_item', {
              method: 'POST',
              body: {
                product_id: li.productId,
                product_variant_id: li.variantId ?? null,
                quantity: li.quantity,
                price: String(li.price),
              },
            })
          }
        }

        serverData = await apiFetch<CartApiResponse>('/api/carts/')
      }

      cartStore.setItems(serverData.items.map(mapCartApiItem))
    } catch {
      // If merge fails, keep local items
    } finally {
      syncing.value = false
    }
  }

  return {
    syncing,
    fetchCart,
    addItemSync,
    removeItemSync,
    updateQuantitySync,
    clearCartSync,
    mergeCartOnLogin,
    checkStock,
  }
}
