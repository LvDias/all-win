"""Apex Trader Funding release (evaluations only) from docs/product/presets/research/apex-trader-funding.md
(R2, rev. R4, 2026-10-23). Content approved by the PO (Rumo) per file.

Usage: apex_trader_funding.py v1 OUT.json (one immutable file per published version, data/presets/releases/<firm>/v<N>.json).
"""
import json, sys
if sys.argv[1] != "v1":
    sys.exit("usage: apex_trader_funding.py v1 OUT.json")
D = "2026-10-23"
H = "https://apextraderfunding.com/help-center/"
S = {
    "EODEVAL": H + "eod-trailing-drawdown-accounts/eod-evaluations/",
    "EODDD": H + "eod-trailing-drawdown-accounts/eod-drawdown-explained/",
    "INTEVAL": H + "evaluation-accounts-ea/intraday-trailing-drawdown-evaluations/",
    "INTDD": H + "intraday-trailing-drawdown-accounts/intraday-trailing-drawdown-explained/",
    "DLL": H + "additional-helpful-items/daily-loss-limit-explained/",
    "HOME": "https://apextraderfunding.com/",
    "TIMES": H + "getting-started/futures-trading-times/",
    "PROHIB": H + "getting-started/prohibited-activities/",
    "MAXPA": H + "performance-accounts-pa/how-many-paid-funded-accounts-am-i-allowed-to-have/",
    "LEGCOMP": H + "performance-accounts-pa/legacy-performance-account-pa-compliance/",
    "LIVE": H + "getting-started/apex-live-prop-trading-program-faq/",
}
# Page titles: title not retrieved: site blocks automated access, 2026-10-24 (HTTP 403 at the title
# check). This concerns the titles only: the values come from the research file, whose pages were
# read and captured on 2026-10-23. Sources carry the URL and access date (page_title is optional
# there). The user's decision (2026-10-24): every blocked title stays blank, with title_note next to
# the URL (presets 1.1); nothing is derived.
TITLE_NOTE = "title not retrieved: site blocks automated access, 2026-10-24"
def src(u):
    return {"url": u, "accessed_at": D, "title_note": TITLE_NOTE}
def quote(text, u, section=None):
    q = {"text": text, "language": "en", "url": u, "accessed_at": D, "title_note": TITLE_NOTE}
    if section: q["section"] = section
    return q
def verified(value, *urls): return {"status": "verified", "value": value, "sources": [src(u) for u in urls]}

SIZES = [("25k", "25K", "25000.00", "1000.00", "500.00", "1500.00", 4),
         ("50k", "50K", "50000.00", "2000.00", "1000.00", "3000.00", 6),
         ("100k", "100K", "100000.00", "3000.00", "1500.00", "6000.00", 8),
         ("150k", "150K", "150000.00", "4000.00", "2000.00", "9000.00", 12)]
PLATFORMS = ["rithmic", "tradovate", "wealthcharts"]

# When the evaluation threshold stops trailing differs by platform (research §2, §3.1, §3.3).
EOD_STOP = {
    "rithmic": "EOD Threshold stops trailing and becomes fixed when it reaches an amount equal to the Target Profit balance.",
    "wealthcharts": "EOD Threshold stops trailing and becomes fixed when it reaches an amount equal to the Target Profit balance.",
    "tradovate": "EOD Drawdown trails indefinitely with the peak EOD account balance.",
}
INT_STOP = {"tradovate": "Intraday Drawdown trails indefinitely with the peak account balance."}
# C5 (r1 row 5, NC): for Rithmic/WealthCharts the page's body and its FAQ word the lock point
# differently (both examples give a $53,000 stop level for 50K). Not confirmed: the user picks
# (Rumo, QA-3; S16 AC-5/AC-6).
INT_STOP_CONFLICT = [
    "Once the Intraday Threshold reaches Target Profit Balance, it stops increasing.",
    "In Rithmic and Wealthcharts Evaluations, trailing stops once the Intraday Threshold reaches Profit Target Balance + $2,000.",
]

def sizes(intraday):
    out = []
    for key, label, amount, mll, dll, target, minis in SIZES:
        sets = []
        for p in PLATFORMS:
            kind = "trailing_intraday" if intraday else "trailing_eod"
            eval_page = S["INTEVAL"] if intraday else S["EODEVAL"]
            # Q90 (presets 1.1): Rithmic/WealthCharts lock at the target profit balance, i.e. the size +
            # the profit target (the firm's 50K example: stop level $53,000); Tradovate never stops.
            lock = {"mode": "never"} if p == "tradovate" else {"mode": "floor_reaches_start", "offset": target}
            if intraday and p != "tradovate":
                # Rumo (Q90): only the body wording ("reaches Target Profit Balance") maps to a lock;
                # the FAQ wording ("+ $2,000") is not interpreted: text only, no structured lock (if
                # the user picks it, the Q90 interim applies: keeps trailing, with a warning).
                body, faq = INT_STOP_CONFLICT
                drawdown = {"status": "not_confirmed", "published_values": [
                    {"value": {"type": kind, "amount": mll, "stops_trailing_at": body, "trailing_lock": lock},
                     "sources": [src(S["INTDD"])]},
                    {"value": {"type": kind, "amount": mll, "stops_trailing_at": faq},
                     "sources": [src(S["INTDD"])]}]}
            else:
                stop = (INT_STOP if intraday else EOD_STOP)[p]
                drawdown = verified({"type": kind, "amount": mll, "stops_trailing_at": stop, "trailing_lock": lock},
                                    eval_page, S["INTDD"] if intraday else S["EODDD"], S["HOME"])
            rules = {
                "max_position_qty": verified({"contracts": minis * 10, "published_text": f"{minis} mini / {minis*10} micro"}, S["HOME"], eval_page),
                "drawdown": drawdown,
                "daily_loss_limit": verified({"kind": "none"}, S["INTEVAL"], S["DLL"]) if intraday
                    else verified({"kind": "soft_pause", "amount": dll}, S["EODEVAL"], S["DLL"], S["HOME"]),
                "trading_day_reset": verified({"local_time": "18:00", "time_zone": "America/New_York", "published_text": "6 PM ET"}, S["TIMES"]),
                "evaluation_profit_target": verified({"amount": target}, eval_page, S["HOME"]),
            }
            sets.append({"firm_platform": p, "rules": rules})
        out.append({"key": key, "size_label": label, "size_amount": amount, "rule_sets": sets})
    return out

AUTOMATION = "No Automation or Algorithm Usage allowed: Rewards are intended to recognize human traders actively participating in the learning process, not to reward automated systems executing preprogrammed logic. Please see the user agreement for more details, and we encourage you to reach out if you have further questions."
release = {
    "schema_version": "1.1",
    "firm": {"slug": "apex-trader-funding", "name": "Apex Trader Funding", "domains": ["apextraderfunding.com"]},
    "release": {"version_number": 1, "verified_on": D, "research_file": "docs/product/presets/research/apex-trader-funding.md", "changes": []},
    "platforms": [{"code": "rithmic", "label": "Rithmic"}, {"code": "tradovate", "label": "Tradovate"},
                  # S-R3-4 (a) principle (Rumo, QA-2): no V1 output reaches WealthCharts; All Win's
                  # statement (PO wording), not the firm's. Architect to confirm the NT8 claim.
                  {"code": "wealthcharts", "label": "WealthCharts", "manual_only": {"reason": {
                      "pt-BR": "Esta plataforma não é alcançada por nenhuma saída da V1 (o NinjaTrader 8 alcança a Apex pela Rithmic ou pela Tradovate). A conta fica só em acompanhamento manual.",
                      "en": "No V1 output reaches this platform (NinjaTrader 8 reaches Apex through Rithmic or Tradovate). The account is tracked manually only."}}}],
    "stances": {
        "automation": [{"kind": "automation", "version": 1, "firm_stance": "forbidden", "quotes": [quote(AUTOMATION, S["PROHIB"])]}],
        # C1: same-person copying allowed (S-MAXPA) vs forbidden on the Legacy PA compliance page.
        "copy": [{"kind": "copy", "version": 1, "firm_stance": "not_confirmed", "quotes": [
            quote("Copy trading across PA accounts under your personal name and business name is allowed. However, the hedging rule still applies; all PAs must trade in the same direction and not hedge against correlated assets.", S["MAXPA"]),
            quote("Copy or Trading Services Operations or Participation — Performance Accounts (PA) and Live Prop Accounts must be traded exclusively by the individual listed on the account. Under no circumstances can these accounts be managed or influenced by any other party, person, system, automated trading bot, copy trading service, or trade mirroring software.", S["LEGCOMP"])]}],
        "vps": {"value": "conditional", "quotes": [
            quote("You may not use VPNs, proxy servers, cloud servers, anonymizing tools, or other methods or tools for the purpose of misrepresenting, concealing, or disguising your identity, device, or location in an attempt: (a) to evade or circumvent any rules, security measures, access controls, or geographic restrictions; (b) to hide prior violations or detection of violations of this Agreement, the Trading Rules, or Company policies; or (c) to engage in or facilitate any illegal, fraudulent, deceptive, or abusive conduct.", S["PROHIB"])]},
        "personal_device": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src(S["PROHIB"]), src(S["LIVE"])]}},
    },
    "plans": [
        {"key": "eod-trailing-drawdown", "name": "EOD Trailing Drawdown", "phases": [{"phase": "evaluation", "sizes": sizes(False)}]},
        {"key": "intraday-trailing-drawdown", "name": "Intraday Trailing Drawdown", "phases": [{"phase": "evaluation", "sizes": sizes(True)}]},
    ],
    # The user's decisions only (S16 AC-9).
    "excluded_plans": [{"name": "Apex Legacy", "reason": {
        "pt-BR": "Excluído dos presets da V1 por decisão do usuário (S16 AC-9, 2026-10-23); cadastre manualmente.",
        "en": "Excluded from V1 presets by the user's decision (S16 AC-9, 2026-10-23); register it manually."}}],
}
json.dump(release, open(sys.argv[2], "w"), indent=2, ensure_ascii=False)
