"""BluSky release (Launch, Propel, Orbit, Instant Sim Funded, Direct to Funded) from
docs/product/presets/research/blusky.md (R3, 2026-10-23; PO recheck rows 20-31, 2026-10-24).
Content approved by the PO (Rumo) per file.

Usage: blusky.py v1 OUT.json (one immutable file per published version, data/presets/releases/<firm>/v<N>.json).

Phases: the Buffer Zone is a second evaluation-type phase, so Launch and Propel get a separate plan
"... Buffer Zone" (phase `evaluation`); Sim Funded is the `funded` phase. Live Brokerage is by call-up
only, with parameters per account (row 31): not a preset.
Presets 1.2 `not_confirmed` (nothing chosen, nothing derived; rows 20-30):
- conflicting (>= 2 published values): consistency method/percent (rows 22, 26), Orbit 200K evaluation
  contracts (row 24), copy stance (row 20);
- partial (verbatim text + where the rest was searched): trailing drawdowns, whose lock point is unclear
  (row 28); Sim Funded drawdown, published as a minimum balance that the pages disagree on (row 25);
  Orbit Sim Funded drawdown, $50,000 start vs $0 start (row 26); Sim Funded contract limits, "the limits
  displayed on your account" (row 25); Propel 300K evaluation daily loss, "Capped at $2,500" vs "None"
  (row 23); Instant / Direct daily loss, hard vs soft not stated (row 28); the Instant Sim Funded
  drawdown, whose lock is published as an absolute level (Rumo (ii)); the trading-day reset, only
  "6PM" (row 27, ASK);
- not_published (where the publisher looked): Buffer Zone contract limits; Sim Funded daily loss
  ("Risk Team may impose", row 28).
Commission (row 29): only Rithmic & Volumetrica and Tradesea evaluation accounts publish one ($0.50 per
micro side); everything else is absent (empty + warning).
Page titles are the pages' h1, read on 2026-10-24 (plain HTTP); the values are the research's (2026-10-23).
Every quote is the page's literal text, table rows as the page renders them (checked against the fetched
pages before review).
"""
import json, sys
if len(sys.argv) != 3 or sys.argv[1] != "v1":
    sys.exit("usage: blusky.py v1 OUT.json")
D = "2026-10-23"
A = "https://help.blusky.pro/en/articles/"
# Research source index: key -> (url, page h1).
S = {
    "S1": (A + "16300846-launch-plans-phases-rules-and-costs", "Launch Plans: Phases, Rules and Costs"),
    "S2": (A + "16300862-propel-plans-phases-rules-and-costs", "Propel Plans: Phases, Rules and Costs"),
    "S3": (A + "16300866-orbit-plans-two-phases-faster-payouts", "Orbit Plans: Two Phases, Faster Payouts"),
    "S4": (A + "12434059-evaluation-rules", "Evaluation Rules"),
    "S5": (A + "12433986-static-growth-plan", "Static Growth Plan"),
    "S6": (A + "12434458-blulive-buffer-stage", "BluLive/Buffer Stage"),
    "S7": (A + "17297032-sim-funded-accounts-rules", "Sim Funded Accounts Rules"),
    "S8": (A + "12434469-sim-funded-accounts-processing", "Sim Funded Accounts Processing"),
    "S9": (A + "16300869-instant-funding-direct-to-funded-and-instant-sim-funded",
           "Instant Funding: Direct to Funded and Instant Sim Funded"),
    "S10": (A + "12433943-multiple-accounts-copy-trading", "Multiple Accounts/Copy Trading"),
    "S11": (A + "12069208-ethical-trading-conduct-policy-blusky-trading-company",
            "Ethical Trading & Conduct Policy | BluSky Trading Company"),
    "S12": (A + "12433926-blusky-supported-platforms", "BluSky Supported Platforms"),
    "S13": (A + "10182306-rithmic-platforms", "Rithmic Platforms"),
    "S17": (A + "17115915-markets-symbols-commissions", "Markets, Symbols & Commissions 🚀"),
    "S18": (A + "14184170-evaluation-overview", "Evaluation Overview"),
    "S19": (A + "12433870-what-to-expect", "What to Expect"),
    "S20": (A + "11490284-blusky-trading-company-llc-dba-blusky-pro-terms-of-use",
            "BluSky Trading Company LLC dba BluSky.pro — Terms of Use"),
    "S22": (A + "11760172-scaling-contract-size-in-sim-funded-brokerage-accounts",
            "Scaling Contract Size in Sim Funded & Brokerage Accounts"),
    "S25": (A + "12292284-sim-funded-payouts", "Sim Funded Payouts"),
    "S26": (A + "13642215-risk-management-brokerage-account-supervision", "Risk Management & Brokerage Account Supervision"),
    "S31": ("https://blusky.pro/terms-and-conditions/", "Terms & Conditions"),
    "S33": (A + "11490309-sim-funded-and-live-brokerage-contract-size", "Sim Funded and Live Brokerage Contract Size"),
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

def not_published(*searched):
    return {"status": "not_confirmed", "not_confirmed_reason": "not_published",
            "research": {"searched_on": D, "sources": [src(k) for k in searched]}}

def conflicting(*values):
    return {"status": "not_confirmed", "not_confirmed_reason": "conflicting",
            "published_values": [{"value": v, "sources": [src(k) for k in keys]} for v, keys in values]}

def money(n): return f"{n}.00"
def usd(n): return f"${n:,}"

# ---- the plan tables (S1, S2, S3, S9), as published ------------------------------------------
# Evaluation: size (K) -> (target, drawdown, "trailing" | "static", minis, consistency %).
LAUNCH_EVAL = {50: (3000, 2000, "trailing", 5, 50), 100: (6000, 2500, "trailing", 10, 50), 200: (6000, 2000, "static", 2, 50)}
LAUNCH_BUFFER = {50: (3000, 2000, "trailing", None, 34), 100: (3500, 2500, "trailing", None, 34), 200: (3000, 2000, "static", None, 34)}
PROPEL_EVAL = {25: (1500, 1200, "trailing", 2, 34), 50: (3000, 2000, "trailing", 5, 34), 100: (6000, 2500, "trailing", 10, 34),
               150: (3000, 1000, "static", 1, 34), 200: (6000, 2000, "static", 2, 34), 300: (20000, 5000, "static", 5, 21)}
PROPEL_BUFFER = {25: (1500, 1200, "trailing", None, 34), 50: (3000, 2000, "trailing", None, 34), 100: (3500, 2500, "trailing", None, 34),
                 150: (2000, 1000, "static", None, 34), 200: (3000, 2000, "static", None, 34), 300: (3500, 2500, "static", None, 21)}
ORBIT_EVAL = {50: (3000, 2000, "trailing", 5), 100: (6000, 3000, "trailing", 10), 150: (9000, 4500, "trailing", 15), 200: (8000, 3000, "static", 3)}
ORBIT_FUNDED = {50: 2, 100: 3, 150: 5, 200: 3}  # minis (S3 "Contracts (funded)")
# Sim Funded (S1/S2 phase 3 tables): size -> (starting balance, 7-day cap).
LAUNCH_SIM = {50: (3000, 2500), 100: (3500, 3000), 200: (3000, 2500)}
PROPEL_SIM = {25: (1500, 1500), 50: (3000, 2500), 100: (3500, 3000), 150: (2000, 1500), 200: (3000, 2500), 300: (3500, 3000)}
# S8 (older processing page): size -> starting balance (150K / 300K differ: row 25).
S8_START = {25: 1500, 50: 3000, 100: 3500, 150: 1500, 200: 3000, 300: 3000}
# S25 (older payout page) minimum balances.
S25_MIN = {25: 26500, 50: 53000, 100: 103500}

EOD_RULE = ("The minimum trailing balance increases with your highest EOD balance. It increases with profits, but it "
            "does not decrease with losses, and is calculated from your starting balance at 6PM.")
BUFFER_LOCK = "Premium accounts: The trailing drawdown that becomes static once you reach the balance indicated in the table below."
METHOD = "We compare your largest single profitable day against the sum of your profitable days."
S4_METHOD = "Calculation Method: Highest profit day becomes the % of your new target. (Depends on what the account % is.)"
S5_RULE = {30: "Your profit target will automatically increase if any single day’s profit exceeds 30% of your current goal.",
           21: "Your profit target will automatically increase if any single day’s profit exceeds 21% of your current goal."}
RESET = partial([quote(EOD_RULE, "S4"),
                 quote("the BluLive/Buffer account will typically be set up the same evening after 6PM EST.", "S19")],
                "S4", "S19", "S20")
SIM_LIMITS = [quote("Your actual contract limits are the limits displayed on your account.", "S7"),
              quote("Most Sim Funded Accounts start with 5 Mini's and 50 Micro's and can be adjusted with your growth of the account.", "S22")]

def trailing_partial(page, row_text, phase):
    rule = quote(EOD_RULE, "S4") if phase == "evaluation" else quote(BUFFER_LOCK, "S6")
    return partial([quote(row_text, page), rule], page, "S4" if phase == "evaluation" else "S6")

def consistency(pct, page, phase, size):
    """Rows 22 and 26: the plan page's method vs the Evaluation Rules wording (and, for Propel 150K/200K
    and 300K evaluations, the Static Growth page's 30% / 21%). Nothing chosen."""
    values = [({"text": METHOD, "threshold_pct": str(pct)}, [page]),
              ({"text": S4_METHOD, "threshold_pct": str(pct)}, ["S4"])]
    if page == "S2" and phase == "evaluation" and size in (150, 200):
        values.append(({"text": S5_RULE[30], "threshold_pct": "30"}, ["S5"]))
    if page == "S2" and phase == "evaluation" and size == 300:
        values.append(({"text": S5_RULE[21], "threshold_pct": "21"}, ["S5"]))
    return conflicting(*values)

# ---- platforms (rows 21, 29, 31; S-R3-4 a) ------------------------------------------------------
ROUTABLE = ["tradovate", "rithmic"]
MANUAL = ["volumetrica", "tradesea", "tickblaze", "blackarrow"]
EVAL_COMMISSION = {"rithmic", "volumetrica", "tradesea"}  # S17: "Micros: $0.50/side" (evaluation only)

def sets(rules_of, phase):
    out = []
    for p in ROUTABLE + MANUAL:
        r = rules_of()
        if phase == "evaluation-eval" and p in EVAL_COMMISSION:
            r["commission_per_contract_side"] = verified({"amount": "0.50"}, "S17")
        out.append({"firm_platform": p, "rules": r})
    return out

def size_entry(n, rule_sets, key=None, label=None, amount=None):
    return {"key": key or f"{n}k", "size_label": label or f"{n}K", "size_amount": amount or money(n * 1000),
            "rule_sets": rule_sets}

# ---- Launch / Propel ---------------------------------------------------------------------------
def lp_eval(table, page, n, buffer):
    target, dd, kind, minis, pct = table[n]
    phase = "buffer" if buffer else "evaluation"
    if kind == "trailing":
        cell = f"{usd(dd)} Trailing (EOD)" if (buffer and page == "S2" and n in (25, 50, 100)) else f"$-{dd:,} Trailing (EOD)"
        drawdown = trailing_partial(page, cell, "evaluation" if not buffer else "buffer")
    else:
        drawdown = verified({"type": "static", "amount": money(dd)}, page)
    if not buffer and page == "S2" and n == 300:
        daily_loss = partial([quote("300K $20,000 $-5,000 Static None 21% $4,200 (21% of target)", "S2"),
                              quote("Daily Loss Limit: Capped at $2,500.", "S5")], "S2", "S5")
    elif buffer:
        daily_loss = verified({"kind": "none"}, page, "S6")
    else:
        daily_loss = verified({"kind": "none"}, page)
    def rules():
        r = {
            "max_position_qty": (not_published(page, "S6", "S33") if buffer else
                                 verified({"contracts": minis * 10, "published_text": f"{minis} Mini / {minis * 10} Micros"}, page)),
            "drawdown": drawdown,
            "daily_loss_limit": daily_loss,
            "trading_day_reset": RESET,
            "consistency_rule": consistency(pct, page, phase, n),
            "evaluation_profit_target": verified({"amount": money(target)}, page),
        }
        return r
    return size_entry(n, sets(rules, "evaluation-buffer" if buffer else "evaluation-eval"))

def lp_funded(sim, page, n):
    start, cap = sim[n]
    quotes = [quote("Account size Starting balance Minimum balance Payout minimum 7-day payout cap", page),
              quote(f"{n}K {usd(start)} $100 $500 {usd(cap)}", page),
              quote("Account Size Starting Balance Static Minimum Balance", "S8"),
              quote(f"${n}K {usd(S8_START[n])} $100", "S8")]
    if n in S25_MIN:
        quotes.append(quote(f"${n * 1000:,} {usd(S25_MIN[n])}", "S25"))
    def rules():
        return {
            "max_position_qty": partial(list(SIM_LIMITS), "S7", "S22", "S6", "S33"),
            "drawdown": partial(quotes, page, "S8", "S25"),
            "daily_loss_limit": not_published("S7", "S26", page),
            "trading_day_reset": RESET,
        }
    return size_entry(n, sets(rules, "funded"))

# ---- Orbit -------------------------------------------------------------------------------------
ORBIT_FLOOR = ("The floor trails your closed daily profit dollar-for-dollar, and once your lifetime net profit matches "
               "your starting drawdown it locks permanently at your starting balance.")
ORBIT_ZERO = ("Your account features a $0.00 Starting Balance, with risk parameters defined by a dedicated operational "
              "buffer based on your account size:")
NO_DLL_ORBIT = "There is no launch fee on any Orbit plan, and no daily loss limit in either phase."

def orbit_eval(n):
    target, dd, kind, minis = ORBIT_EVAL[n]
    if kind == "trailing":
        drawdown = trailing_partial("S3", f"$-{dd:,} Trailing (EOD)", "evaluation")
    else:
        drawdown = verified({"type": "static", "amount": money(dd)}, "S3")
    if n == 200:  # row 24: 3 Mini / 30 Micros (S3) vs 2 Mini & 20 Micro's (S18).
        max_pos = conflicting(({"contracts": 30, "published_text": "3 Mini / 30 Micros"}, ["S3"]),
                              ({"contracts": 20, "published_text": "200K (2 Mini & 20 Micro's)"}, ["S18"]))
    else:
        max_pos = verified({"contracts": minis * 10, "published_text": f"{minis} Mini / {minis * 10} Micros"}, "S3")
    def rules():
        return {
            "max_position_qty": max_pos,
            "drawdown": drawdown,
            "daily_loss_limit": verified({"kind": "none"}, "S3"),
            "trading_day_reset": RESET,
            "evaluation_profit_target": verified({"amount": money(target)}, "S3"),
        }
    return size_entry(n, sets(rules, "evaluation-eval"))

def orbit_funded(n):
    minis = ORBIT_FUNDED[n]
    def rules():
        return {
            "max_position_qty": verified({"contracts": minis * 10, "published_text": f"{minis} Mini / {minis * 10} Micros"}, "S3"),
            "drawdown": partial([quote(ORBIT_FLOOR, "S3"), quote(ORBIT_ZERO, "S8")], "S3", "S8"),
            "daily_loss_limit": verified({"kind": "none"}, "S3"),
            "trading_day_reset": RESET,
            "consistency_rule": verified({"text": METHOD, "threshold_pct": "40"}, "S3"),
        }
    return size_entry(n, sets(rules, "funded"))

# ---- Instant Sim Funded / Direct to Funded (S9) ---------------------------------------------------
# Rumo (format rev.3): a table row is quoted with the table's header row, so the plan it applies to is visible.
HEADER_ROW = "Direct to Funded Instant Sim Funded"
DLL_ROW = "Daily loss limit $2,000 $1,000"
DLL_QUOTES = [quote(HEADER_ROW, "S9"), quote(DLL_ROW, "S9")]

def instant():
    def rules():
        return {
            "max_position_qty": verified({"contracts": 50, "published_text": "5 Mini / 50 Micros"}, "S9"),
            # Rumo (ii): the lock is published as an absolute level ($51,000); an offset would be arithmetic.
            "drawdown": partial([quote("The 50K account carries $2,000 of drawdown that trails your highest intraday balance, so it follows your account up during the session rather than only at the close.", "S9"),
                                 quote("The trailing stops at $51,000 once your balance passes $53,000.", "S9")], "S9"),
            "daily_loss_limit": partial(DLL_QUOTES, "S9"),
            "trading_day_reset": RESET,
            "consistency_rule": verified({"text": METHOD, "threshold_pct": "21"}, "S9"),
        }
    return size_entry(50, sets(rules, "funded"))

def direct():
    def rules():
        return {
            "max_position_qty": verified({"contracts": 50, "published_text": "5 Mini / 50 Micros"}, "S9"),
            "drawdown": verified({"type": "static", "amount": "2500.00"}, "S9"),
            "daily_loss_limit": partial(DLL_QUOTES, "S9"),
            "trading_day_reset": RESET,
            "consistency_rule": verified({"text": METHOD, "threshold_pct": "21"}, "S9"),
        }
    return size_entry(0, sets(rules, "funded"), key="3500", label="$3,500", amount="3500.00")

MANUAL_REASON = {
    "pt-BR": "Nenhuma saída da V1 alcança esta plataforma (na BluSky, o NinjaTrader 8 chega pela conexão Tradovate ou pela Rithmic). A conta fica só em acompanhamento manual.",
    "en": "No V1 output reaches this platform (at BluSky, NinjaTrader 8 connects through Tradovate or Rithmic). The account is tracked manually only.",
}

release = {
    "schema_version": "1.2",
    "firm": {"slug": "blusky", "name": "BluSky", "domains": ["blusky.pro", "help.blusky.pro"]},
    "release": {"version_number": 1, "verified_on": D, "research_file": "docs/product/presets/research/blusky.md", "changes": []},
    "platforms": [
        {"code": "tradovate", "label": "Tradovate (NinjaTrader)"},
        {"code": "rithmic", "label": "Rithmic"},
        {"code": "volumetrica", "label": "Volumetrica", "manual_only": {"reason": MANUAL_REASON}},
        {"code": "tradesea", "label": "Tradesea", "manual_only": {"reason": MANUAL_REASON}},
        {"code": "tickblaze", "label": "Tickblaze", "manual_only": {"reason": MANUAL_REASON}},
        {"code": "blackarrow", "label": "BlackArrow", "manual_only": {"reason": MANUAL_REASON}},
    ],
    "stances": {
        # Row 30: only the conduct-policy clause; a choice is required, nothing preselected.
        "automation": [{"kind": "automation", "version": 1, "firm_stance": "conditional", "quotes": [
            quote("Coordinated Manipulation: Engaging, individually or with others (including across multiple accounts), in trading patterns whose primary intent is to manipulate, abuse, or secure an unfair advantage on our platform. This includes violating our terms and conditions, or using automated software (AI, ultra-high speed, bulk data entry) to unfairly influence outcomes.", "S11")]}],
        # Row 20: "Trade Copier Allowed: Yes" (S10) vs "Account Mirroring" forbidden (S11).
        "copy": [{"kind": "copy", "version": 1, "firm_stance": "not_confirmed", "quotes": [
            quote("Account Size Max Accounts Trade Copier Allowed Propel 25K Unlimited Yes", "S10"),
            quote("Account Mirroring: Copying trades from another trader, account, or signal provider, as this undermines independent trading skill development and risk management.", "S11")]}],
        "vps": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S10"), src("S11"), src("S20"), src("S31")]}},
        "personal_device": {"value": "unknown", "quotes": [], "research": {"searched_on": D, "sources": [src("S10"), src("S11"), src("S20"), src("S31")]}},
    },
    "plans": [
        {"key": "launch", "name": "Launch", "phases": [
            {"phase": "evaluation", "sizes": [lp_eval(LAUNCH_EVAL, "S1", n, False) for n in LAUNCH_EVAL]},
            {"phase": "funded", "sizes": [lp_funded(LAUNCH_SIM, "S1", n) for n in LAUNCH_SIM]}]},
        {"key": "launch-buffer-zone", "name": "Launch Buffer Zone", "phases": [
            {"phase": "evaluation", "sizes": [lp_eval(LAUNCH_BUFFER, "S1", n, True) for n in LAUNCH_BUFFER]}]},
        {"key": "propel", "name": "Propel", "phases": [
            {"phase": "evaluation", "sizes": [lp_eval(PROPEL_EVAL, "S2", n, False) for n in PROPEL_EVAL]},
            {"phase": "funded", "sizes": [lp_funded(PROPEL_SIM, "S2", n) for n in PROPEL_SIM]}]},
        {"key": "propel-buffer-zone", "name": "Propel Buffer Zone", "phases": [
            {"phase": "evaluation", "sizes": [lp_eval(PROPEL_BUFFER, "S2", n, True) for n in PROPEL_BUFFER]}]},
        {"key": "orbit", "name": "Orbit", "phases": [
            {"phase": "evaluation", "sizes": [orbit_eval(n) for n in ORBIT_EVAL]},
            {"phase": "funded", "sizes": [orbit_funded(n) for n in ORBIT_FUNDED]}]},
        {"key": "instant-sim-funded", "name": "Instant Sim Funded", "phases": [
            {"phase": "funded", "sizes": [instant()]}]},
        {"key": "direct-to-funded", "name": "Direct to Funded", "phases": [
            {"phase": "funded", "sizes": [direct()]}]},
    ],
    "excluded_plans": [],
}
json.dump(release, open(sys.argv[2], "w", encoding="utf-8", newline=""), indent=2, ensure_ascii=False)
