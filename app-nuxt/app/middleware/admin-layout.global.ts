const SECTION_MAP: Record<string, string> = {
  '/admin/orders': 'orders',
  '/admin/payments': 'payments',
  '/admin/products': 'products',
  '/admin/inventory': 'inventory',
  '/admin/categories': 'categories',
  '/admin/banners': 'banners',
  '/admin/shipments': 'shipments',
  '/admin/shippings': 'shippings',
  '/admin/addresses': 'addresses',
  '/admin/posts': 'posts',
  '/admin/reviews': 'reviews',
  '/admin/coupons': 'coupons',
  '/admin/customers': 'customers',
  '/admin/users': 'users',
  '/admin/rbac': 'rbac',
  '/admin/emails': 'emails',
  '/admin/audit-logs': 'audit_logs',
  '/admin/settings': 'settings',
  '/admin/monitoring': 'dashboard',
}

function withLocalePrefix(toPath: string, targetPath: string): string {
  const localeMatch = toPath.match(/^\/(en|es)(\/|$)/)
  if (!localeMatch) return targetPath
  return `/${localeMatch[1]}${targetPath}`
}

function resolveSection(path: string): string | null {
  const clean = path.replace(/\/+$/, '') || path
  for (const [prefix, section] of Object.entries(SECTION_MAP)) {
    if (clean === prefix || clean.startsWith(prefix + '/')) {
      return section
    }
  }
  return null
}

export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.includes('/admin')) {
    const auth = useAuth()

    await auth.init()

    if (!auth.isAuthenticated.value) {
      return navigateTo(withLocalePrefix(to.path, '/users/sessions'))
    }

    if (!auth.user.value?.can_manage_admin) {
      return navigateTo(withLocalePrefix(to.path, '/'))
    }

    const section = resolveSection(to.path)
    if (section && !auth.hasSectionAccess(section)) {
      return navigateTo(withLocalePrefix(to.path, '/admin'))
    }

    to.meta.layout = 'admin'
  }
})
