const BACKEND_URL_ENV_KEYS = [
  'NUXT_API_RUST_BASE_URL',
  'API_RUST_BASE_URL',
  'NUXT_PUBLIC_API_RUST_BASE_URL'
]

type BackendUrlResolution =
  | { ok: true, url: string, source: string, host: string }
  | { ok: false, statusCode: number, message: string, current: string | null, source: string | null }

function normalizeBackendUrl(value: unknown): string {
  return String(value || '').trim().replace(/\/+$/, '')
}

function readCloudflareEnv(event: any, key: string): string | undefined {
  return event.context?._platform?.cloudflare?.env?.[key] || (globalThis as any).__env__?.[key]
}

function runtimeEnvValue(event: any, key: string): string | undefined {
  return readCloudflareEnv(event, key) || process.env[key]
}

function configuredBackendUrl(event: any): { value: string, source: string | null } {
  const config = useRuntimeConfig(event)
  const configValue = normalizeBackendUrl(config.apiRustBaseUrl)

  if (configValue && configValue !== 'http://localhost:5150') {
    return { value: configValue, source: 'runtimeConfig.apiRustBaseUrl' }
  }

  for (const key of BACKEND_URL_ENV_KEYS) {
    const value = normalizeBackendUrl(runtimeEnvValue(event, key))
    if (value) return { value, source: key }
  }

  return { value: configValue, source: configValue ? 'runtimeConfig.apiRustBaseUrl' : null }
}

function isLocalBackendHost(hostname: string): boolean {
  return ['localhost', '127.0.0.1', '0.0.0.0', '::1'].includes(hostname)
}

export function resolveBackendBaseUrl(event: any): BackendUrlResolution {
  const requestUrl = getRequestURL(event)
  const { value, source } = configuredBackendUrl(event)

  if (!value) {
    return {
      ok: false,
      statusCode: 500,
      message: 'Set NUXT_API_RUST_BASE_URL in Cloudflare Pages to the Railway backend URL.',
      current: null,
      source
    }
  }

  try {
    const backendUrl = new URL(value)

    if (!['http:', 'https:'].includes(backendUrl.protocol)) {
      return {
        ok: false,
        statusCode: 500,
        message: 'Backend URL must use http or https.',
        current: value,
        source
      }
    }

    if (isLocalBackendHost(backendUrl.hostname) && !isLocalBackendHost(requestUrl.hostname)) {
      return {
        ok: false,
        statusCode: 500,
        message: 'Production backend URL cannot point to localhost.',
        current: value,
        source
      }
    }

    if (backendUrl.origin === requestUrl.origin) {
      return {
        ok: false,
        statusCode: 500,
        message: 'Backend URL cannot point to the Nuxt frontend origin.',
        current: value,
        source
      }
    }

    return {
      ok: true,
      url: backendUrl.origin,
      source: source || 'unknown',
      host: backendUrl.host
    }
  } catch {
    return {
      ok: false,
      statusCode: 500,
      message: 'Backend URL is invalid.',
      current: value,
      source
    }
  }
}
