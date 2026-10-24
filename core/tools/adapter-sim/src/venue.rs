//! A simulated platform account: positions per contract, All Win orders, and the adapter rules
//! of ADR-0004 §5 that the Core depends on (command dedupe, expiry, position-identity guard on
//! reduce, flatten/already_flat, orphan protection). Pure: the caller passes `now`.

use std::collections::{BTreeMap, HashMap, VecDeque};

use chrono::{DateTime, SecondsFormat, Utc};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use uuid::Uuid;

/// How the venue answers the next command.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Behavior {
    /// Fill at once at the venue price.
    Fill,
    /// Fill only `quantity`; the rest stays working.
    Partial { quantity: u32 },
    /// Reject with an adapter reason code (e.g. `order.platform_rejected`).
    Reject { code: String },
    /// Report only `received` / `working`; nothing fills.
    Rest,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Script {
    /// Answers in order, one per command; when exhausted, `default`.
    #[serde(default)]
    pub queue: VecDeque<Behavior>,
    pub default: Behavior,
    /// Price string used for fills (exact decimal).
    pub price: String,
    /// Platform commission per fill (`None` = not reported).
    #[serde(default)]
    pub commission: Option<String>,
}

impl Default for Script {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            default: Behavior::Fill,
            price: "21450.25".into(),
            commission: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Position {
    quantity: i64,
    /// `command_id` of the All Win open that took it from flat (U13), `None` = external/unknown.
    identity: Option<String>,
}

#[derive(Debug, Clone)]
struct Leg {
    command_id: String,
    role: &'static str,
    instrument: Value,
}

pub struct Venue {
    pub account_id: Uuid,
    pub platform_account: String,
    pub script: Script,
    positions: BTreeMap<String, Position>,
    legs: Vec<Leg>,
    /// Manual (non-All Win) working orders, as (contract key, platform order id).
    manual: Vec<(String, Value, String)>,
    /// Last report per handled `command_id` (dedupe record, answered as `duplicate`).
    handled: HashMap<String, Value>,
}

fn ts(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn key(instrument: &Value) -> String {
    format!(
        "{}:{}",
        instrument["symbol"].as_str().unwrap_or(""),
        instrument["contract_month"].as_str().unwrap_or("")
    )
}

impl Venue {
    pub fn new(account_id: Uuid, platform_account: impl Into<String>, script: Script) -> Self {
        Self {
            account_id,
            platform_account: platform_account.into(),
            script,
            positions: BTreeMap::new(),
            legs: Vec::new(),
            manual: Vec::new(),
            handled: HashMap::new(),
        }
    }

    fn report(
        &self,
        command_id: Option<&str>,
        instrument: &Value,
        role: &str,
        status: &str,
        now: DateTime<Utc>,
    ) -> Map<String, Value> {
        let mut r = Map::new();
        r.insert("event_id".into(), json!(Uuid::now_v7()));
        if let Some(c) = command_id {
            r.insert("command_id".into(), json!(c));
        }
        r.insert("account_id".into(), json!(self.account_id));
        r.insert("platform_account".into(), json!(self.platform_account));
        r.insert("instrument".into(), instrument.clone());
        r.insert("origin".into(), json!("all_win"));
        r.insert("child_role".into(), json!(role));
        r.insert("status".into(), json!(status));
        r.insert("occurred_at".into(), json!(ts(now)));
        r
    }

    /// Rejection of a command addressed to an account this venue does not execute.
    pub fn reject_foreign(
        &self,
        cmd: &Value,
        account_id: Uuid,
        platform_account: &str,
        now: DateTime<Utc>,
    ) -> Value {
        let mut r = self.report(
            cmd["command_id"].as_str(),
            &cmd["instrument"],
            "entry",
            "rejected",
            now,
        );
        r.insert("account_id".into(), json!(account_id));
        r.insert("platform_account".into(), json!(platform_account));
        Self::with_reason(r, "order.platform_rejected")
    }

    fn with_reason(mut r: Map<String, Value>, code: &str) -> Value {
        r.insert("reason_code".into(), json!(code));
        Value::Object(r)
    }

    fn fill(
        &mut self,
        r: &mut Map<String, Value>,
        instrument: &Value,
        signed: i64,
        now_id: &str,
    ) -> i64 {
        let qty = u32::try_from(signed.unsigned_abs()).unwrap_or(u32::MAX);
        let mut fill = json!({ "fill_id": format!("SIM-{}", Uuid::now_v7().simple()), "quantity": qty, "price": self.script.price });
        if let Some(c) = &self.script.commission {
            fill["commission"] = json!(c);
        }
        r.insert("fill".into(), fill);
        r.insert(
            "side".into(),
            json!(if signed > 0 { "buy" } else { "sell" }),
        );
        r.insert("quantity".into(), json!(qty));
        let pos = self.positions.entry(key(instrument)).or_default();
        let before = pos.quantity;
        pos.quantity += signed;
        if before == 0 || before.signum() != pos.quantity.signum() {
            pos.identity = (pos.quantity != 0).then(|| now_id.to_owned());
        }
        let after = pos.quantity;
        let mut p = json!({ "quantity": after });
        if after != 0 {
            p["average_price"] = json!(self.script.price);
        }
        r.insert("position_after".into(), p);
        after
    }

    /// Orphan protection (ADR-0004 §5): a flat contract keeps no All Win stop/target.
    /// A working order placed by hand on the platform (tests of Q75-1).
    pub fn add_manual_order(&mut self, instrument: Value) -> String {
        let id = format!("MANUAL-{}", Uuid::now_v7().simple());
        self.manual.push((key(&instrument), instrument, id.clone()));
        id
    }

    pub fn manual_orders(&self) -> usize {
        self.manual.len()
    }

    /// A flatten cancels EVERY working order of the contract, manual ones included (Q75-1);
    /// a manual order is reported with `origin: external`.
    fn cancel_manual(&mut self, instrument: &Value, now: DateTime<Utc>) -> Vec<Value> {
        let k = key(instrument);
        let (gone, kept): (Vec<_>, Vec<_>) = self.manual.drain(..).partition(|(mk, _, _)| *mk == k);
        self.manual = kept;
        gone.into_iter()
            .map(|(_, inst, order_id)| {
                let mut r = self.report(None, &inst, "entry", "cancelled", now);
                r.insert("origin".into(), json!("external"));
                r.remove("child_role");
                r.insert("platform_order_id".into(), json!(order_id));
                Value::Object(r)
            })
            .collect()
    }

    fn cancel_orphans(&mut self, instrument: &Value, now: DateTime<Utc>) -> Vec<Value> {
        let k = key(instrument);
        let (gone, kept): (Vec<Leg>, Vec<Leg>) =
            self.legs.drain(..).partition(|l| key(&l.instrument) == k);
        self.legs = kept;
        gone.into_iter()
            .map(|l| {
                Self::with_reason(
                    self.report(Some(&l.command_id), &l.instrument, l.role, "cancelled", now),
                    "order.orphan_cancelled",
                )
            })
            .collect()
    }

    /// Execution reports for one `command` payload, in order.
    pub fn on_command(&mut self, cmd: &Value, now: DateTime<Utc>) -> Vec<Value> {
        let id = cmd["command_id"].as_str().unwrap_or_default().to_owned();
        let instrument = cmd["instrument"].clone();
        if let Some(last) = self.handled.get(&id) {
            let mut dup = last.clone();
            dup["event_id"] = json!(Uuid::now_v7());
            dup["status"] = json!("duplicate");
            dup["reason_code"] = json!("command.duplicate");
            dup.as_object_mut().map(|o| o.remove("fill"));
            return vec![dup];
        }
        let expired = cmd["expires_at"]
            .as_str()
            .and_then(|e| DateTime::parse_from_rfc3339(e).ok())
            .is_some_and(|e| now >= e.with_timezone(&Utc));
        let mut out = Vec::new();
        match cmd["kind"].as_str().unwrap_or_default() {
            "place" if expired => {
                out.push(Self::with_reason(
                    self.report(Some(&id), &instrument, "entry", "expired", now),
                    "command.expired",
                ));
            }
            "place" => {
                let place = &cmd["place"];
                let qty =
                    i64::from(u32::try_from(place["quantity"].as_u64().unwrap_or(0)).unwrap_or(0));
                let signed = if place["side"] == "sell" { -qty } else { qty };
                let reduce = place["position_effect"] == "reduce";
                let role = if reduce { "exit" } else { "entry" };
                let pos = self
                    .positions
                    .get(&key(&instrument))
                    .cloned()
                    .unwrap_or_default();
                if reduce {
                    if pos.quantity == 0 {
                        out.push(Self::with_reason(
                            self.report(Some(&id), &instrument, role, "cancelled", now),
                            "command.nothing_to_reduce",
                        ));
                        self.remember(&id, &out);
                        return out;
                    }
                    let wanted = cmd["refs"]["position_ref"].as_str();
                    if wanted.is_none()
                        || wanted != pos.identity.as_deref()
                        || pos.quantity.signum() == signed.signum()
                    {
                        out.push(Self::with_reason(
                            self.report(Some(&id), &instrument, role, "rejected", now),
                            "command.position_changed",
                        ));
                        self.remember(&id, &out);
                        return out;
                    }
                }
                let behavior = self
                    .script
                    .queue
                    .pop_front()
                    .unwrap_or_else(|| self.script.default.clone());
                out.push(Value::Object(self.report(
                    Some(&id),
                    &instrument,
                    role,
                    "received",
                    now,
                )));
                match behavior {
                    Behavior::Reject { code } => {
                        out.push(Self::with_reason(
                            self.report(Some(&id), &instrument, role, "rejected", now),
                            &code,
                        ));
                    }
                    Behavior::Rest => out.push(Value::Object(self.report(
                        Some(&id),
                        &instrument,
                        role,
                        "working",
                        now,
                    ))),
                    Behavior::Fill | Behavior::Partial { .. } => {
                        let filled = match behavior {
                            Behavior::Partial { quantity } => {
                                signed.signum() * i64::from(quantity).min(qty)
                            }
                            _ => signed,
                        };
                        // A reduce never flips: it is capped to the position (order.quantity_capped).
                        let filled = if reduce && filled.abs() > pos.quantity.abs() {
                            -pos.quantity
                        } else {
                            filled
                        };
                        let status = if filled.abs() < qty {
                            "partially_filled"
                        } else {
                            "filled"
                        };
                        let mut r = self.report(Some(&id), &instrument, role, status, now);
                        let after = self.fill(&mut r, &instrument, filled, &id);
                        out.push(Value::Object(r));
                        if !reduce && let Some(p) = place.get("protection") {
                            for (role, field) in
                                [("stop", "stop_ticks"), ("target", "target_ticks")]
                            {
                                if p.get(field).is_some() {
                                    self.legs.push(Leg {
                                        command_id: id.clone(),
                                        role,
                                        instrument: instrument.clone(),
                                    });
                                    out.push(Value::Object(self.report(
                                        Some(&id),
                                        &instrument,
                                        role,
                                        "working",
                                        now,
                                    )));
                                }
                            }
                        }
                        if after == 0 {
                            out.extend(self.cancel_orphans(&instrument, now));
                        }
                    }
                }
            }
            "flatten" => {
                // Like the plugin (AdvanceFlatten): every working All Win order of the contract is
                // cancelled and confirmed BEFORE the close, so the cancels come first (ADR-0004 §6.1).
                out.extend(self.cancel_orphans(&instrument, now));
                out.extend(self.cancel_manual(&instrument, now));
                let pos = self
                    .positions
                    .get(&key(&instrument))
                    .cloned()
                    .unwrap_or_default();
                if pos.quantity == 0 {
                    out.push(Self::with_reason(
                        self.report(Some(&id), &instrument, "flatten", "cancelled", now),
                        "command.already_flat",
                    ));
                } else {
                    let mut r = self.report(Some(&id), &instrument, "flatten", "filled", now);
                    self.fill(&mut r, &instrument, -pos.quantity, &id);
                    out.push(Value::Object(r));
                }
            }
            other => {
                // modify/cancel: one outcome per target (ADR-0004 §5), never with fills.
                for t in cmd[other]["targets"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
                {
                    let target = t["command_id"].as_str().unwrap_or_default().to_owned();
                    let role = t["child_role"].as_str().unwrap_or("entry");
                    let exists = self
                        .legs
                        .iter()
                        .any(|l| l.command_id == target && l.role == role);
                    let status = match (other, exists) {
                        (_, false) => "rejected",
                        ("cancel", true) => "cancelled",
                        _ => "working",
                    };
                    if other == "cancel" && exists {
                        self.legs
                            .retain(|l| !(l.command_id == target && l.role == role));
                    }
                    let r = self.report(Some(&id), &instrument, role, status, now);
                    out.push(if exists {
                        Value::Object(r)
                    } else {
                        Self::with_reason(r, "command.target_not_found")
                    });
                }
            }
        }
        self.remember(&id, &out);
        out
    }

    fn remember(&mut self, id: &str, out: &[Value]) {
        if let Some(last) = out.iter().rev().find(|r| r["command_id"] == id) {
            self.handled.insert(id.to_owned(), last.clone());
        }
    }

    /// `position_snapshot` account entry.
    pub fn snapshot_account(&self) -> Value {
        let positions: Vec<Value> = self
            .positions
            .iter()
            .filter(|(_, p)| p.quantity != 0)
            .map(|(k, p)| {
                let (symbol, month) = k.split_once(':').unwrap_or((k, ""));
                json!({ "instrument": {"symbol": symbol, "contract_month": month},
                        "position": {"quantity": p.quantity, "average_price": self.script.price} })
            })
            .collect();
        let orders: Vec<Value> = self
            .legs
            .iter()
            .map(|l| {
                json!({ "platform_order_id": format!("SIM-{}-{}", &l.command_id[..8.min(l.command_id.len())], l.role),
                        "command_id": l.command_id, "child_role": l.role, "instrument": l.instrument,
                        "side": "sell", "quantity": 1, "order": {"type": if l.role == "stop" { "stop" } else { "limit" },
                        "price": self.script.price}, "origin": "all_win", "state": "working" })
            })
            .collect();
        json!({ "account_id": self.account_id, "platform_account": self.platform_account, "connected": true,
                "positions": positions, "working_orders": orders })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2027-01-15T08:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn place(id: &str, side: &str, qty: u32, effect: &str, refs: Option<&str>) -> Value {
        let mut c = json!({ "command_id": id, "kind": "place", "instrument": {"symbol": "MNQ", "contract_month": "2026-12"},
            "place": {"side": side, "quantity": qty, "order": {"type": "market"}, "position_effect": effect} });
        if effect == "open" {
            c["expires_at"] = json!("2027-01-15T08:00:05Z");
            c["place"]["protection"] =
                json!({"mode": "individual", "stop_ticks": 40, "target_ticks": 80});
        }
        if let Some(r) = refs {
            c["refs"] = json!({"position_ref": r});
        }
        c
    }

    fn statuses(rs: &[Value]) -> Vec<(String, String)> {
        rs.iter()
            .map(|r| {
                (
                    r["child_role"].as_str().unwrap().to_owned(),
                    r["status"].as_str().unwrap().to_owned(),
                )
            })
            .collect()
    }

    #[test]
    fn entry_fills_with_bracket_and_flatten_cancels_orphans() {
        let mut v = Venue::new(Uuid::nil(), "Sim101", Script::default());
        let rs = v.on_command(&place("a", "buy", 1, "open", None), now());
        assert_eq!(
            statuses(&rs),
            [
                ("entry", "received"),
                ("entry", "filled"),
                ("stop", "working"),
                ("target", "working")
            ]
            .map(|(a, b)| (a.into(), b.into()))
        );
        assert_eq!(rs[1]["position_after"]["quantity"], 1);
        let rs = v.on_command(&json!({"command_id": "f", "kind": "flatten", "instrument": {"symbol": "MNQ", "contract_month": "2026-12"}, "flatten": {"reason": "close"}}), now());
        // Cancels first (as the plugin's AdvanceFlatten), the close last.
        assert_eq!(rs[0]["reason_code"], "order.orphan_cancelled");
        assert_eq!(rs[1]["reason_code"], "order.orphan_cancelled");
        assert_eq!(rs[2]["child_role"], "flatten");
        assert_eq!(rs[2]["position_after"]["quantity"], 0);
        // Q75-1: a flatten on a flat contract still cancels manual working orders, first.
        v.add_manual_order(json!({"symbol": "MNQ", "contract_month": "2026-12"}));
        let rs = v.on_command(&json!({"command_id": "g", "kind": "flatten", "instrument": {"symbol": "MNQ", "contract_month": "2026-12"}, "flatten": {"reason": "close"}}), now());
        assert_eq!(
            (rs[0]["origin"].as_str(), rs[0]["status"].as_str()),
            (Some("external"), Some("cancelled"))
        );
        assert_eq!(rs[1]["reason_code"], "command.already_flat");
        assert_eq!(v.manual_orders(), 0);
    }

    #[test]
    fn duplicate_expired_and_reduce_guard() {
        let mut v = Venue::new(Uuid::nil(), "Sim101", Script::default());
        v.on_command(&place("a", "buy", 2, "open", None), now());
        assert_eq!(
            v.on_command(&place("a", "buy", 2, "open", None), now())[0]["status"],
            "duplicate"
        );
        let late = now() + chrono::Duration::seconds(10);
        assert_eq!(
            v.on_command(&place("b", "buy", 1, "open", None), late)[0]["reason_code"],
            "command.expired"
        );
        assert_eq!(
            v.on_command(&place("r1", "sell", 1, "reduce", Some("other")), now())[0]["reason_code"],
            "command.position_changed"
        );
        let rs = v.on_command(&place("r2", "sell", 5, "reduce", Some("a")), now());
        assert_eq!(
            rs[1]["position_after"]["quantity"], 0,
            "capped, never flips"
        );
        assert_eq!(
            v.on_command(&place("r3", "sell", 1, "reduce", Some("a")), now())[0]["reason_code"],
            "command.nothing_to_reduce"
        );
    }

    #[test]
    fn scripted_reject_and_partial() {
        let script = Script {
            queue: VecDeque::from([
                Behavior::Reject {
                    code: "order.platform_rejected".into(),
                },
                Behavior::Partial { quantity: 1 },
            ]),
            ..Script::default()
        };
        let mut v = Venue::new(Uuid::nil(), "Sim101", script);
        assert_eq!(
            v.on_command(&place("a", "buy", 2, "open", None), now())[1]["reason_code"],
            "order.platform_rejected"
        );
        let rs = v.on_command(&place("b", "buy", 3, "open", None), now());
        assert_eq!(
            (
                rs[1]["status"].as_str(),
                rs[1]["position_after"]["quantity"].as_i64()
            ),
            (Some("partially_filled"), Some(1))
        );
    }

    #[test]
    fn in_tree_scenarios_parse() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/scenarios");
        let bracket: Script = serde_json::from_str(
            &std::fs::read_to_string(format!("{dir}/sunday-bracket.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(bracket.default, Behavior::Fill);
        assert_eq!(bracket.price, "21450.25");
        let reject: Script = serde_json::from_str(
            &std::fs::read_to_string(format!("{dir}/sunday-reject.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            reject.default,
            Behavior::Reject {
                code: "order.platform_rejected".into()
            }
        );
    }

    #[test]
    fn a_foreign_account_command_is_rejected_for_that_account() {
        let v = Venue::new(Uuid::from_u128(1), "Sim101", Script::default());
        let other = Uuid::from_u128(2);
        let r = v.reject_foreign(&place("x", "buy", 1, "open", None), other, "Live1", now());
        assert_eq!(r["status"], "rejected");
        assert_eq!(r["reason_code"], "order.platform_rejected");
        assert_eq!(r["account_id"], json!(other));
        assert_eq!(r["platform_account"], "Live1");
        assert_eq!(r["command_id"], "x");
    }
}
