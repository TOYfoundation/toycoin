# Sprachen (gemeinsam für Wallet, TOYPAY-App und ToyScan)

Entscheidung 103: Wallet, TOYPAY-App und ToyScan gibt es in **denselben 45 Sprachen wie TOYQUEST 3D**, mit allen Erklärungen.

| Datei | Inhalt |
|---|---|
| `sprachen.tsv` | die 45 Sprachen: Kürzel, Eigenname, Schreibrichtung (`rtl` bei Arabisch, Urdu, Persisch), Schrift. Gleiche Kürzel wie in TOYQUEST 3D (`toypay/docs/01_PROJEKT_UEBERSICHT.md`). Die Eigennamen werden mit der Sprachliste des Spiels (`godot3d/scripts/i18n.gd`) abgeglichen, sobald dessen Code im Repository liegt |
| `begriffe.tsv` | gemeinsame Begriffsliste, damit „Phrase“, „Aktivierungs-Pfand“ usw. in allen Apps und Sprachen gleich übersetzt werden |

**Regeln:** Keine festen Texte im Code, nur Schlüssel. Quelle sind Deutsch und Englisch, alle anderen Sprachen fallen bei fehlenden Texten auf Englisch zurück. Platzhalter `{0}`, `{1}` müssen in jeder Sprache gleich sein. Details: `docs/04-toypay-wallet.md`, Abschnitt 7d.
