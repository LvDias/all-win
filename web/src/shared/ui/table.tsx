import type { ComponentProps } from "react";
import { cn } from "@/shared/lib/utils";

export function Table({ className, ...props }: ComponentProps<"table">) {
  return (
    <div className="w-full overflow-x-auto">
      <table className={cn("w-full caption-bottom text-sm", className)} {...props} />
    </div>
  );
}
export const TableHeader = (props: ComponentProps<"thead">) => <thead className="[&_tr]:border-b" {...props} />;
export const TableBody = (props: ComponentProps<"tbody">) => <tbody {...props} />;
export function TableRow({ className, ...props }: ComponentProps<"tr">) {
  return <tr className={cn("border-b last:border-0", className)} {...props} />;
}
export function TableHead({ className, ...props }: ComponentProps<"th">) {
  return <th scope="col" className={cn("h-11 px-3 text-left align-middle font-medium whitespace-nowrap text-muted-foreground", className)} {...props} />;
}
export function TableCell({ className, ...props }: ComponentProps<"td">) {
  return <td className={cn("px-3 py-3 align-middle", className)} {...props} />;
}
