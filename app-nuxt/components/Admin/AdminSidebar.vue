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
         <NuxtLink to="/admin/products">
            <i class="icon-[tabler--shopping-bag] size-5"></i>
            Product
          </NuxtLink>
        </li>
        <li>
          <a href="#">
            <span class="icon-[tabler--login] size-5"></span>
            Sign In
          </a>
        </li>
        <li>
          <a href="#">
            <span class="icon-[tabler--logout-2] size-5"></span>
            Sign Out
          </a>
        </li>
      </ul>
    </div>
  </aside>
</template>

<script lang="ts" setup>

</script>

<style></style>