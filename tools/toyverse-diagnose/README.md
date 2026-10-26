# toyverse-diagnose

Selbstanpassende Diagnose für das ToyVerse. Zwei Teile, ein Werkzeug, nur Python 3 ohne Zusatzpakete. Es läuft deshalb gleich auf dem Galaxy Tab 2 (armv7), dem Raspberry Pi, Linux- und Windows-PCs und in der CI.

| Teil | Frage | Ergebnis |
|---|---|---|
| `oekosystem` | Passen alle Repositories zueinander und zu den Entscheidungen in toycoin? | Kompatibilitäts-Index je Repository, Befunde, nächste Schritte, sichere Korrekturen |
| `geraet` | Was kann dieses Gerät, und wie läuft ToyMesh darauf am besten? | Leistungsklasse K1 bis K4, Profil für Oberfläche, Browser, ToyChain-Stufe und Apps |

## Ökosystem-Diagnose

Alle Repositories nebeneinander klonen (`toycoin`, `toypost`, `toyverse-web`, `toysafe`, `toymeshos`), dann:

```bash
python3 -m toyverse_diagnose oekosystem --wurzel ~/toyverse --md bericht.md --json bericht.json
python3 -m toyverse_diagnose oekosystem --wurzel ~/toyverse --korrigieren   # sichere Korrekturen anwenden
```

Ausgabe-Code: 0 grün, 1 gelb, 2 rot. So kann die CI bei Rot anhalten.

**Was geprüft wird.** Die Regeln stehen in `toyverse_diagnose/regeln.json`, jede mit der Entscheidung, auf die sie sich stützt. Neue Regeln kommen dort hinzu, nicht in den Code.

| Regel | Prüft | Stufe |
|---|---|---|
| E01 | veraltete Repository-Namen (`Safehaigit/…` statt `TOYfoundation/…`) | gelb, automatisch korrigierbar |
| E02 bis E06 | Aussagen, die Entscheidungen widersprechen: EVM, Polygon-Wallet, ToyCash auf der Kette, Reserve, Bridge | rot |
| E07 | Schriften und Skripte von fremden Servern | gelb |
| E08 | TOY-Beträge als Fließkomma | rot |
| E09 | private Schlüssel im Repository | rot |
| E10 | Stufen nach Tokenomics v0.3 | Hinweis |
| B01 | Kopien von `toyverse.json` gleich der Quelle | gelb |
| B02, B03 | Chain-IDs und Prüfsumme der alten Token-Adressen | rot, gelb |
| B04, B05 | Sprach- und Begriffslisten, Fortschritt der 45 Sprachen | gelb, Hinweis |
| B06 | Roadmap der Webseite gleich `docs/STATUS.md` | gelb |
| B07 | jedes Produkt in jedem Überblick (Webseite, ToyMesh, toycoin) | gelb |
| B08 | Lizenzdatei (MIT) | gelb |
| B09 | gleiche Node-Mindestversion | Hinweis |

**Wie es sich selbst verbessert.** Im Ordner `--zustand` (Standard `.toyverse-diagnose/`) liegt ein Verlauf:

1. **Gedächtnis:** Jeder Befund hat einen stabilen Fingerabdruck. Der Bericht zeigt, was neu, behoben oder zurückgekehrt ist (Rückfall).
2. **Eskalation:** Ein gelber Befund, der 14 Tage offen bleibt, wird rot. So veraltet nichts unbemerkt.
3. **Ausnahmen mit Ablauf:** In `ausnahmen.json` lässt sich ein Befund mit Grund und Enddatum akzeptieren. Nach Ablauf kommt er zurück.
   ```json
   [{"regel": "E01", "repo": "toycoin", "pfad": "docs/02-entscheidungen.md", "bis": "2026-12-31", "grund": "Protokoll, historisch"}]
   ```
4. **Rangfolge:** Die nächsten Schritte sind nach Wirkung sortiert: erst Rot, dann was viele Repositories betrifft, dann was automatisch korrigierbar ist.
5. **Index-Verlauf:** Der Kompatibilitäts-Index je Lauf zeigt den Trend.

Der Index zählt jede Regel einmal je Repository (mit leichtem Aufschlag für viele Fundstellen). 90 veraltete Links sind ein Problem mit einer Lösung, nicht 90 Probleme.

## Geräte-Diagnose für ToyMesh

```bash
toymesh-profil            # auf ToyMesh, schreibt /srv/toyverse/geraet.json
python3 -m toyverse_diagnose geraet --zustand ./z.json --ausgabe ./geraet.json
```

Liest nur `/proc` und `/sys`, ändert keine Einstellung. Der Launcher liest `geraet.json` beim Start.

| Klasse | Typisch | Oberfläche | Effekte | ToyChain |
|---|---|---|---|---|
| K1 Minimal | unter 1,5 GB RAM oder armv7 ohne GPU-Treiber (Galaxy Tab 2) | ToyTiles (Touch) oder ToyFocus | aus | Leicht-Knoten, ToyWork aus |
| K2 Leicht | unter 3 GB oder ohne GPU-Treiber | ToyDroid oder ToyClassic | reduziert | Konto |
| K3 Standard | unter 6 GB | ToyAqua | normal | Attestierer |
| K4 Stark | ab 6 GB mit GPU | ToyAqua | normal | Voll-Knoten, wenn 2 GB frei |

Browser und Zahl gleichzeitiger Apps richten sich nach dem RAM. ToyWork läuft nur am Ladekabel und unter 75 °C. Jede App bekommt einen Modus `voll`, `leicht` oder `web`. TOYQUEST 3D braucht zum Beispiel einen GPU-Treiber, sonst startet die Lernwelt als Webversion.

**Anpassung im Betrieb:** Stündlich misst ToyMesh Speicherdruck (PSI) und OOM-Abbrüche. Eine belastete Stunde stuft sofort eine Klasse herunter, fünf ruhige Stunden in Folge wieder hinauf. Über die Hardware-Klasse geht es nie hinaus. Ein RAM-Ausbau wird beim nächsten Lauf erkannt.

**Installationsweg:** Die Diagnose sagt auch, welcher ToyMesh-Build passt (PC x86_64 mit UEFI oder BIOS, Raspberry Pi, armv7, aarch64-Handy mit postmarketOS-Port) oder ob nur der Browser bleibt (32-Bit-x86).

## Tests

```bash
python3 -m unittest discover -s tests -t .
```

25 Tests: Klassen für Galaxy Tab 2, Raspberry Pi 4, starken PC und 32-Bit-PC, Abstufung bei OOM mit Hysterese, Temperatur, Akku, Regeln mit und ohne Verneinung, Chain-IDs, Prüfsummen, Fließkomma, Lizenz, Manifest-Abweichung, Korrektur, Index, Verlauf, Eskalation und Ausnahmen.

## Lizenz

MIT
