<template>
  <div class="rounded-box shadow-base-300/10 bg-base-100 w-full shadow-md">
    <div ref="tableContainerRef" class="overflow-x-auto" :style="{ maxHeight, overflowY: 'auto' }">
      <table class="table w-full">
        <thead class="sticky top-0 z-10 bg-base-100">
          <tr v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
            <th
              v-for="header in headerGroup.headers"
              :key="header.id"
              :colSpan="header.colSpan"
              :style="{ width: header.getSize() !== 150 ? `${header.getSize()}px` : undefined }"
            >
              <FlexRender
                v-if="!header.isPlaceholder"
                :render="header.column.columnDef.header"
                :props="header.getContext()"
              />
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in table.getRowModel().rows" :key="row.id">
            <td v-for="cell in row.getVisibleCells()" :key="cell.id">
              <FlexRender
                :render="cell.column.columnDef.cell"
                :props="cell.getContext()"
              />
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="border-base-content/10 flex flex-wrap items-center justify-between gap-3 px-6 py-4 border-t">
      <span class="text-xs text-base-content/50">
        {{ paginationSummary }}
      </span>

      <div class="flex flex-wrap items-center gap-2">
        <div class="join">
          <button
            type="button"
            class="join-item btn btn-sm btn-outline"
            :disabled="!table.getCanPreviousPage()"
            @click="table.setPageIndex(0)"
          >
            «
          </button>
          <button
            type="button"
            class="join-item btn btn-sm btn-outline"
            :disabled="!table.getCanPreviousPage()"
            @click="table.previousPage()"
          >
            ‹
          </button>
          <button type="button" class="join-item btn btn-sm btn-active font-mono min-w-12">
            {{ table.getState().pagination.pageIndex + 1 }}
          </button>
          <button
            type="button"
            class="join-item btn btn-sm btn-outline"
            :disabled="!table.getCanNextPage()"
            @click="table.nextPage()"
          >
            ›
          </button>
          <button
            type="button"
            class="join-item btn btn-sm btn-outline"
            :disabled="!table.getCanNextPage()"
            @click="table.setPageIndex(table.getPageCount() - 1)"
          >
            »
          </button>
        </div>

        <span class="text-xs text-base-content/60 whitespace-nowrap">
          {{ t('admin.pagination.pageOf', { page: table.getState().pagination.pageIndex + 1, total: table.getPageCount() }) }}
        </span>

        <div class="flex items-center gap-1">
          <span class="text-xs text-base-content/60">{{ t('admin.pagination.goTo') }}</span>
          <input
            type="number"
            :value="goToPageNumber"
            @change="handleGoToPage"
            min="1"
            :max="table.getPageCount() || 1"
            class="input input-bordered input-sm w-16 text-center"
          />
        </div>

        <select
          :value="table.getState().pagination.pageSize"
          @change="handlePageSizeChange"
          class="select select-bordered select-sm"
        >
          <option v-for="size in pageSizeOptions" :key="size" :value="size">
            {{ t('admin.pagination.show', { size }) }}
          </option>
        </select>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts" generic="TData">
import {
  FlexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useVueTable,
} from '@tanstack/vue-table'
import { useI18n } from '#imports'

interface Props<T> {
  data: T[]
  columns: any[]
  total?: number
  pageIndex?: number
  pageSize?: number
  maxHeight?: string
}

const props = withDefaults(defineProps<Props<TData>>(), {
  pageIndex: 0,
  pageSize: 20,
  maxHeight: '600px',
})

const emit = defineEmits<{
  'update:pageIndex': [value: number]
  'update:pageSize': [value: number]
}>()

const { t } = useI18n()

const isManualPagination = computed(() => props.total !== undefined)

const pageCount = computed(() => {
  if (props.total === undefined) return undefined
  return Math.ceil(props.total / props.pageSize)
})

const table = useVueTable({
  get data() { return props.data },
  get columns() { return props.columns },
  get pageCount() { return pageCount.value ?? -1 },
  get state() {
    if (isManualPagination.value) {
      return { pagination: { pageIndex: props.pageIndex, pageSize: props.pageSize } }
    }
  },
  onPaginationChange: (updater) => {
    const next = typeof updater === 'function'
      ? updater({ pageIndex: props.pageIndex, pageSize: props.pageSize })
      : updater
    if (next.pageIndex !== props.pageIndex) {
      emit('update:pageIndex', next.pageIndex)
    }
    if (next.pageSize !== props.pageSize) {
      emit('update:pageSize', next.pageSize)
    }
  },
  manualPagination: isManualPagination.value,
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
})

const pageSizeOptions = [10, 20, 30, 50, 100]

const goToPageNumber = ref(1)

watch(() => table.getState().pagination.pageIndex, (idx) => {
  goToPageNumber.value = idx + 1
})

const handleGoToPage = (e: Event) => {
  const target = e.target as HTMLInputElement
  const page = target.value ? Number(target.value) : 1
  const clamped = Math.max(1, Math.min(page, table.getPageCount() || 1))
  goToPageNumber.value = clamped
  table.setPageIndex(clamped - 1)
}

const handlePageSizeChange = (e: Event) => {
  table.setPageSize(Number((e.target as HTMLSelectElement).value))
}

const paginationSummary = computed(() => {
  const { pageIndex, pageSize } = table.getState().pagination
  const rowCount = table.getRowModel().rows.length
  if (rowCount === 0) return t('admin.pagination.empty')
  const start = pageIndex * pageSize + 1
  const end = start + rowCount - 1
  const total = props.total ?? table.getCoreRowModel().rows.length
  return t('admin.pagination.showing', { start, end, total })
})
</script>
