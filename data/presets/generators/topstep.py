"""Topstep release (Trading Combine only) from docs/product/presets/research/topstep.md (R2, 2026-10-23).
Page titles are derived from the help-center URL slugs (research does not list them): PO to confirm."""
import json, sys
D = "2026-10-23"
def src(url):
    s = {"url": url, "accessed_at": D}
    if url in TITLES:
        s["page_title"] = TITLES[url]
    return s
# page_title: the real page titles (confirmed by Aquarela, 2026-10-24). `section` names the plan
# phase the text is quoted for (Rumo, Q-TS-2).
def quote(text, url, title, section=None):
    q = {"text": text, "language": "en", "page_title": title, "url": url, "accessed_at": D}
    if section: q["section"] = section
    return q
S7 = "https://help.topstep.com/en/articles/10657969-live-funded-account-parameters"
S1 = "https://help.topstep.com/en/articles/8284197-trading-combine-parameters"
S2 = "https://help.topstep.com/en/articles/8284204-what-is-the-maximum-loss-limit"
S3 = "https://help.topstep.com/en/articles/10490293-daily-loss-limit-in-the-trading-combine-and-express-funded-account"
S4 = "https://help.topstep.com/en/articles/8284208-consistency-at-topstep"
S11 = "https://help.topstep.com/en/articles/14434175-topstepx"
S12 = "https://help.topstep.com/en/articles/11187768-topstepx-api-access"
S17 = "https://help.topstep.com/en/articles/8284199-new-to-topstep-start-here"
S22 = "https://www.topstep.com/topstep-prop"
S25 = "https://www.topstep.com/terms-of-use"
S19 = "https://help.topstep.com/en/articles/15520357-topstep-labs"
S20 = "https://help.topstep.com/en/articles/14645398-what-is-a-pro-account"

# page_title = the page's h1, without the <title> suffix (Maestro rule; titles from Aquarela, 2026-10-24).
TITLES = {
    S1: "Trading Combine® Parameters",
    S2: "What is the Maximum Loss Limit?",
    S3: "Daily Loss Limit in the Trading Combine and Express Funded Account",
    S4: "Consistency at Topstep",
    S7: "Live Funded Account Parameters",
    S11: "TopstepX™",
    S12: "TopstepX™ API Access",
    S17: "New to Topstep? Start here.",
    S22: "The Best Futures Prop Firm",
    S25: "Terms of use",
}

def verified(value, *urls): return {"status": "verified", "value": value, "sources": [src(u) for u in urls]}

def tc(size_key, label, amount, mll, dll, target, minis):
    return {
        "key": size_key, "size_label": label, "size_amount": amount,
        "rule_sets": [{"rules": {
            "max_position_qty": verified({"contracts": minis * 10, "published_text": f"{minis} mini / {minis*10} micro"}, S1, S22),
            "drawdown": verified({"type": "trailing_eod", "amount": mll,
                                  "stops_trailing_at": "Once it reaches your starting balance, it locks permanently."}, S2, S22),
            # Optional at checkout (the trader chooses): not a conflict between pages. Interim: the user
            # picks one of the two published options (never prefilled). PO question Q-TS-1.
            "daily_loss_limit": {"status": "not_confirmed", "published_values": [
                {"value": {"kind": "none"}, "sources": [src(S3)]},
                {"value": {"kind": "soft_pause", "amount": dll}, "sources": [src(S3), src(S22)]}]},
            "trading_day_reset": verified({"local_time": "17:00", "time_zone": "America/Chicago",
                                           "published_text": "5:00 PM CT"}, S1),
            "consistency_rule": verified({"text": "Your single best day of profit must stay at or below 55% of your Profit Target. If it exceeds that, your Profit Target increases.",
                                          "threshold_pct": "55"}, S4),
            "evaluation_profit_target": verified({"amount": target}, S4, S22),
        }}]}

release = {
    "schema_version": "1.0",
    "firm": {
        "slug": "topstep", "name": "Topstep", "domains": ["topstep.com"],
        # S16 AC-7: TopstepX only; orders must originate from the personal device.
        "manual_only": {"reason": quote("TopstepX™ is the only available trading platform.", S17, "New to Topstep? Start here.")},
    },
    "release": {"version_number": 1, "verified_on": D, "research_file": "docs/product/presets/research/topstep.md", "changes": []},
    "platforms": [{"code": "topstepx", "label": "TopstepX"}],
    "stances": {
        # Stance differs by phase (TC/XFA conditional, LFA not possible): the format has one text per
        # firm or platform. This draft covers the Trading Combine only. PO question Q-TS-2.
        "automation": [{"kind": "automation", "version": 1, "firm_stance": "conditional", "quotes": [
            quote("Yes, with conditions. Topstep won't help set up or troubleshoot automated strategies, and no exceptions are made for errant trades or malfunctions. Test on your Practice Account first and review Prohibited Conduct and Prohibited Trading Strategies before going live.", S1, "Trading Combine® Parameters", "Trading Combine"),
            quote("The API Gateway is built for the simulated environment and isn't available on Live, so automated strategies are not possible at this time in the Live Funded Account.", S7, "Live Funded Account Parameters", "Live Funded Account"),
            quote("All trading activity must originate from your personal device. The use of VPS, VPNs, and remote servers is prohibited by Topstep's Terms of Use. Running automation on a VPS can result in account suspension or removal from the program.", S12, "TopstepX™ API Access", "All account types")]}],
        "copy": [{"kind": "copy", "version": 1, "firm_stance": "conditional", "quotes": [
            quote("Trade copying is only available for Trading Combine® and Express Funded Account® accounts. Live Funded Account® accounts cannot use the Trade Copier.", S11, "TopstepX™", "Trading Combine and Live Funded Account")]}],
        "vps": {"value": "forbidden", "quotes": [
            quote("Using any VPN or VPS on Accounts is strictly prohibited and may result in account termination and forfeiture of profits.", S25, "Terms of use")]},
        "personal_device": {"value": "required", "quotes": [
            quote("All trading activity must originate from your personal device. The use of VPS, VPNs, and remote servers is prohibited by Topstep's Terms of Use. Running automation on a VPS can result in account suspension or removal from the program.", S12, "TopstepX™ API Access")]},
    },
    "plans": [{"key": "trading-combine", "name": "Trading Combine", "phases": [{"phase": "evaluation", "sizes": [
        tc("50k", "$50K", "50000.00", "2000.00", "1000.00", "3000.00", 5),
        tc("100k", "$100K", "100000.00", "3000.00", "2000.00", "6000.00", 10),
        tc("150k", "$150K", "150000.00", "4500.00", "3000.00", "9000.00", 15),
    ]}]}],
    # The user's decisions only (S16 AC-9; S-3, 2026-10-23). XFA and LFA are held out pending Q94,
    # not listed here (Rumo, Q-TS-3/4).
    "excluded_plans": [
        {"name": n, "reason": {"pt-BR": "Excluído dos presets da V1 por decisão do usuário (S-3, 2026-10-23); cadastre manualmente.",
                               "en": "Excluded from V1 presets by the user's decision (S-3, 2026-10-23); register it manually."}}
        for n in ["Topstep Labs Challenges", "Topstep Labs $25K Static Trading Combine",
                  "Topstep Labs $50K Static Trading Combine", "Topstep Labs $250K Freedom Trading Combine", "Pro Account"]
    ],
}
json.dump(release, open(sys.argv[1], "w"), indent=2, ensure_ascii=False)
