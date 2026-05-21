export type AppDialogTone = 'default' | 'danger'

export interface AppDialogInputConfig {
  label?: string
  placeholder?: string
  initialValue?: string
  required?: boolean
  type?: 'text' | 'number'
  min?: number
  step?: string
}

export interface AppDialogState {
  title?: string
  message: string
  confirmLabel?: string
  cancelLabel?: string
  tone?: AppDialogTone
  input?: AppDialogInputConfig | null
}

let dialogResolver: ((value: string | boolean | null) => void) | null = null

export function useAppDialog() {
  const state = useState<AppDialogState | null>('app-dialog-state', () => null)

  const open = <T extends string | boolean | null>(dialog: AppDialogState) => {
    if (dialogResolver) {
      dialogResolver(null)
      dialogResolver = null
    }

    state.value = dialog

    return new Promise<T>((resolve) => {
      dialogResolver = resolve as (value: string | boolean | null) => void
    })
  }

  const closeWith = (value: string | boolean | null) => {
    const resolver = dialogResolver
    dialogResolver = null
    state.value = null
    resolver?.(value)
  }

  const confirm = (dialog: Omit<AppDialogState, 'input'>) => {
    return open<boolean>({
      tone: 'default',
      ...dialog,
      input: null
    }).then(result => result === true)
  }

  const prompt = (dialog: AppDialogState) => {
    return open<string | null>(dialog).then(result => {
      if (typeof result === 'string') return result
      return null
    })
  }

  return {
    state,
    confirm,
    prompt,
    resolveConfirm: () => closeWith(true),
    resolvePrompt: (value: string) => closeWith(value),
    cancel: () => closeWith(false)
  }
}
