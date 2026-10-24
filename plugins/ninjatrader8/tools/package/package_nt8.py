#!/usr/bin/env python3
"""Builds the NT8 round delivery (ADR-0005 N9/N12): import zips, the fake Core and the Windows scripts.

Usage (from plugins/ninjatrader8):
  python3 -I tools/package/package_nt8.py --version 0.1.1-dev --build development --fakecore <publish dir> --out <dir>

Produces in --out:
  AllWin-NT8-<version>.zip             NinjaScript Add-On import: host source (AddOns/AllWin/*.cs, C# 7.3) +
                                       AllWin.Adapter.Core.dll (net48) + AdditionalReferences.txt + Info.xml
  AllWin-NT8-CoreSource-<version>.zip  optional E2 test: AllWin.Adapter.Core as source
  AllWin-NT8-<version>-entrega/        delivery folder: the zips above, AllWin.FakeCore.exe (+ scenarios),
                                       start-fakecore.cmd, collect-allwin.cmd/.ps1, LEIA-ME.txt
  AllWin-NT8-<version>-entrega.zip     the delivery folder, zipped
  SHA256SUMS                           next to the zip (not inside): the zip and every file in it
Zips are deterministic (sorted entries, fixed timestamps), so the same inputs give the same sha256.
Checks (fail the build): no NinjaTrader DLL, no Newtonsoft, no AllWin.Adapter.Sim / FakeCore code in an
import zip; the AddOn source compiles against the NT API stubs (tools/NtCompileCheck).
"""
import argparse
import hashlib
import os
import shutil
import subprocess
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
PLUGIN = os.path.abspath(os.path.join(HERE, "..", ".."))
FIXED_TIME = (2026, 1, 1, 0, 0, 0)
NT_VERSION = "8.1.8.4"

INFO_XML = """<?xml version="1.0" encoding="utf-8"?>
<NinjaTrader>
  <Export>
    <Version>{nt}</Version>
  </Export>
</NinjaTrader>
"""


def run(cmd, cwd=PLUGIN):
    print("+ " + " ".join(cmd), flush=True)
    subprocess.run(cmd, cwd=cwd, check=True)


def write_zip(path, entries):
    """entries: list of (archive name, bytes). Deterministic: sorted, fixed time, fixed mode."""
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as z:
        for name, data in sorted(entries, key=lambda e: e[0]):
            info = zipfile.ZipInfo(name, FIXED_TIME)
            info.external_attr = 0o644 << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            z.writestr(info, data)


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def read(path):
    with open(path, "rb") as f:
        return f.read()


def check_import_zip(path):
    forbidden = ("ninjatrader.", "newtonsoft", "allwin.adapter.sim", "allwin.fakecore")
    with zipfile.ZipFile(path) as z:
        for name in z.namelist():
            low = os.path.basename(name).lower()
            if low.endswith(".dll") and any(low.startswith(f) for f in forbidden):
                sys.exit("forbidden DLL in " + path + ": " + name)
            if low.endswith(".cs"):
                text = z.read(name).decode("utf-8")
                for marker in ("FailNext", "CrashNext", "FillOnNextCancel", "AllWin.Adapter.Sim", "AllWin.FakeCore"):
                    if marker in text:
                        sys.exit("fault-injection or test code (" + marker + ") in " + path + ": " + name)


# Development-only capabilities (spike rev. 7.2). A release package must not contain any of them: the build switch must be
# false and none of these markers may appear in the host source.
DEV_MARKERS = (b"fakecore-dev.token", b"dev-core.json", b"127.0.0.1:8765", b"AllWinDiagnostics.Instance.Start", b"ListExecutions")


def check_build(build, host):
    runtime = next(data for name, data in host if name.endswith("AllWinRuntime.cs"))
    dev = b"public const bool Development = true;" in runtime
    if build == "development":
        if not dev:
            sys.exit("--build development but AllWinBuild.Development is not true")
        return
    if dev:
        sys.exit("--build release refused: AllWinBuild.Development is true")
    for name, data in host:
        for marker in DEV_MARKERS:
            if marker in data:
                sys.exit("--build release refused: development capability " + marker.decode() + " in " + name)
    sys.exit("--build release: no release transport exists yet (pairing/DPAPI come with S3); refusing")


def core_dll(build_dir):
    run(["dotnet", "build", "src/AllWin.Adapter.Core/AllWin.Adapter.Core.csproj", "-c", "Release", "-f", "net48",
         "-p:RestoreLockedMode=true", "-o", build_dir, "-v", "q", "--nologo"])
    return os.path.join(build_dir, "AllWin.Adapter.Core.dll")


def core_sources():
    root = os.path.join(PLUGIN, "src", "AllWin.Adapter.Core")
    out = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = sorted(d for d in dirnames if d not in ("bin", "obj"))
        for f in sorted(filenames):
            if f.endswith(".cs"):
                rel = os.path.relpath(os.path.join(dirpath, f), root).replace(os.sep, "/")
                # The project enables nullable annotations; NT8 compiles with its own settings, so say it per file.
                out.append(("AddOns/AllWinCoreSource/" + rel, b"#nullable enable\n" + read(os.path.join(dirpath, f))))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", required=True)
    ap.add_argument("--build", choices=["development", "release"], required=True)
    ap.add_argument("--fakecore", help="dotnet publish output of tools/AllWin.FakeCore for win-x64 (development only)")
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    out = os.path.abspath(args.out)
    os.makedirs(out, exist_ok=True)
    work = os.path.join(out, ".work")
    shutil.rmtree(work, ignore_errors=True)
    os.makedirs(work)

    run(["dotnet", "build", "tools/NtCompileCheck/NtCompileCheck.csproj", "-p:RestoreLockedMode=true", "-v", "q", "--nologo"])
    dll = core_dll(os.path.join(work, "core-net48"))

    info = INFO_XML.format(nt=NT_VERSION).encode("utf-8")
    host_dir = os.path.join(PLUGIN, "src", "AllWin.NinjaTrader8")
    host = [("AddOns/AllWin/" + f, read(os.path.join(host_dir, f))) for f in sorted(os.listdir(host_dir)) if f.endswith(".cs")]
    check_build(args.build, host)
    for name, data in host:
        if b"const string Version = \"" + args.version.encode("utf-8") + b"\"" not in data and name.endswith("AllWinRuntime.cs"):
            sys.exit("AllWinRuntime.Version does not match --version " + args.version)
    probe_zip = os.path.join(out, "AllWin-NT8-" + args.version + ".zip")
    write_zip(probe_zip, [
        ("Info.xml", info),
        ("AdditionalReferences.txt", b"AllWin.Adapter.Core\r\n"),
        ("AllWin.Adapter.Core.dll", read(dll)),
    ] + host)
    check_import_zip(probe_zip)

    source_zip = os.path.join(out, "AllWin-NT8-CoreSource-" + args.version + ".zip")
    write_zip(source_zip, [("Info.xml", info)] + core_sources())
    check_import_zip(source_zip)

    delivery = os.path.join(out, "AllWin-NT8-" + args.version + "-entrega")
    shutil.rmtree(delivery, ignore_errors=True)
    os.makedirs(delivery)
    shutil.copy2(probe_zip, delivery)
    shutil.copy2(source_zip, delivery)
    if not args.fakecore:
        sys.exit("--fakecore is required for a development delivery")
    fake = os.path.abspath(args.fakecore)
    shutil.copy2(os.path.join(fake, "AllWin.FakeCore.exe"), delivery)
    shutil.copytree(os.path.join(fake, "scenarios"), os.path.join(delivery, "scenarios"))
    for f in ("start-fakecore.cmd", "start-r1.cmd", "pair-allwin.cmd", "pair-allwin.ps1", "collect-allwin.cmd", "collect-allwin.ps1", "LEIA-ME.txt"):
        data = read(os.path.join(HERE, f)).replace(b"{version}", args.version.encode("utf-8"))
        if f.endswith((".cmd", ".txt")):
            data = data.replace(b"\r\n", b"\n").replace(b"\n", b"\r\n")
        with open(os.path.join(delivery, f), "wb") as w:
            w.write(data)

    entries = []
    for dirpath, _, filenames in os.walk(delivery):
        for f in filenames:
            p = os.path.join(dirpath, f)
            entries.append((os.path.basename(delivery) + "/" + os.path.relpath(p, delivery).replace(os.sep, "/"), read(p)))
    delivery_zip = delivery + ".zip"
    write_zip(delivery_zip, entries)
    shutil.rmtree(work, ignore_errors=True)

    # SHA256SUMS lives NEXT TO the delivery zip, never inside it (Maestro: the hash travels in his message).
    sums = [sha256(delivery_zip) + "  " + os.path.basename(delivery_zip)]
    for dirpath, _, filenames in os.walk(delivery):
        for f in sorted(filenames):
            p = os.path.join(dirpath, f)
            sums.append(sha256(p) + "  " + os.path.basename(delivery) + "/" + os.path.relpath(p, delivery).replace(os.sep, "/"))
    with open(os.path.join(out, "SHA256SUMS"), "w") as w:
        w.write("\n".join(sums) + "\n")

    for p in (probe_zip, source_zip, delivery_zip):
        print(sha256(p) + "  " + os.path.basename(p))


if __name__ == "__main__":
    main()
