import type { WishlistItem } from '~/types'

export function useWishlist() {
  const { apiFetch } = useApi()
  const { isAuthenticated } = useAuth()
  const wishlist = useState<WishlistItem[]>('wishlist', () => [])
  const loading = ref(false)

  function isAuthError(err: any): boolean {
    const statusCode = err?.statusCode ?? err?.response?.status ?? err?.data?.statusCode
    return statusCode === 401 || statusCode === 403
  }

  async function fetchWishlist() {
    if (!isAuthenticated.value) {
      wishlist.value = []
      return
    }

    loading.value = true
    try {
      const data = await apiFetch<WishlistItem[]>('/api/account/wishlist')
      wishlist.value = data
    } catch (err: any) {
      if (isAuthError(err)) {
        wishlist.value = []
        return
      }
      console.warn('[wishlist] fetchWishlist failed:', err)
      wishlist.value = []
    } finally {
      loading.value = false
    }
  }

  async function toggleWishlist(productId: number) {
    if (!isAuthenticated.value) {
      await navigateTo('/users/sessions')
      return
    }

    loading.value = true
    try {
      const existing = wishlist.value.find(w => w.product_id === productId)
      if (existing) {
        try {
          await apiFetch(`/api/account/wishlist/remove/${existing.id}`, { method: 'DELETE' })
          wishlist.value = wishlist.value.filter(w => w.id !== existing.id)
        } catch (err) {
          console.warn('[wishlist] remove failed:', err)
        }
      } else {
        try {
          const item = await apiFetch<WishlistItem>('/api/account/wishlist/add', {
            method: 'POST',
            body: { product_id: productId }
          })
          wishlist.value.push(item)
        } catch (err) {
          console.warn('[wishlist] add failed:', err)
        }
      }
    } finally {
      loading.value = false
    }
  }

  function isInWishlist(productId: number) {
    return wishlist.value.some(w => w.product_id === productId)
  }

  return { wishlist, loading, fetchWishlist, toggleWishlist, isInWishlist }
}
