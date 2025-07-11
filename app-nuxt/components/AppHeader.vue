<template>
    <div>
        <nav class="navbar rounded-box flex w-full items-center justify-between gap-2 shadow-base-300/20 shadow-sm">
  <div class="navbar-start max-md:w-1/4">
    <a class="link text-base-content link-neutral text-xl font-bold no-underline" href="#">
      FlyonUI
    </a>
  </div>
  <div class="navbar-center max-md:hidden">
    <ul class="menu menu-horizontal p-0 font-medium">
      <li><a href="#">Link 1</a></li>
      <li><a href="#">Link 2</a></li>
      <li><a href="#">Link 3</a></li>
    </ul>
  </div>
  <div class="navbar-end items-center gap-4">
    <div class="dropdown relative inline-flex md:hidden">
      <button id="dropdown-default" type="button" class="dropdown-toggle btn btn-text btn-secondary btn-square" aria-haspopup="menu" aria-expanded="false" aria-label="Dropdown">
        <span class="icon-[tabler--menu-2] dropdown-open:hidden size-5"></span>
        <span class="icon-[tabler--x] dropdown-open:block hidden size-5"></span>
      </button>
      <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-60" role="menu" aria-orientation="vertical" aria-labelledby="dropdown-default">
        <li><a class="dropdown-item" href="#">Link 1</a></li>
        <li><a class="dropdown-item" href="#">Link 2</a></li>
        <li><a class="dropdown-item" href="#">Link 3</a></li>
      </ul>
    </div>
    <a class="btn max-md:btn-square btn-primary" href="#">
      <span class="max-md:hidden">Get started</span>
      <span class="icon-[tabler--arrow-right] rtl:rotate-180"></span>
    </a>
  </div>
</nav>
    </div>
</template>
