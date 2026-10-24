"use client";

import { Cable, ChevronDown, Landmark, Home, Mail, Menu, Radio, Settings, Shield, SlidersHorizontal, X } from "lucide-react";
import { useTranslations } from "next-intl";
import { Dialog as SheetPrimitive } from "radix-ui";
import Link from "next/link";
import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { useEffect, useState, type ComponentType, type ReactNode } from "react";
import { errorMessage } from "@/shared/api/error-message";
import { LanguageMenuSub } from "@/shared/i18n/language-switcher";
import { useAppLocale, useLocalePath } from "@/shared/i18n/navigation";
import { splitLocalePath } from "@/shared/i18n/paths";
import { cn } from "@/shared/lib/utils";
import { useLogout } from "@/shared/session/logout";
import { useMe } from "@/shared/session/me";
import { accountsPath } from "@/shared/routes";
import { loginPath } from "@/shared/session/redirects";
import { Button } from "@/shared/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from "@/shared/ui/dropdown-menu";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";

type NavItem = { href: string; label: string; icon: ComponentType<{ className?: string; "aria-hidden"?: boolean }> };
type NavGroup = { label: string | null; items: NavItem[] };

function useNavigation(isAdmin: boolean): NavGroup[] {
  const t = useTranslations("nav");
  const groups: NavGroup[] = [
    { label: null, items: [{ href: "/", label: t("home"), icon: Home }] },
    ...(accountsPath ? [{ label: null, items: [{ href: accountsPath, label: t("accounts"), icon: Landmark }] }] : []),
    { label: t("connections"), items: [{ href: "/connections/sources", label: t("sources"), icon: Radio }, { href: "/connections/ninjatrader", label: t("ninjatrader"), icon: Cable }] },
    { label: t("settings"), items: [{ href: "/settings/security", label: t("security"), icon: Shield }, { href: "/settings/preferences", label: t("preferences"), icon: SlidersHorizontal }] },
  ];
  if (isAdmin) groups.push({ label: t("admin"), items: [{ href: "/admin/invites", label: t("invites"), icon: Mail }] });
  return groups;
}

function NavLinks({ groups, onNavigate }: { groups: NavGroup[]; onNavigate?: () => void }) {
  const t = useTranslations("nav");
  const path = useLocalePath();
  const { rest } = splitLocalePath(usePathname());
  return (
    <nav aria-label={t("main")} className="grid gap-4">
      {groups.map((group) => (
        <div key={group.label ?? "root"} className="grid gap-1">
          {group.label ? <p className="px-3 text-xs font-medium text-muted-foreground">{group.label}</p> : null}
          {group.items.map((item) => {
            const active = item.href === "/" ? rest === "/" : rest.startsWith(item.href);
            return (
              <Link
                key={item.href}
                href={path(item.href)}
                aria-current={active ? "page" : undefined}
                onClick={onNavigate}
                className={cn("flex min-h-11 items-center gap-2 rounded-md px-3 text-sm outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring", active && "bg-accent font-medium")}
              >
                <item.icon aria-hidden className="size-4" />
                {item.label}
              </Link>
            );
          })}
        </div>
      ))}
    </nav>
  );
}

export function AppShell({ children }: { children: ReactNode }) {
  const t = useTranslations();
  const locale = useAppLocale();
  const path = useLocalePath();
  const router = useRouter();
  const pathname = usePathname();
  const search = useSearchParams();
  const me = useMe();
  const logout = useLogout();
  const [menuOpen, setMenuOpen] = useState(false);
  const groups = useNavigation(me.data?.is_platform_admin === true);

  const query = search.toString();
  useEffect(() => {
    if (me.data === null) router.replace(loginPath(locale, { returnTo: `${pathname}${query ? `?${query}` : ""}` }));
  }, [me.data, locale, pathname, query, router]);

  if (me.isError) {
    const { message, code } = errorMessage(t, me.error);
    return (
      <main id="main" className="mx-auto w-full max-w-xl p-4">
        <ErrorState title={message} code={code} onRetry={() => void me.refetch()} />
      </main>
    );
  }
  if (!me.data) {
    return (
      <main id="main" className="mx-auto grid w-full max-w-5xl gap-4 p-4">
        <Skeleton className="h-10 w-48" />
        <Skeleton className="h-40 w-full" />
      </main>
    );
  }

  return (
    <div className="flex min-h-dvh flex-col md:flex-row">
      <aside className="hidden w-60 shrink-0 border-r p-4 md:block">
        <p className="mb-6 px-3 font-semibold">{t("common.appName")}</p>
        <NavLinks groups={groups} />
      </aside>
      <div className="flex min-w-0 flex-1 flex-col">
        <header className="flex items-center justify-between gap-2 border-b px-4 py-2">
          <SheetPrimitive.Root open={menuOpen} onOpenChange={setMenuOpen}>
            <SheetPrimitive.Trigger asChild>
              <Button variant="ghost" size="icon" className="md:hidden" aria-label={t("nav.menu")}>
                <Menu aria-hidden="true" />
              </Button>
            </SheetPrimitive.Trigger>
            <SheetPrimitive.Portal>
              <SheetPrimitive.Overlay className="fixed inset-0 z-50 bg-black/50" />
              <SheetPrimitive.Content className="fixed inset-y-0 left-0 z-50 w-72 max-w-[85vw] overflow-y-auto border-r bg-background p-4">
                <SheetPrimitive.Title className="mb-6 px-3 font-semibold">{t("common.appName")}</SheetPrimitive.Title>
                <SheetPrimitive.Description className="sr-only">{t("nav.main")}</SheetPrimitive.Description>
                <NavLinks groups={groups} onNavigate={() => setMenuOpen(false)} />
                <SheetPrimitive.Close className="absolute top-2 right-2 inline-flex size-11 items-center justify-center rounded-md outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring">
                  <X aria-hidden="true" className="size-4" />
                  <span className="sr-only">{t("common.actions.close")}</span>
                </SheetPrimitive.Close>
              </SheetPrimitive.Content>
            </SheetPrimitive.Portal>
          </SheetPrimitive.Root>
          <span className="font-semibold md:hidden">{t("common.appName")}</span>
          <div className="ml-auto">
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="ghost" aria-label={t("nav.userMenuName", { email: me.data.email })}>
                  <Settings aria-hidden="true" />
                  <span className="hidden max-w-48 truncate sm:inline">{me.data.email}</span>
                  <ChevronDown aria-hidden="true" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuLabel className="truncate">{me.data.email}</DropdownMenuLabel>
                <DropdownMenuSeparator />
                <LanguageMenuSub />
                <DropdownMenuItem asChild>
                  <Link href={path("/settings/security")}>{t("nav.security")}</Link>
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={() => logout.mutate()}>{t("nav.signOut")}</DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </header>
        <main id="main" className="mx-auto w-full max-w-5xl flex-1 px-4 py-6 md:py-8">
          {children}
        </main>
      </div>
    </div>
  );
}
