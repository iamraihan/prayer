import { BellRing } from "lucide-react";
import { useAppState } from "../store/useAppStore";
import { AlarmActions } from "./AlarmActions";

/** In-app banner shown while an alarm is active (same actions as the popup). */
export function AlarmBanner() {
  const alarm = useAppState()?.activeAlarm;
  if (!alarm) return null;

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
      <AlarmActions />
    </div>
  );
}
