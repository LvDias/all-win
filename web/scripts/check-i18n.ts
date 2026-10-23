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

/** ICU argument names in a message, including those inside plural/select arms. */
export function placeholders(message: string): string[] {
  return [...new Set([...message.matchAll(/\{\s*([A-Za-z_][A-Za-z0-9_]*)\s*[,}]/g)].map((m) => m[1] ?? ""))].sort();
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
