<template>
  <main class="container mx-auto px-4 py-10">
    <div class="mb-8 flex flex-col gap-2">
      <p class="text-sm font-semibold uppercase tracking-wide text-primary">Account</p>
      <h1 class="text-3xl font-bold">Minha conta</h1>
      <p class="text-base-content/60">Área do cliente autenticado.</p>
    </div>

    <div class="grid gap-6 lg:grid-cols-3">
      <section class="rounded-box border border-base-content/10 bg-base-100 p-6 lg:col-span-2">
        <h2 class="mb-4 text-lg font-semibold">Perfil</h2>
        <dl class="grid gap-4 sm:grid-cols-2">
          <div>
            <dt class="text-sm text-base-content/60">Nome</dt>
            <dd class="font-medium">{{ user?.name || '-' }}</dd>
          </div>
          <div>
            <dt class="text-sm text-base-content/60">E-mail</dt>
            <dd class="font-medium">{{ user?.email || '-' }}</dd>
          </div>
        </dl>
      </section>

      <aside class="rounded-box border border-base-content/10 bg-base-100 p-6">
        <h2 class="mb-4 text-lg font-semibold">Acessos</h2>
        <div class="flex flex-wrap gap-2">
          <span
            v-for="role in user?.roles || []"
            :key="role"
            class="badge badge-soft badge-primary"
          >
            {{ role }}
          </span>
          <span v-if="!user?.roles?.length" class="text-sm text-base-content/60">Sem roles.</span>
        </div>
      </aside>
    </div>

    <div class="mt-8 flex flex-wrap gap-3">
      <NuxtLinkLocale to="/orders" class="btn btn-primary">Meus pedidos</NuxtLinkLocale>
      <NuxtLinkLocale to="/products" class="btn btn-soft">Continuar comprando</NuxtLinkLocale>
      <NuxtLinkLocale v-if="user?.can_manage_admin" to="/admin" class="btn btn-outline">
        Administração
      </NuxtLinkLocale>
    </div>
  </main>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

const { user, fetchCurrentUser } = useAuth()

if (!user.value) {
  await fetchCurrentUser()
}
</script>
