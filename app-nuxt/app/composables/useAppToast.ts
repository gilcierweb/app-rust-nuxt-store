export type AppToastType = 'success' | 'error' | 'warning' | 'info'

export interface AppToastItem {
  id: number
  message: string
  type: AppToastType
  duration: number
}

const toastTimers = new Map<number, ReturnType<typeof setTimeout>>()

export function useAppToast() {
  const toasts = useState<AppToastItem[]>('app-toasts', () => [])

  const dismiss = (id: number) => {
    const timer = toastTimers.get(id)
    if (timer) {
      clearTimeout(timer)
      toastTimers.delete(id)
    }

    toasts.value = toasts.value.filter(toast => toast.id !== id)
  }

  const show = (message: string, type: AppToastType = 'info', duration = 4000) => {
    const id = Date.now() + Math.floor(Math.random() * 10000)
    toasts.value = [...toasts.value, { id, message, type, duration }]

    if (import.meta.client) {
      const timer = setTimeout(() => dismiss(id), duration)
      toastTimers.set(id, timer)
    }

    return id
  }

  return {
    toasts,
    dismiss,
    show,
    success: (message: string, duration?: number) => show(message, 'success', duration),
    error: (message: string, duration?: number) => show(message, 'error', duration),
    warning: (message: string, duration?: number) => show(message, 'warning', duration),
    info: (message: string, duration?: number) => show(message, 'info', duration)
  }
}
