export interface KpiStat {
  title: string
  value: string
  trend: string
  trendUp: boolean
  icon: string
  colorClass: string
  textClass: string
}

export interface SalesDataPoint {
  date: string
  sales: number
  orders: number
}

export interface CategoryDataPoint {
  name: string
  value: number
}

export interface TopProduct {
  name: string
  sales: number
}

export interface RecentOrder {
  id: number
  customer: string
  total: number
  status: string
  statusLabel: string
  statusClass: string
}

export interface DashboardStats {
  kpiStats: KpiStat[]
  salesData: SalesDataPoint[]
  categoryData: CategoryDataPoint[]
  topProducts: TopProduct[]
  recentOrders: RecentOrder[]
}
