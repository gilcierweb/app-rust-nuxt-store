<template>
  <img
    :src="imageSrc"
    :alt="alt"
    :width="width"
    :height="height"
    :loading="loading"
    class="transition-opacity duration-300"
    :class="{ 'opacity-0': !loaded, 'opacity-100': loaded }"
    @load="onLoad"
  />
</template>

<script setup lang="ts">
interface Props {
  src: string
  alt: string
  width?: number
  height?: number
  loading?: 'lazy' | 'eager'
}

const props = withDefaults(defineProps<Props>(), {
  loading: 'lazy',
})

const loaded = ref(false)

const imageSrc = computed(() => {
  // Se for path relativo do DummyJSON, construir URL completa
  if (props.src && !props.src.startsWith('http') && !props.src.startsWith('/')) {
    return `https://cdn.dummyjson.com/product-images/${props.src}`
  }
  return props.src
})

function onLoad() {
  loaded.value = true
}
</script>
