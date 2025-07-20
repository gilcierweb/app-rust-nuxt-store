export default defineNuxtRouteMiddleware((to) => {
  if (to.path.startsWith('/admin')) {
    to.meta.layout = 'admin'
  }
})