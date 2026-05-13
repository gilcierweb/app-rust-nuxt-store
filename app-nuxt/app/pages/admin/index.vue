<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="h1">{{ t('admin.dashboard.title') }}</h1>
        <p class="text-gray-500">{{ t('admin.dashboard.welcome') }}</p>
      </div>
      <div class="flex gap-2">
        <button class="btn btn-soft btn-sm">
          <i class="icon-[tabler--calendar] size-4 mr-2"></i>
          {{ t('admin.dashboard.last30Days') }}
        </button>
        <button class="btn btn-primary btn-sm">
          <i class="icon-[tabler--download] size-4 mr-2"></i>
          {{ t('admin.dashboard.report') }}
        </button>
      </div>
    </div>

    <!-- KPI Stats -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <div v-for="stat in kpiStats" :key="stat.title" class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4 flex flex-row items-center gap-4">
          <div :class="['size-12 rounded-full flex items-center justify-center bg-opacity-10', stat.colorClass]">
            <i :class="[stat.icon, 'size-6', stat.textClass]"></i>
          </div>
          <div>
            <p class="text-sm text-gray-500">{{ t(stat.title) }}</p>
            <div class="flex items-baseline gap-2">
              <h3 class="text-2xl font-bold">{{ stat.value }}</h3>
              <span :class="['text-xs font-medium', stat.trendUp ? 'text-success' : 'text-error']">
                {{ stat.trendUp ? '↑' : '↓' }} {{ stat.trend }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Charts Row 1: Sales Trends & Category Distribution -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Sales Chart -->
      <div class="lg:col-span-2 card bg-base-100 shadow-sm border">
        <div class="card-body p-4">
          <div class="flex justify-between items-center mb-6">
            <h3 class="font-bold text-lg">{{ t('admin.dashboard.charts.salesEvolution') }}</h3>
            <div class="flex gap-1">
              <button class="btn btn-xs btn-ghost text-primary">{{ t('admin.dashboard.charts.daily') }}</button>
              <button class="btn btn-xs btn-ghost">{{ t('admin.dashboard.charts.monthly') }}</button>
            </div>
          </div>
          <div class="h-80 w-full">
            <ClientOnly>
              <AreaChart 
                :data="salesData"
                :categories="salesCategories"
                :y-axis="['sales', 'orders']"
                :height="300"
                :x-formatter="salesXFormatter"
                :y-grid-line="true"
                :show-tooltip="true"
                :legend-position="LegendPosition.TopRight"
                :curve-type="CurveType.MonotoneX"
              />
            </ClientOnly>
          </div>
        </div>
      </div>

      <!-- Donut Chart -->
      <div class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4 text-center">
          <h3 class="font-bold text-lg mb-6 text-left">{{ t('admin.dashboard.charts.salesByCategory') }}</h3>
          <div class="h-64 w-full relative flex items-center justify-center">
            <ClientOnly>
              <DonutChart 
                :data="categoryData.map(d => d.value)"
                :categories="donutCategories"
                :height="260"
                :radius="4"
                :arc-width="20"
                :pad-angle="0.1"
                :show-tooltip="true"
              />
            </ClientOnly>
            <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
              <span class="text-3xl font-bold">1.2k</span>
              <span class="text-xs text-gray-400">{{ t('admin.dashboard.charts.totalSales') }}</span>
            </div>
          </div>
          <div class="mt-4 space-y-2">
            <div v-for="(cat, index) in categoryData" :key="cat.name" class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="size-2 rounded-full" :style="{ backgroundColor: ['#FF6F00', '#FF9F40', '#FFCD56', '#4BC0C0', '#9966FF'][index] }"></span>
                <span class="text-xs text-gray-600">{{ cat.name }}</span>
              </div>
              <span class="text-xs font-bold">{{ cat.value }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Charts Row 2: Top Products & Recent Orders -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
       <!-- Top Products Bar Chart -->
       <div class="card bg-base-100 shadow-sm border">
        <div class="card-body p-4">
          <h3 class="font-bold text-lg mb-6">{{ t('admin.dashboard.charts.topProducts') }}</h3>
          <div class="h-80 w-full">
            <ClientOnly>
              <BarChart 
                :data="topProducts"
                :categories="productCategories"
                :y-axis="['sales']"
                :height="300"
                :x-formatter="productXFormatter"
                :y-grid-line="true"
                :show-tooltip="true"
                :radius="4"
              />
            </ClientOnly>
          </div>
        </div>
      </div>

      <!-- Recent Orders Table -->
      <div class="card bg-base-100 shadow-sm border overflow-hidden">
        <div class="card-body p-0">
          <div class="p-4 flex justify-between items-center border-b bg-base-200/30">
            <h3 class="font-bold text-lg">{{ t('admin.dashboard.charts.recentOrders') }}</h3>
            <NuxtLinkLocale to="/admin/orders" class="btn btn-link btn-sm text-primary no-underline">{{ t('admin.dashboard.charts.viewAll') }}</NuxtLinkLocale>
          </div>
          <div class="overflow-x-auto">
            <table class="table table-sm">
              <thead class="bg-base-200/50">
                <tr>
                  <th>{{ t('admin.account.fields.id') }}</th>
                  <th>{{ t('admin.products.table.name') }}</th>
                  <th>{{ t('common.table.total') }}</th>
                  <th>{{ t('common.table.status') }}</th>
                  <th class="text-right">{{ t('common.table.actions') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="order in recentOrders" :key="order.id" class="row-hover">
                  <td class="font-mono text-xs text-primary font-bold">#{{ order.id }}</td>
                  <td>
                    <div class="text-sm font-medium">{{ order.customer }}</div>
                  </td>
                  <td class="font-bold">{{ formatNumberBR(order.total) }}</td>
                  <td>
                    <span :class="['badge badge-soft text-[10px] h-5', order.statusClass]">
                      {{ order.statusLabel }}
                    </span>
                  </td>
                  <td class="text-right">
                    <button class="btn btn-circle btn-text btn-xs">
                      <i class="icon-[tabler--eye] size-4"></i>
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { LegendPosition, CurveType } from 'vue-chrts'

definePageMeta({
  layout: 'admin'
})

const { t } = useI18n()

// KPI Data
const kpiStats = [
  { 
    title: 'admin.dashboard.kpis.monthlyRevenue', 
    value: formatNumberBR(45280), 
    trend: '12%', 
    trendUp: true, 
    icon: 'icon-[tabler--cash]', 
    colorClass: 'bg-success', 
    textClass: 'text-success' 
  },
  { 
    title: 'admin.dashboard.kpis.newOrders', 
    value: '342', 
    trend: '8%', 
    trendUp: true, 
    icon: 'icon-[tabler--package]', 
    colorClass: 'bg-primary', 
    textClass: 'text-primary' 
  },
  { 
    title: 'admin.dashboard.kpis.newCustomers', 
    value: '84', 
    trend: '3%', 
    trendUp: false, 
    icon: 'icon-[tabler--users]', 
    colorClass: 'bg-info', 
    textClass: 'text-info' 
  },
  { 
    title: 'admin.dashboard.kpis.conversionRate', 
    value: '3.42%', 
    trend: '0.5%', 
    trendUp: true, 
    icon: 'icon-[tabler--chart-pie]', 
    colorClass: 'bg-warning', 
    textClass: 'text-warning' 
  }
]

// Charts Data
const salesData = [
  { date: '01/05', sales: 1200, orders: 15 },
  { date: '02/05', sales: 1900, orders: 22 },
  { date: '03/05', sales: 1500, orders: 18 },
  { date: '04/05', sales: 2500, orders: 30 },
  { date: '05/05', sales: 2100, orders: 25 },
  { date: '06/05', sales: 3200, orders: 40 },
  { date: '07/05', sales: 2800, orders: 35 },
]

const salesCategories = {
  sales: { name: t('admin.dashboard.kpis.monthlyRevenue'), color: '#FF6F00' },
  orders: { name: t('admin.dashboard.kpis.newOrders'), color: '#3B82F6' }
}

const salesXFormatter = (tick: number) => {
  return salesData[tick]?.date || ''
}

const categoryData = [
  { name: t('admin.dashboard.categories.electronics'), value: 400 },
  { name: t('admin.dashboard.categories.clothing'), value: 300 },
  { name: t('admin.dashboard.categories.home'), value: 200 },
  { name: t('admin.dashboard.categories.beauty'), value: 150 },
  { name: t('admin.dashboard.categories.others'), value: 100 },
]

const donutCategories = {
  [t('admin.dashboard.categories.electronics')]: { name: t('admin.dashboard.categories.electronics'), color: '#FF6F00' },
  [t('admin.dashboard.categories.clothing')]: { name: t('admin.dashboard.categories.clothing'), color: '#FF9F40' },
  [t('admin.dashboard.categories.home')]: { name: t('admin.dashboard.categories.home'), color: '#FFCD56' },
  [t('admin.dashboard.categories.beauty')]: { name: t('admin.dashboard.categories.beauty'), color: '#4BC0C0' },
  [t('admin.dashboard.categories.others')]: { name: t('admin.dashboard.categories.others'), color: '#9966FF' }
}

const topProducts = [
  { name: 'iPhone 15 Pro', sales: 45 },
  { name: 'MacBook Air M3', sales: 32 },
  { name: 'AirPods Pro 2', sales: 28 },
  { name: 'Samsung S24', sales: 24 },
  { name: 'iPad Air', sales: 18 },
]

const productCategories = {
  sales: { name: t('admin.products.table.actions'), color: '#3B82F6' } // Unidades Vendidas
}

const productXFormatter = (i: number) => {
  return topProducts[i]?.name || ''
}

// Recent Orders
const recentOrders = [
  { id: 1042, customer: 'Gilcier Junior', total: 1250.00, status: 'paid', statusLabel: t('admin.statusLabels.paid'), statusClass: 'badge-success' },
  { id: 1041, customer: 'Maria Silva', total: 450.20, status: 'pending', statusLabel: t('admin.statusLabels.pending'), statusClass: 'badge-warning' },
  { id: 1040, customer: 'João Oliveira', total: 1040, status: 'cancelled', statusLabel: t('admin.statusLabels.cancelled'), statusClass: 'badge-error' },
  { id: 1039, customer: 'Ana Costa', total: 2100.00, status: 'paid', statusLabel: t('admin.statusLabels.paid'), statusClass: 'badge-success' },
  { id: 1038, customer: 'Carlos Pereira', total: 120.00, status: 'paid', statusLabel: t('admin.statusLabels.paid'), statusClass: 'badge-success' },
]
</script>

<style scoped>
:deep(.unovis-axis) {
  --vis-axis-grid-color: #f3f4f6;
  --vis-axis-label-color: #6b7280;
  --vis-axis-tick-label-font-size: 10px;
}
</style>