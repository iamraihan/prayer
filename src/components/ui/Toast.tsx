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
 * and can also be dismissed by clicking. The wrapper centres the toast
 * and stays click-through; only the toast itself catches clicks.
 */
export function Toast({ toast, onDismiss }: { toast: ToastData; onDismiss: () => void }) {
  // Keep the latest callback without making it a timer dep (a new identity
  // from parent re-renders must not restart the countdown). Written in an
  // effect, never during render — declared first so it runs before the timer.
  const dismissRef = useRef(onDismiss);
  useEffect(() => {
    dismissRef.current = onDismiss;
  });

  useEffect(() => {
    const t = setTimeout(() => dismissRef.current(), toast.kind === "error" ? 6000 : 3000);
    return () => clearTimeout(t);
  }, [toast]);

  return (
    <div className="pointer-events-none fixed inset-x-0 bottom-6 z-[60] flex justify-center px-4">
      <div
        role={toast.kind === "error" ? "alert" : "status"}
        onClick={onDismiss}
        className={cn(
          "toast-in pointer-events-auto flex max-w-md cursor-pointer items-center gap-2.5",
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
    </div>
  );
}
