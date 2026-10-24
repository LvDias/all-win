"""Leeloo Trading release (Foundation (LL): Practice Account and Performance Account) from
docs/product/presets/research/leeloo-trading.md (R3, 2026-10-23; PO recheck rows 47-50, 2026-10-24).
Content approved by the PO (Rumo) per file.

Usage: leeloo_trading.py v1 OUT.json (one immutable file per published version, data/presets/releases/<firm>/v<N>.json).

The knowledge base is behind a login, so the preset carries only what the public pages publish (row 47;
the Q94/Q97 principle; the PO's night decision "only the published", reversible with a v2):
- Practice Account drawdown: amount and trailing rule published, the lock rule is in KB a44 (row 48):
  `not_confirmed` / `partial`;
- Performance Account drawdown, maximum position and daily loss: not on any public page:
  `not_confirmed` / `not_published`;
- trading-day reset: not published (row 48, ASK); commission: absent (empty + warning).
Copy (row 49): re-read the public news item on copiers (2024-09-26) as the PO asked: same-person copying
is regulated (at most three accounts copied at once; forbidden on some account types), cross-user
copying is forbidden: `conditional` with those texts.
Page titles are the pages' h1, read on 2026-10-24 (plain HTTP); the values are the research's (2026-10-23).
Every quote is the page's literal text (checked against the fetched pages before review).
"""
import json, sys
if len(sys.argv) != 3 or sys.argv[1] != "v1":
    sys.exit("usage: leeloo_trading.py v1 OUT.json")
D = "2026-10-23"
KB_NOTE = "knowledge base article behind a login: not readable"
# Research source index: key -> (url, page h1 or None, title_note).
S = {
    "S1": ("https://leelootrading.com/", "Leeloo™ Where education meets aspiration", None),
    "S2": ("https://leelootrading.com/support", "We're here to help...", None),
    "S3": ("https://leelootrading.com/terms-of-service",
           "Natural Trading, LLC dba Leeloo Trading Disclaimers, Terms of Use, Terms and Condition, and Refund Policy", None),
    "S4": ("https://leelootrading.com/merchant-products", "Leeloo Store", None),
    "KB": ("https://support.leelootrading.com/", "Search the Portal", None),
    "NEWS": ("https://support.leelootrading.com/news/32/restrictions-on-copying-between-performance-accounts.aspx",
             "Restrictions on Copying between Performance Accounts", None),
    "A44": ("https://support.leelootrading.com/kb/a44/rising-trailing-max-drawdown-explained.aspx", None, KB_NOTE),
    "A103": ("https://support.leelootrading.com/kb/a103/2_-holding-positions-during-the-close.aspx", None, KB_NOTE),
}

def src(k):
    url, title, note = S[k]
    s = {"url": url, "accessed_at": D}
    if title: s["page_title"] = title
    else: s["title_note"] = note
    return s

def quote(text, k):
    url, title, note = S[k]
    q = {"text": text, "language": "en", "url": url, "accessed_at": D}
    if title: q["page_title"] = title
    else: q["title_note"] = note
    return q

def verified(value, *keys): return {"status": "verified", "value": value, "sources": [src(k) for k in keys]}

def partial(quotes, *searched):
    return {"status": "not_confirmed", "not_confirmed_reason": "partial", "quotes": quotes,
            "research": {"searched_on": D, "sources": [src(k) for k in searched]}}

def not_published(*searched):
    return {"status": "not_confirmed", "not_confirmed_reason": "not_published",
            "research": {"searched_on": D, "sources": [src(k) for k in searched]}}

def money(n): return f"{n}.00"
def usd(n): return f"${n:,}"

# Plan cards (S1): size (K) -> (name, minis, profit target, trailing drawdown).
SIZES = {25: ("Aspire", 3, 1500, 1500), 50: ("Launch", 8, 3000, 2500), 100: ("Climb", 12, 6000, 3000),
         150: ("Cruise", 15, 9000, 5000), 250: ("Burst", 25, 15000, 6500), 300: ("Explode", 30, 20000, 7500)}

LTMAB = [
    "The Limited Trailing Minimum Account Balance (LTMAB) or Auto Liquidate Threshold Value (ALTV) is a rising trailing max drawdown that tracks your open equity intra-trade, based on the unrealized highest profit point in your Practice Account.",
    "This limit trails your highest unrealized profit throughout the Practice Account's life.",
]
PA_30 = ("30% Rule: Daily profits in a PA should not exceed 30% of the net profit from the initial starting balance, "
         "promoting responsible risk management and consistent trading growth.")

# Rumo (format rev.3): the sentence is quoted with the page's list, one item per line, so the account types it
# covers are visible.
COPY_RESTRICTED = "\n".join([
    "Traders are not permitted to manually copy, use copiers or any type of copy software on the following account types:",
    "Accelerator Accounts",
    "Accounts with either 10 day or 15 day payout Edge Enhancers",
    "Accounts with the WAIVE 30% Rule Edge Enhancer",
    "Accounts with the Reduce Rule (from 30% to 50%) Edge Enhancer",
    "Weekly accounts",
    "Third-party accounts",
])

def rules(n, phase):
    name, minis, target, dd = SIZES[n]
    reset = not_published("S1", "S2", "S3", "A103")
    if phase == "evaluation":
        r = {
            "max_position_qty": verified({"contracts": minis * 10, "published_text": f"{minis} minis | {minis * 10} micros"}, "S1"),
            "drawdown": partial([quote(f"{usd(dd)} Trailing Drawdown", "S1")] + [quote(t, "S1") for t in LTMAB],
                                "S1", "S2", "A44"),
            "daily_loss_limit": verified({"kind": "none"}, "S1"),
            "trading_day_reset": reset,
            "evaluation_profit_target": verified({"amount": money(target)}, "S1"),
        }
        return {"rules": r, "stored_only": {"min_trading_days": verified({"days": 10}, "S1", "S2")}}
    r = {
        "max_position_qty": not_published("S1", "S2", "S3"),
        "drawdown": not_published("S1", "S2", "S3", "A44"),
        "daily_loss_limit": not_published("S1", "S2", "S3"),
        "trading_day_reset": reset,
        "consistency_rule": verified({"text": PA_30, "threshold_pct": "30"}, "S1", "S2"),
    }
    return {"rules": r}

def size(n, phase):
    name = SIZES[n][0]
    return {"key": f"{n}k", "size_label": f"{name} {usd(n * 1000)}", "size_amount": money(n * 1000),
            "rule_sets": [rules(n, phase)]}

release = {
    "schema_version": "1.2",
    "firm": {"slug": "leeloo-trading", "name": "Leeloo Trading", "domains": ["leelootrading.com", "support.leelootrading.com"]},
    "release": {"version_number": 1, "verified_on": D, "research_file": "docs/product/presets/research/leeloo-trading.md", "changes": []},
    "platforms": [{"code": "rithmic", "label": "Rithmic"}],
    "stances": {
        "automation": [{"kind": "automation", "version": 1, "firm_stance": "unknown", "quotes": [],
                        "research": {"searched_on": D, "sources": [src("S1"), src("S2"), src("S3"), src("S4")]}}],
        "copy": [{"kind": "copy", "version": 1, "firm_stance": "conditional", "quotes": [
            quote("Copying trades across users is expressly prohibited and your accounts will be closed.", "S3"),
            quote(COPY_RESTRICTED, "NEWS"),
            quote("Traders are also restricted from copying more than three accounts, simultaneously.", "NEWS")]}],
        "vps": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S1"), src("S2"), src("S3"), src("KB")]}},
        "personal_device": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S1"), src("S2"), src("S3"), src("KB")]}},
    },
    "plans": [
        {"key": "foundation", "name": "Foundation (LL)", "phases": [
            {"phase": "evaluation", "sizes": [size(n, "evaluation") for n in SIZES]},
            {"phase": "funded", "sizes": [size(n, "funded") for n in SIZES]},
        ]},
    ],
    "excluded_plans": [],
}
json.dump(release, open(sys.argv[2], "w", encoding="utf-8", newline=""), indent=2, ensure_ascii=False)
