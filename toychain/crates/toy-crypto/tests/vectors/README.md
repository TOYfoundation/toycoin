# Testvektoren für toy-crypto (M2)

Alle Dateien sind **unverändert** aus den genannten Quellen kopiert (abgerufen am 20.10.2026). Einzige Ausnahme ist die BLS-JSON-Datei. Sie wird mit `convert_bls_vectors.py` aus dem unveränderten Archiv erzeugt, ändert nur das Format (YAML zu JSON) und prüft dabei die Prüfsumme des Archivs.

| Datei | Quelle | Stand (Commit/Tag) | SHA-256 |
|---|---|---|---|
| `bip39-trezor-vectors.json` | [trezor/python-mnemonic](https://github.com/trezor/python-mnemonic) `vectors.json`, Referenz-Implementierung von BIP-39 | `b57a5ad7` | `fa3b937b…30f8` |
| `bip-0032.mediawiki` | [bitcoin/bips](https://github.com/bitcoin/bips) BIP-32, Testvektoren 1 bis 5 | `130b8e0f` | `e5e00a82…c11f` |
| `ethereum-keyaddrtest.json` | [ethereum/tests](https://github.com/ethereum/tests) `BasicTests/keyaddrtest.json` | `c67e485f` | `a1259937…04d2` |
| `erc-55.md` | [ethereum/ERCs](https://github.com/ethereum/ERCs) ERC-55 (EIP-55), Abschnitt „Test Cases“ | `48dc87a2` | `54a9b25f…dc9e` |
| `eip-155.md` | [ethereum/EIPs](https://github.com/ethereum/EIPs) EIP-155, Abschnitt „Example“ | `0ce679f4` | `3c208813…cc9c` |
| `bls12-381-tests-v0.1.2.tar.gz` | [ethereum/bls12-381-tests](https://github.com/ethereum/bls12-381-tests) Release v0.1.2, `bls_tests_yaml.tar.gz` | Tag `v0.1.2` (`89ff3a86`) | `a4ef909b…9279` |
| `bls12-381-tests-v0.1.2.json` | erzeugt aus dem Archiv darüber | – | – |

Zusätzlich stehen im Test `phrase_to_address_wallet_vectors` (Datei `ethereum_vectors.rs`) Ende-zu-Ende-Fälle Phrase → Adresse, abgeschrieben aus den Testsuiten zweier großer Ethereum-Projekte:

- [alloy-rs/alloy](https://github.com/alloy-rs/alloy) `crates/signer-local/src/mnemonic.rs`, Test `mnemonic_deterministic` (dort „taken from MyCryptoWallet“), Commit `eb2289cc`
- [MetaMask/accounts](https://github.com/MetaMask/accounts) `packages/keyring-eth-hd/src/hd-keyring.test.ts`, Konstanten `sampleMnemonic`, `firstAcct`, `secondAcct`, Zweig `main`, abgerufen am 20.10.2026 (Datei-SHA-256 `7e6846df…bf7c3`). Die früher hier genannte Commit-Angabe `96bc1158` war falsch und ist ersetzt.

Das sind keine Standard-Dokumente, aber Referenzwerte der verbreitetsten Ethereum-Software. Ein Abschreibfehler würde den Test scheitern lassen, nicht fälschlich bestehen lassen.

## Nachprüfung (20.10.2026)

Alle sechs Quelldateien wurden erneut von GitHub geladen (Commits wie in der Tabelle, BLS als Release-Datei). Sie sind **byteweise identisch** mit den Dateien hier. `bls12-381-tests-v0.1.2.json` wurde mit `convert_bls_vectors.py` aus dem frisch geladenen Archiv neu erzeugt und ist ebenfalls identisch. Die Werte aus alloy und MetaMask stehen wörtlich in den genannten Quelldateien.

## Hinweise

- **BIP-39:** Die Spezifikation erlaubt nur 12 oder 24 Wörter. Die 8 offiziellen Fälle mit 18 Wörtern müssen deshalb mit einer klaren Fehlermeldung abgelehnt werden. Die 16 Fälle mit 12 und 24 Wörtern müssen exakt stimmen (Entropie → Phrase → Seed → Hauptschlüssel).
- **BIP-32:** Getestet werden die Vektoren 1 bis 4 (17 Ableitungen, auch „führende Nullen“). Der erwartete Schlüssel wird aus dem offiziellen `xprv`-Text mit einem eigenen Base58-Decoder im Test gelesen und zusätzlich mit dem Parser von `bip32` verglichen. So kann ein Fehler in `bip32` sich nicht selbst bestätigen. Vektor 5 prüft nur das Einlesen ungültiger `xprv`-/`xpub`-Texte. Diese Funktion bietet `toy-crypto` nicht an.
- **ethereum/tests `keyaddrtest.json`:** Getestet wird Schlüssel → Adresse. Das Feld `sig_of_emptystring` ist nicht dokumentiert. Es führt mit keiner naheliegenden Nachricht zur Adresse zurück: geprüft wurden Keccak-256, SHA-256 und SHA3-256 des leeren Textes, der Null-Hash und EIP-191. Es wird deshalb nicht als Signatur-Vektor genutzt. Die Signatur „horse“ hat außerdem ein hohes S (vor EIP-2) und wird von ToyChain zu Recht abgelehnt.
- **BLS:** Ethereum lässt einen öffentlichen Schlüssel „unendlich“ beim Einlesen zu und lehnt ihn erst bei der Benutzung ab. ToyChain lehnt ihn schon beim Einlesen ab (strenger, im Test vermerkt). Die Vektoren für `fast_aggregate_verify` setzen geprüfte Besitznachweise voraus. Darum laufen sie als interner Test (`src/bls/tests.rs`).
- **VRF:** Für die BLS-basierte VRF gibt es keine offiziellen Vektoren (kein RFC 9381). Der Beweis ist eine normale BLS-Signatur und durch die BLS-Vektoren abgedeckt. Ein Regressionswert in `tests/vrf.rs` hält das Ergebnis fest.
