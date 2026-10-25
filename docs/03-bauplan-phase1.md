# Bauplan Phase 1: Testnet mit Knoten, Wallet und ToyScan

Stand 25.10.2026 (Neuausrichtung, Entscheidungen 61 bis 114; Tokenomics v0.4, Entscheidungen 136 bis 152, neuer Meilenstein M3b). Ziel von Phase 1: ein lauffähiges **Testnet** mit totalem Fair Launch (Genesis ohne Guthaben), Startphase ohne Stake, Emission, Kontingent, Gebühren, Schutzregeln, der TOYPAY Wallet als Knoten auf allen Zielplattformen und ToyScan mit Warnampel. Beweis-Schicht (Phase 2) und alles in `docs/spaeter/` kommen später.

## Ehrliche Einschätzung

- Phase 1 ist ein **Testnet**, kein Mainnet. Für ein Mainnet braucht es externe Audits, Lasttests und eine rechtliche Prüfung.
- Die Beweis-Schicht (Falten der Historie) ist Forschungsarbeit und kommt in Phase 2.
- Mit einem Pro-Abo gelten Nutzungslimits. Pro Sitzung ein Meilenstein, danach mit `/clear` neu beginnen. Der Stand steht in `docs/STATUS.md`.

## Ordnerstruktur

Die Dokumentation liegt im Hauptordner unter `docs/`. Der Knoten ist ein Rust-Workspace in `toychain/`, ToyScan ein eigenes Rust-Projekt in `toyscan/`.

```
toychain/
  Cargo.toml                 (Workspace)
  crates/
    toy-types/               Konstanten, Amount (u128), Address, Hash, Fehlertypen
    toy-crypto/              secp256k1, Keccak-256, BLAKE3, BIP-39/32, Argon2id, BLS, VRF (Wrapper)
    toy-emission/            Emissionskurve, Faktor m, Zeitplan-Regler, Aufteilung, Startphase
    toy-state/               Konten, Aktivierungs-Pfand, Stake, Merkle-Baum, Regel I0, Sperr-Status
    toy-tx/                  Transaktionsarten, Signaturprüfung, Kontingent, Gebühren, ToyWork-Lose
    toy-consensus/           Komitee, VRF, BFT, BLS-Sammelsignaturen, Startphase, Strafen
    toy-node/                Speicher, P2P (libp2p), Mempool, Blockproduktion, Energie-Steuerung
    toy-rpc/                 Ethereum-kompatibles JSON-RPC (Teilmenge) und toy_-Methoden
    toy-wallet/              Schlüssel, Keystore, Stufenwahl, andere EVM-Ketten
    toy-cli/                 Kommandozeile: Wallet, ToyNode, Testnet starten
  tests/                     Integrations- und Simulationstests
toyscan/                     Explorer und Warnampel (M11)
```

## Umfang von Testnet 1 (Entscheidung 109, Mittelweg)

| In Testnet 1 | Danach (spätere Testnet-Versionen) |
|---|---|
| M1 bis M3: Emission, Konten mit Pfand, Kontingent mit Grenzen, Gebühren, Arbeits-Lotterie, Schutzregeln | Aufnahme-Pflicht (feste Reihenfolge bleibt in Testnet 1), Strafe nach Ausmaß über den einfachen Fall hinaus |
| M3b: Tokenomics v0.4 (K-Tabelle, Aufteilung, Belohnungs-Zähler, Stufen 1 bis 7, Kontingent 21 bis 49, Arbeits-Lotterie mit Jackpots) | Vorkasse-Kontingent, Gemeinschafts-Lose |
| M4: Konsens im Stil von Tendermint, 64 Start-Validatoren, Lohn, gleitende Übergabe, Komitee mit festen und Los-Sitzen, Dienst-Auslosung | Stake delegieren |
| M5: Speicher, Messbericht. Ruhende Konten nur, wenn die Messung sie nötig macht | Ruhende Konten mit Zusammenführen, falls nicht schon nötig |
| M6: Netzwerk mit libp2p | Dandelion++, Tor |
| M7: JSON-RPC und `toy_`-Methoden | SDKs |
| M8, M9: Wallet und App mit **45 Sprachen**, **vier Modi**, Schutz beim Senden, Sicherungs-Quiz, automatischer Sperre, mehreren Konten, Mitteilungen | Browser-Erweiterung, eingebauter Browser, Täuschungs-PIN, NFC, Kassen-Modus, Offline-Signieren, Sync |
| **Konto-Schutz** aus M12: Tresor-Verzögerung, Notfall-Schlüssel, Sicherheitsadresse, Selbst-Sperre, Erben-Adresse | übrige Teile von M12 (Mehrfachsignatur, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis, Sammelüberweisung, Gebühr übernehmen, Hash-Zeitschloss) und M13 |
| M10: lokales und öffentliches Testnet, Testnet-Kran, `toy-cli devnet` | dreifach signierte Builds (sobald es 3 Mitwirkende gibt) |
| M11: ToyScan mit Warnampel und Erklärungen in 45 Sprachen, Anzeige anderer Ketten | Senden auf anderen Ketten, WalletConnect, Schutz vor Vordrängeln auf anderen Ketten, Netzwerk-Karte, offene Daten |

Ein Meilenstein kann für eine Sitzung zu groß sein. Dann wird er im Plan in Teilschritte (z. B. M8a, M8b) geteilt.

## Meilensteine

Jeder Meilenstein hat **Fertig-Kriterien**. Erst wenn alle erfüllt sind, geht es weiter.

### M1: Grundtypen und Emission
- **Ab M1 in jedem Meilenstein:** Fuzz-Tests für alle Funktionen, die fremde Daten lesen, laufen dauerhaft in der CI (Entscheidung 88). Fremde Bibliotheken werden mit `cargo-deny` und `cargo-vet` auf Lücken, Lizenzen und Herkunft geprüft (Entscheidung 101).
- **Formale Prüfung** der Obergrenze und der Emission mit einem Beweis-Werkzeug für Rust (z. B. Kani, Entscheidung 102).
- Typen, Konstanten (Obergrenze 10^26), sichere Rechenfunktionen (`checked_*`).
- **Genesis ohne Guthaben.** Emission `k × Restmenge` mit k₀ = 505/1.000.000, m ∈ [0,5; 2,0], Restmenge = 10^26 − Summe der Emissionen, Zeitplan-Regler. Aufteilung 50/40/10. In der Startphase geht der Konsens-Anteil zu gleichen Teilen an die Start-Validatoren und sinkt über 180 Tage Planzeit auf 0 %, mit Deckel, der Rest in die Arbeits-Lotterie.
- Epochen-Ende nach mindestens 1.440 Blöcken **und** 86.400 Sekunden.
- Faktor m genau nach Spezifikation 9.1 (Millionstel, Plan nach echter Zeit, Glättung 0,05). Die Rust-Umsetzung muss bei denselben Eingaben exakt dieselben Werte liefern wie `docs/anhang/m-simulation.py`.
- **Fertig, wenn:** Property-Tests belegen, dass die Summe aller Ausschüttungen 10^26 **nie** überschreitet (auch bei m = 2,0 über 100 Jahre). Erste Epoche = 50.500 TOY, Halbwertszeit ≈ 1.372 Epochen bei m = 1. Spam erhöht m nicht. Die vier Fälle aus Spezifikation 9.1 ergeben dieselben Zahlen wie die Simulation. m ändert sich nie um mehr als 0,05 je Epoche. Test mit 10-s-Blöcken: Epoche dauert trotzdem 24 Stunden. Kürzungen und Spenden erhöhen die Restmenge nie. Genesis-Zustand hat Summe 0. Lohn der Start-Validatoren über alle 180 Tage zusammen genau 2.217.786,5926 TOY bei m = 1 und einer Epoche je Tag, und mit schnellerer Emission oder 3-Tage-Epochen nicht mehr (Deckel und Planzeit, Entscheidung 110).

### M2: Kryptografie und Adressen
- Schlüssel aus BIP-39, Pfad m/44'/60'/0'/0/n, Adresse wie Ethereum, Signieren mit Chain-ID (EIP-155), BLS12-381, VRF.
- **Fertig, wenn:** offizielle BIP-39- und Ethereum-Testvektoren ergeben exakt die erwarteten Adressen. Signaturen anderer Chain-IDs werden abgelehnt.

### M3: Zustand, Transaktionen, Kontingent, Gebühren, Schutzregeln
- Beide Transaktionsformate (ToyChain nach EIP-2718, Ethereum für MetaMask). Grenzen für kostenlose Transaktionen (höchstens 50 % je Block, Erstattung höchstens 1/30 des Pools je Epoche). Signierte Lotterie-Lose mit späterer Ziehung (Entscheidungen 106, 107).
- Konten mit **Aktivierungs-Pfand 0,1 TOY** beim ersten Eingang, Transaktion „Konto auflösen“ (an Zieladresse oder als Spende in den Erstattungs-Pool), neue Nonce-Basis bei erneuter Aktivierung. Stake mit 21 Epochen Lösefrist (Spezifikation 4, Entscheidung 106; hier stand früher irrtümlich 7), Merkle-Wurzel. Es wird nie TOY verbrannt.
- **Regel I0:** Summe aller Guthaben = Summe aller Emissionen in jedem Block.
- Kontingent, Erstattungs-Pool, progressive Gebühr mit Höchstwert, Vielnutzer-Pflicht (in M3b durch v0.4 ersetzt).
- Transaktionsfelder Verwendungszweck (32 Byte) und Ablaufzeit (Entscheidung 84).
- **ToyWork-Lose** (Argon2id, 16 MiB) mit Schwierigkeits-Schwelle, Höchstzahl je Block und Peer, Arbeits-Lotterie mit Mindestgewinn 0,2 TOY.
- **Schutzregeln Ebene A** (Spezifikation Abschnitt 8): Sperrzeiten, Verdopplung, Zurücksetzen nach 30 sauberen Epochen. Kein Code-Pfad sperrt Guthaben.
- **Fertig, wenn:** Tests für I0 nach jedem Block (auch ein absichtlich fehlerhafter Block wird abgelehnt), Doppelausgabe, Überlauf, Kontingent bei verschiedener Auslastung, Gebühren, ungültige und wiederverwendete Lose, jede Sperrregel mit Fall „greift“ und „greift nicht“, ein Test, dass ein gesperrtes Konto weiter gegen Gebühr überweisen kann, Tests für Spenden an den Erstattungs-Pool (Betrag landet genau im Pool, Regel I0 hält, nicht rückholbar), Aktivierung (Eingang unter dem Pfand an neue Adresse wird abgelehnt, Senken des Pfands gibt den Unterschied frei), Auflösen mit und ohne Spende und dass eine alte Transaktion nach erneuter Aktivierung ungültig bleibt.

### M3b: Tokenomics v0.4 (Entscheidungen 136 bis 152)
- **K-Tabelle:** Erzeugungs-Skript mit 50-stelliger Dezimal-Rechnung in `docs/anhang/`, Tabelle als Datei im Code (2.434 Stützstellen, `k` in 10^−12, `w` und `W` in ppm), BLAKE3-Hash als Verfassungs-Konstante, ganzzahliges Einsetzen zwischen den Stützstellen (Spezifikation 9.1).
- Emission `floor(floor(Restmenge × k ÷ 10^6) × m ÷ 10^12)`, Plan P(t) aus der Tabelle, m-Formel unverändert. Ersetzt k₀ = 505/1.000.000.
- **Aufteilung je Block** nach Spezifikation 9.2 (Arbeit `w(t)`, Startlohn, Komitee 5 %, Voll-Knoten 8 %, Pool 2 %, Staking-Topf Rest), Überschuss des Pools und Gebühren-Anteil in den Staking-Topf, Gebühren 20/40/40.
- **Belohnungs-Zähler** (Spezifikation 9.4), Gewicht Sparer 0,8, Pflichterfüllung je Epoche, neuer Topf in Regel I0.
- **Stufen 1 bis 7** (1 / 5 / 10 / 25 / 50 / 75 / 100 TOY) als Konstanten, `shared/toyverse.json` neu erzeugen.
- **Kontingent** `min(21 + 28 × (1 − Auslastung), λ)` mit Wasserstand und Anlauf über 7 Epochen. Vielnutzer-Regel neu (doppelte Grundgebühr statt Arbeitspflicht).
- **Arbeits-Lotterie** (Spezifikation 9.3): bis 32 Lose je Block zeitanteilig, zufällige Gewinnzahl, Glücksstufen 1/2/3/5/8, Mindestgewinn 0,01 TOY bzw. 0,2 TOY, Jackpot-Töpfe Funke, Welle, Gezeiten mit Faktoren, Zwang, Meilensteinen und Höchstgrenze, Zufallsmix.
- **Fertig, wenn:** Kani beweist `k × 2 < 1` für jeden Tabelleneintrag, Emission ≤ Restmenge und keine Überläufe. Property-Test über 200 Jahre bei m = 2,0 bleibt unter 10^26. Erste Epoche und erstes Jahr stimmen mit `docs/anhang/v04/kurven.py` überein (rund 15.822 TOY und 10,16 Mio. TOY bei m = 1). Die Aufteilung geht in jedem Block genau auf (I0). 100 Konten mit je 1 TOY bekommen aus dem Staking-Topf bis auf Rundung genau so viel wie 1 Konto mit 100 TOY gleicher Stufe. Kontingent bei 0 % Last 49, bei voller Last 21, mit Wasserstand weniger. Jackpot-Töpfe zählen in I0, eine Ausschüttung überschreitet nie die Höchstgrenze, und Ziehungen hängen nur von `R_(N+2)` ab (Test: anderer Vorschlagender in Block N ändert nichts). Erwartungswert je Los ist in Simulation und Rust-Umsetzung gleich (Abweichung < 1 % über 1 Mio. Blöcke).

### M4: Blöcke, Konsens, Startphase
- Blockformat mit Zeitstempel-Regeln, Komitee mit VRF, BFT im Stil von Tendermint (zwei Abstimmungsphasen je Höhe, Runden bei Zeitüberschreitung), BLS-Sammelsignatur, Finalität nach 1 Block (Entscheidung 106).
- **Komitee** mit 64 festen Sitzen und 64 Los-Sitzen, Stimmgewicht nach wirksamem Stake, Ersatzbank, Dienst-Auslosung für Attestierer und Proben (Spezifikation 4 und 5.1, Entscheidungen 143, 144).
- **Startphase:** 64 Start-Validatoren ohne Stake, Startlohn 25 % → 0 % in 365 Tagen mit Deckel, als Zeitschloss gesperrt, gleitende Übergabe mit sinkendem Ziel (Spezifikation 5.3, Entscheidungen 139 bis 141).
- Strafen: Doppelsignatur mit Kürzung nach Ausmaß (Spezifikation 5.4), Inaktivität, Sperrzeiten für Rollen.
- Feste Reihenfolge im Block und Aufnahme-Pflicht mit Listen des Komitees (Spezifikation 5.1a).
- **Stake delegieren** mit Validator-Anteil, anteiliger Kürzung und ohne Delegation an Start-Validatoren (Spezifikation 7.4).
- Konsens zusätzlich als formales Modell prüfen (z. B. TLA+, Entscheidung 102).
- Pannenschutz (Spezifikation 5.5): Signier-Gedächtnis, Doppelgänger-Erkennung, ein Signier-Gerät je Konto.
- Simulation mit z. B. 20 virtuellen Validatoren, inklusive Ausfällen.
- **Fertig, wenn:** 10.000 Blöcke stabil, bei bis zu 1/3 Ausfall weiter, Doppelsignaturen nach Ausmaß bestraft (Einzelfall 1 %, ein Drittel gleichzeitig 100 %), Signier-Gedächtnis verhindert eine zweite Unterschrift auch nach Neustart und nach Zurückspielen einer alten Sicherung, ein Doppelgänger bleibt stumm, ein falsch sortierter Block und ein Block ohne gemeldete Transaktion (bei freiem Platz) werden abgelehnt, gleitende Übergabe in der Simulation getestet (Stimmgewicht der Start-Gruppe sinkt mit `V`, Ende spätestens ab Tag 540 möglich), Start-Validatoren erhalten genau gleiche, sinkende Anteile mal Verfügbarkeit (nach 365 Tagen Planzeit 0), ihr Lohn ist gesperrt und gibt kein Stimmgewicht, und Stake von Start-Validatoren zählt nicht für `V`. **Komitee:** Ein Teilnehmer mit 86 Validatoren zu je 100 TOY hat nicht mehr Stimmgewicht als mit einem Validator gleichen Stakes. Komitee-Topf und Voll-Knoten-Topf werden linear nach Stake verteilt, Stake über dem Deckel bringt nichts.

### M5: Speicher und 256-MB-Budget
- Persistente Datenbank (z. B. redb), Block-Fenster nach Bytes, Checkpoints mit Signaturen von mindestens 3 unabhängigen Stellen (Entscheidung 87).
- **Aktive und ruhende Konten** (Spezifikation 6.2): Auslagern nach Bedarf ab 90 % Füllung, Ruhe-Schnappschuss je Epoche mit Wurzel, Summe und Aufweck-Merkliste, Beweise für ruhende Konten, Aufwecken und Zusammenführen.
- **Fertig, wenn:** Messbericht in `docs/STATUS.md`: Speicher je aktivem Konto, aktive Konten je 160 MB, Größe der Beweise und der Merkliste. **Simulation mit 100 Mio. Konten**, davon 2 Mio. aktiv: Budget wird gehalten, jedes ruhende Konto lässt sich mit seinem Beweis aufwecken, ein zweites Aufwecken wird abgelehnt, Zusammenführen hält Regel I0. Reicht das Budget nicht, wird der Notfallplan mit Zahlen vorgelegt.

### M6: Netzwerk
- libp2p mit QUIC und Noise, Peer-Entdeckung, Verbreitung von Transaktionen, Blöcken und Losen, Ratenbegrenzung, Peer-Reputation (Schutzregeln Ebene B).
- Dandelion++, optional Tor (z. B. mit der Rust-Bibliothek Arti, Lizenz prüfen), Ersetzen hängender Transaktionen mit höherer Gebühr (Entscheidungen 85, 87).
- **Fertig, wenn:** 5 Knoten in getrennten Prozessen finden sich, tauschen Daten aus und erreichen Konsens. Ein Peer, der ungültige Lose flutet, wird gedrosselt.

### M7: JSON-RPC
- `eth_chainId`, `eth_getBalance`, `eth_getTransactionCount`, `eth_sendRawTransaction`, `eth_blockNumber`, `net_version`.
- Für Wallet und ToyScan: `toy_getSupply`, `toy_getEpoch`, `toy_getBootstrap`, `toy_getValidators`, `toy_getPenalties`, `toy_getConstitution` (`05-toyscan.md`, Abschnitt 3).
- **Fertig, wenn:** MetaMask oder ein Skript mit einer Ethereum-Bibliothek kann ToyChain einbinden, den Kontostand lesen und TOY senden.

### M8: Wallet und CLI
- **Alle Texte nur über Schlüssel** aus `i18n/` (45 Sprachen, Entscheidung 103) und **vier Modi** Einsteiger, Fortgeschritten, Profi, Vollprofi (Entscheidung 104).
- Wallet erzeugen, Phrase sichern (auch als SLIP-39-Teile), Vorschau vor jeder Signatur, Schutz vor Adress-Fallen, Adressbuch, Probebetrag, Nur-Ansehen-Konten, Hardware-Wallets für Überweisungen, QR-Zahlungen, CSV-Export, Daueraufträge, mehrere Konten und Phrasen, Schlüssel-Import mit Passwort, Sicherungs-Quiz, automatische Sperre, Guthaben verbergen, Gebühren-Stufen, verständlicher Verlauf, Keystore (Argon2id, XChaCha20-Poly1305), Senden, Empfangen, Kontingent anzeigen, Stake binden und lösen, Stufenwahl mit Gerätemessung (`04-toypay-wallet.md`).
- **Fertig, wenn:** Ende-zu-Ende-Test im lokalen Testnet: Ein Gerät ohne TOY gewinnt in der Arbeits-Lotterie, sendet dann an eine zweite Wallet, Kontingent wird erstattet, Kontostände stimmen.

### M9: Plattformen und Energie-Modus
- Builds für Linux x86_64, Windows x86_64, Linux aarch64 (Raspberry Pi), Android (Bibliothek plus App). ToyNode als Dienst.
- Energie-Modi Sparsam, Ausgewogen, Leistung, Auto (Akku, Ladegerät, Temperatur, Netz, Nutzung).
- Oberfläche in 45 Sprachen mit mitgelieferten Schriften und Rechts-nach-links, Sprachtest für alle Sprachen (kein Schlüssel fehlt, Platzhalter stimmen, alle Zeichen darstellbar). Die vier Modi in der Oberfläche, Sicherheitswarnungen in jedem Modus.
- Mitteilungen, Ende-zu-Ende verschlüsselter Sync ohne Schlüssel, Browser-Erweiterung für Chrome und Firefox (Entscheidungen 90, 91).
- Zwischenablage-Schutz, Bildschirmfoto-Sperre, Geräte-Prüfung, Täuschungs-PIN, Sicherheits-Check, Übungs-Modus, einfache Sprache, Barrierefreiheit, Kassen-Modus, Offline signieren per QR, NFC, Rechnungen (Entscheidungen 97 bis 99).
- **Fertig, wenn:** Der Knoten läuft auf Linux, Raspberry Pi und Windows, Android mindestens als Stufe 0 mit ToyWork am Ladegerät. Auto-Modus schaltet in Tests richtig um.

### M10: Testnet und Lasttests
- Lokales, dann öffentliches Testnet mit Start-Validatoren. Lasttest mit vielen Wallets und Losen.
- **Testnet-Kran:** gibt Testnet-TOY zum Ausprobieren aus, gefüllt aus Lotteriegewinnen eigener Knoten (Genesis bleibt leer), mit Begrenzung je Adresse und Tag und ToyWork-Los gegen Abräumen.
- **Ein-Befehl-Testnetz:** `toy-cli devnet` startet ein lokales Mini-Netz mit mehreren Knoten (Entscheidung 101).
- **Reproduzierbare, mehrfach signierte Builds**, Prüfung vor jedem Update in Wallet und Knoten. **Notfall-Plan** als `docs/06-notfallplan.md`. **Bug-Bounty-Programm** in `SECURITY.md` (außerhalb der Kette, Entscheidung 88).
- **Fertig, wenn:** Messwerte (Transaktionen pro Sekunde, Finalität, Speicher, Prüfzeit je Los auf altem Android) stehen in `docs/STATUS.md`, Parameter aus Spezifikation Abschnitt 14 sind abgeglichen.

### M11: ToyScan und andere Ketten
- ToyScan nach `05-toyscan.md` in 45 Sprachen mit Schalter für Erklärungen: Indexer, Warnampel A1 bis A9, Muster-Warnungen, Adress-Ansicht mit anderen EVM-Ketten, offene API, Prüfskript, Netzwerk-Karte, Validator-Bewertung, Adress-Alarm, offene Daten.
- **Programm-Bibliotheken (SDKs)** für JavaScript/TypeScript, Kotlin und Python mit Beispielen (Entscheidung 101).
- Wallet: Guthaben und Senden auf anderen EVM-Ketten, Warnung bei gleichnamigen Token, Phishing-Warnliste, Sicherheitswarnungen, Seiten-Berechtigungen, Anmelden mit Adresse, eingebauter Browser, Netzwerk per Klick, Token- und NFT-Erkennung, Gesamtübersicht in Euro, Simulation vor dem Senden, geschützter Sendeweg mit wählbarem Anbieter und Rückerstattung, Warnung bei Preisabweichung, begrenzte Freigaben (Entscheidungen 89 bis 94). NFT-Prägen kommt später mit dem Spiel (Entscheidung 68).
- **Fertig, wenn:** jede Ampel-Regel hat Tests für Grün, Gelb und Rot, ToyScan läuft gegen das Testnet, Rust-Rechner und Prüfskript stimmen überein.

### M12: Konto-Schutz, Zusatz-Transaktionen und Tausch
- Transaktionsarten Mehrfachsignatur-Konto, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis (Spezifikation 7.4), Sammelüberweisung, Gebühr übernehmen, Schutzeinstellungen (Tresor-Verzögerung, Notfall-Schlüssel, Sicherheitsadresse, Selbst-Sperre, Erben-Adresse) und Hash-Zeitschloss (Spezifikation 7.1 bis 7.3, 11).
- Wallet: Assistent für Konto-Schutz, geführter atomarer Tausch mit Bitcoin oder einer EVM-Kette im Testnet, WalletConnect, Anzeige von Freigaben auf anderen Ketten.
- **Fertig, wenn:** Tests zeigen, dass eine verzögerte Abhebung abgebrochen werden kann, ein Dieb ohne Notfall-Schlüssel nichts an fremde Adressen senden kann, solange das Konto gesperrt ist, das Ändern des Schutzes selbst verzögert wirkt, die Erben-Adresse erst nach Ablauf übernehmen kann, ein Hash-Zeitschloss genau einmal abgeholt oder nach Ablauf zurückgeholt wird, und Regel I0 überall hält. Mehrfachsignatur braucht genau m Unterschriften, ein widerrufener Sitzungs-Schlüssel und eine widerrufene Abo-Erlaubnis wirken sofort nicht mehr, ein Sitzungs-Schlüssel kann seine Grenze nie überschreiten, eine Treuhand ohne Entscheidung geht nach der Frist an den Käufer zurück. Ein atomarer Tausch im Testnet gelingt in beide Richtungen, und bei Abbruch bekommen beide ihr Geld zurück.

### M13: Namen, Einmal-Empfangsadressen, Post-Quanten-Schlüssel
- Namen mit Jahresgebühr und Schonfrist, Einmal-Empfangsadressen nach dem Vorbild von EIP-5564, Post-Quanten-Zusatzschlüssel nach ML-DSA (Spezifikation 7.4). Nur mit geprüften Bibliotheken.
- **Fertig, wenn:** Namen sind eindeutig, nur ASCII, laufen ab und lassen sich danach neu vergeben. Eine Zahlung an eine Einmal-Adresse kommt nur beim richtigen Empfänger an und ist für andere nicht zuzuordnen. Ein Konto mit Post-Quanten-Schlüssel lehnt jede Ausgabe ohne beide Signaturen ab. Offizielle NIST-Testvektoren für ML-DSA bestehen. Speicherbedarf aller drei ist gemessen.

## So startest du (Claude Code)

1. Repository klonen, Terminal öffnen, in den Ordner wechseln.
2. `claude` starten. Die Datei `CLAUDE.md` wird automatisch gelesen.
3. Mit dem Prompt für M1 beginnen.
4. Nach jedem Meilenstein: Ergebnis prüfen, `docs/STATUS.md` ansehen, mit `/clear` neu starten, nächster Prompt.

## Fertige Prompts

**M1:**
> Setze Meilenstein M1 aus docs/03-bauplan-phase1.md um (toy-types und toy-emission). Zeige zuerst einen kurzen Plan. Schreibe Property-Tests (z. B. mit proptest), die belegen, dass die Obergrenze von 100.000.000 TOY nie überschritten wird. Führe cargo fmt, clippy und test aus und berichte die Ergebnisse. Aktualisiere docs/STATUS.md.

**M2:**
> Setze M2 um (toy-crypto). Nutze nur geprüfte Crates. Teste mit offiziellen BIP-39- und Ethereum-Testvektoren und mit einem Replay-Test für die Chain-ID. Erkläre mir danach in einfachen Worten, was du gebaut hast.

**M3:**
> Setze M3 um (toy-state und toy-tx). Halte dich genau an die Abschnitte 6 bis 9 der Spezifikation. Schreibe Tests für Regel I0, Kontingent, Gebühren, ToyWork-Lose und jede Sperrregel.

**M4:**
> Setze M4 um (toy-consensus): Komitee, VRF, BFT, BLS-Sammelsignatur, Startphase, Strafen. Simulation mit 20 virtuellen Validatoren über 10.000 Blöcke, auch mit Ausfällen und der Übergabe aus der Startphase. Berichte Messwerte.

**M5:**
> Setze M5 um: persistenter Speicher, Block-Fenster nach Bytes, aktive und ruhende Konten nach Spezifikation 6.2. Simuliere 100 Mio. Konten, miss den Speicher und schreibe einen Messbericht in docs/STATUS.md. Prüfe, ob die 256 MB gehalten werden.

**M6 bis M13:** gleiche Form. „Setze M<Nummer> aus docs/03-bauplan-phase1.md um. Zeige zuerst einen Plan, schreibe Tests, führe fmt, clippy und test aus, aktualisiere docs/STATUS.md.“

## Ohne Claude Code (nur Chat mit Pro)

Lege ein Claude-Projekt an, lade die Dateien als Projektwissen hoch und bearbeite die Meilensteine im Chat. Bei großen Meilensteinen den Prompt teilen (z. B. M3 erst Zustand, dann Gebühren, dann Schutzregeln).

## Danach

Phase 2 Beweis-Schicht. Danach, Reihenfolge offen: Shop-Plugin für Online-Shops (Entscheidung 101), Mitbestimmung, Mainnet, Bridges, Privatsphäre, Offline-Schecks, Spiel-Anbindung (`docs/spaeter/`). Eine EVM-Ausführung gibt es nie (Entscheidung 36).
