<template>
  <div>
    <div id="image" data-carousel='{ "loadingClasses": "opacity-0" }' class="relative w-full">
      <div class="carousel max-h-96">
        <div class="carousel-body opacity-0">
          <!-- Slide 1 -->
          <div class="carousel-slide">
            <div class="flex h-full justify-center">
              <img src="https://cdn.flyonui.com/fy-assets/components/carousel/image-22.png"
                class="size-full object-cover" alt="game" />
            </div>
          </div>
          <!-- Slide 2 -->
          <div class="carousel-slide">
            <div class="flex h-full justify-center">
              <img src="https://cdn.flyonui.com/fy-assets/components/carousel/image-15.png"
                class="size-full object-cover" alt="vrbox" />
            </div>
          </div>
          <!-- Slide 3 -->
          <div class="carousel-slide">
            <div class="flex h-full justify-center">
              <img src="https://cdn.flyonui.com/fy-assets/components/carousel/image-16.png"
                class="size-full object-cover" alt="laptop" />
            </div>
          </div>
          <!-- Slide 4 -->
          <div class="carousel-slide">
            <div class="flex h-full justify-center">
              <img src="https://cdn.flyonui.com/fy-assets/components/carousel/image-8.png"
                class="size-full object-cover" alt="VRBox" />
            </div>
          </div>
          <!-- Slide 5 -->
          <div class="carousel-slide">
            <div class="flex h-full justify-center">
              <img src="https://cdn.flyonui.com/fy-assets/components/carousel/image-23.png"
                class="size-full object-cover" alt="iwatch" />
            </div>
          </div>
        </div>
      </div>
      <!-- Previous Slide -->
      <button type="button"
        class="carousel-prev start-5 max-sm:start-3 carousel-disabled:opacity-50 size-9.5 bg-base-100 flex items-center justify-center rounded-full shadow-base-300/20 shadow-sm">
        <span class="icon-[tabler--chevron-left] size-5 cursor-pointer"></span>
        <span class="sr-only">Previous</span>
      </button>
      <!-- Next Slide -->
      <button type="button"
        class="carousel-next end-5 max-sm:end-3 carousel-disabled:opacity-50 size-9.5 bg-base-100 flex items-center justify-center rounded-full shadow-base-300/20 shadow-sm">
        <span class="icon-[tabler--chevron-right] size-5"></span>
        <span class="sr-only">Next</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped></style>