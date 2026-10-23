# All Win local gates. `make check` runs everything; each part can run alone.
# Needs: Rust toolchain (/opt/rust), Node + pnpm, .NET SDK, Docker usable by this user, psql, python3 + jsonschema.
# Remote CI is pending the user's authorization to create the GitHub repository; this is its local equivalent.
.PHONY: check check-contracts check-core check-web check-plugin

check: check-contracts check-core check-web check-plugin

check-contracts:
	python3 contracts/scripts/validate_fixtures.py contracts
	python3 contracts/scripts/check_openapi.py contracts/http/openapi.yaml

check-core:
	infra/scripts/check-core.sh

check-web:
	cd web && pnpm install --frozen-lockfile && pnpm check

# CI=true makes the props enable RestoreLockedMode and stop suppressing NU1900.
check-plugin:
	CI=true dotnet restore plugins/ninjatrader8 --locked-mode --nologo -v q
	CI=true dotnet test plugins/ninjatrader8 --nologo -v q --no-restore
