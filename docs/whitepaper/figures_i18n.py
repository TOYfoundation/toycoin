"""Renders the whitepaper figures in English or French.

Usage: python3 figures_i18n.py en|fr
Runs figures.py unchanged and translates every label on the fly; the images go to abbildungen/<lang>/.
"""
import os
import re
import runpy
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.figure as mf  # noqa: E402
import matplotlib.text as mt  # noqa: E402

LANG = sys.argv[1] if len(sys.argv) > 1 else "en"

EN = {
    "0 % ab Tag 180": "0 % from day 180",
    "0 · Leicht": "0 · Light",
    "0,1 TOY Pfand": "0.1 TOY deposit",
    "0,25 Mio.": "0.25 m",
    "1  Auswahl": "1  Selection",
    "1  Ben erzeugt ein Geheimnis und sperrt BTC mit dessen Hash (48 h)":
        "1  Ben creates a secret and locks BTC with its hash (48 h)",
    "1 · Konto": "1 · Account",
    "1,3 Mio.": "1.3 m",
    "1.000 TOY Stake": "1,000 TOY stake",
    "10 % gleichzeitig: 30 %": "10 % at once: 30 %",
    "10 TOY Stake": "10 TOY stake",
    "10 bis 100 Transaktionen\nkostenlos je Tag": "10 to 100 transactions\nfree per day",
    "100 TOY Stake": "100 TOY stake",
    "11,6 Mio.": "11.6 m",
    "2  Anna sperrt TOY mit demselben Hash (24 h)": "2  Anna locks TOY with the same hash (24 h)",
    "2  Vorschlag": "2  Proposal",
    "2 · Attestierer": "2 · Attester",
    "3  Abstimmung 1": "3  Vote 1",
    "3  Ben holt die TOY mit dem Geheimnis ab, es wird öffentlich":
        "3  Ben claims the TOY with the secret, which becomes public",
    "3 · Voll": "3 · Full",
    "4  Abstimmung 2": "4  Vote 2",
    "4  Anna holt mit dem nun bekannten Geheimnis die BTC ab": "4  Anna claims the BTC with the now known secret",
    "4 · Validator": "4 · Validator",
    "5  Endgültig": "5  Final",
    "50 % am ersten Tag": "50 % on day one",
    "A · Kette": "A · Chain",
    "Abbrechen möglich": "Can be cancelled",
    "Abbuchung über\nder eigenen Grenze": "Withdrawal above\nyour own limit",
    "Aktives Konto\nim Speicher\njedes Knotens": "Active account\nin the memory\nof every node",
    "Anna\n(hat TOY)": "Anna\n(has TOY)",
    "Anteil der Höchstmenge in %": "Share of maximum supply in %",
    "Anteil des Stakes, der gleichzeitig doppelt signiert, in %": "Share of stake double-signing at the same time, in %",
    "Arbeits-Lotterie": "Work lottery",
    "Aufwecken mit der nächsten Transaktion,\nautomatisch, Guthaben vollständig":
        "Woken up with the next transaction,\nautomatically, full balance",
    "Ausgeführt": "Executed",
    "Auslagern, wenn der Speicher\nüber 90 % voll ist (älteste zuerst)":
        "Moved out when memory is\nover 90 % full (oldest first)",
    "Auswertung,\nnur Anzeige": "Analysis,\ndisplay only",
    "B · Netzwerk": "B · Network",
    "Ben\n(hat z. B. BTC)": "Ben\n(has e.g. BTC)",
    "Beweis (< 1 KB)\nin der Wallet": "Proof (< 1 KB)\nin the wallet",
    "Beweis-Schicht (Phase 2)": "Proof layer (phase 2)",
    "Beweise (8 MB)": "Proofs (8 MB)",
    "Bitcoin (Halbierung etwa alle 4 Jahre)": "Bitcoin (halving about every 4 years)",
    "Bitcoin: 50 % nach 4 Jahren": "Bitcoin: 50 % after 4 years",
    "Block mit fester\nReihenfolge": "Block in a fixed\norder",
    "Block-Fenster (48 MB)": "Block window (48 MB)",
    "C · ToyScan": "C · ToyScan",
    "Chancen": "Opportunities",
    "Clients": "Clients",
    "Datenbudget eines Standard-Knotens: 256 MB": "Data budget of a standard node: 256 MB",
    "Einzelfall: 1 %": "Single case: 1 %",
    "Erstattungs-Topf": "Refund pool",
    "Ethereum-kompatibles JSON-RPC · eigene toy_-Methoden": "Ethereum-compatible JSON-RPC · own toy_ methods",
    "Hauptnetz": "Mainnet",
    "Indizes, Wartebereich, Puffer (40 MB)": "Indexes, mempool, buffers (40 MB)",
    "Insgesamt gelten 53,2 % aller jemals\nerfassten Kryptowerte als gescheitert\n(GeckoTerminal-Daten, Stand Anfang 2026)":
        "In total, 53.2 % of all crypto assets\never tracked are considered failed\n(GeckoTerminal data, early 2026)",
    "Jahre seit Start": "Years since launch",
    "Komitee prüft\nund stimmt vor": "Committee checks\nand pre-votes",
    "Komitee, VRF-Auswahl, Abstimmung im Stil von Tendermint, BLS":
        "Committee, VRF selection, Tendermint-style voting, BLS",
    "Konsens": "Consensus",
    "Konten, Guthaben, Stake, Sperren in einem Merkle-Baum": "Accounts, balances, stake, bans in a Merkle tree",
    "Kontostände aktiver Konten (128 MB)": "Balances of active accounts (128 MB)",
    "Konzept": "Concept",
    "Kürzung des eigenen Stakes in %": "Cut of own stake in %",
    "Lohn-Anteil an neuen TOY in %": "Reward share of new TOY in %",
    "Lose in der\nArbeits-Lotterie": "Tickets in the\nwork lottery",
    "Läuft die Zeit ab, holt jeder sein Geld zurück. Niemand kann betrügen, es braucht keinen Vermittler.":
        "If time runs out, everyone takes their money back. Nobody can cheat, no middleman is needed.",
    "M12, M13": "M12, M13",
    "M2 bis M11": "M2 to M11",
    "Muster erkennen\n→ Warnung in der Ampel, nie automatische Sperre":
        "Detects patterns\n→ warning light, never an automatic ban",
    "Nach der Übergabe": "After the handover",
    "Netzwerk": "Network",
    "Nie ausgelagert: Konten mit Stake, Schutzeinstellungen oder einer bezahlten Transaktion in den letzten 7 Epochen":
        "Never moved out: accounts with stake, protection settings or a paid transaction in the last 7 epochs",
    "Phase 2\nBeweise": "Phase 2\nproofs",
    "Ratenbegrenzung, Verbindungen trennen\n→ Kontostände bleiben unberührt":
        "Rate limits, dropping connections\n→ balances stay untouched",
    "Risiken": "Threats",
    "Ruhendes Konto\nnur als Wurzel\ngespeichert": "Dormant account\nstored only\nas a root",
    "Schnittstellen": "Interfaces",
    "Schwächen": "Weaknesses",
    "Stake-Teilnehmer": "Stakers",
    "Start-Validatoren": "Start validators",
    "Startphase, Tag 0": "Bootstrap, day 0",
    "Startphase, Tag 90": "Bootstrap, day 90",
    "Stärken": "Strengths",
    "TOYPAY App · Browser-Erweiterung · ToyNode · ToyScan": "TOYPAY app · browser extension · ToyNode · ToyScan",
    "Tage Planzeit seit Start": "Plan-time days since launch",
    "Testnetz 1": "Testnet 1",
    "ToyChain (m = 1, Planwert)": "ToyChain (m = 1, plan)",
    "ToyChain 99,0 %": "ToyChain 99.0 %",
    "ToyChain: 50 % nach 3,76 Jahren": "ToyChain: 50 % after 3.76 years",
    "VRF-Zufall wählt\neinen Vorschlagenden": "VRF randomness\npicks a proposer",
    "Versehen kostet wenig,\nein gemeinsamer Angriff alles.\nGekürzte TOY gehen in den\nErstattungs-Topf, nie verbrannt.":
        "A mistake costs little,\na joint attack costs everything.\nSlashed TOY go to the\nrefund pool, never burned.",
    "Wallet und ToyScan\nzeigen sie an": "Wallet and ToyScan\nshow it",
    "Wartezeit, z. B. 24 Stunden": "Waiting time, e.g. 24 hours",
    "Wunsch nach fairen, einfachen Coins\nMiCA schafft klare Regeln in der EU\nLernen und Mitmachen per Handy\nspätere Spiel-Anbindung (TOYQUEST)\nGemeinschaft statt Investoren":
        "Demand for fair, simple coins\nMiCA creates clear rules in the EU\nLearning and joining by phone\nlater game link (TOYQUEST)\nCommunity instead of investors",
    "Zustand": "State",
    "ab 2/3 Zustimmung\nwird bestätigt": "with 2/3 approval\nit is confirmed",
    "ab einem Drittel: 100 %": "from one third: 100 %",
    "alle Knoten nach\nfesten Regeln": "all nodes by\nfixed rules",
    "andere Kette": "other chain",
    "ein gemeinsamer Rust-Kern für alle Plattformen": "one shared Rust core for all platforms",
    "eine BLS-Signatur\n(96 Byte) für alle": "one BLS signature\n(96 bytes) for all",
    "erst jetzt verlässt\ndas Geld das Konto": "only now does the\nmoney leave the account",
    "erst nach Audit\nund Rechtsprüfung": "only after audit\nand legal review",
    "etwa 60 Sekunden je Block · nach Schritt 5 kann der Block nie mehr zurückgenommen werden":
        "about 60 seconds per block · after step 5 the block can never be reverted",
    "fertig": "done",
    "frei für später (32 MB)": "free for later (32 MB)",
    "geplant": "planned",
    "gescheiterte Coins/Token\nin Mio.": "failed coins/tokens\nin millions",
    "heute (18.10.2026): vor der Testphase": "today (18 Oct 2026): before the test phase",
    "hält alle Daten,\nhöherer Anteil": "holds all data,\nlarger share",
    "in Arbeit": "in progress",
    "jeder Knoten\nfür sich": "each node\non its own",
    "jeder Lohn bleibt\n365 Epochen gebunden": "every reward stays\nbonded for 365 epochs",
    "kein Budget aus Premint für Audits,\nWerbung und Börsenlistung\nkleines Team, frühes Stadium\nkeine Apps, Token oder DeFi\nNamensgleichheit mit alten Token":
        "no premint budget for audits,\nmarketing and exchange listings\nsmall team, early stage\nno apps, tokens or DeFi\nsame name as old tokens",
    "kein TOY": "no TOY",
    "libp2p mit QUIC und Noise, Dandelion++, optional Tor": "libp2p with QUIC and Noise, Dandelion++, optional Tor",
    "mehr Beitrag  →  mehr Rechte und mehr Anteil (freiwillig, jederzeit wechselbar)":
        "more contribution  →  more rights and a larger share (voluntary, can change any time)",
    "mit eigenem Schlüssel\noder Notfall-Schlüssel": "with your own key\nor the emergency key",
    "neue TOY in Mio.": "new TOY in millions",
    "nur eindeutig nachweisbare Verstöße\n→ Sperrzeit für Rechte, nie für Guthaben":
        "only clearly provable violations\n→ ban on rights, never on balances",
    "rekursiver Beweis über die gesamte Vorgeschichte": "recursive proof over the entire history",
    "schlägt Blöcke vor,\ngrößter Anteil": "proposes blocks,\nlargest share",
    "totaler Fair Launch, keine Insider-Coins\nMitmachen ohne Guthaben, alte Geräte\nendgültig nach etwa 1 Minute\nkeine Smart-Contract-Risiken\nEthereum-kompatible Adressen":
        "total fair launch, no insider coins\njoin without funds, old devices\nfinal after about 1 minute\nno smart contract risks\nEthereum-compatible addresses",
    "unterschreibt mit,\nAnteil am Lohn": "co-signs,\nshare of rewards",
    "Übergabe möglich, sobald 21 neue Validatoren\nzwei Drittel des Stakes halten":
        "Handover possible once 21 new validators\nhold two thirds of the stake",
    "Übergabe noch\nnicht möglich": "Handover not\nyet possible",
    "über die Hälfte aller Coins scheitert\nNetzwerkeffekte großer Ketten\nStablecoins dominieren Zahlungen\nAngriffe auf ein kleines Netz\nrechtliche Einordnung offen":
        "over half of all coins fail\nnetwork effects of big chains\nstablecoins dominate payments\nattacks on a small network\nlegal status still open",
}

FR = {
    "0 % ab Tag 180": "0 % dès le jour 180",
    "0 · Leicht": "0 · Léger",
    "0,1 TOY Pfand": "dépôt de 0,1 TOY",
    "0,25 Mio.": "0,25 M",
    "1  Auswahl": "1  Sélection",
    "1  Ben erzeugt ein Geheimnis und sperrt BTC mit dessen Hash (48 h)":
        "1  Ben crée un secret et verrouille des BTC avec son hash (48 h)",
    "1 · Konto": "1 · Compte",
    "1,3 Mio.": "1,3 M",
    "1.000 TOY Stake": "1 000 TOY de stake",
    "10 % gleichzeitig: 30 %": "10 % en même temps : 30 %",
    "10 TOY Stake": "10 TOY de stake",
    "10 bis 100 Transaktionen\nkostenlos je Tag": "10 à 100 transactions\ngratuites par jour",
    "100 TOY Stake": "100 TOY de stake",
    "11,6 Mio.": "11,6 M",
    "2  Anna sperrt TOY mit demselben Hash (24 h)": "2  Anna verrouille des TOY avec le même hash (24 h)",
    "2  Vorschlag": "2  Proposition",
    "2 · Attestierer": "2 · Attestateur",
    "3  Abstimmung 1": "3  Vote 1",
    "3  Ben holt die TOY mit dem Geheimnis ab, es wird öffentlich":
        "3  Ben retire les TOY avec le secret, qui devient public",
    "3 · Voll": "3 · Complet",
    "4  Abstimmung 2": "4  Vote 2",
    "4  Anna holt mit dem nun bekannten Geheimnis die BTC ab": "4  Anna retire les BTC avec le secret désormais connu",
    "4 · Validator": "4 · Validateur",
    "5  Endgültig": "5  Définitif",
    "50 % am ersten Tag": "50 % le premier jour",
    "A · Kette": "A · Chaîne",
    "Abbrechen möglich": "Annulation possible",
    "Abbuchung über\nder eigenen Grenze": "Retrait au-dessus\nde sa propre limite",
    "Aktives Konto\nim Speicher\njedes Knotens": "Compte actif\ndans la mémoire\nde chaque nœud",
    "Anna\n(hat TOY)": "Anna\n(a des TOY)",
    "Anteil der Höchstmenge in %": "Part de l'offre maximale en %",
    "Anteil des Stakes, der gleichzeitig doppelt signiert, in %":
        "Part du stake qui signe deux fois en même temps, en %",
    "Arbeits-Lotterie": "Loterie du travail",
    "Aufwecken mit der nächsten Transaktion,\nautomatisch, Guthaben vollständig":
        "Réveil avec la prochaine transaction,\nautomatique, solde complet",
    "Ausgeführt": "Exécuté",
    "Auslagern, wenn der Speicher\nüber 90 % voll ist (älteste zuerst)":
        "Mise en sommeil si la mémoire\ndépasse 90 % (les plus anciens d'abord)",
    "Auswertung,\nnur Anzeige": "Analyse,\naffichage seul",
    "B · Netzwerk": "B · Réseau",
    "Ben\n(hat z. B. BTC)": "Ben\n(a p. ex. des BTC)",
    "Beweis (< 1 KB)\nin der Wallet": "Preuve (< 1 Ko)\ndans le wallet",
    "Beweis-Schicht (Phase 2)": "Couche de preuves (phase 2)",
    "Beweise (8 MB)": "Preuves (8 Mo)",
    "Bitcoin (Halbierung etwa alle 4 Jahre)": "Bitcoin (halving environ tous les 4 ans)",
    "Bitcoin: 50 % nach 4 Jahren": "Bitcoin : 50 % après 4 ans",
    "Block mit fester\nReihenfolge": "Bloc dans un\nordre fixe",
    "Block-Fenster (48 MB)": "Fenêtre de blocs (48 Mo)",
    "C · ToyScan": "C · ToyScan",
    "Chancen": "Opportunités",
    "Clients": "Clients",
    "Datenbudget eines Standard-Knotens: 256 MB": "Budget de données d'un nœud standard : 256 Mo",
    "Einzelfall: 1 %": "Cas isolé : 1 %",
    "Erstattungs-Topf": "Pot de remboursement",
    "Ethereum-kompatibles JSON-RPC · eigene toy_-Methoden": "JSON-RPC compatible Ethereum · méthodes toy_ propres",
    "Hauptnetz": "Mainnet",
    "Indizes, Wartebereich, Puffer (40 MB)": "Index, mempool, tampons (40 Mo)",
    "Insgesamt gelten 53,2 % aller jemals\nerfassten Kryptowerte als gescheitert\n(GeckoTerminal-Daten, Stand Anfang 2026)":
        "Au total, 53,2 % de tous les crypto-actifs\njamais recensés sont considérés comme morts\n(données GeckoTerminal, début 2026)",
    "Jahre seit Start": "Années depuis le lancement",
    "Komitee prüft\nund stimmt vor": "Le comité vérifie\net pré-vote",
    "Komitee, VRF-Auswahl, Abstimmung im Stil von Tendermint, BLS":
        "Comité, sélection VRF, vote de type Tendermint, BLS",
    "Konsens": "Consensus",
    "Konten, Guthaben, Stake, Sperren in einem Merkle-Baum": "Comptes, soldes, stake, blocages dans un arbre de Merkle",
    "Kontostände aktiver Konten (128 MB)": "Soldes des comptes actifs (128 Mo)",
    "Konzept": "Concept",
    "Kürzung des eigenen Stakes in %": "Réduction de son propre stake en %",
    "Lohn-Anteil an neuen TOY in %": "Part des nouveaux TOY en %",
    "Lose in der\nArbeits-Lotterie": "Billets dans la\nloterie du travail",
    "Läuft die Zeit ab, holt jeder sein Geld zurück. Niemand kann betrügen, es braucht keinen Vermittler.":
        "Si le délai expire, chacun récupère son argent. Personne ne peut tricher, aucun intermédiaire.",
    "M12, M13": "M12, M13",
    "M2 bis M11": "M2 à M11",
    "Muster erkennen\n→ Warnung in der Ampel, nie automatische Sperre":
        "Détecte des schémas\n→ alerte du feu, jamais de blocage automatique",
    "Nach der Übergabe": "Après la passation",
    "Netzwerk": "Réseau",
    "Nie ausgelagert: Konten mit Stake, Schutzeinstellungen oder einer bezahlten Transaktion in den letzten 7 Epochen":
        "Jamais mis en sommeil : comptes avec stake, protections ou transaction payante dans les 7 dernières époques",
    "Phase 2\nBeweise": "Phase 2\npreuves",
    "Ratenbegrenzung, Verbindungen trennen\n→ Kontostände bleiben unberührt":
        "Limites de débit, coupure de connexions\n→ les soldes restent intacts",
    "Risiken": "Menaces",
    "Ruhendes Konto\nnur als Wurzel\ngespeichert": "Compte dormant\nstocké seulement\ncomme racine",
    "Schnittstellen": "Interfaces",
    "Schwächen": "Faiblesses",
    "Stake-Teilnehmer": "Stakers",
    "Start-Validatoren": "Validateurs de départ",
    "Startphase, Tag 0": "Démarrage, jour 0",
    "Startphase, Tag 90": "Démarrage, jour 90",
    "Stärken": "Forces",
    "TOYPAY App · Browser-Erweiterung · ToyNode · ToyScan": "App TOYPAY · extension de navigateur · ToyNode · ToyScan",
    "Tage Planzeit seit Start": "Jours de temps planifié depuis le lancement",
    "Testnetz 1": "Testnet 1",
    "ToyChain (m = 1, Planwert)": "ToyChain (m = 1, plan)",
    "ToyChain 99,0 %": "ToyChain 99,0 %",
    "ToyChain: 50 % nach 3,76 Jahren": "ToyChain : 50 % après 3,76 ans",
    "VRF-Zufall wählt\neinen Vorschlagenden": "Le hasard VRF\nchoisit un proposant",
    "Versehen kostet wenig,\nein gemeinsamer Angriff alles.\nGekürzte TOY gehen in den\nErstattungs-Topf, nie verbrannt.":
        "Une erreur coûte peu,\nune attaque commune coûte tout.\nLes TOY réduits vont au pot de\nremboursement, jamais brûlés.",
    "Wallet und ToyScan\nzeigen sie an": "Wallet et ToyScan\nl'affichent",
    "Wartezeit, z. B. 24 Stunden": "Délai d'attente, p. ex. 24 heures",
    "Wunsch nach fairen, einfachen Coins\nMiCA schafft klare Regeln in der EU\nLernen und Mitmachen per Handy\nspätere Spiel-Anbindung (TOYQUEST)\nGemeinschaft statt Investoren":
        "Envie de coins équitables et simples\nMiCA fixe des règles claires dans l'UE\nApprendre et participer par mobile\nlien futur avec le jeu (TOYQUEST)\nCommunauté plutôt qu'investisseurs",
    "Zustand": "État",
    "ab 2/3 Zustimmung\nwird bestätigt": "avec 2/3 d'accord\nil est confirmé",
    "ab einem Drittel: 100 %": "dès un tiers : 100 %",
    "alle Knoten nach\nfesten Regeln": "tous les nœuds,\nrègles fixes",
    "andere Kette": "autre chaîne",
    "ein gemeinsamer Rust-Kern für alle Plattformen": "un noyau Rust commun pour toutes les plateformes",
    "eine BLS-Signatur\n(96 Byte) für alle": "une signature BLS\n(96 octets) pour tous",
    "erst jetzt verlässt\ndas Geld das Konto": "seulement alors l'argent\nquitte le compte",
    "erst nach Audit\nund Rechtsprüfung": "après audit et\nexamen juridique",
    "etwa 60 Sekunden je Block · nach Schritt 5 kann der Block nie mehr zurückgenommen werden":
        "environ 60 secondes par bloc · après l'étape 5, le bloc ne peut plus jamais être annulé",
    "fertig": "terminé",
    "frei für später (32 MB)": "libre pour plus tard (32 Mo)",
    "geplant": "prévu",
    "gescheiterte Coins/Token\nin Mio.": "coins/tokens morts\nen millions",
    "heute (18.10.2026): vor der Testphase": "aujourd'hui (18/10/2026) : avant la phase de test",
    "hält alle Daten,\nhöherer Anteil": "garde toutes les données,\npart plus élevée",
    "in Arbeit": "en cours",
    "jeder Knoten\nfür sich": "chaque nœud\npour lui-même",
    "jeder Lohn bleibt\n365 Epochen gebunden": "chaque rémunération reste\nbloquée 365 époques",
    "kein Budget aus Premint für Audits,\nWerbung und Börsenlistung\nkleines Team, frühes Stadium\nkeine Apps, Token oder DeFi\nNamensgleichheit mit alten Token":
        "pas de budget de premint pour audits,\npublicité et cotations\npetite équipe, stade précoce\npas d'apps, de tokens ni de DeFi\nmême nom que d'anciens tokens",
    "kein TOY": "aucun TOY",
    "libp2p mit QUIC und Noise, Dandelion++, optional Tor": "libp2p avec QUIC et Noise, Dandelion++, Tor en option",
    "mehr Beitrag  →  mehr Rechte und mehr Anteil (freiwillig, jederzeit wechselbar)":
        "plus de contribution  →  plus de droits et une plus grande part (volontaire, modifiable à tout moment)",
    "mit eigenem Schlüssel\noder Notfall-Schlüssel": "avec sa propre clé\nou la clé d'urgence",
    "neue TOY in Mio.": "nouveaux TOY en millions",
    "nur eindeutig nachweisbare Verstöße\n→ Sperrzeit für Rechte, nie für Guthaben":
        "seulement les infractions prouvables\n→ blocage des droits, jamais des soldes",
    "rekursiver Beweis über die gesamte Vorgeschichte": "preuve récursive sur tout l'historique",
    "schlägt Blöcke vor,\ngrößter Anteil": "propose des blocs,\nla plus grande part",
    "totaler Fair Launch, keine Insider-Coins\nMitmachen ohne Guthaben, alte Geräte\nendgültig nach etwa 1 Minute\nkeine Smart-Contract-Risiken\nEthereum-kompatible Adressen":
        "fair launch total, pas de coins d'initiés\nparticiper sans fonds, vieux appareils\ndéfinitif après environ 1 minute\npas de risques de smart contracts\nadresses compatibles Ethereum",
    "unterschreibt mit,\nAnteil am Lohn": "cosigne,\npart des récompenses",
    "Übergabe möglich, sobald 21 neue Validatoren\nzwei Drittel des Stakes halten":
        "Passation possible dès que 21 nouveaux validateurs\ndétiennent les deux tiers du stake",
    "Übergabe noch\nnicht möglich": "Passation pas\nencore possible",
    "über die Hälfte aller Coins scheitert\nNetzwerkeffekte großer Ketten\nStablecoins dominieren Zahlungen\nAngriffe auf ein kleines Netz\nrechtliche Einordnung offen":
        "plus de la moitié des coins échouent\neffets de réseau des grandes chaînes\nles stablecoins dominent les paiements\nattaques contre un petit réseau\nstatut juridique encore ouvert",
}

TABLE = {"en": EN, "fr": FR}[LANG]
YEAR = {"en": "Year", "fr": "Année"}[LANG]
missing = set()


def translate(s):
    if not isinstance(s, str) or not s:
        return s
    if s in TABLE:
        return TABLE[s]
    m = re.fullmatch(r"Jahr (\d+)", s)
    if m:
        return f"{YEAR} {m.group(1)}"
    m = re.fullmatch(r"(\d+),(\d+)", s)
    if m:
        return f"{m.group(1)}.{m.group(2)}" if LANG == "en" else s
    m = re.fullmatch(r"(\d+) MB", s)
    if m:
        return s if LANG == "en" else f"{m.group(1)} Mo"
    if re.search(r"[A-Za-zÄÖÜäöüß]", s) and s not in ("M1", "ToyChain"):
        missing.add(s)
    return s


_orig_set_text = mt.Text.set_text
mt.Text.set_text = lambda self, s: _orig_set_text(self, translate(s))
_orig_savefig = mf.Figure.savefig


def _savefig(self, fname, *args, **kwargs):
    if isinstance(fname, str) and fname.startswith("abbildungen/"):
        fname = fname.replace("abbildungen/", f"abbildungen/{LANG}/", 1)
    return _orig_savefig(self, fname, *args, **kwargs)


mf.Figure.savefig = _savefig
os.makedirs(f"abbildungen/{LANG}", exist_ok=True)
runpy.run_path("figures.py")
if missing:
    print("UNTRANSLATED:", *sorted(missing), sep="\n  ")
