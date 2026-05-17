function firstHeaderValue(value: string | string[] | undefined): string {
  if (Array.isArray(value)) return value[0] || ''
  return value || ''
}

function firstForwardedIp(value: string | string[] | undefined): string {
  return firstHeaderValue(value)
    .split(',')
    .map(ip => ip.trim())
    .find(Boolean) || ''
}

export function resolveForwardedClientHeaders(event: any): Record<string, string> {
  const headers = getRequestHeaders(event)
  const cloudflareIp = firstHeaderValue(headers['cf-connecting-ip']).trim()
  const forwardedIp = firstForwardedIp(headers['x-forwarded-for'])
  const realIp = firstHeaderValue(headers['x-real-ip']).trim()
  const clientIp = cloudflareIp || forwardedIp || realIp

  if (!clientIp) return {}

  return {
    ...(cloudflareIp ? { 'cf-connecting-ip': cloudflareIp } : {}),
    'x-forwarded-for': clientIp,
    'x-real-ip': clientIp
  }
}
