export default defineNuxtPlugin(async () => {
  if (import.meta.client) {
    await import("flyonui/flyonui");
    await import("flyonui/dist/accordion");
    await import("flyonui/dist/dropdown");
    await import("flyonui/dist/carousel");
    await import("flyonui/dist/overlay");
  }

  const router = useRouter();
  router.afterEach(async () => {
    await nextTick();
    try {
      if (window.HSStaticMethods) {
        window.HSStaticMethods.autoInit();
      }
      if (window.HSAccordion) {
        window.HSAccordion.autoInit();
      }
      if (window.HSDropdown) {
        window.HSDropdown.autoInit();
      }
      if (window.HSCarousel) {
        window.HSCarousel.autoInit();
      }
      if (window.HSOverlay) {
        window.HSOverlay.autoInit();
      }
    } catch {
      // Ignore errors from FlyonUI autoInit when DOM nodes are temporarily unavailable
    }
  });
});