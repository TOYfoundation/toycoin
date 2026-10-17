# ToyScan-Kriterien und Analyse der Altbestand-Verträge

## 1. ToyScan-Ampel (live)

ToyScan zeigt für jede Kette den Zustand der Supply-Regeln. **Grün**, wenn alle Kriterien erfüllt sind, **gelb** bei Abweichung, **rot** bei Verletzung.

1. Umlauf ≤ gemintetes TOY + Reserve ≤ 100.000.000.
2. Die Tresor-Verträge halten mindestens den nicht freigegebenen Rest.
3. Reserve höchstens 1.000.000 TOY in höchstens 100 Wallets, Zeitsperre intakt.
4. Emission nicht vor dem Zeitplan.
5. Altbestand-Verträge ohne Mint-Funktion und ohne Upgrade-Mechanismus, Gesamtmenge je Kette ≤ 100.000.000, Tresor ohne Admin-Schlüssel.
6. Notschalter der Bridge nicht ausgelöst, Beweis-Verifizierer aktuell.
7. Fremdbestand je Kette ausgewiesen und nicht größer als der Deckungsposten dieser Kette (Polygon 10.000 TOY, Ethereum 0). Ist er größer, Gelb. Wächst er nach dem Start, Rot (dann wurden Token an Tresor und Bridge vorbei bewegt).

**Große rote Lampe mit Warnung**, wenn auf irgendeiner Kette mehr als 100.000.000 ToyCoin festgestellt werden oder sich nachprägen ließe. Der Notschalter der Bridge kann dann Freigaben pausieren.

Hinweise für die Umsetzung:
- Alle Regeln sind offen dokumentiert, jeder soll sie selbst nachrechnen können. Die Ampel ersetzt keine eigene Prüfung.
- Fremdbestand je Kette = Gesamtmenge − Tresor-Bestand − Bestand der Contract-Wallet (und ggf. Burn-Adresse).
- Der Node stellt Daten maschinenlesbar bereit: Gesamtmenge, gemintet, Reserve, Epoche, Emissionsfaktor m.

## 2. Altbestand-Verträge

| | Polygon | Ethereum |
|---|---|---|
| Adresse | 0x1854b69cd1495c9753c5F1590A35e5DdB556A5BF | 0xA5B04ce6eDD139C0e58ccdBc0B75E8C67CDA678A |
| Chain-ID (im Code fest hinterlegt) | 137 | 1 |
| Compiler laut Metadaten | Solidity 0.8.7 | Solidity 0.8.7 |
| Gemeldete Menge | 100.000.000 | 100.000.000 |

Die Adresse und die Chain-ID stehen jeweils als feste Werte im Code. Das bestätigt, dass der ausgewertete Bytecode zur jeweiligen Adresse und Kette gehört.

### Funktionsliste (14 Funktionen, identisch auf beiden Ketten)

| Selector | Funktion |
|---|---|
| 06fdde03 | name() |
| 095ea7b3 | approve(address,uint256) |
| 18160ddd | totalSupply() |
| 23b872dd | transferFrom(address,address,uint256) |
| 313ce567 | decimals() (fest 18) |
| 3644e515 | DOMAIN_SEPARATOR() |
| 39509351 | increaseAllowance(address,uint256) |
| 70a08231 | balanceOf(address) |
| 7ecebe00 | nonces(address) |
| 95d89b41 | symbol() |
| a457c2d7 | decreaseAllowance(address,uint256) |
| a9059cbb | transfer(address,uint256) |
| d505accf | permit(address,address,uint256,uint256,uint8,bytes32,bytes32) |
| dd62ed3e | allowance(address,address) |

### Ergebnis

- Standard-ERC-20 mit Permit (EIP-2612).
- **Kein** Mint, Burn, Owner, Admin, Pausieren, Sperrliste, Upgrade, Proxy.
- Die Ersteller-Adresse hat im Code keine Sonderrechte, sie hält nur den Bestand.
- Dispatcher kennt nur diese 14 Funktionen, kein Fallback.

### Einschränkungen dieser Prüfung

- Ausgewertet wurde die Funktionsliste im Bytecode (Laufzeitcode). Eine maschinelle Prüfung auf SELFDESTRUCT und DELEGATECALL im gesamten Code steht noch aus und sollte vor dem Mainnet erfolgen.
- Die Gesamtmenge steht nicht im Laufzeitcode, sondern wurde beim Erstellen festgelegt. Sie muss am Vertrag abgelesen werden: `totalSupply()` soll `100000000000000000000000000` ergeben (100.000.000 × 10^18).
- Der Quellcode soll auf Etherscan und Polygonscan als „Verified“ erscheinen.

### ToyScan-Prüfungen am Altbestand (laufend)

- Selector-Menge im Vertrag entspricht exakt der Liste oben.
- Code-Hash an der Adresse unverändert. Alarm bei jeder Änderung.
- `totalSupply()` ≤ 100.000.000 × 10^18 je Kette.
- Verteilung: Contract-Wallet, Tresor, Fremdbestand, Burn-Adresse.

## 3. Tresor-Modell (Vorschlag, zur Bestätigung)

- Was in der Contract-Wallet liegt, wird in einen Tresor-Vertrag übertragen. Der Tresor hat keinen Admin, keine Mint-Funktion und keine manuelle Entsperrung.
- Freigabe nur gegen einen verifizierten ToyChain-Beweis, 1:1. Zusätzlich Komitee-Schwellensignatur, bis der Verifizierer bewährt ist.
- Schutz: Limits, Zeitverzögerung bei großen Beträgen, Notschalter (kann nur pausieren).
- Globale Invariante: Umlauf auf allen Ketten ≤ gemintetes TOY + 1 Mio. Reserve ≤ 100.000.000.
- Fremdbestand erzeugt nie neues TOY. Er wird vor dem Start auf höchstens 10.000 TOY je Kette verringert (Entscheidung 13) und im Genesis-Block durch einen Deckungsposten aus der Reserve gedeckt (Vorschlag). Dann ist jeder Token 1:1 einlösbar.

## 4. On-Chain-Befund vom 17.10.2026 (Etherscan-API, nur lesend)

**Herkunft:** Beide Verträge wurden von derselben Adresse erstellt: `0x18da0be974424d54519c63f62e4b26c0318c00c2` (Ersteller, im Folgenden „Contract-Wallet“). Polygon am 15.07.2022 (Block 30.731.802), Ethereum am 23.07.2022 (Block 15.198.513). Beim Erstellen wurden je **100.000.000 TOY** (10^26 Einheiten, 18 Dezimalstellen) an die Ersteller-Adresse geprägt. Name „ToyCoin“, Symbol „TOY“.

**Quellcode:** Auf beiden Ketten **nicht verifiziert** (Stand 17.10.2026). Ampel-Kriterium 5 kann erst grün werden, wenn der Quellcode verifiziert ist.

**Ethereum:** Die Ersteller-Adresse hat seit der Prägung **keinen einzigen Transfer** ausgeführt. Sie hält alle 100.000.000 TOY. Fremdbestand: **0**.

**Polygon:** Die Ersteller-Adresse hat netto **2.001.000 TOY** abgegeben und hält **97.999.000 TOY**. Wohin der Rest ging:

| Adresse | Bestand (errechnet) | Herkunft |
|---|---|---|
| `0xd908efed6e0c9ee1dea7cb9555c00d2eb57c3546` | 999.000 | 1.000.000 vom Ersteller (03.12.2023), 1.000 weiter an `0x6b9a…` |
| `0x6b9a6dc8c7eedd3f0493a473c9d9752677aa0c99` | 895.111,4 | 894.111,4 vom Ersteller (06.12.2023), 1.000 von `0xd908…` |
| `0x856d3b7ac3e329d65268160fddc867bcb92805e7` | 99.200 | 100.400 vom Ersteller, 1.200 weitergegeben |
| übrige (Liquiditätspool von 07/2022, kleine Wallets) | ca. 7.688,6 | Summe der restlichen Abgänge |

**Folgerung:** Gehören die drei großen Adressen dem Projekt, liegt der echte Fremdbestand bei nur **ca. 7.700 TOY auf Polygon und 0 auf Ethereum**. Das ist für das Bridge-Konzept entscheidend (siehe `05-gesamtsystem.md`, sobald angelegt).

**Entscheidung vom 17.10.2026:** Die Token werden vor dem Start bis auf höchstens 10.000 TOY je Kette an die Contract-Wallet zurückübertragen. Auf Polygon bleiben damit die ca. 7.700 TOY in fremden Händen plus höchstens ca. 2.300 TOY, die bei den drei großen Adressen verbleiben dürfen.

**Warum klein allein nicht reicht:** ERC-20-Token sind austauschbar. Der Tresor kann alte nicht von freigegebenen Token unterscheiden. Ungedeckte 10.000 TOY könnten genau so viele TOY abziehen, die anderen Bridge-Nutzern gehören. Grün ist die Ampel deshalb erst, wenn der Fremdbestand gedeckt ist (Deckungsposten, siehe Abschnitt 3).

Einschränkung: Bestände sind aus den Transfer-Listen errechnet, nicht per `balanceOf()` abgelesen. Vor jeder Entscheidung nochmals direkt am Vertrag prüfen.
