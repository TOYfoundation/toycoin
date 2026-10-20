# Testvektoren für toy-tx (M3)

Abgerufen am 20.10.2026 aus [ethereum/tests](https://github.com/ethereum/tests), Commit `c67e485ff8b5be9abc8ad15345ec21aa22e290d9` (derselbe Stand wie `keyaddrtest.json` in M2).

| Datei | Quelle | SHA-256 |
|---|---|---|
| `rlptest.json` | `RLPTests/rlptest.json`, **unverändert** | `dd11055d…fc00` |
| `invalidRLPTest.json` | `RLPTests/invalidRLPTest.json`, **unverändert** | `f90bf574…85d7` |
| `ethereum-transaction-tests.json` | erzeugt mit `convert_eth_tx_vectors.py` aus `TransactionTests/*/*.json` | – |

`convert_eth_tx_vectors.py` ändert nur das Format: Je Test bleiben die Transaktions-Bytes und das erwartete Ergebnis des neuesten Forks der Datei (Prague, sonst Cancun), also Absender und Hash für gültige, der Name der Ausnahme für ungültige Transaktionen. Die SHA-256 jeder Quelldatei steht im Feld `files`, damit sich das Ergebnis gegen einen frischen Checkout prüfen lässt.

**Weggelassen** (Feld `omitted`, mit erwartetem Ergebnis, ohne Bytes): zwei Transaktionen aus `ttData` mit je 10 MB Daten (`String10MbData`, `String10MbDataNotEnoughGAS`). Sie würden die Datei auf 42 MB aufblähen. ToyChain lehnt Daten in Transaktionen und Blöcke über 128 KB ohnehin ab.

## Was der Test prüft (`tests/vectors.rs`)

- **RLP:** Alle 28 gültigen Fälle werden exakt kodiert und dekodiert, alle 26 ungültigen abgelehnt.
- **Transaktionen (210):** Jede Transaktion, die Ethereum annimmt, ergibt bei ToyChain **genau denselben Absender und Hash** oder wird aus einem dokumentierten Grund abgelehnt:
  - keine Chain-ID in der Signatur (vor EIP-155, wiederholbar auf jeder Kette),
  - anderer Transaktionstyp als Legacy und Typ 2,
  - Vertrags-Erzeugung, Daten oder Zugriffsliste (keine Verträge, Entscheidung 36),
  - Betrag über 128 Bit.
  
  Jede Transaktion, die Ethereum ablehnt, lehnt auch ToyChain ab. Stand 20.10.2026: 49 gültig (16 mit gleichem Absender und Hash, davon 14 von ToyChain angenommene Überweisungen und 2 mit Betrag über 128 Bit; 33 beim Einlesen abgelehnt: 32 ohne Chain-ID, 1 vom Typ 1), 161 ungültig, alle abgelehnt.

## Argon2id (ToyWork)

Der offizielle Argon2id-Vektor aus RFC 9106, Abschnitt 5.3, steht im Test `rfc9106_argon2id_vector` in `toy-crypto/src/work.rs`. Quelle ist die Referenz-Implementierung [P-H-C/phc-winner-argon2](https://github.com/P-H-C/phc-winner-argon2), Datei `kats/argon2id`, Commit `f57e61e19229e23c4445b85494dbf7c07de721cb`, SHA-256 `ba05643e…d28d` (die RFC-Seite selbst war aus der Sandbox nicht erreichbar). Der Regressionswert für die ToyWork-Parameter wurde unabhängig mit der C-Referenzbibliothek libargon2 (Python `argon2-cffi`) gegengeprüft.
