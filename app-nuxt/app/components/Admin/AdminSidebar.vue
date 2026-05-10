<template>
  <aside id="with-navbar-sidebar"
    class="overlay [--auto-close:sm] sm:shadow-none overlay-open:translate-x-0 drawer drawer-start hidden max-w-64 w-64 sm:absolute sm:z-0 sm:flex sm:translate-x-0 pt-16"
    role="dialog" tabindex="-1">
    <div class="drawer-body px-2 pt-4">
      <ul class="menu p-0">
        <li>
          <NuxtLink to="/admin">
            <i class="icon-[tabler--home] size-5"></i>
            Dashboard
          </NuxtLink>
        </li>
        <li>
          <a href="#">
            <span class="icon-[tabler--user] size-5"></span>
            Account
          </a>
        </li>
        <li>
          <a href="#">
            <span class="icon-[tabler--message] size-5"></span>
            Notifications
          </a>
        </li>
        <li>
          <a href="#">
            <span class="icon-[tabler--mail] size-5"></span>
            Email
          </a>
        </li>
        <li>
          <a href="#">
            <span class="icon-[tabler--calendar] size-5"></span>
            Calendar
          </a>
        </li>
        <li>
         <NuxtLink to="/admin/orders">
            <i class="icon-[tabler--truck] size-5"></i>
            Orders
          </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/categories">
            <i class="icon-[tabler--category] size-5"></i>
            Categories
          </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/products">
             <i class="icon-[tabler--package] size-5"></i>
             Products
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/posts">
             <i class="icon-[tabler--article] size-5"></i>
             Posts
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/profiles">
             <i class="icon-[tabler--user-circle] size-5"></i>
             Profiles
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/reviews">
             <i class="icon-[tabler--star] size-5"></i>
             Reviews
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/coupons">
             <i class="icon-[tabler--ticket] size-5"></i>
             Coupons
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/addresses">
             <i class="icon-[tabler--map-pin] size-5"></i>
             Addresses
           </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/shippings">
              <i class="icon-[tabler--truck-delivery] size-5"></i>
              Shipping Methods
            </NuxtLink>
        </li>
        <li>
         <NuxtLink to="/admin/shipments">
              <i class="icon-[tabler--package-export] size-5"></i>
              Shipments
            </NuxtLink>
        </li>
        <li v-if="!isAuthenticated">
          <NuxtLink to="/users/sessions">
            <span class="icon-[tabler--login] size-5"></span>
            Sign In
          </NuxtLink>
        </li>
        <li v-if="isAuthenticated">
          <button class="w-full text-left" @click="handleLogout">
            <span class="icon-[tabler--logout-2] size-5"></span>
            Sign Out
          </button>
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
