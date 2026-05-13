import type { WishlistItem } from '~/types'

export function useWishlist() {
  const { apiFetch } = useApi()
  const wishlist = useState<WishlistItem[]>('wishlist', () => [])
  const loading = ref(false)

  async function fetchWishlist() {
    try {
      const data = await apiFetch<WishlistItem[]>('/api/wishlists/list')
      wishlist.value = data
    } catch {
      wishlist.value = []
    }
  }

  async function toggleWishlist(productId: number) {
    const existing = wishlist.value.find(w => w.product_id === productId)
    if (existing) {
      try {
        await apiFetch(`/api/wishlists/remove?id=${existing.id}`)
        wishlist.value = wishlist.value.filter(w => w.id !== existing.id)
      } catch { /* ignore */ }
    } else {
      try {
        const item = await apiFetch<WishlistItem>(`/api/wishlists/add?product_id=${productId}`)
        wishlist.value.push(item)
      } catch { /* ignore */ }
    }
  }

  function isInWishlist(productId: number) {
    return wishlist.value.some(w => w.product_id === productId)
  }

  return { wishlist, loading, fetchWishlist, toggleWishlist, isInWishlist }
}
