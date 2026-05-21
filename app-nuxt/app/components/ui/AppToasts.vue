<template>
  <Teleport to="body">
    <div
      class="fixed top-5 right-5 z-[9999] flex flex-col gap-2.5 w-full max-w-sm pointer-events-none"
      aria-live="polite"
    >
      <TransitionGroup
        name="toast"
        tag="div"
        class="flex flex-col gap-2.5"
      >
        <div
          v-for="toast in toasts"
          :key="toast.id"
          class="pointer-events-auto alert relative overflow-hidden flex items-center gap-4 shadow-xl transition duration-300 ease-in-out"
          :class="toastClasses[toast.type]"
          role="alert"
        >
          <span :class="[toastIcons[toast.type], 'shrink-0 size-6']"></span>
          <p class="flex-1 text-sm font-medium">{{ toast.message }}</p>
          <button
            class="ms-auto cursor-pointer leading-none opacity-60 hover:opacity-100 transition-opacity"
            @click="dismiss(toast.id)"
            aria-label="Close Button"
          >
            <span class="icon-[tabler--x] size-5"></span>
          </button>
          <div 
            class="absolute bottom-0 left-0 h-1 bg-current opacity-40"
            :style="{ animation: `toast-progress ${toast.duration}ms linear forwards` }"
          ></div>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
const { toasts, dismiss } = useAppToast()

const toastIcons: Record<string, string> = {
  success: 'icon-[tabler--circle-check]',
  error: 'icon-[tabler--alert-circle]',
  warning: 'icon-[tabler--alert-triangle]',
  info: 'icon-[tabler--info-circle]',
}

const toastClasses: Record<string, string> = {
  success: 'alert-soft alert-success',
  error:   'alert-soft alert-error',
  warning: 'alert-soft alert-warning',
  info:    'alert-soft alert-info',
}
</script>

<style scoped>
.toast-enter-active { transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1); }
.toast-leave-active { transition: all 0.2s ease; }
.toast-enter-from { opacity: 0; transform: translateX(24px) scale(0.95); }
.toast-leave-to  { opacity: 0; transform: translateX(24px) scale(0.95); }
.toast-move      { transition: transform 0.3s ease; }
</style>

<style>
@keyframes toast-progress {
  0% { width: 100%; }
  100% { width: 0%; }
}
</style>
