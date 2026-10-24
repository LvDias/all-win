"""OneUp Trader release (Evaluation, Pro Evaluation, Funded Trader Account) from
docs/product/presets/research/oneup-trader.md (R3, 2026-10-23; PO recheck rows 39-46, 2026-10-24).
Content approved by the PO (Rumo) per file.

Usage: oneup_trader.py v1 OUT.json (one immutable file per published version, data/presets/releases/<firm>/v<N>.json).

Presets 1.2 `not_confirmed` / `partial` (verbatim text + where the rest was searched; nothing derived):
- drawdown, both phases: amount and lock published, unrealized treatment not (row 40); the funded
  amount comes only from the homepage card, which is not phase-specific (row 39);
- trading-day reset: "5 PM CT" only as the start of the trading day, no reset event (row 43, ASK);
- funded maximum position: dynamic scaling by Cash On Hand, the tiers are an image (row 45; same
  PROVISIONAL rule as Earn2Trade).
Funded commission: the table is an image (row 45, PLAN: re-check before a release carries it), so absent.
Platform: Rithmic only; NinjaTrader 8.1 desktop connects through it (row 46).
Page titles are the pages' h1, read on 2026-10-24 (plain HTTP); the values are the research's (2026-10-23).
Every quote is the page's literal text (checked against the fetched pages before review).
"""
import json, sys
if len(sys.argv) != 3 or sys.argv[1] != "v1":
    sys.exit("usage: oneup_trader.py v1 OUT.json")
D = "2026-10-23"
A = "https://help.oneuptrader.com/article/"
# Research §1 source index: key -> (url, page h1).
S = {
    "H": ("https://www.oneuptrader.com/", "BECOME A FULLY FUNDED TRADER"),
    "T": ("https://www.oneuptrader.com/terms", "Terms & Conditions"),
    "S1": (A + "385-evaluation-rules-parameters", "Evaluation Rules & Parameters"),
    "S3": (A + "389-rules-definitions", "Rules Definitions"),
    "S4": (A + "388-funded-trader-trailing-drawdown", "Funded Trader Trailing Drawdown"),
    "S5": (A + "372-funded-trader-rules", "Funded Trader Rules"),
    "S6": (A + "386-funded-trader-dynamic-scaling-targets", "Funded Trader Dynamic Scaling Targets"),
    "S7": (A + "390-permissible-products-times", "Permissible Products / Times"),
    "S9": (A + "468-most-popular-questions", "Most Popular Questions"),
    "S11": (A + "391-important-evaluation-info", "Important Evaluation Info"),
    "S12": (A + "361-funded-trader-commissions", "Funded Trader Commissions"),
    "S13": (A + "505-can-i-have-multiple-evaluation-accounts", "Multiple Evaluation Accounts"),
    "S14": (A + "380-supported-platforms", "Supported Platforms"),
    "S16": (A + "491-ninjatrader-desktop-version-v-8-1-connection-guide", "NinjaTrader Desktop Version V.8.1 Connection Guide"),
    "S24": (A + "486-no-more-daily-loss-limit-restrictions", "No more daily loss limit restrictions!"),
}

def src(k):
    url, title = S[k]
    return {"url": url, "accessed_at": D, "page_title": title}

def quote(text, k):
    url, title = S[k]
    return {"text": text, "language": "en", "url": url, "accessed_at": D, "page_title": title}

def verified(value, *keys): return {"status": "verified", "value": value, "sources": [src(k) for k in keys]}

def partial(quotes, *searched):
    return {"status": "not_confirmed", "not_confirmed_reason": "partial", "quotes": quotes,
            "research": {"searched_on": D, "sources": [src(k) for k in searched]}}

def money(n): return f"{n}.00"
def usd(n): return f"${n:,}"

# Research §3.1 / homepage cards: size (K) -> (max contracts, profit target, trailing drawdown).
SIZES = {25: (3, 1500, 1500), 50: (6, 3000, 2500), 100: (12, 6000, 3500), 150: (15, 9000, 5000), 250: (25, 15000, 5500)}

EVAL_DD = ["The Trailing Drawdown is calculated in Real-Time during your trading day.",
           "Once your Trailing Drawdown reaches your initial starting balance, it will not change for the rest of your evaluation."]
FUNDED_DD = ["Once your Trailing Drawdown reaches your initial starting balance, it will not change.",
             "The Trailing Drawdown is calculated in real-time during your trading session and it includes commissions and fees."]
RESET = "Trading day starts at 5 PM CT and ends at 3:15 PM CT (not including holidays and weekends)."
SCALING = ("Your buying power (number of contracts to trade) will be determined by the Cash On Hand value "
           "(seen in R|Trader Pro) at the beginning of the trading session.")

def rules(n, phase, min_days=None):
    contracts, target, dd = SIZES[n]
    card_dd = quote(f"{usd(dd)} Trailing Drawdown", "H")
    reset = partial([quote(RESET, "S1")], "S1", "S7", "H")
    no_dll = verified({"kind": "none"}, "S9", "S24", "H")
    if phase == "evaluation":
        r = {
            "max_position_qty": verified({"contracts": contracts, "published_text": f"Max {contracts} Contracts"}, "H", "S3", "S9"),
            "drawdown": partial([quote(t, "S3") for t in EVAL_DD] + [card_dd], "S3", "S1", "H"),
            "daily_loss_limit": no_dll,
            "trading_day_reset": reset,
            "consistency_rule": verified({"text": "Traders must have three (3) other best trading days' net profits total summed up to equal 80% or more of the largest day's net profit."}, "S1"),
            "commission_per_contract_side": verified({"amount": "1.00"}, "S11"),
            "evaluation_profit_target": verified({"amount": money(target)}, "H"),
        }
        stored = {"min_trading_days": verified({"days": min_days}, "S1", "H")}
        return {"rules": r, "stored_only": stored}
    r = {
        "max_position_qty": partial([quote(SCALING, "S6")], "S6", "S9", "H"),
        "drawdown": partial([quote(t, "S4") for t in FUNDED_DD] + [card_dd], "S4", "H"),
        "daily_loss_limit": no_dll,
        "trading_day_reset": reset,
    }
    return {"rules": r}

def size(n, phase, min_days=None):
    return {"key": f"{n}k", "size_label": f"{n}K", "size_amount": money(n * 1000), "rule_sets": [rules(n, phase, min_days)]}

release = {
    "schema_version": "1.2",
    "firm": {"slug": "oneup-trader", "name": "OneUp Trader", "domains": ["oneuptrader.com", "help.oneuptrader.com"]},
    "release": {"version_number": 1, "verified_on": D, "research_file": "docs/product/presets/research/oneup-trader.md", "changes": []},
    "platforms": [{"code": "rithmic", "label": "Rithmic"}],
    "stances": {
        "automation": [{"kind": "automation", "version": 1, "firm_stance": "unknown", "quotes": [],
                        "research": {"searched_on": D, "sources": [src("S1"), src("S5"), src("S9"), src("T")]}}],
        "copy": [{"kind": "copy", "version": 1, "firm_stance": "conditional", "quotes": [
            quote("Traders are expressly prohibited from engaging in copy-trading activities between funded accounts held under different names or individuals. However, copy-trading is permissible when conducted across multiple funded accounts under the Trader's own name and solely for the purposes of risk management and strategy diversification.", "S5"),
            quote("Traders can analyze and execute trades on a single chart while seamlessly replicating them across multiple accounts via a trade copier.", "S13")]}],
        "vps": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S5"), src("S9"), src("T"), src("H")]}},
        "personal_device": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S5"), src("S9"), src("T"), src("H")]}},
    },
    "plans": [
        {"key": "evaluation", "name": "Evaluation", "phases": [
            {"phase": "evaluation", "sizes": [size(n, "evaluation", 10) for n in SIZES]}]},
        {"key": "pro-evaluation", "name": "Pro Evaluation", "phases": [
            {"phase": "evaluation", "sizes": [size(n, "evaluation", 5) for n in SIZES]}]},
        {"key": "funded-trader-account", "name": "Funded Trader Account", "phases": [
            {"phase": "funded", "sizes": [size(n, "funded") for n in SIZES]}]},
    ],
    "excluded_plans": [],
}
json.dump(release, open(sys.argv[2], "w", encoding="utf-8", newline=""), indent=2, ensure_ascii=False)
