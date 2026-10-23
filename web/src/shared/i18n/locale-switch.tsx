"use client";

import { useLocale, useTranslations } from "next-intl";
import { usePathname, useRouter } from "next/navigation";
import { createContext, useCallback, useContext, useEffect, useId, useMemo, useRef, useState, type ReactNode } from "react";
import { toast } from "sonner";
import { errorMessage } from "@/shared/api/error-message";
import { useMe, useUpdateMe } from "@/shared/session/me";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { isLocale, type Locale } from "./config";
import { writeLocaleCookie } from "./cookie";
import { splitLocalePath, withLocale } from "./paths";

export const focusHeadingFlag = "aw.focus-h1";

type LocaleSwitch = {
  switchLocale: (target: Locale) => void;
  pending: boolean;
  /** True while a view-once dialog is open: the switcher is disabled so a secret is never lost (spec §3.6). */
  locked: boolean;
  register: (kind: "dirty" | "lock", id: string, active: boolean) => void;
};

const LocaleSwitchContext = createContext<LocaleSwitch | null>(null);

export function useLocaleSwitch(): LocaleSwitch {
  const context = useContext(LocaleSwitchContext);
  if (!context) throw new Error("useLocaleSwitch must be used inside LocaleSwitchProvider");
  return context;
}

/** Marks the page as having unsaved form data; switching language then asks for confirmation first. */
export function useUnsavedChanges(dirty: boolean): void {
  const { register } = useLocaleSwitch();
  const id = useId();
  useEffect(() => {
    register("dirty", id, dirty);
    return () => register("dirty", id, false);
  }, [register, id, dirty]);
}

/** Disables the language switcher while a view-once dialog (secret shown once) is open. */
export function useViewOnceLock(open: boolean): void {
  const { register } = useLocaleSwitch();
  const id = useId();
  useEffect(() => {
    register("lock", id, open);
    return () => register("lock", id, false);
  }, [register, id, open]);
}

export function LocaleSwitchProvider({ children }: { children: ReactNode }) {
  const t = useTranslations();
  const current = useLocale();
  const pathname = usePathname();
  const router = useRouter();
  const me = useMe();
  const updateMe = useUpdateMe();

  const [pending, setPending] = useState(false);
  const [confirmTarget, setConfirmTarget] = useState<Locale | null>(null);
  const [dirtyCount, setDirtyCount] = useState(0);
  const [lockCount, setLockCount] = useState(0);
  const registered = useRef({ dirty: new Set<string>(), lock: new Set<string>() });
  const busy = useRef(false);
  const retry = useRef<(target: Locale) => void>(() => {});

  const register = useCallback((kind: "dirty" | "lock", id: string, active: boolean) => {
    const set = registered.current[kind];
    if (active) set.add(id);
    else set.delete(id);
    (kind === "dirty" ? setDirtyCount : setLockCount)(set.size);
  }, []);

  const perform = useCallback(
    async (target: Locale) => {
      if (busy.current) return;
      busy.current = true;
      setPending(true);
      try {
        // A switch right after load must not skip the profile save because /me is still in flight.
        const profile = me.isPending ? (await me.refetch()).data : me.data;
        if (profile) await updateMe.mutateAsync({ locale: target });
        writeLocaleCookie(target);
        try {
          sessionStorage.setItem(focusHeadingFlag, "1");
        } catch {
          // Storage can be blocked; focus then stays where the browser puts it.
        }
        const { rest } = splitLocalePath(pathname);
        router.replace(`${withLocale(target, rest)}${window.location.search}${window.location.hash}`, { scroll: false });
      } catch (error) {
        const { message, code } = errorMessage(t, error);
        toast.error(t("language.failed"), {
          description: `${message} (${code})`,
          action: { label: t("common.actions.retry"), onClick: () => retry.current(target) },
        });
      } finally {
        busy.current = false;
        setPending(false);
      }
    },
    [me, pathname, router, t, updateMe],
  );

  useEffect(() => {
    retry.current = (target) => void perform(target);
  }, [perform]);

  const switchLocale = useCallback(
    (target: Locale) => {
      if (target === current || busy.current || lockCount > 0) return;
      if (dirtyCount > 0) setConfirmTarget(target);
      else void perform(target);
    },
    [current, dirtyCount, lockCount, perform],
  );

  const value = useMemo<LocaleSwitch>(() => ({ switchLocale, pending, locked: lockCount > 0, register }), [switchLocale, pending, lockCount, register]);

  return (
    <LocaleSwitchContext.Provider value={value}>
      {children}
      <AlertDialog open={confirmTarget !== null} onOpenChange={(open) => !open && setConfirmTarget(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("language.dirty.title")}</AlertDialogTitle>
            <AlertDialogDescription>{t("language.dirty.description")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("language.dirty.stay")}</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                const target = confirmTarget;
                setConfirmTarget(null);
                if (isLocale(target)) void perform(target);
              }}
            >
              {t("language.dirty.confirm")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </LocaleSwitchContext.Provider>
  );
}
