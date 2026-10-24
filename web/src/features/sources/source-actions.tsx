"use client";

import { Ban, KeyRound, MoreVertical, Pencil } from "lucide-react";
import { useTranslations } from "next-intl";
import type { Source } from "@/shared/api/types";
import { Button } from "@/shared/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/shared/ui/dropdown-menu";

type Props = { source: Source; onRename: () => void; onRotate: () => void; onRevoke: () => void };

/** Row actions. A revoked source keeps its name for history and has no actions (D4). */
export function SourceActions({ source, onRename, onRotate, onRevoke }: Props) {
  const t = useTranslations("sources.actions");
  if (source.status === "revoked") return null;
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" aria-label={t("for", { name: source.name })}>
          <MoreVertical aria-hidden="true" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onSelect={onRename}>
          <Pencil aria-hidden="true" />
          {t("rename")}
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={onRotate}>
          <KeyRound aria-hidden="true" />
          {t("rotate")}
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={onRevoke} className="text-status-danger">
          <Ban aria-hidden="true" />
          {t("revoke")}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
