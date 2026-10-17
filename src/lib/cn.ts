/** Tiny class-name joiner — avoids pulling in clsx/tailwind-merge. */
export function cn(...parts: Array<string | false | null | undefined>): string {
  return parts.filter(Boolean).join(" ");
}
