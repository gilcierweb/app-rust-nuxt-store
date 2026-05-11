<template>
  <!-- Layout wrapper -->
  <div class="bg-base-200 flex min-h-screen flex-col">
    
    <!-- Layout Navbar (AdminNavbarTop now handles its own sticky and ps-75 container) -->
    <AdminNavbarTop />

    <!-- Layout Menu (AdminSidebar handles its own fixed/overlay logic) -->
    <AdminSidebar />

    <!-- Layout Container -->
    <div class="lg:ps-64 flex grow flex-col transition-all duration-300">
      <!-- Content -->
      <main class="mx-auto w-full max-w-7xl flex-1 grow space-y-6 p-4 md:p-6">
        <slot />
      </main>

      <!-- Footer -->
      <AppFooterGilcierweb />
    </div>
  </div>
</template>

<script setup lang="ts">
// Layout for Admin Dashboard
</script>

<style scoped>
/* Ensure the layout is responsive and smooth */
</style>