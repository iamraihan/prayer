import type { HTMLAttributes } from "react";
import { cn } from "../../lib/cn";

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  title?: string;
  description?: string;
}

export function Card({ title, description, className, children, ...rest }: CardProps) {
  return (
    <section
      className={cn(
        "rounded-2xl border border-slate-800 bg-slate-900/60 p-5",
        className,
      )}
      {...rest}
    >
      {(title || description) && (
        <header className="mb-4">
          {title && <h2 className="text-sm font-semibold text-slate-200">{title}</h2>}
          {description && (
            <p className="mt-1 text-xs leading-relaxed text-slate-400">{description}</p>
          )}
        </header>
      )}
      {children}
    </section>
  );
}
