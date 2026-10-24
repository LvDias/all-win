"""Take Profit Trader release (Test and PRO) from docs/product/presets/research/take-profit-trader.md
(R2, rev. R4, 2026-10-23). Content approved by the PO (Rumo) per file.

Page titles: title not retrieved: site blocks automated access, 2026-10-24 (HTTP 403 at the title
check). This concerns the titles only: the values come from the research file, whose pages were read
and captured on 2026-10-23. The user's decision (2026-10-24): every blocked title stays blank, with
title_note next to the URL (presets 1.1); nothing is derived. The homepage title is its h1.

Usage: take_profit_trader.py v1 OUT.json (one immutable file per published version, data/presets/releases/<firm>/v<N>.json).
"""
import json, sys
if sys.argv[1] != "v1":
    sys.exit("usage: take_profit_trader.py v1 OUT.json")
D = "2026-10-23"
Z = "https://takeprofittraderhelp.zendesk.com/hc/en-us/articles/"
S = {
    "PLAT": Z + "15173017163549-Choosing-Your-Platform",
    "R1": Z + "15169070804125-Rule-1-Hit-Your-Profit-Target",
    "R2": Z + "15169066911133-Rule-2-Do-Not-Exceed-Maximum-Position-Size",
    "R3": Z + "15170265979165-Rule-3-Do-Not-Hit-End-Of-Day-EOD-Maximum-Trailing-Drawdown",
    "R4": Z + "15170347090461-Rule-4-Trade-Approved-Products-During-Approved-Hours",
    "R5": Z + "15170316538013-Rule-5-Be-Consistent",
    "PRO": Z + "15171769361053-PRO-Account-Rules",
    "UTP": Z + "34431153546397-TakeProfitTrader-Universal-Trading-Policies-UTP",
    "COPY": Z + "34431176505245-Trade-Copier-Policy",
    "VPN": Z + "37085469996573-Registration-Denial-FAQ",
    "HOME": "https://takeprofittrader.com/",
    "TERMS": "https://takeprofittrader.com/terms",
}
TITLE_NOTE = "title not retrieved: site blocks automated access, 2026-10-24"
# The homepage opened (Aquarela, 2026-10-24): its h1 is used.
HOME_TITLE = "We Fund Futures Traders"
def src(u):
    if u == S["HOME"]:
        return {"url": u, "accessed_at": D, "page_title": HOME_TITLE}
    return {"url": u, "accessed_at": D, "title_note": TITLE_NOTE}
def quote(text, u, section=None):
    q = {"text": text, "language": "en", "url": u, "accessed_at": D, "title_note": TITLE_NOTE}
    if section: q["section"] = section
    return q
def verified(value, *urls): return {"status": "verified", "value": value, "sources": [src(u) for u in urls]}

SIZES = [("25k", "25K", "25000.00", "1500.00", "1500.00", 3),
         ("50k", "50K", "50000.00", "2000.00", "3000.00", 6),
         ("75k", "75K", "75000.00", "2500.00", "4500.00", 9),
         ("100k", "100K", "100000.00", "3000.00", "6000.00", 12),
         ("150k", "150K", "150000.00", "4500.00", "9000.00", 15)]
# Research §9 #2 (Rumo, QT-1): "Eastern" (America/New_York, with daylight saving) vs "EST" (a fixed
# UTC-5 zone, IANA Etc/GMT+5): one hour apart for about eight months a year. Not confirmed; the
# user picks. Never mapped to one another.
RESET = {"status": "not_confirmed", "published_values": [
    {"value": {"local_time": "18:00", "time_zone": "America/New_York",
               "published_text": "Trading window: 6 PM Eastern – 5 PM Eastern"}, "sources": [src(S["R4"])]},
    {"value": {"local_time": "18:00", "time_zone": "Etc/GMT+5",
               "published_text": "the end of the trading day (5PM EST)"}, "sources": [src(S["HOME"])]},
]}
NO_DLL = verified({"kind": "none"}, S["HOME"])

# Research §9 #1: the consistency boundary is worded two ways ("below 50%" vs "cannot be more than 50%").
CONSISTENCY_TEXTS = [
    ("Highest profit day / Net P/L = Consistency Percentage. This percentage must be below 50% to qualify for a PRO account.", S["R5"]),
    ("Your biggest trading day cannot be more than 50% of your total profit.", S["HOME"]),
]

def test_size(key, label, amount, mll, target, minis):
    return {"key": key, "size_label": label, "size_amount": amount, "rule_sets": [{"rules": {
        "max_position_qty": verified({"contracts": minis * 10, "published_text": f"{minis} Contracts / {minis*10} Micros"}, S["R2"], S["HOME"]),
        # The firm's general sentence (homepage tooltip); Rule 3 words it for the 25K example only.
        "drawdown": verified({"type": "trailing_eod", "amount": mll,
                              "stops_trailing_at": "The drawdown across all three account types will STOP trailing once it has reached the initial account balance.",
                              "trailing_lock": {"mode": "floor_reaches_start"}}, S["R3"], S["HOME"]),
        "daily_loss_limit": NO_DLL,
        "trading_day_reset": RESET,
        "consistency_rule": {"status": "not_confirmed", "published_values": [
            {"value": {"text": t, "threshold_pct": "50"}, "sources": [src(u)]} for t, u in CONSISTENCY_TEXTS]},
        "evaluation_profit_target": verified({"amount": target}, S["R1"], S["HOME"]),
    }, "stored_only": {"min_trading_days": verified({"days": 3}, S["R5"], S["HOME"])}}]}

def pro_size(key, label, amount, mll, target, minis):
    return {"key": key, "size_label": label, "size_amount": amount, "rule_sets": [{"rules": {
        # The firm publishes that the PRO position limit and drawdown equal the passed Test's.
        "max_position_qty": verified({"contracts": minis * 10, "published_text": f"{minis} Contracts / {minis*10} Micros"}, S["R2"], S["HOME"]),
        "drawdown": verified({"type": "trailing_intraday", "amount": mll,
                              "stops_trailing_at": "The drawdown continues to trail profits until it reaches your starting balance; once it reaches that level, it no longer moves.",
                              "trailing_lock": {"mode": "floor_reaches_start"}}, S["PRO"]),
        "daily_loss_limit": NO_DLL,
        "trading_day_reset": RESET,
    }}]}

release = {
    "schema_version": "1.1",
    "firm": {"slug": "take-profit-trader", "name": "Take Profit Trader",
             "domains": ["takeprofittrader.com", "takeprofittraderhelp.zendesk.com"]},
    "release": {"version_number": 1, "verified_on": D, "research_file": "docs/product/presets/research/take-profit-trader.md", "changes": []},
    # The trader picks the data feed at purchase; no trading rule differs by feed (research §2).
    "platforms": [{"code": "cqg", "label": "CQG"}, {"code": "rithmic", "label": "Rithmic"}],
    "stances": {
        "automation": [{"kind": "automation", "version": 1, "firm_stance": "forbidden", "quotes": [
            quote("#1: No Trading Bots or Algos. Automated trading systems, bots, or algorithmic execution tools are not permitted.", S["UTP"])]}],
        "copy": [{"kind": "copy", "version": 1, "firm_stance": "conditional", "quotes": [
            quote("TakeProfitTrader permits the use of trade copiers solely for managing accounts owned and controlled by you.", S["COPY"])]}],
        "vps": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src(S["UTP"]), src(S["VPN"]), src(S["TERMS"])]}},
        "personal_device": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src(S["UTP"]), src(S["TERMS"])]}},
    },
    "plans": [
        {"key": "test", "name": "Test", "phases": [{"phase": "evaluation", "sizes": [test_size(*s) for s in SIZES]}]},
        {"key": "pro", "name": "PRO", "phases": [{"phase": "funded", "sizes": [pro_size(*s) for s in SIZES]}]},
    ],
    "excluded_plans": [{"name": n, "reason": {
        "pt-BR": "Excluído dos presets da V1 por decisão do usuário (S-4, 2026-10-23); cadastre manualmente.",
        "en": "Excluded from V1 presets by the user's decision (S-4, 2026-10-23); register it manually."}}
        for n in ["PRO+", "PRO+ Development"]],
}
json.dump(release, open(sys.argv[2], "w"), indent=2, ensure_ascii=False)
