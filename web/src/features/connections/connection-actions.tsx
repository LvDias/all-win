"use client";

import { ArrowRightLeft, KeyRound, MoreVertical, Pencil, Unlink } from "lucide-react";
import { useTranslations } from "next-intl";
import type { Connection } from "@/shared/api/types";
import { Button } from "@/shared/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/shared/ui/dropdown-menu";

type Props = { connection: Connection; onRename: () => void; onNewCode: () => void; onMove: () => void; onUnpair: () => void };

/** Unpaired connections are read-only history and have no actions (Q18). */
export function ConnectionActions({ connection, onRename, onNewCode, onMove, onUnpair }: Props) {
  const t = useTranslations("connections.actions");
  if (connection.status === "unpaired") return null;
  const awaiting = connection.status === "awaiting_pairing";
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" aria-label={t("for", { name: connection.name })}>
          <MoreVertical aria-hidden="true" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onSelect={onRename}>
          <Pencil aria-hidden="true" />
          {t("rename")}
        </DropdownMenuItem>
        {awaiting ? (
          <DropdownMenuItem onSelect={onNewCode}>
            <KeyRound aria-hidden="true" />
            {t("newCode")}
          </DropdownMenuItem>
        ) : (
          <DropdownMenuItem onSelect={onMove}>
            <ArrowRightLeft aria-hidden="true" />
            {t("move")}
          </DropdownMenuItem>
        )}
        <DropdownMenuItem onSelect={onUnpair} className="text-status-danger">
          <Unlink aria-hidden="true" />
          {t("unpair")}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
