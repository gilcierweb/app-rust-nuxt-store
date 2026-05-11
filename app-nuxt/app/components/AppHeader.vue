<template>
    <div>
        <nav class="navbar rounded-box flex w-full items-center justify-between gap-2 shadow-base-300/20 shadow-sm">
            <div class="navbar-start max-md:w-1/4">
                <NuxtLink to="/" class="link text-base-content link-neutral text-xl font-bold no-underline">
                    App Rust Nuxt Store
                </NuxtLink>
            </div>
            <div class="navbar-center max-md:hidden">
                <ul class="menu menu-horizontal p-0 font-medium">
                    <li><NuxtLink to="/">{{ t('header.home') }}</NuxtLink></li>
                    <li><NuxtLink to="/categories">{{ t('header.categories') }}</NuxtLink></li>
                    <li><NuxtLink to="/products">{{ t('header.products') }}</NuxtLink></li>
                    <li><NuxtLink to="/stores">{{ t('header.stores') }}</NuxtLink></li>
                    <li><NuxtLink to="/posts">{{ t('header.posts') }}</NuxtLink></li>
                    <li><NuxtLink to="/profiles">{{ t('header.profiles') }}</NuxtLink></li>
                    <li><NuxtLink to="/contact">{{ t('header.contact') }}</NuxtLink></li>
                    <li><NuxtLink to="/about">{{ t('header.about') }}</NuxtLink></li>
                </ul>
            </div>
            <div class="navbar-end items-center gap-4">
                <NuxtLink to="/wishlist" class="btn btn-text btn-square relative">
                    <span class="icon-[tabler--heart] size-5"></span>
                </NuxtLink>
                <button class="btn btn-text btn-square relative" @click="toggleCart">
                    <span class="icon-[tabler--shopping-cart] size-5"></span>
                    <span v-if="cartStore.totalItems > 0" class="badge badge-primary badge-xs absolute -top-1 -right-1">
                        {{ cartStore.totalItems > 99 ? '99+' : cartStore.totalItems }}
                    </span>
                </button>
                <div class="dropdown relative inline-flex md:hidden">
                    <button id="dropdown-default" type="button"
                        class="dropdown-toggle btn btn-text btn-secondary btn-square" aria-haspopup="menu"
                        aria-expanded="false" aria-label="Dropdown">
                        <span class="icon-[tabler--menu-2] dropdown-open:hidden size-5"></span>
                        <span class="icon-[tabler--x] dropdown-open:block hidden size-5"></span>
                    </button>
                    <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-60" role="menu"
                        aria-orientation="vertical" aria-labelledby="dropdown-default">
                        <li><NuxtLink to="/" class="dropdown-item">{{ t('header.home') }}</NuxtLink></li>
                        <li><NuxtLink to="/categories" class="dropdown-item">{{ t('header.categories') }}</NuxtLink></li>
                        <li><NuxtLink to="/products" class="dropdown-item">{{ t('header.products') }}</NuxtLink></li>
                        <li><NuxtLink to="/cart" class="dropdown-item">{{ t('header.cart') }}</NuxtLink></li>
                        <li><NuxtLink to="/stores" class="dropdown-item">{{ t('header.stores') }}</NuxtLink></li>
                        <li><NuxtLink to="/posts" class="dropdown-item">{{ t('header.posts') }}</NuxtLink></li>
                        <li><NuxtLink to="/profiles" class="dropdown-item">{{ t('header.profiles') }}</NuxtLink></li>
                        <li><NuxtLink to="/contact" class="dropdown-item">{{ t('header.contact') }}</NuxtLink></li>
                        <li><NuxtLink to="/about" class="dropdown-item">{{ t('header.about') }}</NuxtLink></li>
                        <li v-if="isAuthenticated"><hr class="my-1" /></li>
                        <li v-if="isAuthenticated">
                            <NuxtLink to="/admin" class="dropdown-item">{{ t('header.admin') }}</NuxtLink>
                        </li>
                    </ul>
                </div>

                <template v-if="isAuthenticated">
                    <NuxtLink to="/admin" class="btn btn-text btn-sm max-md:hidden">
                        <span class="icon-[tabler--layout-dashboard] size-4"></span>
                        {{ t('header.admin') }}
                    </NuxtLink>
                    <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
                        <button type="button" class="dropdown-toggle btn btn-text flex items-center gap-2" aria-haspopup="menu" aria-expanded="false">
                            <span class="icon-[tabler--user] size-4"></span>
                            <span class="max-md:hidden">{{ user?.name || t('header.profile') }}</span>
                        </button>
                        <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-40" role="menu">
                            <li><NuxtLink to="/profiles" class="dropdown-item">{{ t('header.profile') }}</NuxtLink></li>
                            <li><button class="dropdown-item" @click="handleLogout">{{ t('header.signOut') }}</button></li>
                        </ul>
                    </div>
                </template>
                <template v-else>
                    <NuxtLink to="/users/sessions" class="btn max-md:btn-square btn-text">
                        <span class="max-md:hidden">{{ t('header.logIn') }}</span>
                        <span class="icon-[tabler--arrow-right] rtl:rotate-180"></span>
                    </NuxtLink>
                    <NuxtLink to="/users/registrations" class="btn max-md:btn-square btn-primary">
                        <span class="max-md:hidden">{{ t('header.getStarted') }}</span>
                        <span class="icon-[tabler--arrow-right] rtl:rotate-180"></span>
                    </NuxtLink>
                </template>
            </div>
        </nav>
    </div>
</template>

<script lang="ts" setup>
const { t } = useI18n()
const { isAuthenticated, user, logout } = useAuth()
const { toggleCart } = useCartUI()
const cartStore = useCartStore()

function handleLogout() {
    logout()
}
</script>
