# ToyScan-Kriterien und Analyse der Altbestand-Verträge

## 1. ToyScan-Ampel (live)

ToyScan zeigt für jede Kette den Zustand der Supply-Regeln. **Grün**, wenn alle Kriterien erfüllt sind, **gelb** bei Abweichung, **rot** bei Verletzung.

1. Umlauf ≤ gemintetes TOY + Reserve ≤ 100.000.000.
2. Die Tresor-Verträge halten mindestens den nicht freigegebenen Rest.
3. Reserve höchstens 1.000.000 TOY in höchstens 100 Wallets, Zeitsperre intakt.
4. Emission nicht vor dem Zeitplan.
5. Altbestand-Verträge ohne Mint-Funktion und ohne Upgrade-Mechanismus, Gesamtmenge je Kette ≤ 100.000.000, Tresor ohne Admin-Schlüssel.
6. Notschalter der Bridge nicht ausgelöst, Beweis-Verifizierer aktuell.
7. Fremdbestand je Kette ausgewiesen und seine Einlösung gesperrt.

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
- Fremdbestand erzeugt nie neues TOY. Vorschlag für später: Rücktausch nur im Rahmen zuvor aus dem Tresor ausgegebener Token. Halter offen informieren, dass eine 1:1-Einlösung nicht garantiert ist.
