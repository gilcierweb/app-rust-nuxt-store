<template>
  <div class="bg-base-200 flex min-h-screen flex-col">
    <AccountNavbarTop />
    <AccountSidebar />

    <div class="lg:ps-64 flex grow flex-col transition-all duration-300">
      <main class="mx-auto w-full max-w-7xl flex-1 grow space-y-6 p-4 md:p-6">
        <slot />
      </main>

      <AppFooterGilcierweb />
    </div>
  </div>
</template>

<script setup lang="ts">
// Authenticated customer account layout.
</script>
