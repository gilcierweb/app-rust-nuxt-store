<template>
    <div>
      <AppHeader />
      <main class="w-full space-y-6 rounded-xl bg-base-100 p-6 shadow-md shadow-base-300/20 lg:p-8">
        <slot />
      </main>
      <AppFooter />
    </div>
  </template>