<template>
  <NuxtImg
    :src="src"
    :alt="alt"
    :width="width"
    :height="height"
    :sizes="sizes"
    :placeholder="placeholder"
    :loading="loading"
    :preload="preload"
    format="webp"
    quality="80"
    fit="cover"
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
  sizes?: string
  placeholder?: string
  loading?: 'lazy' | 'eager'
  preload?: boolean
}

withDefaults(defineProps<Props>(), {
  loading: 'lazy',
  sizes: 'sm:100vw md:50vw lg:33vw',
})

const loaded = ref(false)

function onLoad() {
  loaded.value = true
}
</script>
