import { useRouter } from "vue-router";

// FlyonUI
import "flyonui/flyonui";
import "flyonui/dist/accordion";
import "flyonui/dist/dropdown";
import "flyonui/dist/carousel";

// import "flyonui/dist/modal";
// import "flyonui/dist/tooltip";
// import "flyonui/dist/tab";
// import "flyonui/dist/collapse";

export default defineNuxtPlugin(() => {
  const router = useRouter();
  router.afterEach(async () => {
    setTimeout(() => {
        if (window.HSAccordion) {
            window.HSAccordion.autoInit();
        }
        if (window.HSDropdown) {
            window.HSDropdown.autoInit();
        } 
        if (window.HSCarousel) {
            window.HSCarousel.autoInit();      
        }
    }, 100);
   
  });
});