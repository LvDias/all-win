"""Earn2Trade release (Trader Career Path, Gauntlet Mini) from docs/product/presets/research/earn2trade.md
(R3, 2026-10-23; PO recheck rows 32-38, 2026-10-24). Content approved by the PO (Rumo) per file.

Usage: earn2trade.py v1 OUT.json (one immutable file per published version, data/presets/releases/<firm>/v<N>.json).

Presets 1.2: a field the firm publishes only in part is `not_confirmed` with `not_confirmed_reason`
`partial` (the firm's verbatim text + where the rest was searched); nothing is derived (Compasso,
Rumo, 2026-10-24):
- funded / live daily loss: the amount is published, hard vs soft is not (row 33);
- fixed drawdown 200K (TCP25 path) / 400K: published as a minimum balance, not an amount (row 34);
- maximum position: a Progression Ladder that grows with profit (PROVISIONAL, Torno: the top step
  would be a limit the account is not held to at the start; the user types their own; Rumo to confirm).
Page titles are the pages' h1, read on 2026-10-24 (plain HTTP); the values are the research's (2026-10-23).
Every quote is the page's literal text (checked against the fetched pages before review).
"""
import json, sys
if len(sys.argv) != 3 or sys.argv[1] != "v1":
    sys.exit("usage: earn2trade.py v1 OUT.json")
D = "2026-10-23"
H = "https://help.earn2trade.com/en/articles/"
# Research §1 source index: key -> (url, page h1).
S = {
    "S1": ("https://www.earn2trade.com/trader-career-path", "Trader Career Path®"),
    "S2": ("https://www.earn2trade.com/gauntlet-mini", "The Gauntlet Mini™"),
    "S3": ("https://www.earn2trade.com/_next/static/chunks/0d45t7274e6.p.js", None),
    "S4": (H + "3224507-what-is-the-maximum-drawdown-on-the-trader-career-path-and-the-gauntlet-mini-challenges",
           "What is the Maximum Drawdown on The Trader Career Path® and The Gauntlet Mini™ Challenges?"),
    "S5": (H + "5372687-how-does-end-of-day-drawdown-work", 'How Does "End of Day Drawdown" Work?'),
    "S6": (H + "3292356-what-is-a-trailing-drawdown", "What is a Trailing Drawdown?"),
    "S7": (H + "8117088-drawdown-types", "Drawdown types"),
    "S8": (H + "8129235-what-is-a-fixed-drawdown", "What is a Fixed Drawdown?"),
    "S9": (H + "3395926-how-is-my-daily-loss-calculated", "How is My Daily Loss Calculated?"),
    "S10": (H + "3849975-what-is-the-maintain-consistency-rule", 'What is the "Maintain Consistency" Rule?'),
    "S11": (H + "5941958-what-are-the-evaluation-rules", "What Are the Evaluation Rules?"),
    "S12": (H + "3292351-what-is-the-progression-ladder-in-the-gauntlet-mini",
            "What is the Progression Ladder in the Gauntlet Mini™?"),
    "S13": (H + "5922873-what-is-the-progression-ladder-on-the-trader-career-path",
            "What is the Progression Ladder on the Trader Career Path®?"),
    "S16": (H + "2425493-what-is-the-commission-on-earn2trade-evaluations",
            "What is the commission on Earn2Trade Evaluations?"),
    "S18": (H + "12034590-am-i-allowed-to-copy-trades-across-multiple-accounts",
            "Am I Allowed to Copy Trades Across Multiple Accounts?"),
    "S20": (H + "9286647-prohibited-conduct-in-earn2trade-evaluations-and-in-the-livesim-live-trading-environment",
            "Prohibited Conduct in Earn2Trade Evaluations and in the LiveSim®/ Live Trading Environment"),
    "S21": ("https://www.earn2trade.com/terms-and-conditions", "Terms and Conditions"),
    "S22": (H + "2090521-what-platforms-can-i-use-for-the-gauntlet-mini-trader-career-path",
            "What Platforms can I use for the Gauntlet Mini™ / Trader Career Path®?"),
    "S23": (H + "15359154-ninjatrader-access-at-earn2trade", "NinjaTrader Access at Earn2Trade"),
    "S24": (H + "13460913-ninjatrader-live-account-access", "NinjaTrader® Live Account Access"),
    "S26": (H + "6877364-what-happens-when-i-complete-my-evaluation", "What Happens When I Complete My Evaluation?"),
    "S45": (H + "1541075-what-are-the-recommended-computer-specifications-i-should-have-to-trade-on-gauntlet-mini-trader-career-path-or-the-simulator",
            "What are the Recommended Computer Specifications I Should Have to Trade on  Gauntlet Mini™/ Trader Career Path® or the Simulator?"),
}
JS_NOTE = "no page title: the product pages' JavaScript data file"

def src(k):
    url, title = S[k]
    s = {"url": url, "accessed_at": D}
    if title: s["page_title"] = title
    else: s["title_note"] = JS_NOTE
    return s

def quote(text, k, section=None):
    url, title = S[k]
    q = {"text": text, "language": "en", "url": url, "accessed_at": D, "page_title": title}
    if section: q["section"] = section
    return q

def verified(value, *keys): return {"status": "verified", "value": value, "sources": [src(k) for k in keys]}

def partial(quotes, *searched):
    return {"status": "not_confirmed", "not_confirmed_reason": "partial", "quotes": quotes,
            "research": {"searched_on": D, "sources": [src(k) for k in searched]}}

def money(n): return f"{n}.00"

# Research §3.1 / §3.2 / S3: size -> (drawdown, daily loss limit, ladder top in minis, evaluation target).
VALUES = {25: (1500, 550, 3, 1750), 50: (2000, 1100, 6, 3000), 100: (3500, 2200, 12, 6000),
          150: (4500, 3300, 15, 9000), 200: (6000, 4400, 16, 11000), 400: (None, 8800, 30, None)}
# What the product page cards print as text (S1: TCP25 tab and the TCP25 growth card; S2: GAU50 tab).
# (plan, phase, size) -> (page, daily loss text, maximum position text). Other sizes are published only
# in the pages' JS data (S3) and the help images, so their quotes are the help-center sentences alone.
CARDS = {
    ("tcp", "evaluation", 25): ("S1", None, "Follow Progression Ladder Up to 3 Contracts"),
    ("tcp", "funded", 25): ("S1", "Daily Loss Limit: $550", "Max Contracts: 3"),
    ("tcp", "live", 25): ("S1", "Daily Loss Limit: $550", "Max Contracts: 3"),
    ("tcp", "live", 50): ("S1", "Daily Loss Limit: $1,100", "Max Contracts: 6"),
    ("tcp", "live", 100): ("S1", "Daily Loss Limit: $2,200", "Max Contracts: 12"),
    ("tcp", "live-fixed", 200): ("S1", "Daily Loss Limit: $4,400", "Max Contracts: 16"),
    ("gau", "evaluation", 50): ("S2", None, "Follow Progression Ladder Up to 6 Contracts"),
    ("gau", "funded", 50): ("S2", "Daily Loss Limit $1,100", "Follow Progression Ladder Up to 6 Contracts"),
    ("gau", "live", 50): ("S2", "Daily Loss Limit $1,100", "Follow Progression Ladder Up to 6 Contracts"),
}

EOD_STOP = "This continues until your Maximum Drawdown reaches the starting balance of your account"
TRAIL_STOP = "Increases dollar-for-dollar with your positive account performance during the trading day until it reaches the starting balance."
FUNDED_DLL = "Funded accounts have a daily loss limit that must not be breached."
LADDER = ["The maximum position size defines the total number of contracts a trader can have open across all positions.",
          "Exceeding the allowed contract limit is a hard fail and will immediately end the evaluation attempt."]
FIXED = {200: "Traders who pass the TCP25 and reach the $200,000 funded trading account have their drawdown fixed at $194,000.",
         400: "Traders who pass the TCP50 or TCP100 and reach the $400,000 funded trading account have their drawdown fixed at $388,000."}

def max_position(plan, size, phase):
    ladder = "S13" if plan == "tcp" else "S12"
    q = [quote(t, ladder) for t in LADDER]
    card = CARDS.get((plan, phase, size))
    if card:
        q.append(quote(card[2], card[0]))
    return partial(q, ladder, "S1" if plan == "tcp" else "S2", "S3")

def rules(plan, size, phase):
    dd, dll, minis, target = VALUES[size]
    card = "S1" if plan == "tcp" else "S2"
    if phase == "evaluation":
        drawdown = verified({"type": "trailing_eod", "amount": money(dd), "stops_trailing_at": EOD_STOP,
                             "trailing_lock": {"mode": "floor_reaches_start"}}, "S4", "S5", card, "S3")
    elif phase == "funded":  # LiveSim: EOD, the evaluation's amount (S26).
        drawdown = verified({"type": "trailing_eod", "amount": money(dd), "stops_trailing_at": EOD_STOP,
                             "trailing_lock": {"mode": "floor_reaches_start"}}, "S26", "S5", card, "S3")
    elif plan == "tcp" and (size == 400 or phase == "live-fixed"):
        drawdown = partial([quote(FIXED[size], "S8")], "S8", "S1", "S3")
    else:  # Live: trailing intraday, the same amount (S26, S6/S7).
        drawdown = verified({"type": "trailing_intraday", "amount": money(dd), "stops_trailing_at": TRAIL_STOP,
                             "trailing_lock": {"mode": "floor_reaches_start"}}, "S26", "S7", "S6", card, "S3")
    if phase == "evaluation":
        daily_loss = verified({"kind": "hard", "amount": money(dll)}, "S9", "S11", card, "S3")
    else:
        q = [quote(FUNDED_DLL, "S26")]
        printed = CARDS.get((plan, phase, size))
        if printed and printed[1]:
            q.append(quote(printed[1], printed[0]))
        daily_loss = partial(q, "S26", "S9", card, "S3")
    r = {
        "max_position_qty": max_position(plan, size, phase),
        "drawdown": drawdown,
        "daily_loss_limit": daily_loss,
        "trading_day_reset": verified({"local_time": "17:00", "time_zone": "America/Chicago",
                                       "published_text": "5:00 pm to 5:00 pm (Central Time)"}, "S9"),
    }
    if phase == "evaluation":
        r["consistency_rule"] = verified({"text": 'The "Maintain Consistency" rule states that over the course of your examination, no single trading day can account for 30% or more of your total PnL.',
                                          "threshold_pct": "30"}, "S10")
        r["evaluation_profit_target"] = verified({"amount": money(target)}, card, "S3")
    return r

# Firm platforms (research §2; PO rows 36, S-R3-4): NinjaTrader 8 reaches E2T through the
# NinjaTrader-Tradovate option (evaluation, LiveSim) and the NinjaTrader desktop live credentials (Live).
EVAL_PLATFORMS = ["tradovate", "rithmic", "blackarrow", "dxfeed"]
LIVE_PLATFORMS = ["ninjatrader", "rithmic"]

def size(plan, n, phase, key=None):
    platforms = LIVE_PLATFORMS if phase.startswith("live") else EVAL_PLATFORMS
    sets = [{"firm_platform": p, "rules": rules(plan, n, phase)} for p in platforms]
    return {"key": key or f"{n}k", "size_label": f"{n}K", "size_amount": money(n * 1000), "rule_sets": sets}

MANUAL = {
    "pt-BR": "Nenhuma saída da V1 alcança esta plataforma (na Earn2Trade, o NinjaTrader 8 chega pela opção NinjaTrader–Tradovate e, nas contas Live, pelas credenciais do NinjaTrader). A conta fica só em acompanhamento manual.",
    "en": "No V1 output reaches this platform (at Earn2Trade, NinjaTrader 8 connects through the NinjaTrader–Tradovate option and, on Live accounts, through the NinjaTrader credentials). The account is tracked manually only.",
}
RITHMIC = {
    "pt-BR": "Em assinaturas novas, o NinjaTrader pela Rithmic só funciona com uma licença própria do NinjaTrader com integração Rithmic (o acesso legado terminou em 30/06/2026). A conta fica só em acompanhamento manual nesta conexão.",
    "en": "On new subscriptions, NinjaTrader over Rithmic works only with your own NinjaTrader license with a Rithmic integration (the legacy access ended on 2026-06-30). The account is tracked manually only on this connection.",
}

release = {
    "schema_version": "1.2",
    "firm": {"slug": "earn2trade", "name": "Earn2Trade", "domains": ["earn2trade.com", "help.earn2trade.com"]},
    "release": {"version_number": 1, "verified_on": D, "research_file": "docs/product/presets/research/earn2trade.md", "changes": []},
    "platforms": [
        {"code": "tradovate", "label": "NinjaTrader (via Tradovate)"},
        {"code": "ninjatrader", "label": "NinjaTrader (Live)"},
        {"code": "rithmic", "label": "Rithmic", "manual_only": {"reason": RITHMIC}},
        {"code": "blackarrow", "label": "BlackArrow", "manual_only": {"reason": MANUAL}},
        {"code": "dxfeed", "label": "dxFeed/DeepCharts", "manual_only": {"reason": MANUAL}},
    ],
    "stances": {
        # Row 35: the only text is one clause; a choice is required, nothing preselected.
        "automation": [{"kind": "automation", "version": 1, "firm_stance": "conditional", "quotes": [
            quote("Utilizing software, artificial intelligence, or ultra-fast data entry techniques that could potentially manipulate the trading environment or provide an unfair advantage", "S20")]}],
        "copy": [{"kind": "copy", "version": 1, "firm_stance": "forbidden", "quotes": [
            quote("The use of trade copiers is not allowed on any of our programs.", "S18")]}],
        "vps": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S20"), src("S21"), src("S45")]}},
        "personal_device": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S20"), src("S21"), src("S45")]}},
    },
    "plans": [
        {"key": "trader-career-path", "name": "Trader Career Path", "phases": [
            {"phase": "evaluation", "sizes": [size("tcp", n, "evaluation") for n in (25, 50, 100)]},
            {"phase": "funded", "sizes": [size("tcp", n, "funded") for n in (25, 50, 100)]},
            {"phase": "live", "sizes": [size("tcp", n, "live") for n in (25, 50, 100, 150, 200)]
                + [size("tcp", 200, "live-fixed", "200k-fixed"), size("tcp", 400, "live")]},
        ]},
        {"key": "gauntlet-mini", "name": "Gauntlet Mini", "phases": [
            {"phase": "evaluation", "sizes": [size("gau", n, "evaluation") for n in (50, 100, 150, 200)]},
            {"phase": "funded", "sizes": [size("gau", n, "funded") for n in (50, 100, 150, 200)]},
            {"phase": "live", "sizes": [size("gau", n, "live") for n in (50, 100, 150, 200)]},
        ]},
    ],
    "excluded_plans": [],
}
json.dump(release, open(sys.argv[2], "w", encoding="utf-8", newline=""), indent=2, ensure_ascii=False)
