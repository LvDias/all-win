"use client";

import { useRef, type ChangeEvent, type ClipboardEvent } from "react";
import { cn } from "@/shared/lib/utils";

export const defaultCodeLength = 8;
const groupSize = 4;
const groupSeparator = "–";

/** Removes separators and look-alike noise, uppercases and limits the length. The Core is the judge of the value. */
export function normalizeCode(raw: string, length: number): string {
  return raw.replace(/[^A-Za-z0-9]/g, "").toUpperCase().slice(0, length);
}

type CodeInputProps = {
  id: string;
  value: string;
  onChange: (value: string) => void;
  onComplete?: (value: string) => void;
  length?: number;
  readOnly?: boolean;
  disabled?: boolean;
  invalid?: boolean;
  describedBy?: string;
  autoFocus?: boolean;
};

/**
 * One text input (so paste, autofill and assistive technology see a single field) drawn as slots.
 * The slots are decoration: they are hidden from assistive technology and never focusable.
 */
export function CodeInput({ id, value, onChange, onComplete, length = defaultCodeLength, readOnly, disabled, invalid, describedBy, autoFocus }: CodeInputProps) {
  const completed = useRef<string | null>(null);

  function apply(raw: string) {
    const next = normalizeCode(raw, length);
    onChange(next);
    if (next.length === length && completed.current !== next) {
      completed.current = next;
      onComplete?.(next);
    }
    if (next.length < length) completed.current = null;
  }

  return (
    <div className="group relative w-fit max-w-full">
      <input
        id={id}
        type="text"
        inputMode="text"
        autoComplete="one-time-code"
        autoCapitalize="characters"
        autoCorrect="off"
        spellCheck={false}
        enterKeyHint="done"
        maxLength={length * 2}
        value={value}
        readOnly={readOnly}
        disabled={disabled}
        autoFocus={autoFocus}
        aria-invalid={invalid || undefined}
        aria-describedby={describedBy}
        onChange={(event: ChangeEvent<HTMLInputElement>) => apply(event.target.value)}
        onPaste={(event: ClipboardEvent<HTMLInputElement>) => {
          event.preventDefault();
          apply(event.clipboardData.getData("text"));
        }}
        className="peer absolute inset-0 z-10 size-full cursor-text bg-transparent text-transparent caret-transparent outline-none"
      />
      <div aria-hidden="true" className="flex flex-wrap items-center gap-x-2 gap-y-2">
        {Array.from({ length }, (_, index) => (
          <span key={index} className="contents">
            {index > 0 && index % groupSize === 0 ? <span className="text-muted-foreground">{groupSeparator}</span> : null}
            <span
              className={cn(
                "flex h-12 w-8 items-center justify-center rounded-md border bg-background font-mono text-2xl uppercase sm:w-10",
                invalid ? "border-status-danger" : "border-input",
                (readOnly || disabled) && "bg-muted",
                index === value.length && "peer-focus:ring-2 peer-focus:ring-ring",
              )}
            >
              {value[index] ?? ""}
            </span>
          </span>
        ))}
      </div>
    </div>
  );
}
