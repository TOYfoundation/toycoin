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
- Vorerst keine Layer-2-Token, keine NFTs, keine eigenen Smart Contracts.
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
| 1 | Zustandsablauf | Ja, nach 1001 Tagen (ca. 3 Jahre), Wiederbelebung unkompliziert |
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
| 10 | Zeitsperre der Reserve | 4 Jahre linear, auf der Kette erzwungen (Vorschlag bestätigt) |
| 11 | Epochenlänge | Eine Epoche endet nach 1.440 Blöcken, nicht nach Zeit. Schnellere Epochen gleicht der Zeitplan-Regler über m aus. Für den 25-Jahres-Plan: 1 Jahr = 365 Epochen |
| 12 | Lizenz | MIT-Lizenz für den gesamten Code |
| 13 | Fremdbestand | Vor dem Start werden die Token außerhalb der Contract-Wallet bis auf höchstens 10.000 TOY je Kette zurückübertragen. Ersetzt Nr. 7 |
| 14 | Deckungsposten | **Bestätigt am 17.10.2026:** 10.000 TOY aus der Reserve decken im Genesis-Block den Polygon-Fremdbestand, damit jeder Token 1:1 einlösbar ist. Reserve für Team usw. dann 990.000 TOY |
| 15 | TOYPAY und Ketten | TOYPAY nimmt ToyCoin auf allen Ketten an (ToyChain, Polygon, Ethereum, später weitere) und bridgt nativen TOY |
| 16 | Spiel-NFTs | Spielgegenstände und Spielfiguren werden als NFTs auf **Polygon** geprägt, bezahlt mit Polygon-TOY. ToyChain selbst bleibt in Phase 1 ohne NFTs. Details und offene Punkte (K8 bis K10) in `07-toypay.md` |

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
