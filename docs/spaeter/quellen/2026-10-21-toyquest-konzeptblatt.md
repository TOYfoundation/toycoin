# TOYQUEST – Konzeptblatt: Items als NFT auf Polygon

Entwurf vom 21.10.2026 · Zu TOYQUEST (Spiel) und ToyCoin (Polygon) · Status: Entscheidungen eingearbeitet, Vorschläge markiert

## 1. Ziel

Spieler erspielen Items in TOYQUEST und können sie optional als NFT auf Polygon prägen. Die Prägung kostet eine kleine Gebühr in ToyCoin (TOY). Die Netzgebühr (POL) übernimmt ein Topf, damit die Spieler selbst kein POL brauchen.

## 2. Grundprinzip (entschieden)

- Items werden **im Spiel erspielt**, nicht gekauft. So gibt es kein Pay-to-Win.
- **TOY dient nur für die Prägegebühr** beim Mint als NFT.
- Das Item funktioniert im Spiel auch ohne NFT (Offline- und Online-Modus).
- Ein **Topf mit POL** zahlt die Polygon-Gasgebühr (Gas-Sponsoring).

## 3. Ablauf

1. Der Spieler verdient ein Item im Spiel.
2. Der Spielserver signiert einen **einmaligen Mint-Gutschein** (Wallet, Item, Menge, Preis-Angebot, Ablaufzeit).
3. Der Spieler tippt auf „Als NFT prägen“, stellt optional den Verteilungs-Regler ein und signiert die Zahlung per `permit` (keine Extra-Transaktion).
4. Ein Paymaster oder Relayer zahlt das POL aus dem Topf.
5. Der Mint-Vertrag prüft den Gutschein, zieht die TOY ein, verteilt sie und prägt das ERC-1155-Token.
6. ToyScan zeigt Topf-Stand und Mint-Zahlen.

## 4. Prägegebühr (entschieden)

**Formel:** Prägegebühr in TOY = Gaskosten in POL × Faktor ÷ Kurs (TOY pro POL).

- **Faktor:** Start bei **2**, später **1,5**, sobald der Betrieb stabil läuft.
- **Gaskosten:** Ein Mint verbraucht ungefähr gleich viel Gas. Der Gaspreis wird aktuell gemessen.
- **POL-Kurs:** Chainlink-Preisfeed (POL/USD).
- **TOY-Kurs:** Durchschnittswert (TWAP) aus einem TOY/POL-Pool auf Polygon. Das ist nur sicher bei genug Liquidität. Sonst ein vom Spielserver signierter Preis mit festen Grenzen.
- Der Preis steckt im Mint-Gutschein und gilt nur wenige Minuten.
- **Schutz:** Mindest- und Höchstgebühr, Begrenzung der Preisänderung pro Stunde, Notschalter bei veralteten Preisdaten.

## 5. Verteilung der Gebühr

**Entschieden:** Die Gebühr wird zwischen **Projekt-Wallet** und **POL-Topf** aufgeteilt (kein Burn). Der Spieler kann die Aufteilung **in einem gewissen Rahmen** selbst bestimmen.

**Vorschlag (zur Bestätigung):**

- Die ersten **1× Gaskosten** gehen immer in den Topf. So bleibt der Topf stabil und läuft nicht leer.
- Die übrigen 0,5 bis 1× teilt der Spieler per Regler zwischen Topf und Projekt-Wallet auf (Standard 70 % Projekt-Wallet, 30 % Topf, jeweils im Rahmen).
- Wer mehr in den Topf gibt, könnte ein Zeichen im Spiel bekommen (Titel oder Kosmetik-Item).
- Aufteilung und Grenzen stehen fest im Mint-Vertrag. Der Gesamtbetrag der Gebühr bleibt gleich.

## 6. Überschüsse

**Entschieden:** Spätere Überschüsse im Topf gehen **anteilig an alle Spieler, die geprägt haben**.

**Vorschlag (zur Bestätigung):**

- **Überschuss** = alles im Topf über dem Zielstand (z. B. 3 Monate erwartete Gaskosten).
- **Anteil** nach gezahlter Prägegebühr, nicht nach Zahl der Mints.
- **Deckel:** Niemand bekommt mehr zurück, als er selbst eingezahlt hat.
- **Auszahlung:** Standardmäßig als Rabatt auf spätere Prägungen (kostet kein Gas). Optional Auszahlung in TOY mit einer Abhol-Transaktion.
- **Rhythmus:** Monatlich, mit öffentlichem Merkle-Beweis für jeden Anspruch und Darstellung in ToyScan.
- Zusätzlich kann der Vertrag bei geringeren tatsächlichen Gaskosten die Differenz sofort erstatten.

## 7. Schutz vor Missbrauch

- Nur Items mit **Gutschein vom Spielserver** (einmalig, mit Nonce und Ablaufzeit) dürfen geprägt werden. Bots können sonst beliebig viele erzeugen.
- Die Prägegebühr liegt immer über den Gaskosten.
- Limits pro Wallet und Tag, Notschalter für das Sponsoring.
- Mint-Vertrag und Paymaster brauchen ein externes Sicherheits-Audit vor dem Start.
- Zufällige Eigenschaften nur mit fairem Zufall (Commit-Reveal oder Chainlink VRF).
- Metadaten auf IPFS, ein Provenance-Hash im Vertrag belegt, dass sie nach dem Verkauf nicht verändert wurden.

## 8. Technik

- **Netz:** Polygon (später auch Ethereum möglich).
- **Token:** ERC-1155 für Items und Editionen, eine feste maximale Anzahl je Item im Vertrag.
- **Zahlung:** ToyCoin-Vertrag unterstützt `permit` (EIP-2612), also Signatur statt separater Freigabe.
- **Gas-Sponsoring:** ERC-4337 Paymaster oder Relayer mit Meta-Transaktionen, gespeist aus dem POL-Topf.
- **Auffüllen des Topfs:** Der Topf-Anteil kommt in TOY und muss in POL getauscht werden, automatisch über eine Börse auf Polygon. Das geht nur mit einem TOY/POL-Pool mit genug Liquidität. Ohne Pool wird der Anteil manuell getauscht.
- **ToyScan:** Zeigt Topf-Stand, Mint-Zahlen, Verteilung und gelbe Warnung bei niedrigem Topf.

## 9. Lernwelt: ToyChain erleben und verstehen (Vorschlag)

**Ziel:** Die Prozesse hinter ToyCoin werden im Spiel sichtbar und erklärbar, besonders für Menschen ohne technisches Wissen. Das Spiel zeigt, wie ToyCoin geschürft, abgebaut, erzeugt, bearbeitet und gemintet wird. Das ist ein wichtiger Baustein für Verständnis und Vertrauen.

**Grundregeln:**

- **Echt oder Demo:** Hat der Spieler eine Stufe wirklich aktiviert, treiben echte Ereignisse vom Node die Animationen an (Share angenommen, Block signiert, Beweis eingereicht, Belohnung gutgeschrieben). Sonst läuft ein klar gekennzeichneter Demo-Modus.
- **Das Spiel erzeugt kein TOY.** Echte TOY entstehen nur durch echte Arbeit im Netzwerk. Spielerfolge geben höchstens Dukaten oder Kosmetik.
- **Nachweis:** Ein Abzeichen „Stufe aktiv“ gibt es nur mit einem signierten Nachweis vom Node.
- **Schutz für das Gerät:** Nur bei Ladekabel, WLAN und normaler Temperatur, mit Leistungsregler.
- **Bis zum Start:** Alles ist mit Testnet-TOY beschriftet.

**Stufen und Orte:**

0. **Lehrling (Demo):** Zeitraffer in der Werkstatt, als Simulation markiert. Zeigt die Halving-Kurve und was Spam kostet.
1. **Karawanenreisender (Light-Wallet):** Die Karawanserei zeigt den Herzschlag der Kette, jeder Block ist eine Ankunft.
2. **Minenarbeiter (ToyWork, CPU), schürfen und abbauen:** Edelsteinmine, jeder angenommene Share ist ein Fund.
3. **Siegelmeister (Attestierer), signieren:** Die Siegelhalle, jeder mitgesiegelte Block bekommt sein Siegel.
4. **Schmied (Prover, GPU), bearbeiten:** Die Schmiede, viele kleine Barren werden zu einem verschmolzen.
5. **Münzmeister, erzeugen und minten:** Die Münzpresse startet nur, wenn wirklich eine Belohnung eintrifft.
6. **Bibliothekar (Archiv-Knoten):** Die Bibliothek, jeder Block ist ein Buch.

Weitere Orte: Basar (Gebühren und Kontingent), Schatzkammer (Emission und 100-Mio.-Grenze), Akademie (Einsteiger-Lehrpfad).

**NPCs mit Rollen, die ausführlich erklären:**

- **Lehrmeister:** Führt Einsteiger mit Geschichten und Gleichnissen, ohne Fachwörter.
- **Basarhändler:** Erklärt Kontingent, Gebühren und warum Spam teuer ist, mit Marktbeispielen.
- **Siegelmeister und Wächter:** Erklären Signieren, Sicherheit und Angriffe.
- **Schmied:** Erklärt, wie Beweise die Historie klein halten (256-MB-Budget).
- **Bibliothekar:** Erklärt Archiv und Speicher.
- **Münzmeister und Schatzmeister:** Erklären Emission, Halving, die 100-Mio.-Grenze, die 1-%-Reserve und den Fair Launch.
- **Karawanenführer:** Erklärt Bridge, Polygon, Offline-Zahlung, Rückholfrist und Gutscheine.
- **Der Schleierhändler:** Erklärt ToyCash, Privatsphäre und Besitzbeweise.
- **Die weise Warnerin:** Erklärt Betrugsmaschen und Phrasen-Sicherheit (niemals weitergeben).

**Didaktik:**

- **Drei Erklärtiefen pro NPC:** Kurz (ein Satz), Normal (Gleichnis und Beispiel), Tief (technisch, mit echten Zahlen aus ToyScan). Der Spieler wählt die Tiefe.
- **Missionen und Quiz:** Kleine Aufgaben mit Dukaten als Belohnung.
- **Zeig mir live:** Ein Knopf blendet echte Daten ein.
- **Glossar im Spiel** und Texte in allen Spielsprachen.
- **Eine geprüfte Textquelle:** Die NPC-Texte werden aus einer gepflegten Wissensbasis erzeugt, die an Spezifikation und ToyScan gekoppelt ist. So erklären die NPCs nie etwas Falsches oder Veraltetes.
- **Keine Renditeversprechen** in Dialogen.

**Empfehlung für den Start:** Zuerst Akademie, Karawanserei, Minenarbeiter (ToyWork) und Münzpresse. Sie decken die wichtigsten Fragen ab: Was ist das, wie entsteht es, wie wird es geprägt.

## 10. Zeitreise: 5 Level von der Antike zur Moderne (Vorschlag)

**Idee:** Die 5 Level von TOYQUEST führen als Zeitreise von der Antike zur Moderne. Dieselbe Aufgabe wird von Level zu Level mit effizienteren Methoden gelöst. Das spiegelt, wie die ToyChain Arbeit und Speicher immer effizienter organisiert.

**Zwei Achsen:** Die Level 1 bis 5 sind die didaktische Zeitreise. Die echten Stufen aus Abschnitt 9 (Light, ToyWork, Siegelmeister, Prover und so weiter) aktiviert der Spieler unabhängig vom Level.

**Die fünf Level:**

1. **Antike Werkstatt (Handarbeit):** ToyWork als Muskelarbeit, Prinzip Arbeitsnachweis. Maschinen zum Beispiel Hebel, Keil, Rolle, Tretrad-Kran, Schöpfwerk.
2. **Kraft und Bündelung:** Viele Hände, ein Antrieb. Das entspricht der Sammelsignatur, bei der viele Siegel zu einem werden. Maschinen zum Beispiel Wasserrad, Archimedische Schraube, Kolbenpumpe des Ktesibios, Windrad.
3. **Zeit und Messen:** Blockzeit, Epochen, Halving-Kurve. Maschinen zum Beispiel Wasseruhr, Sonnenuhr, Antikythera-Mechanismus, Wegmesser.
4. **Archiv und Prägung:** Historie, Merkle-Beweise, Minten. Maschinen zum Beispiel Papierherstellung, Holzdruck, Münzprägung.
5. **Automaten und Beweise:** Prover, Privatsphäre, Offline-Zahlung, ToyCash. Maschinen zum Beispiel Herons Münzautomat, selbstöffnende Tempeltüren und die Automaten von Al-Dschazari (passt zur 1001-Nacht-Welt).

**50 antike Maschinen:** Je 10 pro Level, nach diesem Prinzip sortiert. Jede Maschine hat einen Lehrmeister, eine Lerneinheit und eine thematische Verknüpfung zu einem Prozess (zum Beispiel Hebel ↔ Arbeitsnachweis, Antikythera ↔ Halving, Herons Münzautomat ↔ Minten). Eine Liste mit 50 Maschinen liegt bereits im Zielordner und wird nicht überschrieben, sondern ergänzt. In der Endversion sind bis zu 256 Maschinen möglich (etwa 51 pro Level).

**Belohnungssystem:** Wissens-Siegel pro Maschine, Meisterbrief pro Level, Dukaten und Kosmetik. Items lassen sich optional als NFT prägen (Abschnitte 1 bis 8). Echtes TOY gibt es dadurch nicht.

**Gang und Teamarbeit:**

- Spieler treten als Gang oder einzeln an. Synergien entstehen aus Eigenschaften, Fähigkeiten und Items der Member.
- Open-World-Spieler finden sich nach Level für gemeinsame Challenges zusammen und leveln auf.
- Challenges spiegeln echte Prinzipien: Ein Tor öffnet sich nur, wenn mindestens zwei Drittel der Gang ihr Siegel einsetzen (Mehrheits-Konsens). Ein Wasserrad treibt nur an, wenn mehrere Spieler gemeinsam arbeiten (Bündelung).

**Texte:** Wie bei den NPCs aus einer geprüften Quelle und in allen Spielsprachen.

## 11. Offene Punkte

1. Aufteilungs-Rahmen des Reglers und Standardwerte bestätigen (Abschnitt 5).
2. Gibt es einen TOY/POL-Pool auf Polygon mit genug Liquidität, oder muss er angelegt werden?
3. Größe und Aufbau der Kollektion (fest begrenzt oder wachsend, welche Item-Arten).
4. Überschuss-Regeln bestätigen (Zielstand, Rhythmus, Rabatt oder Auszahlung).
5. Lernwelt: Welche Orte und NPC-Rollen im ersten Release, und wie viele Erklärtiefen (Abschnitt 9).
6. Zeitreise: Ergänzung der bestehenden Maschinenliste (ohne Überschreiben) um Level-Zuordnung, Lehrmeister und Prozess-Verknüpfung, Ausbau bis 256, Details zu Team-Mechanik und Belohnungen (Abschnitt 10).
7. Rechtliche Prüfung zum Schluss (laut Plan).
