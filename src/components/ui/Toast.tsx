import { useEffect, useRef } from "react";
import { AlertTriangle, CheckCircle2 } from "lucide-react";
import { cn } from "../../lib/cn";

export type ToastKind = "success" | "error";

export interface ToastData {
  kind: ToastKind;
  message: string;
}

/**
 * Bottom-centre transient notice. Auto-dismisses (errors linger longer)
 * and can also be dismissed by clicking.
 */
export function Toast({ toast, onDismiss }: { toast: ToastData; onDismiss: () => void }) {
  // Ref so a new `onDismiss` identity from re-renders doesn't restart the timer.
  const dismissRef = useRef(onDismiss);
  dismissRef.current = onDismiss;

  useEffect(() => {
    const t = setTimeout(() => dismissRef.current(), toast.kind === "error" ? 6000 : 3000);
    return () => clearTimeout(t);
  }, [toast]);

  return (
    <div
      role={toast.kind === "error" ? "alert" : "status"}
      onClick={onDismiss}
      className={cn(
        "toast-in fixed bottom-6 left-1/2 z-[60] flex max-w-md -translate-x-1/2 cursor-pointer items-center gap-2.5",
        "rounded-xl border px-4 py-2.5 text-sm shadow-lg shadow-slate-950/60 backdrop-blur",
        toast.kind === "success"
          ? "border-emerald-700/60 bg-emerald-950/85 text-emerald-200"
          : "border-red-800/60 bg-red-950/85 text-red-200",
      )}
    >
      {toast.kind === "success" ? (
        <CheckCircle2 className="h-4 w-4 shrink-0" />
      ) : (
        <AlertTriangle className="h-4 w-4 shrink-0" />
      )}
      <span>{toast.message}</span>
    </div>
  );
}
