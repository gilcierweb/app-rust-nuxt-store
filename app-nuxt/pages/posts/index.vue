<template>
  <div>
    <h1>Page Posts</h1>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-4 gap-4 mt-6">

      <div v-if="status === 'pending'">
        Loading ... <br>
        <span class="loading loading-spinner  size-32"></span>
      </div>
      <div v-else v-for="post in posts" :key="post">
        <div class="card sm:max-w-sm">
          <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
          <div class="card-body space-y-3">
            <h5 class="card-title">{{ post.title }}</h5>
            <p>{{ post.content }}</p>
            <div class="card-actions">
              <NuxtLink :to="`/posts/${post.id}`" class="btn btn-primary">Read More</NuxtLink>
              <button class="btn btn-secondary btn-soft"> Add to cart</button>
            </div>
          </div>
        </div>
      </div>

      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>
      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>
      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>
      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>
      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>
      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>
      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>

      <div class="card sm:max-w-sm">
        <figure><img src="https://cdn.flyonui.com/fy-assets/components/card/image-7.png" alt="headphone" /></figure>
        <div class="card-body space-y-3">
          <h5 class="card-title">Airpods Max</h5>
          <p>A perfect balance of exhilarating high-fidelity audio and the effortless magic of Airpods.</p>
          <div class="card-actions">
            <button class="btn btn-primary">Buy Now</button>
            <button class="btn btn-secondary btn-soft">Add to cart</button>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig();

const { status, data: posts } = await useLazyFetch(`${config.public.baseURL}/api/posts`)
watch(posts, (newPosts) => {
  // Because posts might start out null, you won't have access
  // to its contents immediately, but you can watch it.
})
</script>

<style scoped></style>
