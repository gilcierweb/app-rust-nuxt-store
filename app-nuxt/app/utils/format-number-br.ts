export function formatNumberBR(value: unknown): string {
  if (value == null) return '-'
  const num = Number(value)
  if (Number.isNaN(num)) return '-'

  return new Intl.NumberFormat(getAppLocale(), {
    style: 'currency',
    currency: 'BRL'
  }).format(num)
}

export function formatDate(value: unknown): string {
  if (!value) return '-'
  const d = new Date(String(value))
  if (Number.isNaN(d.getTime())) return '-'

  return d.toLocaleDateString(getAppLocale())
}
