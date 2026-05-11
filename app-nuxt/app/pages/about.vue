<template>
  <div class="space-y-20 pb-20">
    <!-- Hero Section -->
    <section class="text-center max-w-4xl mx-auto pt-10">
      <span class="badge badge-primary badge-soft px-4 py-1.5 mb-6">About Our Journey</span>
      <h1 class="h1 mb-6 gradient-text">Elevating the E-commerce Experience</h1>
      <p class="text-xl text-base-content/60 leading-relaxed">
        We are more than just a store. We are a team of enthusiasts dedicated to bringing you the finest products combined with cutting-edge technology.
      </p>
    </section>

    <!-- Our Story Section -->
    <section class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
      <div class="relative">
        <div class="absolute -top-4 -left-4 size-24 bg-primary/20 rounded-full blur-2xl"></div>
        <div class="absolute -bottom-4 -right-4 size-24 bg-secondary/20 rounded-full blur-2xl"></div>
        <div class="aspect-video rounded-[2.5rem] overflow-hidden shadow-2xl bg-base-300">
           <!-- I will assume there's a way to show a nice image here -->
           <div class="flex items-center justify-center h-full bg-gradient-to-br from-base-300 to-base-200">
             <span class="icon-[tabler--building-store] size-20 text-base-content/10"></span>
           </div>
        </div>
      </div>
      <div>
        <h2 class="h2 mb-6">Our Story</h2>
        <div class="space-y-4 text-base-content/70 leading-relaxed">
          <p>
            Founded in 2024, App Rust Nuxt Store began with a simple idea: to create a seamless shopping experience that combines the performance of Rust with the flexibility of Nuxt.
          </p>
          <p>
            What started as a small project has grown into a full-scale platform serving customers worldwide. We believe in quality, transparency, and innovation.
          </p>
          <div class="pt-6 grid grid-cols-2 gap-6">
            <div>
              <div class="text-3xl font-bold text-primary mb-1">10k+</div>
              <div class="text-sm font-medium opacity-60">Happy Customers</div>
            </div>
            <div>
              <div class="text-3xl font-bold text-secondary mb-1">500+</div>
              <div class="text-sm font-medium opacity-60">Premium Products</div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Mission & Vision -->
    <section class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <div class="card bg-base-200/50 p-10 rounded-[2rem] hover-lift">
        <div class="size-14 rounded-2xl bg-primary/10 flex items-center justify-center mb-6">
          <span class="icon-[tabler--target-arrow] size-7 text-primary"></span>
        </div>
        <h3 class="h3 mb-4">Our Mission</h3>
        <p class="text-base-content/60 leading-relaxed">
          To provide an unparalleled shopping experience by curating the best products and delivering them with speed and security.
        </p>
      </div>
      <div class="card bg-base-200/50 p-10 rounded-[2rem] hover-lift">
        <div class="size-14 rounded-2xl bg-secondary/10 flex items-center justify-center mb-6">
          <span class="icon-[tabler--eye] size-7 text-secondary"></span>
        </div>
        <h3 class="h3 mb-4">Our Vision</h3>
        <p class="text-base-content/60 leading-relaxed">
          To become the leading e-commerce platform recognized for its technological excellence and customer-centric approach.
        </p>
      </div>
    </section>

    <!-- Core Values -->
    <section class="text-center py-10">
      <h2 class="h2 mb-12">Core Values</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8">
        <div v-for="value in values" :key="value.title" class="space-y-4">
          <div class="size-16 rounded-full bg-white shadow-md mx-auto flex items-center justify-center">
            <span :class="[value.icon, 'size-8 text-primary']"></span>
          </div>
          <h4 class="font-bold">{{ value.title }}</h4>
          <p class="text-sm text-base-content/50 px-4">{{ value.description }}</p>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="bg-base-200 rounded-[2.5rem] p-12 text-center">
      <h2 class="h3 mb-6">Want to know more about us?</h2>
      <p class="mb-8 text-base-content/60">We're always open for questions and partnerships.</p>
      <div class="flex flex-wrap justify-center gap-4">
        <NuxtLink to="/contact" class="btn btn-primary px-8">Contact Us</NuxtLink>
        <NuxtLink to="/products" class="btn btn-ghost px-8">Browse Products</NuxtLink>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
useSeoMeta({
  title: t('pages.about.title'),
  ogTitle: t('pages.about.title'),
  description: t('pages.about.description'),
  ogDescription: t('pages.about.description'),
})

const values = [
  { title: 'Quality First', icon: 'icon-[tabler--award]', description: 'We never compromise on the quality of our products.' },
  { title: 'Innovation', icon: 'icon-[tabler--bulb]', description: 'Constantly improving our tech for better performance.' },
  { title: 'Customer Love', icon: 'icon-[tabler--heart]', description: 'Your satisfaction is our primary goal.' },
  { title: 'Integrity', icon: 'icon-[tabler--shield-check]', description: 'Transparent and honest in every transaction.' }
]
</script>