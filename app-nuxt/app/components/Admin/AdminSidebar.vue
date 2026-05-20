<template>
  <aside id="layout-sidebar"
    class="overlay [--auto-close:lg] lg:z-50 lg:block lg:translate-x-0 lg:shadow-none overlay-open:translate-x-0 drawer drawer-start hidden max-w-75 w-75 sm:fixed sm:inset-y-0 sm:start-0 bg-base-100 border-e border-base-content/20 transition-transform duration-300"
    role="dialog" tabindex="-1">
    
    <div class="flex h-full flex-col">
      <!-- Sidebar Header (Profile) -->
      <div class="text-base-content border-base-content/10 flex flex-col items-center gap-4 border-b px-4 py-8">
        <div class="avatar">
          <div class="size-20 rounded-full border-2 border-primary/20 p-1">
            <img :src="user?.avatar || 'https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png'" class="rounded-full" alt="avatar" />
          </div>
        </div>
        <div class="text-center">
          <h3 class="text-base-content text-lg font-bold">{{ user?.name || 'Administrator' }}</h3>
          <p class="text-base-content/60 text-xs truncate max-w-[200px]">{{ user?.email || 'admin@store.com' }}</p>
        </div>
        <div class="flex gap-3 mt-2">
           <a href="#" class="btn btn-text btn-circle btn-xs"><i class="icon-[tabler--brand-github] size-4"></i></a>
           <a href="#" class="btn btn-text btn-circle btn-xs"><i class="icon-[tabler--brand-twitter] size-4"></i></a>
           <NuxtLinkLocale to="/admin/settings" class="btn btn-text btn-circle btn-xs"><i class="icon-[tabler--settings] size-4"></i></NuxtLinkLocale>
        </div>
      </div>

      <!-- Navigation Menu -->
      <div class="h-full overflow-y-auto px-4 py-4">
        <ul class="menu p-0 gap-1">
          <li>
            <NuxtLinkLocale to="/admin" class="flex items-center gap-3 px-4 py-3 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--home] size-5"></i>
              {{ $t('admin.sidebar.dashboard') }}
            </NuxtLinkLocale>
          </li>
          
          <li class="menu-title mt-4 mb-2 px-4 text-xs font-bold uppercase tracking-wider text-base-content/40">
            {{ $t('admin.sidebar.category_ecommerce') || 'E-commerce' }}
          </li>
          <li>
            <NuxtLinkLocale to="/admin/orders" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--shopping-cart] size-5"></i>
              {{ $t('admin.orders.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/payments" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--credit-card] size-5"></i>
              Payments
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/products" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--package] size-5"></i>
              {{ $t('admin.products.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/inventory" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--packages] size-5"></i>
              Inventory
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/categories" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--category] size-5"></i>
              {{ $t('admin.categories.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/banners" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--photo] size-5"></i>
              {{ $t('admin.banners.title') }}
            </NuxtLinkLocale>
          </li>

          <li class="menu-title mt-6 mb-2 px-4 text-xs font-bold uppercase tracking-wider text-base-content/40">
            {{ $t('admin.sidebar.category_logistics') || 'Logistics' }}
          </li>
          <li>
            <NuxtLinkLocale to="/admin/shipments" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--truck-delivery] size-5"></i>
              {{ $t('admin.shipments.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/shippings" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--map-pin] size-5"></i>
              {{ $t('admin.shippings.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/addresses" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--address-book] size-5"></i>
              Addresses
            </NuxtLinkLocale>
          </li>

          <li class="menu-title mt-6 mb-2 px-4 text-xs font-bold uppercase tracking-wider text-base-content/40">
            {{ $t('admin.sidebar.category_content') || 'Content' }}
          </li>
          <li>
            <NuxtLinkLocale to="/admin/posts" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--article] size-5"></i>
              {{ $t('admin.posts.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/reviews" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--star] size-5"></i>
              {{ $t('admin.reviews.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/coupons" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--ticket] size-5"></i>
              {{ $t('admin.coupons.title') }}
            </NuxtLinkLocale>
          </li>

          <li class="menu-title mt-6 mb-2 px-4 text-xs font-bold uppercase tracking-wider text-base-content/40">
            {{ $t('admin.sidebar.category_admin') || 'Administration' }}
          </li>
          <li>
            <NuxtLinkLocale to="/admin/customers" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--users] size-5"></i>
              {{ $t('admin.customers.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/users" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--user-shield] size-5"></i>
              {{ $t('admin.users.title') }}
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/emails" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--mail] size-5"></i>
              Emails
            </NuxtLinkLocale>
          </li>
          <li>
            <NuxtLinkLocale to="/admin/settings" class="flex items-center gap-3 px-4 py-2.5 rounded-lg hover:bg-base-200 transition-colors" active-class="bg-primary/10 text-primary font-bold">
              <i class="icon-[tabler--settings] size-5"></i>
              {{ $t('admin.settings.title') }}
            </NuxtLinkLocale>
          </li>
        </ul>
      </div>

      <!-- Sidebar Footer -->
      <div class="mt-auto border-t border-base-content/10 p-4">
        <button v-if="isAuthenticated" class="btn btn-error btn-soft btn-block gap-2" @click="handleLogout">
          <i class="icon-[tabler--logout] size-5"></i>
          {{ $t('admin.navbar.profile.logout') }}
        </button>
      </div>
    </div>
  </aside>
</template>

<script lang="ts" setup>
const { isAuthenticated, user, logout } = useAuth()

function handleLogout() {
    logout()
}
</script>

<style scoped>
/* Custom scrollbar for sidebar */
.overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}
.overflow-y-auto::-webkit-scrollbar-thumb {
  background: rgba(0,0,0,0.1);
  border-radius: 10px;
}
</style>
