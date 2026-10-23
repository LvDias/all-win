"use client";

import { useEffect, useRef, type ComponentProps } from "react";
import { cn } from "@/shared/lib/utils";

type OtpInputProps = Omit<ComponentProps<"input">, "value" | "onChange" | "maxLength" | "type"> & {
  value: string;
  onChange: (value: string) => void;
  /** Called once when the sixth digit is entered. */
  onComplete?: (value: string) => void;
};

const length = 6;

/** Six visual slots over one real input, so paste, autofill and `one-time-code` keep working. */
export function OtpInput({ value, onChange, onComplete, className, disabled, readOnly, ...props }: OtpInputProps) {
  const lastCompleted = useRef("");
  useEffect(() => {
    if (value.length < length) lastCompleted.current = "";
  }, [value]);

  function handleChange(raw: string) {
    const next = raw.replace(/\D/g, "").slice(0, length);
    onChange(next);
    if (next.length === length && lastCompleted.current !== next) {
      lastCompleted.current = next;
      onComplete?.(next);
    }
  }

  return (
    <div className={cn("relative inline-flex gap-2", className)}>
      {Array.from({ length }, (_, index) => (
        <div
          key={index}
          aria-hidden="true"
          className={cn("flex size-11 items-center justify-center rounded-md border border-input bg-background font-mono text-lg", index === Math.min(value.length, length - 1) && !disabled && "ring-2 ring-ring")}
        >
          {value[index] ?? ""}
        </div>
      ))}
      <input
        {...props}
        value={value}
        onChange={(event) => handleChange(event.target.value)}
        inputMode="numeric"
        autoComplete="one-time-code"
        pattern="\d{6}"
        maxLength={length}
        disabled={disabled}
        readOnly={readOnly}
        spellCheck={false}
        className="absolute inset-0 w-full cursor-text bg-transparent text-transparent caret-transparent opacity-0 outline-none"
      />
    </div>
  );
}
