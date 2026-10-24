# Gemeinsame Werte

`toyverse.json` enthält alle festen Werte, die mehrere Projekte im ToyVerse brauchen: Ticker, Dezimalstellen, Obergrenze, Chain-IDs, Blockzeit, Epoche, Emission, Gebühren, Kontingent, Aktivierungs-Pfand, Stufen, ToyWork, Typ-Bytes der Transaktionen, die alten ToyCoin-Token (nur als Warnung, Entscheidung 63), Sprachen und Grundregeln (Entscheidung 135).

- **Nicht von Hand ändern.** Der Test `toychain/tests/tests/toyverse_manifest.rs` erzeugt die Datei aus den Rust-Konstanten und schlägt in der CI fehl, wenn sie abweicht. Neu erzeugen (im Ordner `toychain/`):
  `UPDATE_TOYVERSE_JSON=1 cargo test -p toy-integration-tests --test toyverse_manifest`
- **Beträge sind Text** in kleinsten Einheiten (z. B. `"100000000000000000"` = 0,1 TOY), weil sie größer als 2⁵³ werden. In JavaScript `BigInt`, in Kotlin `BigInteger`, in Python `int`.
- **Nutzer:** ToyVerse-Webseite und ToyPost holen die Datei täglich per Pull Request, später auch ToySafe, TOYPAY, ToyScan und TOYQUEST. Einzelheiten: `docs/spaeter/15-verknuepfung-und-kompatibilitaet.md`.
