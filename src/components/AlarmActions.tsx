import { useState } from "react";
import { AlarmClock, BellOff, Check } from "lucide-react";
import { Button } from "./ui/Button";
import { alarmDismiss, alarmDone, alarmSnooze } from "../lib/actions";
import { cn } from "../lib/cn";

const SNOOZE_PRESETS = [5, 10, 15, 30];

/**
 * Actions available while an alarm is active:
 * mark done (silences this prayer for the rest of the day),
 * remind-me-later (snooze N minutes, with a custom input),
 * or dismiss (stop the sound; later reminders still fire).
 *
 * Shared by the in-app banner and the standalone alarm popup window.
 */
export function AlarmActions() {
  const [minutes, setMinutes] = useState(10);
  const [busy, setBusy] = useState(false);

  async function run(fn: () => Promise<void>) {
    if (busy) return;
    setBusy(true);
    try {
      await fn();
    } catch {
      setBusy(false);
    }
  }

  return (
    <div className="space-y-3">
      <Button
        variant="primary"
        size="lg"
        className="w-full"
        disabled={busy}
        onClick={() => void run(alarmDone)}
      >
        <Check className="h-5 w-5" /> Mark as done
      </Button>

      <div>
        <p className="mb-1.5 text-xs font-medium text-slate-400">
          Remind me again in
        </p>
        <div className="flex flex-wrap items-center gap-2">
          {SNOOZE_PRESETS.map((m) => (
            <Button
              key={m}
              size="sm"
              disabled={busy}
              onClick={() => void run(() => alarmSnooze(m))}
            >
              <AlarmClock className="h-3.5 w-3.5" /> {m} min
            </Button>
          ))}
          <div className="flex items-center gap-1.5">
            <input
              type="number"
              min={1}
              max={720}
              value={minutes}
              onChange={(e) => setMinutes(Number(e.target.value))}
              className={cn(
                "h-8 w-20 rounded-lg border border-slate-700 bg-slate-950/60 px-2 text-xs text-slate-100",
                "focus:border-emerald-500 focus:outline-none",
              )}
              aria-label="Custom minutes"
            />
            <Button
              size="sm"
              disabled={busy || !Number.isFinite(minutes) || minutes < 1}
              onClick={() => void run(() => alarmSnooze(minutes))}
            >
              Remind me
            </Button>
          </div>
        </div>
      </div>

      <Button
        variant="ghost"
        className="w-full"
        disabled={busy}
        onClick={() => void run(alarmDismiss)}
      >
        <BellOff className="h-4 w-4" /> Dismiss for now
      </Button>
    </div>
  );
}
