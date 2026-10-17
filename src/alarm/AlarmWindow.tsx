import { useEffect } from "react";
import { BellRing } from "lucide-react";
import { AlarmActions } from "../components/AlarmActions";
import { startClock, useAppStore } from "../store/useAppStore";
import { PRAYER_EMOJIS, PRAYER_LABELS } from "../lib/types";

/**
 * Content of the always-on-top alarm popup window. The Rust backend shows and
 * hides this window; here we only render the active alarm's message + actions.
 */
export function AlarmWindow() {
  const init = useAppStore((s) => s.init);
  const alarm = useAppStore((s) => s.state?.activeAlarm ?? null);

  useEffect(() => {
    startClock();
    void init();
  }, [init]);

  if (!alarm) {
    // Window is hidden by the backend when no alarm is active.
    return null;
  }

  return (
    <div className="flex h-screen items-center justify-center bg-slate-950 p-4">
      <div className="alarm-pulse w-full max-w-sm rounded-2xl border border-emerald-600/60 bg-slate-900 p-5">
        <div className="mb-4 flex items-start gap-3">
          <div className="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-emerald-600/20 text-2xl">
            {PRAYER_EMOJIS[alarm.prayer]}
          </div>
          <div className="min-w-0">
            <p className="flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wider text-emerald-400">
              <BellRing className="h-3.5 w-3.5" />
              {PRAYER_LABELS[alarm.prayer]} alarm
            </p>
            <h2 className="mt-0.5 text-lg font-semibold leading-snug">
              {alarm.message.title}
            </h2>
            <p className="mt-1 text-sm leading-relaxed text-slate-300">
              {alarm.message.body}
            </p>
          </div>
        </div>
        <AlarmActions />
      </div>
    </div>
  );
}
