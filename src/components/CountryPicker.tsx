import { useMemo, useState } from "react";
import { ChevronDown, Search } from "lucide-react";
import { COUNTRIES, type Country } from "../lib/countries";
import { cn } from "../lib/cn";

export interface CountryPickerProps {
  value: Country | null;
  onChange: (country: Country | null) => void;
}

/** Searchable country dropdown. The Aladhan API receives the ISO alpha-2 code. */
export function CountryPicker({ value, onChange }: CountryPickerProps) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return COUNTRIES;
    return COUNTRIES.filter(
      (c) =>
        c.name.toLowerCase().includes(q) || c.code.toLowerCase() === q,
    );
  }, [query]);

  return (
    <div className="relative">
      <button
        type="button"
        onClick={() => {
          setOpen((v) => !v);
          setQuery("");
        }}
        className={cn(
          "flex h-10 w-full items-center justify-between rounded-lg border border-slate-700 bg-slate-950/60 px-3 text-sm",
          "hover:border-slate-500 focus-visible:border-emerald-500 focus-visible:outline-none",
        )}
      >
        <span className={value ? "text-slate-100" : "text-slate-500"}>
          {value ? value.name : "Select a country…"}
        </span>
        <ChevronDown className="h-4 w-4 text-slate-400" />
      </button>

      {open && (
        <>
          <div
            className="fixed inset-0 z-10"
            onClick={() => setOpen(false)}
          />
          <div className="absolute z-20 mt-1 w-full rounded-lg border border-slate-700 bg-slate-900 shadow-xl shadow-black/40">
            <div className="flex items-center gap-2 border-b border-slate-800 px-3 py-2">
              <Search className="h-4 w-4 shrink-0 text-slate-500" />
              <input
                autoFocus
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search country…"
                className="w-full bg-transparent text-sm text-slate-100 placeholder:text-slate-500 focus:outline-none"
              />
            </div>
            <ul className="max-h-56 overflow-y-auto py-1">
              {filtered.length === 0 && (
                <li className="px-3 py-2 text-sm text-slate-500">No match</li>
              )}
              {filtered.map((c) => (
                <li key={c.code}>
                  <button
                    type="button"
                    onClick={() => {
                      onChange(c);
                      setOpen(false);
                    }}
                    className={cn(
                      "w-full px-3 py-1.5 text-left text-sm hover:bg-slate-800",
                      c.code === value?.code
                        ? "text-emerald-400"
                        : "text-slate-200",
                    )}
                  >
                    {c.name}
                    <span className="ml-2 text-xs text-slate-500">{c.code}</span>
                  </button>
                </li>
              ))}
            </ul>
          </div>
        </>
      )}
    </div>
  );
}
