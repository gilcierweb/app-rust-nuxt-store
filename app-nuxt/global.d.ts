import type { HSAccordion, HSDropdown, IStaticMethods } from "flyonui/flyonui";

declare global {
  interface Window {
     // Optional third-party libraries
    // _;
    // $: typeof import("jquery");
    // jQuery: typeof import("jquery");
    // DataTable;
    // Dropzone;

    // Specific JS component
    // FlyonUI
    HSStaticMethods: IStaticMethods;
    HSAccordion: typeof HSAccordion;
    HSDropdown: typeof HSDropdown;
  }
}

export {};  