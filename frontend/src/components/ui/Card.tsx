import { HTMLAttributes } from "react";
import { cn } from "@/lib/utils";

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  title?: string;
  headerAction?: React.ReactNode;
}

export function Card({ title, headerAction, children, className, ...props }: CardProps) {
  return (
    <div
      className={cn(
        "bg-surface border border-border rounded-lg",
        className
      )}
      {...props}
    >
      {title && (
        <div className="flex items-center justify-between px-5 py-3 border-b border-border">
          <h2 className="text-text-primary font-semibold text-sm uppercase tracking-wide">
            {title}
          </h2>
          {headerAction && <div>{headerAction}</div>}
        </div>
      )}
      <div className="p-5">{children}</div>
    </div>
  );
}
