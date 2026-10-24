# Migration register — Wave 1

Owner: Database Architect (Lastro). State as of 2026-10-24, data model rev. 16.23.

This is the single list of migration versions: who owns each one, which hash was approved, where it is committed and whether it deletes data. The DDL lives in [wave1-data-model.md](wave1-data-model.md); the rules are in its §8 "Conventions".

## Rules

- The DBA assigns every version. No branch picks its own. From `…061900` on, numbers are assigned on request.
- Main merges migrations in numeric order. A branch that must land after a higher number is already on `main` is re-stamped by the DBA **before** its merge, never after. Re-stamping renames the file only: sqlx checksums the content, and the version comes from the file name.
- A file on `main` is never edited or renamed. A file committed only on an unmerged branch and never applied in a shared environment may still be re-stamped. Throwaway test databases are recreated from the migrations.
- **Approved hash** = `sha256(file)[:12]` of the reviewed file, unless marked "patch" (`sha256` of the reviewed `git diff`). On 2026-10-24 each file was compared with the DBA's saved approved copy: every saved copy matches the file on the listed branch. For 060610–061100 no copy was saved; the row shows the file hash at the listed commit.
- **Deletes data:** a migration that removes or rewrites non-test data needs explicit user authorization, and the Maestro is warned before it (user rule 2026-10-24). Dropping a constraint in order to re-add a wider one, or dropping an index, deletes no data.

## Register

Versions are `20261023` + the number shown. "Train" = `s/v1-sunday-integration--core`.

| Version | # | File (module, object) | Owner | Approved hash | Commit (branch) | State | Deletes data |
|---|---|---|---|---|---|---|---|
| 060000 | 1 | `app_bootstrap` | Ferro | — | `main` | on `main` | no |
| 060010 | 1b | `app_outbox_delivery_attempts` | Ferro | — | `main` | on `main` | no |
| 060100 | 2 | `identity_create_users_sessions` | Brasa | — | `main` | on `main` | no |
| 060200 | 3 | `identity_create_sources` | Brasa | — | `main` | on `main` | no |
| 060300 | 4 | `catalog_create_instrument` | Ferro | — | `main` | on `main` | no |
| 060500 | 6 | `routing_create_instruction` | Ferro | — | `main` | on `main` | no |
| 060600 | 7 | `audit_create_audit_entry` | Ferro | — | `main` | on `main` | no |
| 060610 | 1c | `app_outbox_seq` | Ferro | approved before copies were kept; file `9a9c84a066d7` | 196a30a (`s/v1-audit-codes--core`) | in train | no |
| 060620 | 7b | `audit_entry_activity_columns` | Ferro | approved before copies were kept; file `3da39cc3e616` | 196a30a | in train | no (re-adds `audit_entry_no_token` wider; replaces an index) |
| 060630 | 2c | `identity_resolve_source_key_name` | Ferro (Brasa approves) | approved before copies were kept; file `3dfdd728182e` | 196a30a | in train | no (function re-created) |
| 060700 | 8 | `identity_create_adapter_connections` | Brasa | approved before copies were kept; file `68c761514471` | c7a0429 (S3) | in train | no |
| 060800 | 2b | `identity_passwordless` | Brasa | approved before copies were kept; file `ef83bbe2af87` | 2d22e07 (S3) | in train | **yes**: drops the password/TOTP/reset tables and columns, deletes password sessions; authorized by the user (dev data only, rev. 6) |
| 060900 | 5a | `catalog_create_opt_in_rule_version` | Malha | approved before copies were kept; file `3766e740ef4c` | 047ec4a (S5) | in train | no |
| 061000 | 5 | `accounts_create_account` | Malha | approved before copies were kept; file `bcb728a481d4` | 3cc10f2 (S5) | in train | no |
| 061050 | 5b | `accounts_create_rule_values` | Malha | approved before copies were kept; file `af3368409395` | 3cc10f2 (S5) | in train | no |
| 061100 | 5c | `accounts_create_bracket_profiles` | Torno | approved before copies were kept; file `f7308d986ecc` | e924bf0 (S6) | in train | no |
| 061200 | 6b | `routing_create_decision_subscription` | Ferro | patches `52a97bd140fa` (leg) + `6f3a77829e95` (outcome); file `b4ce6e393484` | eea31cf (`s/v1-s7-routing--core`) | in train | no |
| 061250 | 6d | `routing_message_receipt_body_hash` | Ferro | `16f104b00fd0` | d61d2d7 (`s/v1-ingress-duplicate-conflict--core`, WIP) | joins the train before `main` (Maestro, 2026-10-24) | no (expand only) |
| 061300 | 6c | `routing_message_id_no_token_check` | Ferro | patch `44c5cb522587` (Lastro, Crivo, Brasa), file `324e4fb70c3e` | 3ebf247 (`s/v1-routing-message-id-no-token--core`, on 061250; `git diff d61d2d7 3ebf247` = the approved patch) | joins the train before `main` (Maestro, 2026-10-24) | no (CHECK only; authorized by the user 2026-10-24) |
| 061310 | 2d | `identity_source_name_check` | Brasa | `d7e419e49d1a` | c1fce6c (S3) | in train | no (CHECK only; authorized by the user 2026-10-24) |
| 061400 | 14 | `app_create_outbox_consumer_tracking` | Brasa | `51a75e5bd003` | 4e1be59 (`s/v1-outbox-consumer-tracking--core`) | in train | no (drops `outbox_due_idx`, an index) |
| 061500 | 10 | `risk_create_account_limits` | Malha | patch `bbe2b9a2828e`, file `d7821ed786de` | 68d907a (`s/v1-s13-risk--core`) | committed, not in train | no |
| 061510 | 10b | `risk_create_applied_fill` | Malha | patch `476fdeb64f1b`, file `5ee737c8d89a` | 712846c (`s/v1-s13-risk--core`) | committed, not in train | no |
| 061520 | 10c | `risk_create_position` | Malha | — | — | assigned (rev. 16.23), not written | no |
| 061530 | 10d | `risk_applied_fill_cost_basis` | Malha | — | — | assigned (rev. 16.23), not written | no (adds columns; S13 test databases with rows are recreated) |
| 061600 | 9 | `gateway_create_delivery` | Bigorna | patch `e859c35d4212`, file `2fcbf535ff2a` | 1debc53 (`s/v1-s2-gateway--core`) | in train | no |
| 061610 | 9b | `gateway_create_deferred_inbound` | Bigorna | patch `b1b2301bd1ec`, file `5941cb1a5515` | ecf44f6 (`s/v1-s2-gateway--core`) | in train | no |
| 061620 | 9c | `gateway_create_intake_dead_letter` | Bigorna | — | — | assigned (rev. 16.16), not received | — |
| 061650 | — | — | — | — | — | withdrawn (Esquadro chose (B)); do not reuse | — |
| 061700 | 11 | `catalog_create_presets` | Torno | concatenation `4f4fe7029dff`; file `4e685699f65a` | 60b40d0 (`s/v1-s16-presets--core`) | committed, not in train; needs the `allwin_publisher` role (infra 78b60d7) in every environment | no |
| 061710 | 11b | `catalog_opt_in_rule_version_release` | Torno | same concatenation; file `0a3d739abdb6` | 4190c09 | committed, not in train | no |
| 061720 | 11c | `accounts_create_catalog_release_cursor` | Torno | same concatenation; file `6bc4b3b6b7c9` | 4190c09 | committed, not in train | no |
| 061730 | 11d | `accounts_worker_find_release_affected` | Torno | `8a427fb2a7bc`; Malha's review pending | a2c797c | committed, not in train | no |
| 061800 | 2e | `identity_admin_channel_recovery` | Cinzel | `9607bc9b8afc` | 3d34bb7 (`s/v1-s3-admin-recovery--core`) | committed, not in train | no (CHECKs dropped and re-added wider) |
| 061810 | 2f | `identity_step_up_account_actions` | Brasa | `684054624429` | 2ead509 (`s/v1-identity-step-up-account-actions--core`, on 061800) | committed, not in train | no (CHECKs dropped and re-added wider) |
| 061900+ | 13 … | `trades_create_trade` and later | on request | — | — | not assigned | — |

## Merge order

1. **Train** (`s/v1-sunday-integration--core`, 9878402): 060610 → 061200, 061310, 061400, 061600, 061610. These are in numeric order and apply cleanly on top of `main` (≤ 060600).
2. **061250 + 061300 (Ferro)** join the train in its step 4, before the train merges to `main`, and keep their numbers (Maestro decision 2026-10-24, on Esmeril's recommendation). QA databases built from the earlier train are recreated, not migrated forward. The rev. 16.21 reservation of 061900/061910 for a re-stamp is released; if they still miss the train, the DBA assigns re-stamp numbers before their merge.
3. **061500 → 061530** (S13, Malha): after the train; no re-stamp needed.
4. **061620** (Bigorna): after 061610; no re-stamp needed if it merges after the train.
5. **061700–061730** (S16, Torno): after the `allwin_publisher` infra (78b60d7) is merged and the role exists in every environment, and after Malha approves 061730.
6. **061800, then 061810** (Cinzel, then Brasa): 061810's branch already contains 061800. Board merge-train notes: rename the consumer `identity-operator` → `identity.operator` when AC-12 meets 061400; the step-up DB test loop sets `session = (action != AdminRecovery)`.
7. Groups 3–6 may land in any order. When a group lands while a lower-numbered group is still unmerged, the DBA re-stamps the lower group before its own merge, as in step 2.
