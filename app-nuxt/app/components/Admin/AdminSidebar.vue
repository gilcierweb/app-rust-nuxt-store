<template>
  <aside id="with-navbar-sidebar"
    class="overlay [--auto-close:sm] sm:shadow-none overlay-open:translate-x-0 drawer drawer-start hidden max-w-64 w-64 sm:absolute sm:z-0 sm:flex sm:translate-x-0 pt-16"
    role="dialog" tabindex="-1">
    <div class="drawer-body px-2 pt-4">
      <ul class="menu p-0">
        <li>
          <NuxtLink to="/admin">
            <i class="icon-[tabler--home] size-5"></i>
            {{ $t('admin.sidebar.dashboard') }}
          </NuxtLink>
        </li>
        
        <li class="menu-title mt-4">
          <span>{{ $t('admin.sidebar.category_ecommerce') || 'E-commerce' }}</span>
        </li>
        <li>
         <NuxtLink to="/admin/orders">
            <i class="icon-[tabler--shopping-cart] size-5"></i>
            {{ $t('admin.orders.title') }}
          </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/products">
             <i class="icon-[tabler--package] size-5"></i>
             {{ $t('admin.products.title') }}
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/categories">
            <i class="icon-[tabler--category] size-5"></i>
            {{ $t('admin.categories.title') }}
          </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/coupons">
             <i class="icon-[tabler--ticket] size-5"></i>
             {{ $t('admin.coupons.title') }}
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/reviews">
             <i class="icon-[tabler--star] size-5"></i>
             {{ $t('admin.reviews.title') }}
           </NuxtLink>
        </li>

        <li class="menu-title mt-4">
          <span>{{ $t('admin.sidebar.category_logistics') || 'Logistics' }}</span>
        </li>
        <li>
         <NuxtLink to="/admin/shipments">
              <i class="icon-[tabler--truck-delivery] size-5"></i>
              {{ $t('admin.shipments.title') }}
            </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/shippings">
              <i class="icon-[tabler--map-pin] size-5"></i>
              {{ $t('admin.shippings.title') }}
            </NuxtLink>
        </li>

        <li class="menu-title mt-4">
          <span>{{ $t('admin.sidebar.category_content') || 'Content' }}</span>
        </li>
        <li>
         <NuxtLink to="/admin/posts">
             <i class="icon-[tabler--article] size-5"></i>
             {{ $t('admin.posts.title') }}
           </NuxtLink>
        </li>

        <li class="menu-title mt-4">
          <span>{{ $t('admin.sidebar.category_admin') || 'Administration' }}</span>
        </li>
        <li>
         <NuxtLink to="/admin/customers">
             <i class="icon-[tabler--users] size-5"></i>
             {{ $t('admin.customers.title') }}
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/users">
             <i class="icon-[tabler--user-shield] size-5"></i>
             {{ $t('admin.users.title') }}
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/settings">
              <i class="icon-[tabler--settings] size-5"></i>
              {{ $t('admin.settings.title') }}
            </NuxtLink>
        </li>

        <li class="mt-8">
          <button v-if="isAuthenticated" class="w-full text-left" @click="handleLogout">
            <span class="icon-[tabler--logout-2] size-5"></span>
            {{ $t('admin.navbar.profile.logout') }}
          </button>
          <NuxtLink v-else to="/users/sessions">
            <span class="icon-[tabler--login] size-5"></span>
            {{ $t('auth.login.submit') }}
          </NuxtLink>
        </li>
      </ul>
    </div>
  </aside>
</template>

<script lang="ts" setup>
const { isAuthenticated, logout } = useAuth()

function handleLogout() {
    logout()
}
</script>

<style></style>
