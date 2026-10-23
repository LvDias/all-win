// Key parity between catalogs (root CLAUDE.md §5): every key exists in both, and empty strings count as missing.
import { readFileSync } from "node:fs";

type Tree = { [key: string]: string | Tree };

export function flatten(tree: Tree, prefix = ""): Map<string, string> {
  const out = new Map<string, string>();
  for (const [key, value] of Object.entries(tree)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "string") out.set(path, value);
    else for (const [k, v] of flatten(value, path)) out.set(k, v);
  }
  return out;
}

/** Index of the `}` that closes the `{` at `open`, or -1. */
function closing(message: string, open: number): number {
  let depth = 0;
  for (let i = open; i < message.length; i++) {
    if (message[i] === "{") depth++;
    else if (message[i] === "}" && --depth === 0) return i;
  }
  return -1;
}

function collect(message: string, found: Set<string>): void {
  for (let i = 0; i < message.length; i++) {
    if (message[i] !== "{") continue;
    const end = closing(message, i);
    if (end < 0) return;
    const body = message.slice(i + 1, end);
    const comma = body.indexOf(",");
    const name = (comma < 0 ? body : body.slice(0, comma)).trim();
    if (/^[A-Za-z_][A-Za-z0-9_]*$/.test(name)) found.add(name);
    if (comma >= 0) {
      // plural/select arms: `selector {text}`; the text may hold more arguments.
      const rest = body.slice(comma + 1);
      for (let j = rest.indexOf("{"); j >= 0; j = rest.indexOf("{", j + 1)) {
        const armEnd = closing(rest, j);
        if (armEnd < 0) break;
        collect(rest.slice(j + 1, armEnd), found);
        j = armEnd;
      }
    }
    i = end;
  }
}

/** ICU argument names in a message, including those inside plural/select arms (arm text is not an argument). */
export function placeholders(message: string): string[] {
  const found = new Set<string>();
  collect(message, found);
  return [...found].sort();
}

export function compareCatalogs(a: Map<string, string>, b: Map<string, string>, nameA: string, nameB: string): string[] {
  const problems: string[] = [];
  for (const [key, value] of a) {
    const other = b.get(key);
    if (other === undefined) problems.push(`${key}: missing in ${nameB}`);
    else if (placeholders(value).join() !== placeholders(other).join()) problems.push(`${key}: placeholders differ between ${nameA} and ${nameB}`);
    if (value.trim() === "") problems.push(`${key}: empty in ${nameA}`);
  }
  for (const [key, value] of b) {
    if (!a.has(key)) problems.push(`${key}: missing in ${nameA}`);
    if (value.trim() === "") problems.push(`${key}: empty in ${nameB}`);
  }
  return problems;
}

function load(name: string): Map<string, string> {
  const file = new URL(`../messages/${name}.json`, import.meta.url);
  return flatten(JSON.parse(readFileSync(file, "utf8")) as Tree);
}

if (process.argv[1] && import.meta.url === new URL(`file://${process.argv[1]}`).href) {
  const problems = compareCatalogs(load("pt-BR"), load("en"), "pt-BR", "en");
  if (problems.length > 0) {
    console.error(`i18n parity failed (${problems.length}):\n${problems.map((p) => `  ${p}`).join("\n")}`);
    process.exit(1);
  }
  console.log("i18n parity ok");
}
