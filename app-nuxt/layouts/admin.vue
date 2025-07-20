<template>
  <div class="min-h-screen bg-base-100 flex flex-col">

    <AdminNavbarTop />

    <!-- Sidebar + Content -->
    <div class="flex flex-1 overflow-hidden">

      <AdminSidebar />

      <main class="flex-1 overflow-auto p-6 ml-0 sm:ml-64">
       
          <div class="container mx-auto px-4 py-8">
            <h1 class="h1">layout admin</h1>
            <slot />
          </div>

          <AppFooter />
    
      </main>

    </div>

  </div>

</template>

<style scoped>
@import '~/assets/css/tailwind.css';
</style>