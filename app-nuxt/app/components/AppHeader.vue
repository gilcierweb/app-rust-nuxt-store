<template>
    <div>
        <nav class="navbar rounded-box flex w-full items-center justify-between gap-2 shadow-base-300/20 shadow-sm">
            <div class="navbar-start max-md:w-1/4">
                <NuxtLinkLocale to="/" class="link text-base-content link-neutral text-xl font-bold no-underline">
                    {{ t('common.appName') }}
                </NuxtLinkLocale>
            </div>
            <div class="navbar-center max-md:hidden">
                <ul class="menu menu-horizontal p-0 font-medium">
                    <li><NuxtLinkLocale to="/">{{ t('header.home') }}</NuxtLinkLocale></li>
                    <li><NuxtLinkLocale to="/categories">{{ t('header.categories') }}</NuxtLinkLocale></li>
                    <li><NuxtLinkLocale to="/products">{{ t('header.products') }}</NuxtLinkLocale></li>
                    <li><NuxtLinkLocale to="/stores">{{ t('header.stores') }}</NuxtLinkLocale></li>
                    <li><NuxtLinkLocale to="/posts">{{ t('header.posts') }}</NuxtLinkLocale></li>
                    <li><NuxtLinkLocale to="/profiles">{{ t('header.profiles') }}</NuxtLinkLocale></li>
                    <li><NuxtLinkLocale to="/contact">{{ t('header.contact') }}</NuxtLinkLocale></li>
                    <li><NuxtLinkLocale to="/about">{{ t('header.about') }}</NuxtLinkLocale></li>
                </ul>
            </div>
            <div class="navbar-end items-center gap-4">
                <!-- Language Switcher -->
                <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
                    <button id="lang-dropdown-header" type="button" class="dropdown-toggle btn btn-text btn-circle size-10" aria-haspopup="menu" aria-expanded="false">
                        <span class="icon-[tabler--language] size-5.5"></span>
                    </button>
                    <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-32 shadow-xl border border-base-content/10 mt-2" role="menu">
                        <li v-for="lang in locales" :key="lang.code">
                            <button @click="setLocale(lang.code)" class="dropdown-item flex items-center gap-2" :class="{ 'active': locale === lang.code }">
                                <span class="text-lg">{{ (lang as any).flag }}</span>
                                <span>{{ lang.name }}</span>
                            </button>
                        </li>
                    </ul>
                </div>

                <NuxtLinkLocale to="/wishlist" class="btn btn-text btn-square relative">
                    <span class="icon-[tabler--heart] size-5"></span>
                </NuxtLinkLocale>
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
                        <li><NuxtLinkLocale to="/" class="dropdown-item">{{ t('header.home') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/categories" class="dropdown-item">{{ t('header.categories') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/products" class="dropdown-item">{{ t('header.products') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/cart" class="dropdown-item">{{ t('header.cart') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/stores" class="dropdown-item">{{ t('header.stores') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/posts" class="dropdown-item">{{ t('header.posts') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/profiles" class="dropdown-item">{{ t('header.profiles') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/contact" class="dropdown-item">{{ t('header.contact') }}</NuxtLinkLocale></li>
                        <li><NuxtLinkLocale to="/about" class="dropdown-item">{{ t('header.about') }}</NuxtLinkLocale></li>
                        <li v-if="isAuthenticated"><hr class="my-1" /></li>
                        <li v-if="isAuthenticated">
                            <NuxtLinkLocale to="/admin" class="dropdown-item">{{ t('header.admin') }}</NuxtLinkLocale>
                        </li>
                    </ul>
                </div>

                <template v-if="isAuthenticated">
                    <NuxtLinkLocale to="/admin" class="btn btn-text btn-sm max-md:hidden">
                        <span class="icon-[tabler--layout-dashboard] size-4"></span>
                        {{ t('header.admin') }}
                    </NuxtLinkLocale>
                    <div class="dropdown relative inline-flex [--auto-close:inside] [--offset:8] [--placement:bottom-end]">
                        <button type="button" class="dropdown-toggle btn btn-text flex items-center gap-2" aria-haspopup="menu" aria-expanded="false">
                            <span class="icon-[tabler--user] size-4"></span>
                            <span class="max-md:hidden">{{ user?.name || t('header.profile') }}</span>
                        </button>
                        <ul class="dropdown-menu dropdown-open:opacity-100 hidden min-w-40" role="menu">
                            <li><NuxtLinkLocale to="/account" class="dropdown-item">{{ t('header.profile') }}</NuxtLinkLocale></li>
                            <li><button class="dropdown-item" @click="handleLogout">{{ t('header.signOut') }}</button></li>
                        </ul>
                    </div>
                </template>
                <template v-else>
                    <NuxtLinkLocale to="/users/sessions" class="btn max-md:btn-square btn-text">
                        <span class="max-md:hidden">{{ t('header.logIn') }}</span>
                        <span class="icon-[tabler--arrow-right] rtl:rotate-180"></span>
                    </NuxtLinkLocale>
                    <NuxtLinkLocale to="/users/registrations" class="btn max-md:btn-square btn-primary">
                        <span class="max-md:hidden">{{ t('header.getStarted') }}</span>
                        <span class="icon-[tabler--arrow-right] rtl:rotate-180"></span>
                    </NuxtLinkLocale>
                </template>
            </div>
        </nav>
    </div>
</template>

<script lang="ts" setup>
const { t, locale, locales, setLocale } = useI18n()
const { isAuthenticated, user, logout } = useAuth()
const { toggleCart } = useCartUI()
const cartStore = useCartStore()

function handleLogout() {
    logout()
}
</script>
