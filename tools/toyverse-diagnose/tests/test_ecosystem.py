import datetime as dt
import json
import tempfile
import unittest
from pathlib import Path

from toyverse_diagnose import ecosystem, history

MANIFEST = {
    "schema": "toyverse/1",
    "toy": {"ticker": "TOY", "decimals": 18},
    "chainIds": {"mainnet": 7630713, "testnet": 7630714, "devnet": 7630715},
    "legacyTokens": [{"chainId": 137, "address": "0x1854b69cd1495c9753c5F1590A35e5DdB556A5BF"}],
    "i18n": {"languages": 45},
}


def workspace(files: dict[str, str]) -> Path:
    root = Path(tempfile.mkdtemp())
    base = {
        "toycoin/shared/toyverse.json": json.dumps(MANIFEST),
        "toycoin/LICENSE": "MIT License",
        "toymeshos/LICENSE": "MIT License",
    }
    base.update(files)
    for rel, text in base.items():
        p = root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")
    return root


def rules_of(findings):
    return {f.rule for f in findings}


class EcosystemTests(unittest.TestCase):
    def test_clean_workspace_has_no_red(self):
        f, _ = ecosystem.diagnose(workspace({}))
        self.assertFalse([x for x in f if x.severity == "rot"])

    def test_outdated_claims_in_app_list_are_red(self):
        apps = json.dumps({"apps": [
            {"desc": "Schlanke Blockchain, EVM-kompatibel."},
            {"desc": "Wallet für ToyCoin auf Polygon."},
            {"desc": "Stablecoin auf der ToyChain."},
        ]}, ensure_ascii=False)
        f, _ = ecosystem.diagnose(workspace({"toymeshos/overlay/srv/toyverse/apps.json": apps}))
        self.assertTrue({"E02", "E03", "E04"} <= rules_of(f))

    def test_negated_reserve_is_not_a_finding(self):
        text = '{"why": "Totaler Fair Launch: kein Premint, keine Reserve."}'
        f, _ = ecosystem.diagnose(workspace({"toysafe/content/a.json": text, "toysafe/LICENSE": "MIT"}))
        self.assertNotIn("E05", rules_of(f))

    def test_reserve_claim_is_a_finding(self):
        text = '{"why": "Premint mit 1 % Reserve für das Team."}'
        f, _ = ecosystem.diagnose(workspace({"toysafe/content/a.json": text, "toysafe/LICENSE": "MIT"}))
        self.assertIn("E05", rules_of(f))

    def test_unknown_chain_id(self):
        f, _ = ecosystem.diagnose(workspace({"toycoin/x.md": "Chain-ID 7630799"}))
        self.assertIn("B02", rules_of(f))

    def test_wrong_checksum_of_legacy_token(self):
        f, _ = ecosystem.diagnose(workspace({"toycoin/x.md": "0x1854B69CD1495C9753C5F1590A35E5DDB556A5BF"}))
        self.assertIn("B03", rules_of(f))

    def test_float_amount(self):
        f, _ = ecosystem.diagnose(workspace({"toypost/src/pay.ts": "const v = parseFloat(amount) * 2;", "toypost/LICENSE": "MIT"}))
        self.assertIn("E08", rules_of(f))

    def test_missing_license(self):
        f, _ = ecosystem.diagnose(workspace({"toypost/README.md": "x"}))
        self.assertTrue(any(x.rule == "B08" and x.repo == "toypost" for x in f))

    def test_manifest_copy_drift(self):
        copy = dict(MANIFEST, chainIds={"mainnet": 1})
        f, _ = ecosystem.diagnose(workspace({"toyverse-web/src/data/toyverse.json": json.dumps(copy)}))
        self.assertIn("B01", rules_of(f))

    def test_autofix_repo_names(self):
        root = workspace({"toypost/README.md": "Siehe Safehaigit/toycoin und Safehaigit/ToyVerse-web.", "toypost/LICENSE": "MIT"})
        f, ws = ecosystem.diagnose(root)
        changed = ecosystem.apply_fixes(ws, f)
        self.assertEqual(len(changed), 1)
        text = (root / "toypost/README.md").read_text()
        self.assertIn("TOYfoundation/toycoin", text)
        self.assertIn("TOYfoundation/ToyVerse-web", text)
        f2, _ = ecosystem.diagnose(root)
        self.assertNotIn("E01", rules_of(f2))

    def test_score_counts_rules_not_copies(self):
        many = "\n".join(["Safehaigit/toycoin"] * 50)
        f, ws = ecosystem.diagnose(workspace({"toypost/README.md": many, "toypost/LICENSE": "MIT"}))
        self.assertGreaterEqual(ecosystem.score(f, ws.repos)["toypost"], 90)


class HistoryTests(unittest.TestCase):
    def _f(self, rule="E01", sev="gelb", excerpt="x"):
        return ecosystem.Finding(rule, "t", sev, "toypost", "a.md", 1, excerpt)

    def test_new_fixed_and_regression(self):
        h = {}
        a, b = self._f(excerpt="a"), self._f(excerpt="b")
        d = history.compare([a, b], h)
        h = history.record(h, [a, b], {}, d)
        d = history.compare([a], h)
        self.assertEqual(d["behoben"], [b.fingerprint])
        h = history.record(h, [a], {}, d)
        b2 = self._f(excerpt="b")
        d = history.compare([a, b2], h)
        self.assertEqual(d["rueckfall"], [b.fingerprint])

    def test_escalation_after_two_weeks(self):
        f = self._f()
        start = dt.date(2026, 10, 1)
        h = {}
        h = history.record(h, [f], {}, history.compare([f], h, start), start)
        f2 = self._f()
        history.compare([f2], h, start + dt.timedelta(days=history.ESCALATE_AFTER_DAYS))
        self.assertEqual(f2.severity, "rot")
        self.assertTrue(f2.extra["eskaliert"])

    def test_exception_with_expiry(self):
        f = self._f()
        exc = [{"regel": "E01", "repo": "toypost", "pfad": "a.md", "bis": "2026-11-01", "grund": "Archiv"}]
        kept, acc, _ = history.apply_exceptions([f], exc, dt.date(2026, 10, 26))
        self.assertEqual((len(kept), len(acc)), (0, 1))
        kept, acc, exp = history.apply_exceptions([self._f()], exc, dt.date(2026, 11, 2))
        self.assertEqual((len(kept), len(exp)), (1, 1))
        self.assertIn("abgelaufen", kept[0].title)

    def test_next_steps_ranks_red_first(self):
        steps = history.next_steps([self._f("E01", "gelb"), self._f("E02", "rot")])
        self.assertEqual(steps[0]["regel"], "E02")


if __name__ == "__main__":
    unittest.main()
