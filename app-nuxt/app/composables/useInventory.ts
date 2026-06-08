import type { InventoryListResponse, StockAdjustment, StockReservation } from '~/types'

export function useInventory() {
  const { apiFetch } = useApi()

  async function listVariants(params: {
    page?: number
    page_size?: number
    search?: string
    status?: string
    low_stock_threshold?: number
  } = {}) {
    const query: Record<string, unknown> = {}
    if (params.page) query.page = params.page
    if (params.page_size) query.page_size = params.page_size
    if (params.search) query.search = params.search
    if (params.status) query.status = params.status
    if (params.low_stock_threshold != null) query.low_stock_threshold = params.low_stock_threshold
    return apiFetch<InventoryListResponse>('/api/admin/inventory', { query })
  }

  async function updateQuantity(variantId: number, inventoryQuantity: number) {
    return apiFetch(`/api/admin/inventory/${variantId}`, {
      method: 'PATCH',
      body: { inventory_quantity: inventoryQuantity },
    })
  }

  async function adjustStock(payload: {
    variant_id: number
    quantity: number
    type: string
    reason?: string
    reference?: string
  }) {
    return apiFetch('/api/admin/inventory/stock-adjustments', {
      method: 'POST',
      body: payload,
    })
  }

  async function getLogs(params: { variant_id?: number; page?: number; page_size?: number } = {}) {
    const query: Record<string, unknown> = {}
    if (params.variant_id) query.variant_id = params.variant_id
    if (params.page) query.page = params.page
    if (params.page_size) query.page_size = params.page_size
    return apiFetch<{ items: StockAdjustment[]; total: number }>('/api/admin/inventory/logs', { query })
  }

  async function getReservations(params: { status?: string; page?: number; page_size?: number } = {}) {
    const query: Record<string, unknown> = {}
    if (params.status) query.status = params.status
    if (params.page) query.page = params.page
    if (params.page_size) query.page_size = params.page_size
    return apiFetch<{ items: StockReservation[]; total: number }>('/api/admin/inventory/reservations', { query })
  }

  async function expireReservations() {
    return apiFetch('/api/admin/inventory/reservations/expire', { method: 'POST' })
  }

  return {
    listVariants,
    updateQuantity,
    adjustStock,
    getLogs,
    getReservations,
    expireReservations,
  }
}
