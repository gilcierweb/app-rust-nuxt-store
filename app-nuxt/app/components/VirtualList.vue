<template>
  <div ref="containerRef" class="virtual-scroll-container">
    <div class="virtual-spacer" :style="{ height: `${totalHeight}px` }">
      <div
        v-for="item in visibleItems"
        :key="item.key"
        class="virtual-item"
        :style="{ transform: `translateY(${item.index * itemHeight}px)` }"
      >
        <slot :item="item.data" :index="item.index" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts" generic="T">
interface Props {
  items: T[]
  itemHeight: number
  buffer?: number
  containerHeight?: number
}

const props = withDefaults(defineProps<Props>(), {
  buffer: 5,
  containerHeight: 600,
})

const containerRef = ref<HTMLElement>()
const scrollTop = ref(0)

const totalHeight = computed(() => props.items.length * props.itemHeight)

const visibleRange = computed(() => {
  const start = Math.max(0, Math.floor(scrollTop.value / props.itemHeight) - props.buffer)
  const visibleCount = Math.ceil((props.containerHeight + props.itemHeight * props.buffer * 2) / props.itemHeight)
  const end = Math.min(props.items.length, start + visibleCount)
  return { start, end }
})

const visibleItems = computed(() => {
  const { start, end } = visibleRange.value
  return props.items.slice(start, end).map((data, idx) => ({
    key: start + idx,
    index: start + idx,
    data,
  }))
})

function onScroll() {
  scrollTop.value = containerRef.value?.scrollTop || 0
}

onMounted(() => {
  containerRef.value?.addEventListener('scroll', onScroll, { passive: true })
})

onUnmounted(() => {
  containerRef.value?.removeEventListener('scroll', onScroll)
})
</script>

<style scoped>
.virtual-scroll-container {
  height: v-bind(containerHeight + 'px');
  overflow: auto;
  will-change: transform;
}
.virtual-spacer {
  position: relative;
}
.virtual-item {
  position: absolute;
  left: 0;
  right: 0;
  will-change: transform;
}
</style>
