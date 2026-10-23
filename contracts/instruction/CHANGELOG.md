# Instruction contract — changelog

## 1.0 — 2026-10-23 (Proposed, ADR-0003)
- Initial version: envelope, actions buy/sell/close/reverse/modify/cancel, `position_effect`, `side`, `order`, `changes`, `refs` (position_ref, order_ref, parent_order_ref), decimals as strings.

## 1.0 — accepted 2026-10-23
- No schema change. ADR-0003 acceptance adds Core policy only: automation opt-in gate, per-(source, account) signal filter, token format, command step codes.
