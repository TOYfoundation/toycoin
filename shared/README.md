# Gemeinsame Werte

`toyverse.json` enthält alle festen Werte, die mehrere Projekte im ToyVerse brauchen: Ticker, Dezimalstellen, Obergrenze, Chain-IDs, Blockzeit, Epoche, Emission, Gebühren, Kontingent, Aktivierungs-Pfand, Stufen, ToyWork, Typ-Bytes der Transaktionen, die alten ToyCoin-Token (nur als Warnung, Entscheidung 63), Sprachen und Grundregeln (Entscheidung 135).

- **Nicht von Hand ändern.** Der Test `toychain/tests/tests/toyverse_manifest.rs` erzeugt die Datei aus den Rust-Konstanten und schlägt in der CI fehl, wenn sie abweicht. Neu erzeugen (im Ordner `toychain/`):
  `UPDATE_TOYVERSE_JSON=1 cargo test -p toy-integration-tests --test toyverse_manifest`
- **Beträge sind Text** in kleinsten Einheiten (z. B. `"100000000000000000"` = 0,1 TOY), weil sie größer als 2⁵³ werden. In JavaScript `BigInt`, in Kotlin `BigInteger`, in Python `int`.
- **Schema `toyverse/2` (seit M3b, 25.10.2026):** Tokenomics v0.4. Entfallen sind `emission.k0Ppm`, `emission.consensusSharePpm`, `emission.workSharePpm`, `fees.lotterySharePpm` und `stakeTiersUnits`. Neu sind `emission.kTable*` (K-Tabelle mit BLAKE3-Hash), `split`, `bootstrap`, `lottery` (Grundtopf, Jackpots Funke, Welle, Gezeiten, Glücksstufen, Mindestgewinne), `fees.stakingSharePpm` und die Vielnutzer-Werte, `quota.waterLevelPpm` und `quota.rampBlocks`, `stake` (7 Stufen mit Rolle, Gewicht Sparer, Deckel) und die Los-Grenzen in `toyWork`. Wer die Datei liest, prüft zuerst `schema`.
- **Nutzer:** ToyVerse-Webseite und ToyPost holen die Datei täglich per Pull Request, später auch ToySafe, TOYPAY, ToyScan und TOYQUEST. Einzelheiten: `docs/spaeter/15-verknuepfung-und-kompatibilitaet.md`.
