import { BellRing, CheckCircle2 } from "lucide-react";
import { useAppState, useNow } from "../store/useAppStore";
import { AlarmActions } from "./AlarmActions";
import { countdown } from "../lib/time";

/** In-app banner shown while an alarm is active (same actions as the popup). */
export function AlarmBanner() {
  const state = useAppState();
  const now = useNow();
  const alarm = state?.activeAlarm;
  if (!alarm) return null;

  const done = (state?.doneToday ?? {})[alarm.prayer] === true;
  const endMs = new Date(alarm.endsAt).getTime();
  const endsIn = isNaN(endMs) ? null : countdown(endMs - now);

  return (
    <div className="alarm-pulse fade-in rounded-2xl border border-emerald-600/60 bg-emerald-950/30 p-5">
      <div className="mb-4 flex items-start gap-3">
        <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-emerald-600/20 text-emerald-400">
          <BellRing className="h-5 w-5" />
        </div>
        <div>
          <h3 className="font-semibold text-emerald-300">{alarm.message.title}</h3>
          <p className="mt-0.5 text-sm text-slate-300">{alarm.message.body}</p>
        </div>
      </div>
      <div className="mb-4 flex items-center justify-between gap-3 rounded-lg border border-slate-800 bg-slate-950/40 px-3 py-2 text-xs">
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
  );
}
