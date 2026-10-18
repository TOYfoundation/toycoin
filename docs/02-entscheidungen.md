# Entscheidungsprotokoll ToyChain

Stand: 18.10.2026. Alle Punkte stammen aus den Vorgaben des Projektinhabers. Mit „zur Bestätigung“ markierte Punkte sind Vorschläge von Claude, die noch bestätigt werden müssen.

## Neuausrichtung vom 18.10.2026 (gilt vor allen älteren Einträgen)

Das Projekt wird schlanker, unabhängiger und dezentraler. Priorität hat ein **Testnet** aus ToyChain, TOYPAY Wallet und ToyScan. Die Konzepte, die dadurch wegfallen oder später kommen, liegen in `docs/spaeter/`.

| Nr. | Thema | Entscheidung |
|---|---|---|
| 61 | Fokus | Kern sind **ToyChain mit nativem TOY**, die **TOYPAY Wallet** als Knoten mit Mint-Funktionen und Unterstützung anderer Ketten, und **ToyScan** mit Warnampel und Anzeige anderer Ketten. ToyChain läuft zuerst als **Testnet**, Funktionen kommen schrittweise dazu |
| 62 | Totaler Fair Launch | **Genesis-Block ohne Guthaben.** Kein Premint, keine Reserve, kein Team-Anteil, kein Bug-Bounty-Topf aus der Kette. **Alle 100 Mio. TOY** entstehen per Emission (erste Epoche 50.500 TOY). Steht in der Verfassung |
| 63 | ToyCoin-Token auf Polygon und Ethereum | Werden **ignoriert**. Keine 1:1-Bridge, keine Deckung, kein Altbestand-Wächter. Wallet und ToyScan kennzeichnen gleichnamige Token als „Nicht ToyChain-TOY, nicht eintauschbar“ |
| 64 | Mitbestimmung | **Später.** Die Verfassung bleibt und gilt schon jetzt (Spezifikation Abschnitt 13) |
| 65 | Teilnahme-Stufen | Stufe 0 braucht **kein Guthaben und keinen Stake**. Mehr Rechte gibt es gestaffelt mit Stake: 10 / 100 / 1.000 TOY (Startwerte, Spezifikation Abschnitt 4) |
| 66 | Energie-Modus | Die Wallet passt ihren Aufwand automatisch an Akku, Ladegerät, Temperatur, Netz und Nutzung an. Der Nutzer wählt die Obergrenze und die Stufe (`04-toypay-wallet.md`) |
| 67 | Schutzregeln und Sperrzeiten | **Bestätigt am 18.10.2026:** drei Ebenen. Die Kette sperrt nur bei festen, eindeutig nachweisbaren Verstößen, und zwar nur Rechte, nie Guthaben. Das Netzwerk drosselt lokal. ToyScan erkennt Muster und warnt nur (Spezifikation Abschnitt 8) |
| 68 | Mint-Funktionen | TOY verdienen über Arbeits-Lotterie, Mitsignieren und Validieren. **NFT-Prägen kommt später mit der Spiel-Anbindung** (bestätigt am 18.10.2026, siehe Nr. 72). Die Testnet-Wallet prägt keine NFTs |
| 69 | Andere Ketten | Dieselbe Adresse wird auf EVM-Ketten angezeigt und genutzt (Ethereum, Polygon, Arbitrum, Base, BNB Chain, eigene hinzufügbar). Keine Bridge. Nicht-EVM-Ketten später |
| 70 | Startphase ohne Stake | **Bestätigt am 18.10.2026:** 7 bis 21 Start-Validatoren im Genesis-Block, ohne Stake, **mit Lohn aus dem Konsens-Anteil** zu gleichen Teilen, begrenzt durch Nr. 78. Automatische Übergabe, sobald 21 Validatoren mit je 1.000 TOY Stake zusammen mindestens 1 % der geminteten TOY binden. Ersetzt Nr. 37 |
| 71 | Arbeits-Lotterie | ToyWork-Lose mit Schwierigkeits-Schwelle. Gewinner je Block per VRF, jeder Gewinn mindestens 0,2 TOY (geändert durch Nr. 73). So kann auch ein Gerät ohne Konto sein erstes TOY verdienen, und es entsteht kein Staub |
| 73 | Konten aktivieren und auflösen | Konten gehen **nie verloren**. Beim ersten Eingang werden **0,1 TOY als Aktivierungs-Pfand** gebunden, der Rest ist frei. Eingang an eine neue Adresse mindestens 0,1 TOY, Lotterie-Gewinn mindestens 0,2 TOY. Auflösen nur auf Wunsch: alles samt Pfand an eine Zieladresse oder als Spende in den Erstattungs-Pool. Rechte (Kontingent, Stufe) sind nicht übertragbar. Ersetzt das Mindestguthaben mit Entfernen |
| 74 | Nichts wird verbrannt | Auf ToyChain wird **nie TOY verbrannt**. Gekürzter Stake bei Doppelsignatur geht in den Erstattungs-Pool. Regel I0: Summe aller Guthaben = Summe aller Emissionen. Ändert Nr. 60 |
| 75 | 100 Mio. Konten | **Aktive und ruhende Konten:** Nur die zuletzt genutzten Konten (ca. 2 Mio.) liegen im 256-MB-Speicher. Ab 90 % Füllung werden die am längsten unbenutzten ausgelagert, aber nie gelöscht. Ruhe-Schnappschuss je Epoche, Beweis je Konto knapp 1 KB, automatisches Aufwecken. Ersetzt den festen Ablauf nach 1001 Tagen (Nr. 1). Phase 2: Umstieg auf Betrieb ganz ohne Kontenspeicher prüfen. Notfallplan: größerer SSD-Speicher für Voll-Knoten |
| 76 | Pfand senkbar | Das Aktivierungs-Pfand ist ein Parameter zwischen 0,01 und 0,1 TOY und darf nur sinken. Bestehende Konten können den Unterschied dann freigeben |
| 77 | Lizenz MPL-2.0 | Für **unverändert** genutzte Bibliotheken ist MPL-2.0 erlaubt, z. B. UniFFI für die Anbindung der Wallet an den Rust-Kern. Eigener Code bleibt MIT |
| 78 | Schutz in der Startphase | (1) Für die Übergabe zählen **nur neue Validatoren** und nur deren Stake. (2) Der Lohn der Start-Validatoren **sinkt über 180 Epochen von 50 % auf 0 %**, der Rest geht in die Arbeits-Lotterie, zusammen höchstens rund 2,2 Mio. TOY. (3) Der Lohn wird sofort **365 Epochen als Stake gebunden** |
| 79 | Spenden an den Erstattungs-Pool | Jeder kann **jederzeit aus der Wallet** TOY in den Erstattungs-Pool spenden, der die kostenlosen Transaktionen bezahlt. Eigene Transaktionsart, nicht rückholbar, in ToyScan sichtbar |
| 80 | Doppelsignatur | **Strafe nach Ausmaß:** sofort 7 Epochen Rollensperre, nach 18 Epochen Kürzung `max(1 %, 3 × Anteil der gleichzeitig doppelt signierenden am Stake)`, höchstens 100 %. Pannen kosten wenig, Angriffe alles. **Pflicht-Schutz in der Software:** Signier-Gedächtnis, Doppelgänger-Erkennung (2 Epochen), ein Signier-Gerät je Konto mit eigenem Schlüssel je Gerät |
| 81 | Formel für m | Leistungsteil aus **Nutzung und Beteiligung je zur Hälfte** (höchstens ±0,25), Zeitplanteil nach **echter Zeit** mit **mittlerer Stärke** (Faktor 10, Ausgabe höchstens ca. ±2,5 % vom Plan), Änderung höchstens 0,05 je Epoche, alles in Millionsteln (Spezifikation 9.1) |
| 82 | Wallet-Schutz beim Senden und Zugang | Vorschau vor jeder Signatur, Schutz vor Adress-Fallen, Adressbuch, Probebetrag bei neuer Adresse, Hardware-Wallets, Phrase in Teile (SLIP-39), Nur-Ansehen-Konten. **Nicht gewählt:** Passkey/Biometrie |
| 83 | Konto-Schutz auf der Kette | Tresor-Verzögerung, Notfall-Schlüssel mit Sicherheitsadresse, Selbst-Sperre, Erben-Adresse (Totmannschalter). Nur vom Eigentümer selbst setzbar, Änderungen wirken verzögert. **Nicht gewählt:** Wiederherstellung über Vertraute |
| 84 | Zahlungs-Funktionen | Verwendungszweck (32 Byte), Zahlung per QR/Link, Sammelüberweisung (bis 32 Empfänger), Ablaufzeit für jede Transaktion |
| 85 | Komfort | Gebühr übernehmen (zweiter Unterzeichner zahlt), hängende Transaktion ersetzen oder abbrechen, Daueraufträge in der Wallet, CSV-Export |
| 86 | Andere Ketten | Atomarer Tausch per Hash-Zeitschloss (SHA-256) ohne Bridge, WalletConnect, lesbare Signier-Anfragen (EIP-712), Warnung bei Token-Freigaben |
| 87 | Netzwerk und Software | Dandelion++ schon im Testnet, optional Tor (später I2P), Checkpoints mit Signaturen von mindestens 3 unabhängigen Stellen, reproduzierbare Builds mit mindestens 3 unabhängigen Signaturen vor jedem Update |
| 88 | Projekt-Sicherheit | Testnet-Kran aus Lotteriegewinnen eigener Knoten, Dauer-Fuzzing ab M1, Bug-Bounty-Programm außerhalb der Kette, öffentlicher Notfall-Plan |
| 89 | Konten und App-Schutz (MetaMask) | Mehrere Konten und mehrere Phrasen, Schlüssel-Import und -Anzeige nur mit Passwort, Sicherungs-Quiz, Phishing-Warnliste (öffentlich), Sicherheitswarnungen auf dem Gerät, automatische Sperre, Guthaben verbergen |
| 90 | Web-Apps (MetaMask) | Seiten-Berechtigungen, Anmelden mit Adresse (EIP-4361), Browser-Erweiterung für Chrome und Firefox, eingebauter Browser in der Handy-App |
| 91 | Netzwerke, Gebühren, Überblick (MetaMask) | Netzwerk per Klick (EIP-3085, Eintrag in Chainlist), Token- und NFT-Erkennung, eigener Knoten wählbar, Gebühren-Stufen, Simulation vor dem Senden auf anderen Ketten, Gesamtübersicht in Euro (abschaltbar), Mitteilungen, verständlicher Verlauf, Ende-zu-Ende verschlüsselter Sync ohne Schlüssel |
| 92 | Nicht übernommen (MetaMask) | Erweiterungen (Snaps), Tauschen über Börsen-Dienste. Schutz vor Vordrängeln siehe Nr. 93 |
| 93 | Schutz vor Vordrängeln und Zensur | Auf ToyChain: **feste Reihenfolge** im Block (Gebühr, dann Block-Zufall), **Aufnahme-Pflicht** für Transaktionen, die mindestens ein Drittel des Komitees meldet, **verschlüsselter Wartebereich** in Phase 2. Für andere Ketten siehe Nr. 94 |
| 94 | Vordrängel-Schutz auf anderen Ketten | **Geschützter Sendeweg standardmäßig an** (abschaltbar, Rückfrage vor dem Ausweichen), **Anbieter wählbar** oder eigener, **Rückerstattungen** an den Nutzer, **Warnung bei Preisabweichung** ab 2 % mit Anzeige des Mindestbetrags, **begrenzte Freigaben** statt unbegrenzter |
| 72 | Spiel, Spiel-NFTs, Dukaten | **Später** mit der Spiel-Anbindung. Die bisherigen Regeln bleiben als Ideensammlung in `spaeter/07-toypay-spiel-und-nft.md` und `toypay/docs/` |

**Was mit den älteren Entscheidungen passiert:**

| Gilt weiter | Kommt später | Entfällt |
|---|---|---|
| 1 Zustandsablauf (ersetzt durch 75), 9 k₀ (erste Epoche jetzt 50.500 TOY), 12 MIT-Lizenz, 30 Plattformen, 32 Wallet und Spiele getrennt, 36 nur nativer Coin, 38 Mainnet erst nach Audit, 40 eigene Chain-ID, 56 ToyScan ohne Schlüssel, 58 Epoche nach Blöcken und Zeit, 60 Regel I0 (jetzt: Summe = gemintet, siehe 74) | 2 privater Pool, 5 Bridge-Typ, 6 Hybrid-Signaturen, 15 TOY auf anderen Ketten bridgen, 16 bis 29, 31, 33 bis 35 und 46 Spiel und NFT-Regeln, 39 Bridge-Phase, 45 und 47 Mitbestimmung, 48 bis 54 und 59 Bridge-Details, 55 Bug-Bounty-Prämien | 3, 8, 10, 41 bis 44 Reserve und Cold-Wallets, 4, 7, 13, 14 Bridge-Deckung und Fremdbestand, 11 (ersetzt durch 58), 37 (ersetzt durch 70), 57 Altbestand-Wächter |

**Die folgenden Abschnitte sind das Protokoll bis zum 17.10.2026.** Was dort steht, gilt nur, soweit die Tabelle oben es nicht ändert.

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

Nur noch Hintergrund: Die Token werden derzeit ignoriert (Nr. 63). Details in `spaeter/04-toyscan-und-altbestand.md`.

- Ethereum (Chain-ID 1): 0xA5B04ce6eDD139C0e58ccdBc0B75E8C67CDA678A
- Polygon (Chain-ID 137): 0x1854b69cd1495c9753c5F1590A35e5DdB556A5BF
- Beide: Standard-ERC-20 mit Permit (14 Funktionen), kein Mint, kein Burn, kein Owner/Admin, kein Pausieren, keine Sperrliste, kein Upgrade, kein Proxy. Details in `04-toyscan-und-altbestand.md`.

## Noch offen (Stand 18.10.2026)

- Schutz vor teurem Auffüllen des Speichers (P5, M5): Durch das Auslagern nach Bedarf (Nr. 75) kann niemand den Speicher dauerhaft verstopfen. Massenhaft neue Konten verdrängen nur alte in den Ruhezustand. Die Schutzregel gegen massenhaftes Anlegen bleibt.
- Aus dem Prüfbericht für die passenden Meilensteine: P5, P6 (M5), P9 und Höchstwert der progressiven Gebühr (M3).
- Rechtliche Prüfung vor einem Mainnet (z. B. MiCA, Steuer).
