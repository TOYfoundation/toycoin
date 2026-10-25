# Projektstatus ToyChain

Stand: 25.10.2026 · Phase 1 (Testnet) · Aktueller Schritt: **M3b fertig** (Tokenomics v0.4 im Code, Entscheidungen 136 bis 153). Nächster Schritt: **M4** (Blöcke, Konsens, Startphase).

## Überblick Meilensteine

| Meilenstein | Inhalt | Status |
|---|---|---|
| Start | Rust-Workspace, leere Crates, CI, STATUS.md | ✅ fertig |
| M1 | Grundtypen und Emission, Genesis ohne Guthaben (`toy-types`, `toy-emission`) | ✅ fertig (18.10.2026) |
| M2 | Kryptografie und Adressen (`toy-crypto`) | ✅ fertig (20.10.2026), VRF und Chain-IDs entschieden (Nr. 115, 116) |
| M3 | Zustand, Transaktionen, Kontingent, Gebühren, ToyWork-Lose, Schutzregeln (`toy-state`, `toy-tx`) | ✅ fertig (20.10.2026), Entscheidungen 117 bis 121 |
| M3b | Tokenomics v0.4 im Code: K-Tabelle, Aufteilung, Belohnungs-Zähler, Stufen 1 bis 7, Kontingent 21 bis 49, Arbeits-Lotterie mit Jackpots (`toy-emission`, `toy-state`, `toy-tx`) | ✅ fertig (25.10.2026), Entscheidung 153 |
| M4 | Blöcke, Konsens, Startphase (`toy-consensus`) | ⏳ offen, nächster Schritt |
| M5 | Speicher und 256-MB-Budget (`toy-node`) | ⏳ offen |
| M6 | Netzwerk (`toy-node`) | ⏳ offen |
| M7 | JSON-RPC und `toy_`-Methoden (`toy-rpc`) | ⏳ offen |
| M8 | Wallet und CLI mit Stufenwahl (`toy-wallet`, `toy-cli`) | ⏳ offen |
| M9 | Plattformen und Energie-Modus | ⏳ offen |
| M10 | Testnet und Lasttests | ⏳ offen |
| M11 | ToyScan und andere Ketten (`toyscan/`) | ⏳ offen |
| M12 | Konto-Schutz, Zusatz-Transaktionen, atomarer Tausch | ⏳ offen |
| M13 | Namen, Einmal-Empfangsadressen, Post-Quanten-Schlüssel | ⏳ offen |

## M3b: Tokenomics v0.4 im Code (25.10.2026)

**Gebaut:**

| Baustein | Wo | Inhalt |
|---|---|---|
| K-Tabelle | `toy-emission` (`ktable`, `k_table_data.rs`, `data/k-tabelle.csv`) | 2.434 Stützstellen (je 30 Tage Planzeit, 200 Jahre) mit `k`, `w` und `W`, ganzzahlig eingesetzt. BLAKE3-Hash `cf0ea8cd…8bf032` als Verfassungs-Konstante. Ein Test vergleicht Rust-Tabelle, CSV und Hash, das Python-Skript erzeugt auch die Rust-Datei (`k_tabelle.py --rust`) |
| Emission und Plan | `toy-emission` (`schedule`, `plan`) | `floor(floor(R × k ÷ 10^6) × m ÷ 10^12)`, Plan P(t) aus der Tabelle, m-Formel unverändert. Ersetzt k₀ = 0,0505 % |
| Aufteilung je Block | `toy-emission` (`mint`) | Arbeit `w(t)` (Grundtopf 85 %, Funke 8 %, Welle 5 %, Gezeiten 2 %), Startlohn `a(d)` mit Deckel, Komitee 5 %, Voll-Knoten 8 %, Pool 2 % plus Rundung, Staking-Topf Rest. Jeder Teil höchstens so groß wie der Rest, so gibt es keinen Fehlerpfad |
| Töpfe | `toy-state` (`pots`) | 19 Töpfe in Regel I0, neu: Grundtopf, drei Jackpots, Staking-Topf, Komitee-Topf, Voll-Knoten-Topf (Gebühren-Lotterie und Konsens-Topf entfallen) |
| Belohnungs-Zähler | `toy-state` (`state`) | Skala 10^15, Gewichte je Epoche (Deckel 5 % bzw. 1 %, Sparer 0,8, unter 1 TOY nichts, Start-Validatoren in der Startphase ohne Anteil), Gutschrift bei jeder eigenen Transaktion und zu jedem Epochenbeginn, höchstens 100.000 TOY je Epoche |
| Startlohn | `toy-state` | Zeitschloss 365 Epochen, danach frei (kein Stake). Unter dem Pfand wartet der Lohn, bis er 0,1 TOY deckt |
| Stufen | `toy-types` (`params`) | 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY, Validator ab 100 TOY |
| Kontingent und Gebühren | `toy-tx` (`fees`) | `min(21 + 28 × (1 − Auslastung), λ)` mit Wasserstand, Anlauf über 7 Epochen, Vielnutzer ab der 1.000. Transaktion nur bezahlt mit doppelter Grundgebühr, Gebühren 20 / 40 / 40 (Vorschlagender, Staking-Topf, Pool), Pool-Überschuss in den Staking-Topf |
| Arbeits-Lotterie | `toy-tx` (`lottery`, `block`) | Zufallsmix je Block, bis 32 Lose je Block zeitanteilig (mindestens 8), Gewinnzahl 50 bis 100 %, Glücksstufen 1 / 2 / 3 / 5 / 8, Mindestgewinn 0,01 bzw. 0,2 TOY, Jackpots mit Chance nach Topfstand und Faktoren (Ebbe, Beteiligung), Zwang beim Vierfachen, Meilensteine, Höchstgrenze 0,5 % der Jahres-Emission, Schwierigkeit mit Ziel 16 Lose je Block |
| Gemeinsame Werte | `shared/toyverse.json` | neu erzeugt, Schema `toyverse/2` (Änderungen in `shared/README.md`) |

**Messwerte:**

| Messung | Wert |
|---|---|
| Erste Epoche bei m = 1 | 15.821,94 TOY (wie `docs/anhang/v04/kurven.py`) |
| Erstes Jahr Plan | 10.163.927 TOY |
| 100 Jahre bei hoher Nutzung | 99.999.505 TOY ausgegeben, m zwischen 1,00 und 1,25 (wie `m_simulation_v04.py`) |
| Startlohn je Start-Validator und Block am ersten Tag | rund 0,0515 TOY (Lohn plus Komitee-Topf ÷ 64), das erste Konto entsteht mit dem 2. Block |
| Fairness der Lotterie, 1 Mio. Blöcke | Wer die Hälfte aller Lose hält, bekommt 49,88 % aller Gewinne (Abweichung 0,24 %, Ziel unter 1 %). Funke 4.973-mal, Welle 181-mal, Gezeiten 13-mal |
| Aufteilen lohnt nicht | 100 Konten mit je 5 TOY bekommen bis auf Rundung (höchstens 1 Einheit je Konto und Block) genau so viel wie ein Konto mit 500 TOY |
| Speicher je Konto | ohne Stake 49 Byte, mit Nutzung in der Epoche 82 Byte, mit Stake 97 Byte (Zählerstand nur bei Gewicht) |
| Speicher der Los-Listen | höchstens rund 2,8 MB für zwei Epochen (1,1 % von 256 MB), im Zustands-Hash nur als Prüfsumme |

**Fertig-Bedingungen aus dem Bauplan:** Emission ≤ Restmenge und `k × 2 < 1` für jeden Eintrag (Test über alle Zeilen, Kani-Beweise bei größtem `k` und m = 2), 200 Jahre bei m = 2 unter 10^26, erste Epoche und erstes Jahr wie `kurven.py`, Aufteilung geht in jedem Block auf (I0), Aufteilen bringt nichts, Kontingent 49 bei 0 % Last und 21 bei voller Last, mit Wasserstand weniger, Jackpot-Töpfe in I0, Ausschüttung nie über der Höchstgrenze, Ziehung hängt vom Zufall von Block N + 2 ab (anderer Vorschlagender in Block N ändert nichts, andere VRF-Ausgabe in N + 2 schon), Erwartungswert je Los gleich (Abweichung 0,24 % über 1 Mio. Blöcke).

**Nicht in M3b:** Vorkasse-Kontingent und Gemeinschafts-Lose (später), Komitee-Topf der normalen Phase, Dienste, Verfügbarkeit, Proben, Beteiligung für die Jackpot-Faktoren und gleitende Übergabe (M4), Grenze von 8 Losen je Peer (Netzwerk, M6). Bis dahin: Voll-Knoten-Topf geht in den Staking-Topf, Beteiligung zählt als 100 %, alle Start-Validatoren bekommen gleich viel.

**Ehrlich zu Kani:** Die Kani-Beweise laufen nur in der CI, die Sandbox kann die Nightly-Toolchain von Kani nicht laden. Die Aufteilung je Block rechnet jetzt mehr Anteile als vorher. Damit der Beweis schnell bleibt, nimmt sie jeden Teil höchstens bis zum Rest (es gibt keinen Fehlerpfad mehr). Ob der Beweis `block_mint_adds_up_bootstrap_no_share` weiter unter 15 Minuten bleibt, zeigt erst die CI.

**Ergebnisse:** `cargo fmt --check` ✅, `cargo clippy --all-targets -D warnings` ✅, `cargo test` ✅ **246 Tests** (207 vorher), `cargo deny check` ✅ (advisories, bans, licenses, sources ok), `cargo vet --locked` ✅ (21 geprüft, 65 Ausnahmen). Keine neue fremde Bibliothek. Lokal mit Rust 1.97.0 (stable).

## Tokenomics v0.4 vom 25.10.2026 (kein Meilenstein, nur Dokumente)

**Anlass:** Prüfbericht 3 (`12-pruefbericht-tokenomics.md`) fand drei kritische Punkte: Die Übergabe aus der Startphase war rechnerisch unmöglich, der Grundbetrag je Kopf lud zum Aufteilen ein, und Stufe 3 bekam keinen Lohn für mehr Aufwand. Die Varianten-Rechnung (`13-tokenomics-v04-entwurf.md`, Skripte in `docs/anhang/v04/`) hat 15 Kurven, 6 Markt-Szenarien, 4 Übergabe-Regeln, 5 Größen der Start-Gruppe, 2 Stufensysteme, Kontingente und 7 Lotterie-Varianten mit Jackpots verglichen. Der Projektinhaber hat alles Empfohlene übernommen.

**Geändert:** Spezifikation v0.4 (Abschnitte 1, 2, 4, 5.1, 5.1a, 5.3, 6.2, 7, 9, 10, 12, 13, 14, 16), Entscheidungen 136 bis 152 (ältere Einträge mit Verweis markiert), Bauplan (neuer Meilenstein M3b, M4 angepasst), `CLAUDE.md` (feste Vorgaben), Wallet- und ToyScan-Konzept.

**Kernpunkte:** Gezeiten-Kurve mit K-Tabelle (Verfassung Punkt 3 geändert, vor dem Mainnet), Staker ab Tag 1, 64 Start-Validatoren mit gesperrtem Lohn und gleitender Übergabe, Stufen 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY, Komitee mit Stimmgewicht nach Stake, Kontingent 21 bis 49 mit Wasserstand, Arbeits-Lotterie mit zufälliger Gewinnzahl, Glücksstufen und den Jackpots Funke, Welle und Gezeiten.

**Nicht geändert:** kein Code, keine Konstanten (das kam mit M3b, siehe oben).

**Offen:** rechtliche Prüfung der Jackpots, neue Begriffe in `i18n/begriffe.tsv` (Jackpot, Funke, Welle, Gezeiten, Sparer, Helfer, Beweis-Knoten, Ersatz-Validator, Vorkasse-Kontingent) mit Übersetzungen in 45 Sprachen.

## Verknüpfung im ToyVerse vom 24.10.2026 (kein Meilenstein)

**Anlass:** Alle Projekte des ToyVerse sollen logisch und technisch verknüpft sein und ihren Stand automatisch abgleichen. Neu ist ToySafe, die Schutz- und Prüf-App.

**Gebaut:**

| Was | Wo |
|---|---|
| Gemeinsame Datei mit allen festen Werten (Chain-IDs, Dezimalstellen, Obergrenze, Emission, Gebühren, Kontingent, Typ-Bytes, alte Token, Sprachen, Grundregeln) | `shared/toyverse.json` |
| Test, der die Datei aus den Rust-Konstanten erzeugt und Abweichungen meldet (2 Tests) | `toychain/tests/tests/toyverse_manifest.rs` |
| 4 neue Begriffe (ToySafe, Schutzstufe, Warnliste, Prüf-Freigabe) | `i18n/begriffe.tsv` |
| Konzept ToySafe, Bestandsaufnahme mit Befunden K1 bis K8, Vorschläge V1 bis V11 | `docs/spaeter/14-toysafe.md`, `15-verknuepfung-und-kompatibilitaet.md`, `11-toyverse.md` Abschnitt 9 |
| Entscheidungen 134 (ToySafe) und 135 (Verknüpfung, Vorschläge) | `docs/02-entscheidungen.md` |

**Nicht geändert:** kein Konsens- oder Protokoll-Code, keine neue Transaktionsart, keine neue Bibliothek. M4 bleibt der nächste Meilenstein. Das Godot-Skript `nft_items.gd` rechnet noch mit den alten Präge-Regeln (Befund K4) und wird erst nach Entscheidung 127 angepasst.

**Sicherheits-Update RUSTSEC-2026-0387:** Bei der Prüfung meldete `cargo-deny` eine neue Lücke in `crypto-bigint` 0.7.5 (Rechenzeit nicht immer konstant, über `k256` auch in den Kontoschlüsseln). Angehoben auf **0.7.7** in `Cargo.lock` und `fuzz/Cargo.lock`, ohne Code-Änderung, weiter mit Rust 1.87 als Mindestversion. `cargo vet`: Ausnahme von 0.7.5 auf 0.7.7 umgestellt, ehrlich als „noch nicht durchgesehen“.

**Ergebnisse:** `cargo fmt --check` ✅, `cargo clippy --all-targets -D warnings` ✅, `cargo test` ✅ **207 Tests** (205 bisher, 2 neu), `cargo deny check` ✅, `cargo vet --locked` ✅ (21 geprüft, 65 Ausnahmen). Lokal mit Rust 1.97.0 (stable).

## Sicherheits-Update vom 24.10.2026 (RUSTSEC-2026-0385)

**Anlass:** Nach dem Zusammenführen von M3 schlug `cargo-deny` in der CI fehl. Grund war die Sicherheitsmeldung [RUSTSEC-2026-0385](https://rustsec.org/advisories/RUSTSEC-2026-0385): Die Hilfsbibliothek `ctutils` (RustCrypto) erzeugte Masken und verglich `u128`-Werte vor Version 0.4.4 **nicht in konstanter Zeit**. Solche Zeitunterschiede können bei Kryptografie geheime Werte verraten. `ctutils` kommt nicht direkt ins Projekt, sondern über `k256` → `elliptic-curve` → `crypto-bigint` sowie `digest` und `sec1`, also auch in den Kontoschlüsseln.

**Geändert:**

| Was | Vorher | Nachher |
|---|---|---|
| `ctutils` im Workspace (`Cargo.lock`) | 0.4.2 | **0.4.5** |
| `ctutils` im Fuzzing (`fuzz/Cargo.lock`) | 0.4.3 | **0.4.5** |
| Mindestversion von Rust (`rust-version`) | 1.85 | **1.87**, weil `ctutils` ab 0.4.3 Rust 1.87 verlangt. Gebaut wird weiter mit 1.97.0 |
| Code | `i % 2 == 0` | `i.is_multiple_of(2)` an 3 Stellen (1 in `toy-crypto/src/address.rs`, 2 in Tests). Clippy verlangt das ab Mindestversion 1.87, das Verhalten ist gleich |
| `cargo vet` | Ausnahme für 0.4.2 | Ausnahme für **0.4.5**, ehrlich als „noch nicht durchgesehen“. Für 0.4.5 gab es am 24.10.2026 keinen Prüfbericht einer der importierten Organisationen. Unterschied zu 0.4.2: 12 Dateien, rund 700 Zeilen (`cargo vet diff ctutils 0.4.2 0.4.5`) |

**Ergebnisse:** `cargo fmt --check` ✅, `cargo clippy --all-targets -D warnings` ✅, `cargo test` ✅ **205 Tests** wie nach M3, `cargo deny check` ✅ (advisories, bans, licenses, sources ok), `cargo vet --locked` ✅ (21 geprüft, 65 Ausnahmen). Lokal mit Rust 1.97.0 (stable), weil die Sandbox die festgenagelte Toolchain nicht laden konnte, die Versionen sind gleich.

## M3: Zustand, Transaktionen, Kontingent, Gebühren, Schutzregeln (20.10.2026)

**Gebaut:**

| Baustein | Wo | Inhalt |
|---|---|---|
| Konten und Töpfe | `toy-state` (`state`, `account`, `pots`) | Freies Guthaben, Aktivierungs-Pfand 0,1 TOY (senkbar bis 0,01), Stake, Lösen in 21 Epochen, gebundener Lohn der Start-Validatoren (365 Epochen), festgehaltener Stake, Validator-Schlüssel, Zähler je Epoche. **Eine einzige Topf-Liste** (15 Töpfe, davon 5 für spätere Meilensteine, bis dahin 0) |
| Regel I0 | `State::check_i0` | Summe aller Töpfe = Summe aller Emissionen, nach jedem Block gegen die unabhängig gerechnete Summe aus `toy-emission` |
| Geld nur bewegen, nie erzeugen | `State` | Neues TOY entsteht nur in `mint_block`. Alle anderen Methoden buchen genau so viel ab, wie sie gutschreiben, oder ändern nichts (erst prüfen, dann ändern). Die Felder der Konten sind von außen nicht schreibbar |
| Auflösen, Aktivieren | `State` | Auflösen mit oder ohne Spende, nur ohne Stake, Lösen, Lohn-Bindung, festgehaltenen Stake oder Validator-Rolle. Sperr-Verlauf bleibt 64 Epochen. Jedes neue Konto beginnt bei Nonce `Höhe × 2^16` |
| Merkle-Wurzel | `merkle` | BLAKE3, getrennte Bereiche für Blätter und Knoten, Anzahl in der Wurzel, kein Verdoppeln einzelner Blätter |
| Sperrzeiten (Ebene A) | `penalty` | 5 Verstöße, 4 Rechte, Verdopplung bis 64 Epochen, Neubeginn nach 30 sauberen Epochen. Keine Methode der Sperren kennt Beträge |
| Transaktionsformate | `toy-tx` (`transaction`, `ethereum`, `rlp`) | ToyChain-Format mit Typ-Byte je Art (0x70 bis 0x77), Ablaufzeit, Verwendungszweck. Ethereum Legacy (EIP-155) und Typ 2 (EIP-1559), nur reine Überweisungen. Strenger RLP-Decoder: jede Eingabe hat genau eine gültige Schreibweise |
| Arten in M3 | `transaction::Action` | Überweisung, Stake, Unstake, Validator an- und abmelden (mit BLS-Besitznachweis), Spende, Konto auflösen, Doppelsignatur melden |
| Gebühren und Kontingent | `fees` | Progressive Gebühr mit Höchstfaktor 1.024, Grundgebühr nach EIP-1559, Trinkgeld, Aufteilung 40/20/40, Kontingent nach Auslastung, Erstattungs-Budget 1/30, Pool-Obergrenze |
| ToyWork | `toy-crypto::work`, `ticket`, `lottery` | Argon2id 16 MiB, Lose an Adresse, Epoche, Saat und Chain-ID gebunden und signiert, Ziehung 2 Blöcke später, höchstens 64 Gewinner, Schwierigkeit je Epoche |
| Doppelsignatur-Nachweis | `evidence` | Stimmformat für M4, Prüfung „gleiche Höhe, Runde und Phase, anderer Block“ |
| Block ausführen | `block::execute_block` | Alles oder nichts, I0 am Ende |

**Neue fremde Bibliothek:** `argon2` 0.6.0 (RustCrypto, MIT/Apache-2.0) mit `blake2` 0.11.0. Nur Argon2id, ohne Passwort-Format und ohne Threads.

**Ergebnisse der Prüfungen:**

- `cargo fmt --check`: ✅ ohne Abweichungen
- `cargo clippy --all-targets -D warnings`: ✅ ohne Warnungen
- `cargo test`: ✅ **205 Tests** (91 aus M1 und M2, **114 neu**: 4 in `toy-crypto`, 37 in `toy-state`, 73 in `toy-tx`), alle bestanden, rund 45 Sekunden
- `cargo deny check`: ✅ Sicherheitsmeldungen, Lizenzen, Herkunft in Ordnung. 5 Warnungen wegen doppelter Hilfs-Bibliotheken (`digest`, `crypto-common`, `block-buffer`, `cpufeatures`, `sha2`, je eine alte und eine neue Version). Unkritisch
- `cargo vet --locked`: ✅ 21 geprüft, **65 Ausnahmen** (2 neu: `argon2`, `blake2`). Keine der fünf vertrauten Organisationen hat sie bisher geprüft. Ehrlich als „noch nicht durchgesehen“ eingetragen
- Fuzzing: 3 neue Ziele (`rlp_decode` mit der Eigenschaft „angenommen heißt kanonisch“, `decode_transaction`, `decode_ticket`). Lokal nicht ausgeführt (kein Nightly in der Sandbox), dafür prüfen Tests 20.000 zufällig veränderte RLP-Eingaben und jede Kürzung und Bit-Änderung einer Transaktion. In der CI liefen alle 9 Ziele je 5 Minuten ohne Fund
- CI auf GitHub (Pull Request #5, 20.10.2026): ✅ alle 34 Prüfläufe bestanden (17 Prüfungen, je für Push und Pull Request), darunter Tests auf Linux x86_64, Windows x86_64 und Linux aarch64, `cargo-deny`, `cargo-vet`, die 3 Kani-Beweise aus M1 und das Fuzzing

| Pflicht-Test aus dem Bauplan | Ergebnis |
|---|---|
| I0 nach jedem Block | ✅ Zufallstest über 2 Epochen (rund 2.880 Blöcke) mit Überweisungen, neuen Konten, kostenlosen Transaktionen, Spenden, Stake, Auflösen, gültigen und ungültigen Losen: I0 hält nach jedem Block, der Test prüft, dass jeder Weg vorkam |
| Absichtlich fehlerhafter Block wird abgelehnt | ✅ Block, der 1 kleinste Einheit mehr oder weniger mintet als die Emission, wird von I0 abgelehnt. Nicht aufgehende Teile und falsche Summe ebenso. Ein abgelehnter Block ändert nichts. Dazu ein Test, der eine Manipulation des Zustands an I0 scheitern lässt |
| Doppelausgabe | ✅ gleiche Nonce zweimal im Block, Wiederholung im späteren Block, zwei Überweisungen, die einzeln passen, zusammen nicht |
| Überlauf | ✅ Betrag `u128::MAX`, Gebühren-Obergrenze und Trinkgeld `u128::MAX`, Überlauf eines Topfs: abgelehnt, Zustand unverändert |
| Kontingent bei verschiedener Auslastung | ✅ 0 %, 25 %, 50 %, 90 %, 100 % ergeben 100, 75, 50, 10, 10. Genau Q kostenlose Transaktionen, die nächste abgelehnt, neue Epoche neues Kontingent. Kontoalter unter 100 Blöcken abgelehnt, ab 100 erlaubt. Budget 1/30 des Pools. Kostenlose höchstens 50 % des Blocks und nur hinter bezahlten |
| Gebühren | ✅ 25 Transaktionen eines Absenders kosten Faktor 1, 2 und 4, Aufteilung 40/20/40 auf die kleinste Einheit genau, Rundungsrest in den Pool. Zu kleine Obergrenze abgelehnt, Trinkgeld berechnet. Grundgebühr ±1/8. Erstattete Gebühren werden nicht weiterverteilt. Pool-Überschuss geht genau in die Arbeits-Lotterie |
| Vielnutzer | ✅ Die 1.001. Transaktion ohne Los wird abgelehnt, mit Los angenommen. Danach gilt: mindestens die Hälfte der Überschreitung gedeckt |
| Ungültige und wiederverwendete Lose | ✅ Ungültige Arbeit sperrt die Lose des Eigentümers für den Rest der Epoche und eine volle Epoche. Wiederverwendung, fremde Epoche, fremde Kette: Block ungültig, keine Strafe. Höchstens 16 Lose je Block, angehängte mitgezählt. Ein Los macht eine Transaktion gebührenfrei und kann danach nicht mehr verwendet werden |
| Arbeits-Lotterie | ✅ Ein Gerät ohne Konto gewinnt sein erstes TOY 2 Blöcke später (Konto mit Pfand entsteht), Mindestgewinn 0,2 TOY, höchstens 64 Gewinner, Topf wandert weiter, Gewinnchance proportional zu den Losen (2.000 Ziehungen), Schwierigkeit ×4 nach einer Epoche ohne Lose |
| Jede Sperrregel „greift“ und „greift nicht“ | ✅ Doppelsignatur (greift: widersprüchliche Stimmen; greift nicht: neue Runde, andere Phase, gleiche Stimme, andere Höhe, fremder Schlüssel, Adresse ohne Validator; keine zweite Strafe). Ungültiger Block (Regel, Nachweis folgt mit M4). Ungültige Lose. Massenhaftes Anlegen (20 neue Konten nicht, das 21. ja, Überweisungen an bestehende zählen nicht, nur einmal je Epoche, neue Epoche zählt neu). Inaktivität (2 Epochen nicht, 3 ja, Ende durch neues Anmelden). Steigerung 1 → 2 Epochen und Neubeginn nach 30 sauberen Epochen |
| Gesperrtes Konto kann weiter gegen Gebühr überweisen | ✅ für alle 5 Verstöße, freies Guthaben und Pfand unverändert |
| Spenden | ✅ Betrag landet genau im Pool, I0 hält, kein Weg führt zurück zum Spender |
| Aktivierung | ✅ Eingang unter dem Pfand an neue Adresse abgelehnt, genau das Pfand angenommen. Senken des Pfands gibt den Unterschied mit der nächsten eigenen Transaktion frei |
| Auflösen | ✅ mit Teil-Spende, mit voller Spende ohne Ziel, blockiert durch Stake und Lösen |
| Alte Transaktion nach erneuter Aktivierung | ✅ bleibt ungültig (neue Nonce-Basis) |
| Offizielle Vektoren | ✅ 28 + 26 RLP-Vektoren, 210 Ethereum-Transaktionen (16 mit identischem Absender und Hash, alle 161 ungültigen abgelehnt), Argon2id-Vektor aus RFC 9106. Quellen: `toychain/crates/toy-tx/tests/vectors/README.md` |

**Messwerte** (Release-Build, ein Kern eines Intel Xeon mit 2,8 GHz):

| Größe | Wert |
|---|---|
| Überweisung im ToyChain-Format | 123 Byte (mit 32-Byte-Verwendungszweck 155, kostenlos 115) |
| Ethereum Legacy / EIP-1559 | 112 / 120 Byte |
| Eigenständiges ToyWork-Los | 79 Byte |
| Gebühr einer Überweisung bei Grundgebühr | rund 0,000123 TOY |
| Transaktion einlesen und Absender prüfen | 0,09 bis 0,14 ms |
| ToyWork-Los prüfen (Argon2id, 16 MiB) | rund 10 ms, also höchstens 160 ms je Block |
| Konto im Zustand (Kodierung) | 49 Byte, im laufenden Epochen-Zähler 90 Byte |
| Zustandswurzel über 10.100 Konten | 5,4 ms |
| Kopie des Zustands mit 10.100 Konten | 4,0 ms |
| Voller Block (1.000 Überweisungen, 113 KB) bei 10.100 Konten | **97 ms** (vor dem Umbau 5,8 s, siehe unten) |
| Pool-Obergrenze beim Start | 2.831,1552 TOY, Erstattungs-Budget dann 94,4 TOY je Epoche |
| Eine Epoche (1.440 Blöcke) im Test-Netz, Debug-Build | rund 0,2 s |

**Umbau während M3:** Die erste Fassung kopierte für jede einzelne Geldbewegung den ganzen Zustand, damit ein Fehler nichts verändert. Bei 10.000 Konten dauerte ein voller Block so 5,8 Sekunden. Jetzt prüft jede Methode zuerst alles und ändert erst dann. Ein Test belegt für 19 fehlschlagende Operationen, dass der Zustand danach exakt gleich ist.

**Entschieden am 20.10.2026** (Entscheidungen 117 bis 121, Spezifikation 7.2, 8, 9, 14 und 16 angepasst):

1. **Progressive Gebühr:** Faktor höchstens 1.024, Grundgebühr ab 0,000001 TOY je Byte (Nr. 117).
2. **Auslastung:** Bytes aller Transaktionen der Vorepoche, auch kostenloser (Nr. 118).
3. **ToyWork-Lose:** 16 je Block, 4 je Peer, 1 Los = 1 gebührenfreie Transaktion (Nr. 119).
4. **Erstattungs-Pool (P9):** Überschuss über 30 Epochen größtmöglicher Erstattung geht in die Arbeits-Lotterie (Nr. 120). Ohne das wäre der Pool ein Topf, aus dem nie etwas abfließt.
5. **Festlegungen** (Nr. 121): Kontingent nur auf Anforderung, Sperrdauer, Vielnutzer-Prüfung, Inaktivität als Pause, Strafe für Lose nur bei eigener Signatur, Nonce-Basis für jedes neue Konto, Lohn zu klein für das Pfand, festgehaltener Stake, Ethereum-Gebühren.

**Berichtigt:** Im Bauplan stand für M3 „Stake mit 7 Epochen Lösefrist“. Die Spezifikation sagt 21 Epochen (Entscheidung 106). Gebaut ist 21, der Bauplan ist korrigiert.

**Einschränkungen (ehrlich):**

- **Speicher je Block:** Der Block-Ausführer kopiert den Zustand einmal je Block (alles oder nichts) und berechnet die Wurzel vollständig neu. Bei 10.000 Konten zusammen rund 9 ms, hochgerechnet bei 1 Mio. Konten rund 1 Sekunde je Block, bei 2 Mio. aktiven Konten rund 2 Sekunden. Bei 60 s Blockzeit geht das noch, für alte Android-Geräte nicht. **M5** braucht deshalb einen dauerhaften Speicher mit Änderungs-Journal und schrittweise aktualisiertem Merkle-Baum.
- **Kürzung nach Doppelsignatur** kommt mit M4. Bis dahin wird der Stake 18 Epochen festgehalten und danach **ungekürzt** zurückgegeben. Die Rollensperre wirkt schon.
- **Nachweise für „ungültiger Block“ und „Inaktivität“** brauchen das Blockformat und die Signaturzählung aus M4. Die Sperrregeln selbst (Dauer, Rechte, Steigerung) sind gebaut und getestet, für Inaktivität auch die Schwelle (3 Epochen).
- **Feste Reihenfolge nach Gebühr je Byte** und die Aufnahme-Pflicht (Spezifikation 5.1a) gehören zu M4. M3 prüft nur „kostenlose hinter bezahlten“.
- **Lose prüfen kostet Zeit:** 16 Lose sind 160 ms je Block auf einem PC, auf altem Android geschätzt 1,6 bis 5 Sekunden. Stufe-0-Geräte müssen nicht jedes Los selbst prüfen. M9 und M10 messen das auf echten Geräten. Ob 16 MiB Grafikkarten genug bremsen, ist weiter offen (Spezifikation 12).
- **MetaMask-Nutzer zahlen**, außer sie stellen den Gaspreis auf 0. Und das Ethereum-Format kann keine Lose tragen: Wer über 1.000 Transaktionen je Epoche kommt, braucht das ToyChain-Format.
- **Neue Konten beginnen bei einer großen Nonce** (`Höhe × 2^16`). Für Wallets ist das egal, sie fragen die Nonce ab. Es fällt nur in ToyScan auf.
- **Liste gemeldeter Doppelsignaturen** wächst (32 Byte je Fall). Doppelsignaturen sind selten, M4 oder M5 räumt alte Einträge auf.
- **Pool-Obergrenze** hängt an der Grundgebühr. Steigt die Grundgebühr, steigt die Obergrenze mit. Das ist gewollt, weil dann auch die Erstattungen teurer werden.
- **Keine neuen Kani-Beweise** in M3. I0 wird in jedem Block zur Laufzeit geprüft, dazu der Zufallstest. Ein formaler Beweis der Topf-Bewegungen ist ein Kandidat für später.
- **Fuzzing** der 3 neuen Ziele lief nur 5 Minuten je Ziel in der CI. Lange Läufe gehören vor das öffentliche Testnetz (M10).

## M2: Kryptografie und Adressen (20.10.2026)

**Gebaut** in `toy-crypto`. Es sind nur dünne Hüllen um geprüfte Bibliotheken, kein eigenes Verfahren. Der übrige Code sieht keine Typen der Bibliotheken (Krypto-Agilität).

| Baustein | Modul | Bibliothek (Version festgenagelt) |
|---|---|---|
| Keccak-256, BLAKE3 (mit Bereichstrennung) | `hash` | `sha3` 0.12.0, `blake3` 1.8.7 |
| Kontoschlüssel secp256k1, Signatur nur in „low S“-Form (EIP-2), deterministisch (RFC 6979) | `secp256k1` | `k256` 0.14.0 (RustCrypto, von NCC Group geprüft) |
| Adresse = letzte 20 Byte von Keccak-256, Schreibweise mit Prüfsumme (EIP-55) | `secp256k1`, `address` | – |
| Chain-ID in jeder Signatur (EIP-155) | `eip155` | `k256` |
| Phrase mit 12 oder 24 Wörtern (BIP-39), Pfad m/44'/60'/0'/0/n (BIP-32) | `hd` | `bip39` 3.0.0 (rust-bitcoin), `bip32` 0.6.0 (iqlusion) |
| BLS12-381 wie Ethereum: Signatur 96 Byte, Besitznachweis gegen untergeschobene Schlüssel, Sammelsignatur | `bls` | `blst` 0.3.17 (Supranational, extern geprüft), Modus `portable` |
| VRF hinter austauschbarer Schnittstelle `Vrf` | `vrf` | `blst`, `blake3` |
| Sichere Zufallszahlen, Löschen von Geheimnissen | intern | `getrandom` 0.4.3, `zeroize` 1.9.1 |

**Ergebnisse der Prüfungen:**

- `cargo fmt --check`: ✅ ohne Abweichungen
- `cargo clippy --all-targets -D warnings`: ✅ ohne Warnungen
- `cargo test`: ✅ **91 Tests** (45 aus M1, **46 neu**), alle bestanden
- `cargo deny check`: ✅ Sicherheitsmeldungen (RustSec-Datenbank, Stand 17.10.2026), Lizenzen, Herkunft, Sperrliste in Ordnung. 5 Warnungen wegen doppelter Hilfs-Bibliotheken (`sha2`, `digest` u. a. in zwei Versionen, weil `bip32` für das `xprv`-Format noch `bs58` mit älterem `sha2` nutzt). Unkritisch.
- `cargo vet`: ✅ 21 Bibliotheken geprüft (Prüfberichte von Mozilla, Google, Bytecode Alliance, Zcash, ISRG übernommen, dazu Vertrauen in 4 Herausgeber, denen diese Organisationen übereinstimmend vertrauen), **63 als Ausnahme** („noch nicht durchgesehen“) in `toychain/supply-chain/config.toml`. Eigene Prüfberichte wurden nicht eingetragen.
- Fuzzing: ✅ alle 6 Ziele (Adressen, Phrasen, Pfade, EIP-155-Signaturen, BLS, VRF) liefen in der CI je 5 Minuten ohne Fund (Pull Request #2, 20.10.2026, zwei Läufe). Lokal nicht ausgeführt, weil die Sandbox kein Nightly laden kann.
- CI auf GitHub (Pull Request #2): ✅ alle 14 Prüfungen bestanden, darunter Tests auf Linux x86_64, Windows x86_64 und Linux aarch64, `cargo-deny`, `cargo-vet` und die 3 Kani-Beweise aus M1.

| Pflicht-Test aus dem Bauplan | Ergebnis |
|---|---|
| Offizielle BIP-39-Vektoren (Trezor-Referenz) | ✅ alle 16 Fälle mit 12/24 Wörtern exakt: Entropie → Phrase → Seed → Hauptschlüssel. Die 8 Fälle mit 18 Wörtern werden wie von der Spezifikation verlangt klar abgelehnt |
| Offizielle BIP-32-Vektoren | ✅ 17 Ableitungen aus Vektor 1 bis 4 exakt, auch „führende Nullen“. Erwartete Schlüssel mit eigenem Base58-Decoder gelesen, unabhängig von `bip32` |
| Offizielle Ethereum-Vektoren ergeben exakt die erwarteten Adressen | ✅ ethereum/tests Schlüssel → Adresse, alle 8 EIP-55-Fälle **exakt nachgebildet** (auch „All caps“ und „All Lower“), EIP-155-Beispiel **bitgenau** (Signierdaten, Hash, v = 37, r, s, Ende der signierten Transaktion) |
| Phrase → Adresse Ende zu Ende | ✅ 4 Fälle aus alloy/MyCrypto (mit und ohne Passphrase, Index 0 bis 3) und 2 aus MetaMasks Testsuite |
| Signaturen anderer Chain-IDs werden abgelehnt (Replay-Test) | ✅ Ethereum-Signatur auf ToyChain abgelehnt und umgekehrt. Umgeschriebenes `v` hilft dem Angreifer nicht (falscher Absender). Signatur ohne Chain-ID (v = 27/28) abgelehnt. Zwilling mit hohem S abgelehnt. 200 Zufallsfälle |
| BLS12-381 | ✅ 85 offizielle Ethereum-Vektoren (Signieren, Prüfen, Sammeln, Sammel-Prüfung, Einlesen), Besitznachweis, Abstimmung mit 21 Validatoren |
| VRF | ✅ Prüfen, Ablehnen fremder Schlüssel und Eingaben, Stimmen und VRF-Beweise nicht vertauschbar, Gleichverteilung grob geprüft, Regressionswert |
| Zufallstests für alle Funktionen mit fremden Daten | ✅ Adressen, Phrasen, Pfade, Signaturen, BLS-Schlüssel und -Signaturen, VRF-Beweise. Kein Absturz, Unsinn wird abgelehnt |
| Geheimnisse | ✅ `Debug` zeigt nie Schlüssel, Phrase oder Seed. Fehlermeldungen nennen bei falschem Wort nur die Position |

Quellen, Stände und Prüfsummen aller Vektoren: `toychain/crates/toy-crypto/tests/vectors/README.md`.

**Nachprüfung vor dem Commit (20.10.2026, zweite Sitzung):**

- **Herkunft der Vektoren:** Alle 6 Quelldateien frisch von GitHub geladen, **byteweise identisch**. Die BLS-JSON-Datei aus dem frischen Archiv neu erzeugt, identisch. Die Werte aus alloy und MetaMask stehen wörtlich in deren Quellen. Eine falsche Commit-Angabe für MetaMask in der README ist berichtigt.
- **Gegenprobe (Mutationstest):** Die Prüfung der Chain-ID wurde testweise abgeschaltet. 3 Replay-Tests schlugen sofort fehl. Der Test „umgeschriebenes `v`“ blieb grün, weil dann die zweite Schutzschicht greift (die Chain-ID steckt auch im signierten Hash).
- **Verbessert:** Eine Adresse, die genau ihrer EIP-55-Schreibweise entspricht, gilt jetzt als geprüft, auch wenn sie zufällig nur Groß- oder nur Kleinbuchstaben hat. Vorher galt sie als „ohne Prüfsumme“. Das hätte in der Wallet eine unnötige Warnung ausgelöst.
- **Verbessert:** Der BIP-32-Test liest die erwarteten Schlüssel jetzt unabhängig von der geprüften Bibliothek.
- **Berichtigt:** Es sind 85 BLS-Vektoren, nicht 72.

**Messwerte** (Release-Build, ein Kern eines Intel Xeon mit 2,1 GHz):

| Vorgang | Zeit |
|---|---|
| Phrase → Seed (BIP-39, 2.048 Runden PBKDF2) | 1,5 ms |
| Seed → Kontoschlüssel m/44'/60'/0'/0/0 | 0,38 ms |
| secp256k1 signieren / Absender wiederherstellen | 0,06 ms / 0,07 ms |
| BLS signieren / prüfen / Besitznachweis prüfen | 0,44 ms / 1,06 ms / 1,08 ms |
| BLS 21 Stimmen sammeln / Sammelsignatur prüfen | 1,4 ms / 1,2 ms |
| BLS 128 Stimmen sammeln / Sammelsignatur prüfen | 9,0 ms / 1,3 ms |
| VRF beweisen / prüfen | 0,45 ms / 1,10 ms |

Ein voller Block mit 128 Stimmen wird also in rund 1,3 ms geprüft. Bei 60 s Blockzeit ist das unkritisch. Auf altem Android (M9/M10 messen) ist mit dem 10- bis 30-Fachen zu rechnen.

**Entschieden am 20.10.2026:**

1. **VRF-Verfahren (Entscheidung 115).** Die Spezifikation nannte RFC 9381. Dafür gibt es in Rust keine geprüfte Bibliothek. Die Kandidaten waren am 20.10.2026:
   - `vrf-rfc9381`: Version 0.0.7, rund 2.400 Downloads
   - `ecvrf`: Lizenz GPL-3.0, nicht MIT-verträglich
   - `ecvrf-rs`: seit 2022 verwaist
   - `vrf`: braucht OpenSSL und setzt einen Entwurfsstand um
   - `vrf-r255`: setzt einen Entwurf auf ristretto255 um, nicht RFC 9381
   - `ark-vrf` 0.5.3: weit verbreitet (rund 4,3 Mio. Downloads, MIT), aber nach eigener Aussage nur „lose an RFC 9381 angelehnt“, kein Prüfbericht genannt
   - `vrf_fun` 0.12.1: RFC-9381-Verfahren auf secp256k1. Diese Kurve ist in RFC 9381 nicht vorgesehen, rund 5.900 Downloads, kein Prüfbericht genannt

   **Gebaut und für das Testnet entschieden ist deshalb eine VRF aus eindeutigen BLS-Signaturen:**
   - Der Beweis ist eine BLS-Signatur mit eigenem Bereichs-Tag.
   - Das Ergebnis ist ein BLAKE3-Hash davon.
   - Es wird nur die geprüfte `blst`-Bibliothek benutzt.
   - Das gleiche Prinzip nutzen Ethereum (RANDAO) und das Internet Computer.

   Es steckt hinter der Schnittstelle `Vrf`, ein späterer Wechsel betrifft nur `toy-crypto`. **Wiedervorlage vor dem Mainnet-Audit (Entscheidung 38):** Gibt es dann eine geprüfte RFC-9381-Bibliothek, wird neu entschieden. Spezifikation 10 ist angepasst.
2. **Chain-IDs (Entscheidung 116, setzt Nr. 40 um):** Mainnet **7630713** (`0x746F79` = „toy“), Testnet **7630714**, lokales Devnet **7630715**. Am 20.10.2026 in ethereum-lists/chains frei. Im Code als `toy_types::params::CHAIN_ID_*` und `ChainId::TOYCHAIN_*`, per Test geprüft (verschieden, unter 2^32, nicht 1, 137, 1337, 31337). Ein ungültiger Wert hält schon das Bauen von `toy-crypto` an. Eine reine Prüfung beim Bauen in `toy-types` (`const _: () = assert!(…)`) hat Kani in der CI nicht verarbeitet, deshalb ist sie dort ein Test. Neuer Test: Mainnet, Testnet und Devnet lehnen Signaturen der jeweils anderen ab. **Offen:** Eintrag in die öffentliche Liste mit dem öffentlichen Testnet (M10), bis dahin vor jedem Meilenstein prüfen, ob die Nummern noch frei sind.

**Einschränkungen (ehrlich):**

- **Löschen von Geheimnissen ist „bestmöglich“.** Unsere eigenen Typen löschen beim Freigeben. In den Bibliotheken bleiben Reste:
  - `bip39` gibt den Seed als Kopie zurück.
  - `bip32` löscht den Kettencode der Zwischenschlüssel nicht.
  - Rust kann Werte beim Verschieben im Speicher kopieren.

  Für das Testnet ist das vertretbar. Für die Wallet (M8) kommt der Schutz durch das Gerät dazu (Keystore, StrongBox).
- **Nur englische Wortliste, nur Kleinschreibung.** BIP-39-Wortlisten gibt es nur in rund 10 Sprachen. Für den Austausch mit anderen Wallets ist Englisch der Standard. Groß-/Kleinschreibung und Tippfehler-Hilfe gehören in die Wallet (M8).
- **Eigener Ableitungspfad für ToyChain** (Spezifikation 10, optional) kommt mit M8. Er braucht eine eigene Münznummer nach SLIP-44. `DerivationPath::parse` kann schon jeden Pfad.
- **Argon2id** (ToyWork-Lose, Keystore) kommt mit M3 bzw. M8 in `toy-crypto` dazu.
- **Fuzzing** läuft je Ziel nur 5 Minuten in der CI. Das findet grobe Fehler, ersetzt aber keine langen Läufe (Stunden bis Tage). Lange Läufe gehören vor das öffentliche Testnet (M10).
- **`cargo vet`**: 63 Ausnahmen sind ehrlich als „noch nicht durchgesehen“ eingetragen. Sie sollten mit der Zeit durch echte Prüfungen (`cargo vet certify` durch einen Menschen) oder neue Berichte anderer Organisationen ersetzt werden.
- **`blst` im Modus `portable`:** keine CPU-Sonderbefehle, die vom Build-Rechner abhängen. Das hält Builds reproduzierbar und lauffähig auf älteren PCs ohne ADX. Messbar langsamer war es nicht.

## M1: Grundtypen und Emission (18.10.2026)

**Gebaut** (ohne fremde Bibliotheken):

- `toy-types`: `Amount` (u128 in kleinsten Einheiten, nur geprüfte Rechnung, Abrunden), `Ppm` (Millionstel), alle Parameter aus Spezifikation 14 in `params.rs` (Obergrenze 10^26 als Verfassungswert), `Address` und `Hash` als Grundgerüst für M2.
- `toy-emission`:
  - Plan-Kurve P(t) und Faktor m genau nach 9.1.
  - Emission je Epoche, Verteilung auf die ersten 1.440 Blöcke, Aufteilung 50/40/10 mit Rundungsrest in den Erstattungs-Pool.
  - Lohn der 21 Start-Validatoren nach Planzeit-Tagen mit Deckel.
  - Planzeit (Blockabstand höchstens 600 s zählt) und Epochen-Ende (1.440 Blöcke **und** 24 Stunden).
  - `EmissionEngine` verarbeitet Block für Block. Jeder Fehler lässt den Zustand unverändert.

**Ergebnisse der Prüfungen:**

- `cargo fmt --check`: ✅ ohne Abweichungen
- `cargo clippy --all-targets -D warnings`: ✅ ohne Warnungen
- `cargo test`: ✅ **45 Tests**, alle bestanden, Laufzeit rund 5 Sekunden

| Pflicht-Test aus dem Bauplan | Ergebnis |
|---|---|
| Summe nie über 10^26, auch m = 2,0 über 100 Jahre | ✅ dazu 12 Zufalls-Verläufe, 2 davon über 100 Jahre |
| Erste Epoche 50.500 TOY | ✅ genau |
| Halbwertszeit ≈ 1.372 Epochen | ✅ rechnerisch 1.372,25, die Hälfte ist in Epoche 1.373 erreicht |
| Spam erhöht m nicht | ✅ 2.000 Zufallsfälle, kostenlose Transaktionen ändern m nie |
| Vier Fälle aus 9.1 wie die Simulation | ✅ **auf die kleinste Einheit genau**, dazu 100 Jahre und 64 Prüfvektoren aus `docs/anhang/factor-vectors.py` |
| m ändert sich höchstens um 0,05 je Epoche | ✅ |
| 10-s-Blöcke: Epoche dauert 24 Stunden | ✅ 8.640 Blöcke, 86.400 s, nur die ersten 1.440 Blöcke minten |
| Kürzungen und Spenden erhöhen die Restmenge nie | ✅ ausgegebene Summe sinkt nie, Restmenge + Summe = 10^26. Es gibt keine Funktion, die die Summe senkt |
| Genesis-Zustand hat Summe 0 | ✅ gemintet und ausgegeben 0 |
| Lohn der Start-Validatoren | ✅ genau 2.217.786,5926 TOY bei m = 1, mit m bis 2,0 oder Epochen von 2, 3, 7 Tagen nie mehr |

**Messwerte:**

| Größe | Wert |
|---|---|
| Ausgegeben im 1. Jahr (Plan, m = 1) | 16.837.325,09 TOY |
| Ausgegeben im 1. Jahr bei hoher Nutzung | 17.213.576,05 TOY (+2,23 %) |
| Rest nach 100 Jahren hoher Nutzung | 0,0195 TOY bis zur Obergrenze |
| Größte Zwischenzahl der Emission | ca. 10^35 (u128 reicht bis 3,4 × 10^38) |
| Lohn je Start-Validator im 1. Block | ca. 0,835 TOY |

**Präzisierungen** (Entscheidung 113, in der Spezifikation nachgetragen): Genesis zählt nicht zu den 1.440 Blöcken von Epoche 0, die Emission von Epoche 0 wird mit Block 1 festgelegt, der Deckel ist genau `a(t) × (P(t + 1) − P(t' + 1))`, und die Summe des Lohns war in der Spezifikation um 0,0026 TOY zu knapp gerundet.

**Einschränkungen dieser Sitzung (ehrlich):**

- In der Cloud-Umgebung war **crates.io gesperrt**. Deshalb:
  - Statt `proptest` gibt es einen kleinen eigenen Testgenerator (SplitMix64, nur für Testeingaben). Jeder Fehlerfall nennt seinen Startwert.
  - Die **Kani-Beweise** (`toy-emission/src/verification.rs`: Emission ≤ Restmenge, Summe ≤ 10^26, Block-Mint geht genau auf) und **cargo-deny** (`toychain/deny.toml`) laufen nur auf GitHub (`.github/workflows/ci.yml`). Der Kani-Job war anfangs nicht blockierend, bis seine Laufzeit bekannt war. **Seit 20.10.2026 blockierend** (Kani 0.68.0 und Action auf festen Stand genagelt, Zeitlimit 15 Minuten), weil der nicht blockierende Job in PR #3 einen echten Fehler verdeckt hatte.
  - **Kani, erster Lauf (18.10.2026):** nach 90 Minuten abgebrochen, ohne Ergebnis. Ursache: Die Beweise multiplizierten zwei frei wählbare 128-Bit-Zahlen, das ist für den Beweis-Löser sehr aufwendig. **Änderung:** Jeder Beweis hält jetzt einen Faktor fest und lässt den anderen frei. Die Emission wird bei m = 2,0 für **jede** Restmenge bewiesen, das deckt wegen der Monotonie alle m bis 2,0 ab. Die Block-Aufteilung wird für jede Emission und jeden Block bei den Anteilen 0 %, 12,3457 % und 50 % bewiesen. Jeder der 6 Beweise läuft als eigener CI-Job mit 30 Minuten Zeitlimit.
  - **Kani, zweiter Lauf (18.10.2026):** ✅ bewiesen in je unter 2 Minuten: Emission ≤ Restmenge für jede Restmenge, ausgegebene Summe bleibt ≤ 10^26, Block-Aufteilung geht genau auf (ohne Start-Validator-Anteil). ⏳ Die drei Beweise der Block-Aufteilung mit Anteil (Normalphase, 50 %, 12,3457 %) sind nach 30 Minuten abgebrochen und laufen vorerst nicht mehr in der CI. Abgesichert ist das trotzdem: `block_mint` prüft bei jedem Block selbst, dass die Teile genau aufgehen, sonst gibt es einen Fehler statt einer falschen Ausschüttung. Dazu kommen Property-Tests über 300 zufällige Epochen mit allen Blöcken. Ein neuer Versuch folgt, wenn die Aufteilung in M3 oder M4 ohnehin überarbeitet wird.
- **cargo-vet** und **Fuzzing** beginnen mit M2. M1 hat noch keine fremden Bibliotheken und liest keine fremden Daten.
- Für M2 werden geprüfte Krypto-Bibliotheken von crates.io gebraucht (z. B. `k256`, `sha3`, `bip39`).

## Was im Start-Schritt angelegt wurde

**Workspace** (`Cargo.toml`) mit 10 Crates unter `crates/` und einem Test-Crate unter `tests/`. Alle Crates sind leer (nur eine Beschreibung, welcher Meilenstein sie füllt). Es gibt noch **keine externen Bibliotheken** und **keine Logik**.

Abhängigkeiten zwischen den Crates (von unten nach oben, nur in diese Richtung erlaubt):

```
toy-types
  ├── toy-crypto
  ├── toy-emission
  └── toy-state      (types, crypto)
        └── toy-tx   (types, crypto, state)
              ├── toy-consensus (types, crypto, state, tx, emission)
              │     └── toy-node (alle darunter)
              │           └── toy-rpc (types, tx, node)
              └── toy-wallet (types, crypto, tx)
toy-cli: node, rpc, wallet, types   ·   tests/: alle
```

**Festgelegte Werkzeuge und Regeln:**

- `rust-toolchain.toml`: Rust **1.97.0** festgenagelt (reproduzierbare Builds), Edition 2024, Mindestversion 1.85, **seit 24.10.2026 1.87** (Abschnitt „Sicherheits-Update vom 24.10.2026“ oben).
- Workspace-Lints, die die festen Vorgaben aus `CLAUDE.md` technisch absichern:
  - `unsafe_code = forbid` (kein unsicherer Code im Kern).
  - `clippy::float_arithmetic`: Fließkomma-Rechnung ist ein Fehler. Beträge nur als `u128`.
  - `clippy::arithmetic_side_effects`: Jedes `+`, `-`, `*` auf Zahlen muss als `checked_*` (oder bewusst `saturating_*`) geschrieben werden. Überläufe sind Fehler.
  - `unwrap`, `expect`, `panic`, ungeprüfte Indizes: im Produktivcode verboten, in Tests erlaubt (`clippy.toml`).
  - `print_stdout`/`print_stderr`/`dbg!`: verboten, damit keine Geheimnisse versehentlich ausgegeben werden. Die CLI bekommt dafür in M8 eine gezielte Ausnahme.
- Release-Profil: kleine Binärdateien (LTO, `panic = abort`, Symbole entfernt) für Raspberry Pi und Android.

**Prüfbefehle (CI):**

| Wo | Befehl |
|---|---|
| Linux/macOS/Pi | `toychain/scripts/check.sh` |
| Windows | `powershell -File toychain\scripts\check.ps1` |
| GitHub Actions | `.github/workflows/ci.yml` (Linux x86_64, Windows x86_64, Linux aarch64) |

Die Skripte führen genau das aus, was `CLAUDE.md` verlangt:
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.

## Ergebnisse der Prüfungen (17.10.2026)

- `cargo fmt --check`: ✅ ohne Abweichungen
- `cargo clippy --all-targets -D warnings`: ✅ ohne Warnungen
- `cargo test`: ✅ 0 Tests (erwartet, noch keine Logik), alle 11 Crates bauen

Hinweis zur Bauumgebung: In der Cloud-Sandbox, in der das Gerüst erstellt wurde, konnte `rustup` die Toolchain nicht herunterladen. Die Prüfungen liefen deshalb mit `RUSTUP_TOOLCHAIN=stable`, das dort genau 1.97.0 war. Auf deinem Rechner lädt `rustup` 1.97.0 beim ersten `cargo`-Aufruf automatisch.

## Neuausrichtung vom 18.10.2026

Das Projekt ist auf den Kern reduziert (Entscheidungen 61 bis 114):

- **Totaler Fair Launch:** Genesis ohne Guthaben, keine Reserve, alle 100 Mio. TOY per Emission. Erste Epoche 50.500 TOY, erstes Jahr rund 16,84 Mio., Halbwertszeit 3,76 Jahre (nachgerechnet).
- **Kern:** ToyChain-Testnet, TOYPAY Wallet als Knoten mit Stufen 0 bis 4, Energie-Modus und Mint-Funktionen, ToyScan mit Warnampel A1 bis A9 und Anzeige anderer EVM-Ketten.
- **Neu (18.10.2026):** 45 Sprachen für Wallet, TOYPAY-App und ToyScan, gemeinsame Sprachliste und Begriffsliste in `i18n/` (Nr. 103), vier Modi in der Wallet von Einsteiger bis Vollprofi (Nr. 104).
- **Neu (18.10.2026 abends, Runde 3):** 32 weitere Funktionen (Nr. 95 bis 102): Delegieren, Mehrfachsignatur, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis, Namen, Einmal-Adressen, Post-Quanten-Schlüssel, Alltagsschutz, Kassen-Modus, Offline-Signieren, Einsteiger-Hilfen, Netzwerk-Karte, SDKs, formale Prüfung, Transparenz-Bericht.
- **Neu (18.10.2026 abends, MetaMask):** 22 Wallet-Funktionen nach dem Vorbild von MetaMask (Nr. 89 bis 91), darunter mehrere Konten, Phishing-Warnliste, Browser-Erweiterung, Netzwerk per Klick, Mitteilungen und verschlüsselter Sync.
- **Neu (18.10.2026 abends):** 30 Funktionen aus anderen Projekten übernommen (Nr. 82 bis 88): Wallet-Schutz, Konto-Schutz mit Tresor, Notfall-Schlüssel und Erben-Adresse, Zahlungsfunktionen, atomarer Tausch, Dandelion++, Tor, mehrfach signierte Checkpoints und Builds, Testnet-Kran, Fuzzing, Bug-Bounty, Notfall-Plan.
- **Neu:** Startphase ohne Stake, Arbeits-Lotterie, Schutzregeln mit Sperrzeiten in drei Ebenen, Aktivierungs-Pfand statt Entfernen von Konten (später senkbar), es wird nie TOY verbrannt, aktive und ruhende Konten für 100 Mio. Konten und mehr.
- **Verschoben nach `docs/spaeter/`:** Bridge, Altbestand-Token, Reserve, Mitbestimmung, Spiel und Spiel-NFTs, alte Spezifikation v0.2.

## Dokumente

| Dokument | Stand |
|---|---|
| `01-spezifikation.md` | v0.3, schlank, mit Verfassung (Abschnitt 13) |
| `02-entscheidungen.md` | Nr. 1 bis 133, Abschnitt „Neuausrichtung“ sagt, was gilt, später kommt oder entfällt. Nr. 117 bis 121: Festlegungen aus M3. Nr. 122 bis 133: Ergänzung ToyVerse und Webseite (für später) |
| `03-bauplan-phase1.md` | M1 bis M13 |
| `whitepaper/` | Whitepaper Entwurf 0.4 (20.10.2026, Stand nach M3) und Kurzanleitung für Nutzer |
| `04-toypay-wallet.md` | v0.1: Clients, Stufen, Energie-Modus, Mint-Funktionen, andere Ketten |
| `05-toyscan.md` | v0.2: Warnampel A1 bis A9, Muster-Warnungen, andere Ketten, API |
| `11-pruefbericht-2.md` | Prüfbericht 2 vom 18.10.2026: 3 kritische, 10 hohe Befunde. Eindeutige Fehler korrigiert (Nr. 106 bis 108), Entscheidungen offen |
| `10-pruefbericht-1.md` | Prüfbericht vom 17.10.2026, mit Vermerk, welche Befunde nach der Neuausrichtung noch gelten |
| `spaeter/11-toyverse.md`, `12-toycash.md`, `13-toyquest-items-nft.md` | Ergänzung ToyVerse vom 24.10.2026: Ökosystem, ToyCash, TOYQUEST-Items. **Für später**, ändert nichts an M3 bis M13 |
| `toypay/docs/08_LERNWELT_ZEITREISE.md` | TOYQUEST-Lernwelt und Zeitreise, Zuordnung der 50 Maschinen zu 5 Level (für später) |
| `toypay/docs/06_NFT_GEGENSTAENDE.md`, `07_SPIELDESIGN.md` | für später. Katalog und Effekt-Paket bleiben getestet (`nft_items_test`, `vfx_test`: ALL OK) |

## Noch offen

- Bestätigt am 18.10.2026: Nr. 67 (drei Ebenen), 68 (NFT-Prägen später), 70 (Start-Validatoren mit Lohn), 77 (MPL-2.0 für UniFFI). Schutz in der Startphase: Nr. 78 (nur Neue lösen die Übergabe aus, Lohn sinkt über 180 Epochen auf 0, Lohn 365 Epochen gebunden).
- Formel für m festgelegt (Nr. 81, Spezifikation 9.1, Simulation in `docs/anhang/m-simulation.py`).
- Ab M1: Lizenzen, Sicherheitsmeldungen und Herkunft der fremden Bibliotheken automatisch prüfen (`cargo-deny`, `cargo-vet`). Erlaubt sind Lizenzen, die zu MIT passen (MIT, Apache-2.0, BSD, ISC, CC0, Zlib, Unicode). Dazu MPL-2.0 für unverändert genutzte Bibliotheken (Nr. 77) und SIL Open Font License 1.1 für Schriften (Nr. 105).
- Aus dem Prüfbericht: P5, P6 (M5). P9 und der Höchstwert der progressiven Gebühr sind seit 20.10.2026 entschieden (Nr. 117, 120).
- ToyVerse (später, Stand 24.10.2026): Währung und Kurs der Prägegebühr (Nr. 127), Anker-Transaktion nach M13 (Nr. 124), Stufenwerte des ToyCash-Offline-Rahmens. Liste in `spaeter/11-toyverse.md`, Abschnitt 8.
