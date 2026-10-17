# Entscheidungsprotokoll ToyChain

Stand: 17.10.2026. Alle Punkte stammen aus den Vorgaben des Projektinhabers. Mit „Vorschlag“ markierte Punkte sind Vorschläge von Claude, die noch bestätigt werden müssen.

## Ursprüngliche Vorgaben

- Eigenständige Blockchain mit nativer Währung ToyCoin, Ticker **TOY**, 18 Dezimalstellen.
- Ressourcensparend wie das Mina-Protokoll, sicher wie Bitcoin, Adressen und Wiederherstellungsphrase wie Ethereum.
- Kompatibel mit EVM-basierten dApps, damit später eine Bridge zu EVM-Token möglich ist.
- Läuft auf alten Android-Geräten, Raspberry Pi, Linux-PC und Windows (als App oder dApp).
- Jeder Nutzer kann an Signaturen und am Mint-Prozess teilhaben.
- Möglichst Monero-Logik (z. B. Ringsignaturen).
- Offline-Transaktionen zwischen Wallets künftig möglich.
- Vorerst keine Layer-2-Token, keine NFTs, keine eigenen Smart Contracts. (Am 17.10.2026 verschärft: auf ToyChain nie, siehe Entscheidung 36.)
- Gesamtmenge 100.000.000 TOY, in ca. 25 Jahren durch Programm und Netzwerk gemintet.
- Je Wallet 10 bis 100 kostenlose Transaktionen je nach Auslastung (Gebühr wird erstattet). Bei Wallets mit hohem Volumen geringe Gebühr, die zufällig anderen Teilnehmern zugutekommt, neben dem Mint-Prozess.
- Transaktionen dauern höchstens ca. 1 Minute oder bis der Pool voll ist.
- Blockchain nicht größer als 256 MB.
- Halving des Mint-Prozesses dynamisch je nach Auslastung, belohnt hohe Leistung, macht Spam teuer.
- Wer viel transferiert, muss das Netzwerk mit CPU und/oder GPU unterstützen.
- Moderne, sichere Konzepte anderer Blockchains, eigene sicherere Verschlüsselung.

## Entscheidungen vom 16.10.2026

| Nr. | Thema | Entscheidung |
|---|---|---|
| 1 | Zustandsablauf | Ja, nach 1001 Tagen (ca. 2,7 Jahre), Wiederbelebung unkompliziert |
| 2 | Privater Pool | Die sicherere und kompatiblere Variante: abgeschirmter zk-Pool statt Ringsignaturen (Wahl dem Assistenten überlassen) |
| 3 | Startverteilung | Fair Launch mit 1 % offengelegter Reserve (1 Mio. TOY, höchstens 100 Wallets, für Team, Entwickler, Bug-Bounty), Reserve entsteht im Genesis-Block. Zeitsperre 4 Jahre linear (am 17.10.2026 bestätigt, siehe Nr. 10) |
| 4 | Bridge | 1:1 zu Polygon, Ethereum und weiteren Ketten. Nie mehr als 100.000.000 und nie mehr im Umlauf als auf ToyChain gemintet. ToyScan zeigt die Daten |
| 5 | Bridge-Typ | Das Sicherste: zk-Light-Client, Komitee als zweite Sicherung |
| 6 | Hybrid-Signaturen | Antwort „Alles“, umgesetzt als: ab Phase 1 optional verfügbar |
| 7 | Fremdbestand | ~~Token außerhalb der Contract-Wallet dürfen zum Start nicht gebridgt werden~~ Ersetzt durch Nr. 13 und 14 (technisch nicht durchsetzbar, siehe Spezifikation Abschnitt 12) |
| 8 | Altbestand-Verträge | Ersteller-Adresse auf Polygon wird als Admin bezeichnet. Laut Verträgen kein Nachprägen möglich, nie mehr als 100.000.000 je Chain. Verstoß löst große rote Lampe und Warnung auf ToyScan aus |

## Entscheidungen vom 17.10.2026

| Nr. | Thema | Entscheidung |
|---|---|---|
| 9 | Erste Epoche | k₀ = 0,0505 % gilt. Die erste Epoche bringt ca. **49.995 TOY** (in Spezifikation und Bauplan korrigiert, vorher irrtümlich 49.950) |
| 10 | Zeitsperre der Reserve | ~~4 Jahre linear~~ ersetzt durch Nr. 41 |
| 11 | Epochenlänge | ~~Eine Epoche endet nach 1.440 Blöcken, nicht nach Zeit.~~ **Ersetzt durch Nr. 58.** Für den 25-Jahres-Plan: 1 Jahr = 365 Epochen |
| 12 | Lizenz | MIT-Lizenz für den gesamten Code |
| 13 | Fremdbestand | Vor dem Start werden die Token außerhalb der Contract-Wallet bis auf höchstens 10.000 TOY je Kette zurückübertragen. Ersetzt Nr. 7 |
| 14 | Deckungsposten | **Bestätigt am 17.10.2026:** 10.000 TOY aus der Reserve decken im Genesis-Block den Polygon-Fremdbestand, damit jeder Token 1:1 einlösbar ist. Reserve für Team usw. dann 990.000 TOY |
| 15 | TOYPAY und Ketten | TOYPAY nimmt ToyCoin auf allen Ketten an (ToyChain, Polygon, Ethereum, später weitere) und bridgt nativen TOY |
| 16 | Spiel-NFTs | Spielgegenstände und Spielfiguren werden als NFTs auf **Polygon** geprägt, bezahlt mit Polygon-TOY. ToyChain selbst hat keine NFTs (Entscheidung 36). Details und offene Punkte (K8 bis K10) in `07-toypay.md` |
| 17 | Start der NFT-Prägung | NFTs werden erst **nach** dem Start der Bridge mit TOY geprägt (K8, Weg a). Bis zur Einzahlung in den Tresor gibt die Ersteller-Adresse keine TOY mehr aus, nur Rückübertragungen an sie sind erlaubt (ergänzt durch Nr. 44: Übertragungen an offizielle Cold-Wallets sind erlaubt) |
| 18 | NFT-Vertrag: Schutz | Kein Admin-Recht zum beliebigen Prägen. Tageslimit und Notschalter (nur Pause). Ist ein Gegenstand geprägt, gilt das NFT als Eigentumsnachweis, im Spiel gibt es ihn nicht doppelt |
| 19 | NFT-Präge-Regeln | Prägen nur mit mindestens **100 TOY** auf der Adresse. Je Adresse höchstens **5 Spielfiguren** (teils selbst, teils zufällig konfiguriert, aufwertbar) und **50 Spielgegenstände**. Pro Woche höchstens **1 Spielfigur und 10 Gegenstände**. Offene Feinheiten (Sperre der 100 TOY, Prägen oder Besitz, Aufwertung, Zufall) in `07-toypay.md` |
| 20 | Gang-Prinzip | Spielfiguren erst ab **Stufe 20** prägbar. Danach mindestens **5 Gegenstände**, erst dann die nächste Figur. So entsteht Stück für Stück eine Gang mit gemeinsamen Gegenständen und Synergien. Löst die Lootbox-Frage K9 |
| 21 | Sperre je NFT | Pro geprägtem NFT werden **10 TOY für 20 Tage** im NFT-Vertrag gesperrt und danach an den Präger zurückgegeben. Der Notschalter darf das Abholen nie blockieren |
| 22 | NFT-Feinheiten | 100 TOY Mindestbestand bleibt **zusätzlich** zur Sperre. Grenzen 5 Figuren und 50 Gegenstände gelten für das **Prägen**, gekaufte NFTs dürfen dazukommen, höchstens 5 Figuren sind gleichzeitig aktiv in der Gang. Aufwertungen werden auf der Kette festgehalten, nur steigend, ohne Extrakosten. Vor jeder neuen Figur mindestens 5 Gegenstände, die Zahl steigt nicht |
| 23 | Präge-Gebühr und Kreislauf | Die Präge-Gebühr fließt zurück ins Spiel: Belohnungen im Wert von 0,01 bis 2 TOY werden zufällig an logischen Orten verteilt und sind nur mit geprägten Spielfiguren einsammelbar. Kurs **1 TOY = 100 Dukaten = 1.000 Gold**. Offen: ob Belohnungen als echte TOY oder als Spielwerte ausgezahlt werden (K3) |
| 24 | Gas | Vorerst zahlt der Spieler das Polygon-Gas (POL). Später kann er mit Dukaten zahlen, und ein Dienst des Projekts bezahlt die Prägung |
| 25 | Dukaten und TOY | **Einbahnstraße (A):** TOY → Dukaten möglich, Dukaten und Gold nie zurück in TOY. Belohnungen im Spiel nur als Spielwerte (Dukaten, Gold, Gegenstände) |
| 26 | Präge-Gebühren | Spielfigur **1 TOY**. Gegenstände **1 bis 100 TOY** je nach Wert (z. B. Flöte 1, Dolch 2, Kamel 5, Boot bis 25, schönes Haus 100). Gebühren gehen an eine Multisig-Gebühren-Adresse des Projekts auf Polygon |
| 27 | Gegenstands-Katalog | 500 prägbare Gegenstände (5 Beispiele + 495 neue) in 16 Kategorien mit Gebühr 1–100 TOY, Seltenheit und Boni. Quelle `toypay/data/nft_items.tsv`, Beschreibung `toypay/docs/06_NFT_GEGENSTAENDE.md`. Boni und Balance sind Startwerte |
| 28 | Zwei Sorten Dukaten | **Sultansdukaten**: nur gegen TOY in der Wechselstube, dürfen Präge-Gebühren zahlen, nicht übertragbar. **Königsdukaten** (und Gold): erspielt, nur für Dinge im Spiel, keine Präge-Gebühren. Kein Tausch zwischen den Sorten, keine Sorte zurück in TOY |
| 29 | Altersgruppe | TOYPAY und Spiele **ab 16 Jahren**. Offen: TOY-Funktionen ab 18 oder mit Zustimmung der Eltern (K4 in `07-toypay.md`) |
| 30 | Plattformen TOYPAY | **Android, Windows, Linux, Raspberry Pi**. Kein iPhone vorerst. Vorschlag: eine Codebasis mit Compose Multiplatform und dem gemeinsamen Rust-Kern |
| 31 | Alter für TOY-Funktionen | Spiele ab 16. Wechselstube, Sultansdukaten, Prägen, Bridge und NFT-Handel **ab 18 mit Altersnachweis** |
| 32 | Apps getrennt | **TOYPAY Wallet** (Wallet und Knoten, 4 Plattformen, ab 18) getrennt von den Spiel-Apps (2D-Spiele und TOYQUEST 3D, ab 16). Verbindung nur über die Wallet-Adresse, nie über Schlüssel |
| 33 | Auflage Sternenfeuer | Jede Gegenstandsart der Seltenheit Sternenfeuer (80–100 TOY) gibt es höchstens **1.001-mal**. Der NFT-Vertrag leitet die Grenze aus der Gebühr ab, ohne Admin |
| 34 | Gold | Gold gehört zur erspielten Sorte: 10 Gold = 1 Königsdukat |
| 35 | Königsdukaten verschenken | Königsdukaten und Gold dürfen zwischen Spielern verschenkt und im Spiel gehandelt werden. Sultansdukaten bleiben an das Konto gebunden |
| 36 | Nur nativer Coin | Auf ToyChain gibt es **nur TOY**: keine Token, keine NFTs, keine Smart Contracts, keine EVM-Ausführung, **auch nicht später**. Alle Funktionen sind fest eingebaute Transaktionsarten (Spezifikation Abschnitt 6a). Verträge gibt es nur auf anderen Ketten (Tresore, NFT-Vertrag auf Polygon und Ethereum) |
| 37 | Start-Validatoren | Offengelegtes **Start-Komitee** aus 7 bis 21 Betreibern. Es validiert mit delegiertem, weiter gesperrtem Reserve-Stake. Reserve-Stake bekommt **keine Emission** (Anteil geht in den Erstattungs-Pool). Ab Epoche 1 verdienen alle über ToyWork und Attestierung. Übergabe, sobald unabhängiger Stake z. B. das Dreifache erreicht |
| 38 | Mainnet-Zeitpunkt | Mainnet **nach Phase 1 und Audit**. Bis zur Beweis-Schicht starten Knoten von veröffentlichten Checkpoints. Phase 2 muss innerhalb von etwa 12 Monaten folgen |
| 39 | Bridge vorziehen | Neue Reihenfolge: Phase 2 Beweise, **Phase 3 Bridge**, Phase 4 Privatsphäre, Phase 5 Offline-Schecks |
| 40 | Chain-ID | Vor dem Testnet wird eine freie Chain-ID gewählt und gegen die öffentlichen Listen geprüft. Testnet und Mainnet bekommen verschiedene IDs |
| 41 | Reserve-Empfänger und Sperre | Die 990.000 TOY Reserve gehen an **eine** Adresse: die Ersteller-Adresse des Ethereum-Tokens `0x18da0be974424d54519c63f62e4b26c0318c00c2`. In den ersten **3 Jahren** nach dem Mainnet-Start wird nichts freigegeben, danach **schrittweise**. (Freigabeplan festgelegt in Nr. 42: je Epoche über 3 Jahre.) Ersetzt Nr. 10 und die Verteilung auf bis zu 100 Wallets aus Nr. 3 |
| 42 | Freigabe der Reserve | Nach den 3 Jahren Sperre wird **je Epoche gleichmäßig über 3 Jahre** freigegeben (rund 822 TOY pro Epoche), voll frei nach 6 Jahren. Begründung: nur 3 bis 5 % der täglich neuen TOY, fair für spätere Teilnehmer (vom Projektinhaber Claude überlassen) |
| 43 | Bug-Bounty-Topf | **90.000 TOY** der Reserve sind sofort verfügbar, auf einem offiziellen Mehrfachsignatur-Konto (z. B. 3 von 5) nur für Prämien. Die gesperrte Reserve beträgt damit 900.000 TOY |
| 44 | Offizielle Cold-Wallets | Große Bestände liegen auf **offiziellen, öffentlich bekannten Cold-Wallets**: die 900.000 TOY Reserve auf ToyChain und die Altbestand-Token auf Polygon und Ethereum bis zur Tresor-Einzahlung. Die Zugehörigkeit zum Ersteller wird mit signierten Nachrichten der Ersteller-Adresse belegt. Übertragungen von der Ersteller-Adresse an diese Cold-Wallets sind erlaubt, sie zählen nicht zum Fremdbestand |
| 45 | Mitbestimmung | Die Community stimmt über Parameter und Protokoll-Updates ab. Unveränderbar bleibt eine **Verfassung** (u. a. Obergrenze, Emissionsformel, Reserve, nur nativer Coin, Bridge-Deckung, kein Zugriff auf fremde Guthaben). Ausgestaltung: `09-mitbestimmung.md` |
| 46 | Auflage je Saison (E6, Option D) | Je Gegenstandsart und Saison (101 Nächte) höchstens **1.001 Rubinglut** und **101 Sternenfeuer**. Für Sternenfeuer gilt weiter die Gesamtgrenze 1.001 je Art (Nr. 33). Der NFT-Vertrag zählt Saisons ab einem festen Startzeitpunkt, ohne Admin |
| 47 | Mitbestimmung im Detail | Stimmgewicht √(gebundener Stake), Deckel beim Gewicht von 10.000 TOY. Quorum 20 %, Fristen 7 + 7 + 14 Epochen. Reserve und Start-Komitee ohne Stimmrecht |
| 48 | Tresor-Umzug (B1) | Tresore ziehen **nie** um. Bei einem Fehler: pausieren, neuer Tresor nur für neue Einzahlungen |
| 49 | Bridge-Limits (B2) | 50.000 TOY je Kette, Richtung und Epoche. Beträge über 10.000 TOY 24 Stunden verzögert. Änderbar per Mitbestimmung |
| 50 | Takt der Beweise (B4) | Zunächst Polygon **alle 12 Stunden**, Ethereum **alle 12 Monate** (Ethereum geändert durch Nr. 52). Änderbar per Mitbestimmung |
| 51 | Bridge-Gebühr (B5) | **Gratis** im kostenlosen Kontingent, darüber **höchstens 1 TOY**. Mindestbetrag 10 TOY gegen Spam. Gas zahlt der Relayer aus der Gebühren-Adresse auf Polygon |
| 52 | Ethereum-Bridge später (B4) | Zum Bridge-Start **nur Polygon**. Die Ethereum-Bridge ist eingebaut, aber ausgeschaltet. Sie startet erst nach einer Abstimmung (Protokoll-Update, 2/3-Mehrheit), die auch den Takt festlegt (Vorschlag: alle 12 Monate). Bis dahin bleiben die Ethereum-Token auf der offiziellen Cold-Wallet |
| 53 | Bridge-Komitee (B3) | (Start-Besetzung geändert durch Nr. 59) Start mit **Projekt-Komitee 5 von 9 plus Wächtern** (nur Pausieren). Ab Stufe 1 der Mitbestimmung jeden Monat ein Platz an einen gewählten unabhängigen Betreiber mit 10.000 TOY Stake, bis alle 9 gewählt sind |
| 54 | Einzahlungen von Ethereum (B6) | nach der Freischaltung im 12-Stunden-Takt wie bei Polygon |
| 55 | Bug-Bounty-Prämien (T1) | Prämien über 5.000 TOY erzwingt das **Protokoll**: Die Auszahlung muss auf einen angenommenen Vorschlag mit gleichem Empfänger und Betrag verweisen. Auszahlungen an denselben Empfänger in 30 Epochen zählen zusammen |
| 56 | ToyScan ohne Schlüssel (T2) | ToyScan pausiert nie selbst. Bei Rot alarmiert es die Wächter (Nr. 53) |
| 57 | Altbestand-Wächter zuerst (T3) | ToyScan Stufe 0 wird als erster Baustein vor M1 gebaut (`toyscan/`, nur lesend) |
| 58 | Epoche nach Blöcken **und** Zeit (P1) | Eine Epoche endet erst nach mindestens **1.440 Blöcken und mindestens 24 Stunden** (geprüfte Zeitstempel). Ersetzt Nr. 11. Grund: Sonst könnten Blockerzeuger mit schnellen Blöcken die Zeit stauchen (Reserve-Sperre, Fristen, Limits, Emission). Neu in der Verfassung |
| 59 | Bridge-Sicherheit (P2) | Bridge-Start erst, wenn **mindestens 5 der 9** Komitee-Plätze mit Betreibern außerhalb von Projekt und Start-Komitee besetzt sind. **Pfand 20.000 TOY je Mitglied auf Polygon**, das der Tresor bei nachgewiesener falscher Bestätigung selbst einzieht. Limit 50.000 TOY je Kette, Richtung und Epoche (eine Epoche dauert mindestens 24 Stunden). Mindestens 3 Wächter, **jeder allein** darf pausieren, Fortsetzen nur mit 5 von 9, kein automatisches Fortsetzen. **Frist von 30 Tagen** für jede Überweisung, danach Rückholung auf der Herkunftskette in beide Richtungen, ohne doppelte Auszahlung. Ändert Nr. 53 (Start-Besetzung) |
| 60 | Mengenregel I0 (P3) | Die **Summe aller Guthaben** ist in jedem Block genau gemintet + 1 Mio. − verbrannt, sonst ist der Block ungültig. Jede Wiederbelebung nur einmal (Einmal-Markierung). Staub und gekürzter Stake werden **verbrannt**. Restmenge = 99 Mio. − Summe aller Emissionen, Verbranntes kommt nie zurück |

## Prüfergebnis Altbestand-Verträge (Bytecode ausgewertet)

- Ethereum (Chain-ID 1): 0xA5B04ce6eDD139C0e58ccdBc0B75E8C67CDA678A
- Polygon (Chain-ID 137): 0x1854b69cd1495c9753c5F1590A35e5DdB556A5BF
- Beide: Standard-ERC-20 mit Permit (14 Funktionen), kein Mint, kein Burn, kein Owner/Admin, kein Pausieren, keine Sperrliste, kein Upgrade, kein Proxy. Details in `04-toyscan-und-altbestand.md`.

## Noch offen



- Quellcode auf Etherscan und Polygonscan verifizieren (am 17.10.2026 auf beiden Ketten noch **nicht verifiziert**). Prägung von je 10^26 Einheiten ist bestätigt.
- Schlüssel der Contract-Wallet `0x18da…00c2`: Wer hält ihn, wie ist er gesichert? Liegt der ursprüngliche Quellcode der Verträge vor?
- TOYPAY: Grundsatzfragen K1 bis K7 in `07-toypay.md` (Spielwährung oder Kryptowährung, Poker, Umtausch Dukaten ↔ TOY, Jugendschutz, Verwahrung, Datenschutz, eine oder zwei Apps).
- Rechtliche Prüfung vor dem Mainnet (MiCA, Steuer, Börsenlistings, 1 % Reserve, Fremdbestand).
- Technisch: Beweisprüfung auf Ethereum (Kosten, Trusted-Setup-Frage), in Phase 2 klären.
