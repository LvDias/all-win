"use client";

import { useEffect, useState } from "react";

const dialogSelector = '[role="dialog"],[role="alertdialog"]';

/** Dialogs mounted right now, and the element that opened the first of them (chained dialogs share it). */
let open = 0;
let base: HTMLElement | null = null;

/** Who opened a dialog: the open menu's trigger (its item is about to vanish), else the focused element. */
function findOpener(): HTMLElement | null {
  if (typeof document === "undefined") return null;
  const menu = document.querySelector<HTMLElement>('[aria-haspopup="menu"][aria-expanded="true"]');
  if (menu) return menu;
  const active = document.activeElement;
  return active instanceof HTMLElement && active !== document.body ? active : null;
}

/**
 * Render this inside a dialog's content. Dialogs here are mounted on demand, without a Radix trigger, so the opener
 * is captured when the content first renders (before focus moves in) and focus goes back to it once the last dialog
 * of a chain closes (WCAG 2.4.3). A follow-up dialog opened in the same tick (create, then the one-time value) keeps
 * the original opener, and nothing steals focus while another dialog is open (for example a step-up).
 */
export function ReturnFocus() {
  const [captured] = useState(findOpener);

  useEffect(() => {
    if (open === 0) base = captured;
    open += 1;
    return () => {
      // Deferred, so a dialog mounted in the same commit counts before we decide the chain ended.
      window.setTimeout(() => {
        open -= 1;
        if (open > 0) return;
        const target = base;
        base = null;
        if (!target || document.querySelector(dialogSelector)) return;
        if (target.isConnected) target.focus();
      }, 0);
    };
  }, [captured]);

  return null;
}
