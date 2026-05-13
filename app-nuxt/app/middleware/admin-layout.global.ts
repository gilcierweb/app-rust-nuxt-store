export default defineNuxtRouteMiddleware((to) => {
  if (to.path.includes('/admin')) {
    to.meta.layout = 'admin'
  }
})