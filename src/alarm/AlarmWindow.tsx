import { useEffect } from "react";
import { BellRing, CheckCircle2 } from "lucide-react";
import { AlarmActions } from "../components/AlarmActions";
import { startClock, useAppStore } from "../store/useAppStore";
import { PRAYER_EMOJIS, PRAYER_LABELS } from "../lib/types";
import { countdown } from "../lib/time";

/**
 * Content of the always-on-top alarm popup window. The Rust backend shows and
 * hides this window; here we render the active alarm's message, the prayer's
 * done status, a live countdown to the window end, and the actions.
 */
export function AlarmWindow() {
  const init = useAppStore((s) => s.init);
  const alarm = useAppStore((s) => s.state?.activeAlarm ?? null);
  const done = useAppStore((s) => {
    const prayer = s.state?.activeAlarm?.prayer;
    return prayer ? (s.state?.doneToday ?? {})[prayer] === true : false;
  });
  const now = useAppStore((s) => s.now);

  useEffect(() => {
    startClock();
    void init();
  }, [init]);

  if (!alarm) {
    // Window is hidden by the backend when no alarm is active.
    return null;
  }

  const endMs = new Date(alarm.endsAt).getTime();
  const endsIn = isNaN(endMs) ? null : countdown(endMs - now);

  return (
    <div className="flex h-screen items-center justify-center bg-slate-950 p-4">
      <div className="alarm-pulse w-full max-w-sm rounded-2xl border border-emerald-600/60 bg-slate-900 p-5">
        <div className="mb-3 flex items-start gap-3">
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

        <div className="mb-4 flex items-center justify-between gap-3 rounded-lg border border-slate-800 bg-slate-950/60 px-3 py-2 text-xs">
          <span className={done ? "flex items-center gap-1 text-emerald-400" : "text-slate-400"}>
            {done ? (
              <>
                <CheckCircle2 className="h-3.5 w-3.5" /> Marked done
              </>
            ) : (
              "Not marked done yet"
            )}
          </span>
          {endsIn && <span className="font-mono text-slate-400">ends in {endsIn}</span>}
        </div>

        <AlarmActions />
      </div>
    </div>
  );
}
