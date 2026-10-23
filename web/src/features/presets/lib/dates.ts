import { formatDate } from "@/shared/format";
import type { Locale } from "@/shared/i18n/config";

/** A calendar date from the Core (`YYYY-MM-DD`, no zone) shown as that same day in every time zone. */
export const formatCalendarDate = (day: string, locale: Locale): string => formatDate(`${day}T12:00:00Z`, locale, "UTC");
