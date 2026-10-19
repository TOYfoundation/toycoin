"""ToyChain whitepaper, draft 0.2, French edition.
Run: python3 figures_i18n.py fr && python3 build_fr.py"""
import re

from wp_common import story, L, H, H2, P, MORE, B, T, BOX, FIG, toc_table, finish, set_words, set_filter
from wp_common import title, subtitle, center, abstract, small, h1
from reportlab.platypus import Paragraph, Spacer, CondPageBreak
from reportlab.lib.units import cm

set_words("Figure", "Pour en savoir plus :")
NB, NNB = " ", " "


def french_typography(t):
    t = re.sub(r"(?<=\d) (?=\d{3}\b)", NNB, t)
    t = re.sub(r" (?=[%:;?!»])", NB, t)
    t = t.replace("« ", "«" + NB)
    t = re.sub(r"^Figure (\d+): ", r"Figure \1" + NB + ": ", t)
    return t


set_filter(french_typography)
F = "abbildungen/fr/"

U = dict(
    btc_pdf="https://bitcoin.org/bitcoin.pdf",
    btc_fr="https://coingate.com/fr/gift-cards/articles/article/livre-blanc-du-bitcoin-explique",
    btc_supply="https://en.bitcoin.it/wiki/Controlled_supply",
    btc_size="https://www.spark.money/research/bitcoin-full-node-cost-requirements-2026",
    cbeci="https://ccaf.io/cbnsi/about/cbeci",
    eth_consensus="https://ethereum.org/fr/developers/docs/consensus-mechanisms/",
    eth_pos="https://ethereum.org/fr/developers/docs/consensus-mechanisms/pos/",
    eth_mev="https://ethereum.org/fr/developers/docs/mev/",
    eth_bridges="https://ethereum.org/fr/bridges/",
    eip1559="https://eips.ethereum.org/EIPS/eip-1559",
    eip155="https://eips.ethereum.org/EIPS/eip-155",
    eip5564="https://www.quicknode.com/guides/ethereum-development/wallets/how-to-use-stealth-addresses-on-ethereum-eip-5564",
    bip39="https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki",
    tendermint="https://arxiv.org/abs/1807.04938",
    bls="https://hackmd.io/@benjaminion/bls12-381",
    bls_wiki="https://en.wikipedia.org/wiki/BLS_digital_signature",
    merkle="https://www.bitpanda.com/fr/academy/tout-ce-que-vous-devez-savoir-sur-les-arbres-de-merkle",
    argon2="https://www.rfc-editor.org/rfc/rfc9106",
    vrf="https://www.rfc-editor.org/rfc/rfc9381",
    dandelion="https://arxiv.org/abs/1805.11060",
    mldsa="https://csrc.nist.gov/pubs/fips/204/final",
    pqk="https://spectrum.ieee.org/post-quantum-cryptography-nist",
    atomic="https://glossary.blockstream.com/atomic-swap",
    htlc="https://en.bitcoin.it/wiki/Hashed_Timelock_Contracts",
    mica="https://eur-lex.europa.eu/eli/reg/2023/1114/oj?locale=fr",
    esma="https://www.esma.europa.eu/publications-and-data/interactive-single-rulebook/mica",
    amf="https://www.amf-france.org/en/news-publications/news/end-mica-transitional-period-esma-sets-out-its-expectations-professionals-and-warns-retail-investors",
    amf_guide="https://www.amf-france.org/sites/institutionnel/files/pdf/59190/fr/Investir_dans_les_crypto-actifs_%28bitcoin%2C_etc_%29.pdf",
    inc="https://www.inc-conso.fr/node/15245",
    esas_warning="https://esma.europa.eu/press-news/esma-news/eu-supervisory-authorities-warn-consumers-risks-and-limited-protection-certain",
    metamask="https://support.metamask.io/configure/networks/how-to-add-a-custom-network-rpc",
    repro="https://reproducible-builds.org/",
    kani="https://rust-training.ferrous-systems.com/latest/book/kani",
    libp2p="https://docs.libp2p.io/",
    mit="https://opensource.org/license/mit",
    repo="https://github.com/Safehaigit/toycoin",
    market="https://dailyforex.com/forex-technical-analysis/2026/10/crypto-overview-18-october-2026/250826",
    cg_charts="https://www.coingecko.com/en/charts",
    dead="https://forklog.com/en/news/coingecko-reports-record-token-mortality-in-2025",
    bridgehacks="https://decrypt.co/106718/cross-chain-crypto-bridge-hacks-hit-2-billion-2022-chainalysis",
)

# ================================================================ Titre
story.append(Spacer(1, 0.3 * cm))
story.append(Paragraph("ToyChain et ToyCoin (TOY)", title))
story.append(Spacer(1, 4))
story.append(Paragraph("Un réseau de paiement léger, lancé équitablement,<br/>auquel chacun peut participer sans fonds",
                       subtitle))
story.append(Spacer(1, 10))
story.append(Paragraph("Projet ToyChain · " + L("github.com/Safehaigit/toycoin", U["repo"]), center))
story.append(Paragraph("Livre blanc provisoire · Version 0.2 · 18 octobre 2026 · édition française", center))
story.append(Spacer(1, 12))
BOX("<b>État : avant la phase de test.</b> ToyChain n'existe pour l'instant que sous forme de concept et d'un premier "
    "composant logiciel. Les types de base et les règles de création de la monnaie sont terminés et testés (jalon M1 "
    "sur 13). Il n'existe <b>encore ni testnet, ni mainnet, ni TOY négociables</b>. Tous les chiffres sont des valeurs "
    "de départ qui seront mesurées et ajustées sur le testnet. Ce document n'est ni un conseil en investissement ni une "
    "offre d'achat de quoi que ce soit. Les soldes du testnet n'ont aucune valeur et ne seront jamais repris dans un "
    "futur mainnet. Les termes soulignés en bleu sont des liens vers des explications complémentaires. L'édition "
    "allemande fait foi.")

P("<b>Résumé.</b> ToyChain est une blockchain indépendante dotée de sa propre monnaie, le ToyCoin (TOY), limitée à "
  "100 millions d'unités. Son lancement est entièrement équitable : le premier bloc ne contient pas un seul TOY, il "
  "n'y a ni réserve pour les fondateurs, ni prévente, ni attribution spéciale. Tous les TOY sont créés peu à peu selon "
  "une règle fixe et publiquement vérifiable, comme pour " + L("Bitcoin", U["btc_pdf"]) + ". Contrairement à Bitcoin, "
  "la sécurité ne provient pas de la consommation d'électricité, mais de participants qui bloquent des TOY en dépôt "
  "et signent ensemble les blocs. Un bloc est définitif après environ une minute. Chaque nœud se contente de 256 Mo de "
  "données et fonctionne aussi sur de vieux téléphones et sur un Raspberry Pi. On peut participer sans fonds ; celui "
  "qui contribue davantage obtient progressivement plus de droits. ToyChain renonce délibérément aux smart contracts, "
  "aux tokens et aux NFT : chaque fonction est intégrée, facile à vérifier et ne peut jamais toucher aux fonds "
  "d'autrui. Les adresses sont les mêmes que sur Ethereum, si bien que les wallets connus continuent de fonctionner. "
  "Le document se termine par une évaluation honnête des chances du projet sur le marché des cryptomonnaies.", abstract)

story.append(Spacer(1, 4))
toc_table(["1 Introduction", "2 ToyChain en un coup d'œil", "3 Notions de base expliquées simplement",
           "4 Architecture", "5 Lancement équitable et masse monétaire", "6 Participer : niveaux, quota, loterie",
           "7 Se mettre d'accord dans le réseau", "8 La phase de démarrage", "9 Des nœuds légers pour vieux appareils",
           "10 Sécurité pour les utilisateurs", "11 Wallet, ToyScan et autres chaînes", "12 Cas d'usage",
           "13 Points communs et différences", "14 Avantages", "15 Risques et questions ouvertes",
           "16 Chances sur le marché des cryptos", "17 État et feuille de route", "18 Conclusion",
           "Annexes : valeurs de départ, liens, sources"], "Sommaire")

# ================================================================ 1
H("Introduction")
P("En 2008, Bitcoin a montré que des personnes peuvent se mettre d'accord, sans banque, sur qui possède quel argent. "
  "Depuis, des millions de crypto-actifs sont apparus. Pour beaucoup d'entre eux, les fondateurs et les premiers "
  "investisseurs ont reçu une grande partie des coins avant même le lancement (premint). D'autres exigent de grands "
  "centres de calcul, beaucoup d'électricité ou des disques de plusieurs centaines de gigaoctets, si bien que les "
  "gens ordinaires peuvent à peine vérifier le réseau eux-mêmes. La chaîne Bitcoin complète pèse aujourd'hui environ "
  "760 Go. Et de nombreuses chaînes sont devenues si polyvalentes grâce aux smart contracts que des erreurs dans le "
  "code d'autrui entraînent régulièrement des pertes.")
P("ToyChain tente de réaliser l'idée fondamentale de Bitcoin, à savoir une quantité fixe, un lancement équitable et "
  "des règles ouvertes, de manière légère et adaptée au quotidien avec les méthodes d'aujourd'hui. Les questions "
  "directrices sont : chacun peut-il participer avec un vieil appareil ? Chacun peut-il vérifier lui-même les règles ? "
  "Vos propres fonds restent-ils à vous en toutes circonstances ? Ce livre blanc explique les idées en langage simple. "
  "Les règles exactes figurent dans la spécification technique (version 0.3, en allemand) du "
  + L("dépôt public", U["repo"]) + ".")
MORE(L("Livre blanc de Bitcoin (anglais)", U["btc_pdf"], "Bases"),
     L("Le livre blanc de Bitcoin expliqué", U["btc_fr"], "Bases"),
     L("Taille d'un nœud Bitcoin en 2026 (anglais)", U["btc_size"], "Bases"))

# ================================================================ 2
H("ToyChain en un coup d'œil")
T([["Caractéristique", "ToyChain (valeurs de départ prévues)"],
   ["Monnaie", "ToyCoin, symbole TOY, divisible en 18 décimales"],
   ["Offre maximale", "100 000 000 TOY, inscrite dans le code comme règle immuable"],
   ["Lancement", "Fair launch total : premier bloc sans solde, pas de premint, pas de réserve"],
   ["Nouveaux TOY", "par époque (au moins 24 heures) 0,0505 % de la quantité restante, ajustée par un facteur m "
                    "entre 0,5 et 2,0 ; première année environ 16,84 millions, moitié après environ 3,76 ans, 99 % "
                    "après environ 25 ans"],
   ["Accord (consensus)", "Preuve d'enjeu avec un comité d'au plus 128 validateurs, une signature commune, "
                          "définitif après 1 bloc"],
   ["Temps de bloc", "60 secondes, plus tôt si la file d'attente est pleine, au plus tôt après 10 secondes"],
   ["Stockage par nœud", "Objectif 256 Mo, aussi sur de vieux appareils Android et le Raspberry Pi"],
   ["Comptes", "comme Ethereum : mêmes adresses, même phrase de 12 ou 24 mots"],
   ["Fonctions", "uniquement des types de transaction intégrés, pas de smart contracts, pas de tokens, pas de NFT"],
   ["Frais", "10 à 100 transactions gratuites par compte et par jour, au-delà de petits frais ou du travail de calcul"],
   ["Langage", "Rust, un noyau commun pour toutes les plateformes"],
   ["Licence", L("MIT", U["mit"]) + " (logiciel libre)"]],
  [4.0, 12.4])

# ================================================================ 3
H("Notions de base expliquées simplement")
P("Si vous n'avez jamais eu affaire aux cryptomonnaies, voici les notions les plus importantes. Les sections suivantes "
  "s'appuient sur elles. Un clic sur un terme bleu mène à une bonne explication sur le web.")
T([["Notion", "En termes simples"],
   ["Blockchain", "Un registre commun tenu par de nombreux ordinateurs en même temps. Les nouvelles écritures sont "
                  "ajoutées par paquets (blocs) ; chaque bloc renvoie au précédent de façon infalsifiable."],
   ["Nœud", "Un appareil qui exécute le logiciel ToyChain. Il vérifie lui-même que toutes les écritures respectent les "
            "règles et n'a besoin de faire confiance à personne pour cela."],
   ["Wallet", "L'application qui garde vos clés et signe les virements. Sur ToyChain, le wallet TOYPAY est aussi un "
              "nœud."],
   [L("Clé et phrase", U["bip39"], "Comptes et clés"),
    "Une clé secrète prouve qu'un compte vous appartient. Elle peut être restaurée à partir de 12 ou 24 mots. Qui "
    "connaît les mots peut disposer de l'argent."],
   ["Signature", "Une signature numérique. Chacun peut vérifier qu'elle est authentique, mais personne ne peut la "
                 "falsifier sans la clé."],
   [L("Consensus", U["eth_consensus"], "Consensus"), "La procédure par laquelle tous les nœuds s'accordent sur le même "
                                                     "bloc suivant."],
   [L("Stake (enjeu)", U["eth_pos"], "Consensus"), "Des TOY que l'on bloque volontairement en dépôt. Qui respecte les "
                                                   "règles reçoit des récompenses. Qui triche de façon prouvée perd "
                                                   "une partie du dépôt."],
   ["Validateur", "Un nœud avec du stake qui propose et cosigne des blocs."],
   [L("Arbre de Merkle", U["merkle"], "Stockage"), "Une sorte de table des matières faite de sommes de contrôle. Une "
                                                   "petite preuve montre qu'une entrée en fait partie sans tout "
                                                   "stocker."],
   ["Époque", "Une section de la chaîne d'au moins 1 440 blocs et d'au moins 24 heures. Beaucoup de règles comptent en "
              "époques, par exemple l'émission de nouveaux TOY."],
   ["Testnet", "Une chaîne d'essai dont les coins ne valent rien. On y cherche les erreurs avant de passer aux choses "
               "sérieuses."]],
  [3.6, 12.8])

# ================================================================ 4
H("Architecture")
P("ToyChain se compose de couches superposées. Chacune a une tâche clairement délimitée. Toutes les couches se "
  "trouvent dans un programme Rust commun qui fonctionne de la même façon sur Android, Windows, Linux et le Raspberry "
  "Pi. Rien ne doit donc être écrit ni vérifié deux fois.")
FIG(F + "f_arch.png", "Les couches de ToyChain. La couche de preuves (en pointillés) suivra en phase 2.", 14.8)
MORE(L("libp2p, la base réseau (anglais)", U["libp2p"], "Technique"),
     L("Dandelion++ : qui a envoyé ? (anglais)", U["dandelion"], "Technique"))

# ================================================================ 5
H("Lancement équitable et masse monétaire")
H2("5.1 Pas un seul TOY dans le premier bloc")
P("Le premier bloc (bloc de genèse) de ToyChain ne contient <b>aucun solde</b>. Personne, pas même les développeurs, "
  "ne possède de TOY au lancement. Il n'y a ni prévente, ni réserve, ni part des fondateurs. C'est plus strict que la "
  "plupart des nouveaux projets et fidèle à l'esprit de Bitcoin. Cette règle fait partie de la « constitution » "
  "(section 10.3) et ne peut être modifiée par aucune mise à jour.")
H2("5.2 Comment naissent les nouveaux TOY")
P("À chaque époque, une fraction fixe de la quantité non encore émise est créée : 0,0505 % de la quantité restante. "
  "Lors de la première époque, cela représente exactement 50 500 TOY. Comme la quantité restante diminue, l'émission "
  "baisse régulièrement au lieu d'être divisée par deux d'un coup tous les quatre ans comme chez Bitcoin. Après "
  "environ 3,76 ans, la moitié est émise, après environ 25 ans environ 99 %. Le maximum n'est jamais atteint ni "
  "dépassé. Des tests le prouvent déjà : même avec une émission deux fois plus rapide pendant 100 ans, le total reste "
  "en dessous.")
FIG(F + "f_emission.png", "Part émise de l'offre maximale au fil du temps. ToyChain baisse régulièrement, Bitcoin "
                          "par paliers. Après 25 ans, les deux sont à environ 99 %.")
FIG(F + "f_per_year.png", "Nouveaux TOY par an selon le plan (m = 1). Chaque année, environ 17 % de moins sont créés "
                          "que l'année précédente.")
MORE(L("Comment Bitcoin limite son offre (anglais)", U["btc_supply"], "Masse monétaire"))
H2("5.3 Le facteur m : ajusté, mais bridé")
P("L'émission est multipliée par un facteur m compris entre 0,5 et 2,0. Il augmente légèrement lorsque le réseau est "
  "<i>réellement</i> utilisé (transactions payantes ou couvertes par du travail de calcul) et lorsque de nombreux "
  "validateurs cosignent de façon fiable. Les transactions gratuites ne comptent pas, sinon on pourrait gonfler "
  "l'émission avec du spam. Une composante de calendrier rattrape les écarts, et m ne peut varier que de 0,05 au plus "
  "par époque. Dans les simulations, l'émission de la première année s'écarte du plan d'environ 2 % au maximum. m "
  "ne déplace donc que le <i>moment</i> où les TOY sont créés, pas leur <i>nombre</i> total.")
H2("5.4 Qui reçoit les nouveaux TOY")
P("Chaque émission est répartie de façon fixe : <b>50 %</b> à tous ceux qui sécurisent le réseau, <b>40 %</b> à une "
  "loterie du travail à laquelle tout appareil peut participer sans fonds (section 6.3) et <b>10 %</b> à un pot de "
  "remboursement qui paie les transactions gratuites de tous. Pendant la phase de démarrage, la part de sécurité va "
  "aux validateurs de départ et diminue jusqu'à zéro ; ce qu'ils ne reçoivent pas va à la loterie. Tous les calculs "
  "se font en nombres entiers dans la plus petite unité (10<super>-18</super> TOY). Les restes d'arrondi vont au pot "
  "de remboursement, si bien que chaque époque crée exactement la quantité prévue.")
FIG(F + "f_split.png", "Répartition des nouveaux TOY. Pendant le démarrage, la rémunération des validateurs de "
                       "départ passe peu à peu à la loterie du travail ; après la passation, tous les stakers la "
                       "reçoivent.")
H2("5.5 Aucun TOY n'est jamais détruit")
P("Beaucoup de chaînes « brûlent » des frais ou des pénalités, Ethereum par exemple une partie de chaque frais selon "
  + L("EIP-1559", U["eip1559"], "Frais") + ". Pas ToyChain : les pénalités et les dons vont au pot de remboursement "
  "et profitent ainsi aux utilisateurs. Une règle de contrôle (I0) garantit dans <b>chaque</b> bloc que la somme de "
  "tous les soldes est exactement égale à la somme de tous les TOY émis. Un bloc qui viole cette règle est invalide.")

# ================================================================ 6
H("Participer : niveaux, quota et loterie du travail")
H2("6.1 Niveaux de participation")
P("Chacun décide de sa contribution. Au premier démarrage, le wallet mesure l'appareil et propose un niveau adapté. "
  "Il ne propose un niveau supérieur que si l'appareil peut l'assumer.")
FIG(F + "f_tiers.png", "Les cinq niveaux de participation. Même le niveau 0 ne demande aucun fonds.", 15.0)
P("Les TOY bloqués restent la propriété de l'utilisateur et peuvent être libérés après 21 époques. Pour que personne "
  "ne devienne trop puissant, le stake d'un participant ne compte que jusqu'à 5 % (à partir de 100 validateurs, 1 %) "
  "du stake total. Les récompenses augmentent de façon linéaire avec le stake, auxquelles s'ajoute un montant de base "
  "égal pour chaque validateur actif. Répartir son stake sur de nombreux comptes n'apporte donc aucun avantage.")
MORE(L("Qu'est-ce que la preuve d'enjeu ? (ethereum.org)", U["eth_pos"], "Consensus"))
H2("6.2 Transactions gratuites et frais")
P("Chaque compte activé dispose d'un quota gratuit par époque : 100 transactions quand le réseau est calme, au moins "
  "10 en période chargée. Cela suffit pour le quotidien. Au-delà, on paie de petits frais, qui augmentent vite en cas "
  "de nombreuses transactions dans le même bloc, ou on joint une preuve de travail. Le réseau reste ainsi gratuit pour "
  "les utilisateurs normaux, tandis que le spam devient coûteux. Les transactions gratuites occupent au plus la "
  "moitié d'un bloc et sont payées par le pot de remboursement, auquel chacun peut aussi faire des dons.")
H2("6.3 La loterie du travail")
P("Pour que les personnes sans TOY puissent aussi obtenir de nouveaux coins, il existe une loterie du travail de "
  "calcul. L'appareil calcule des billets avec " + L("Argon2id", U["argon2"], "Technique") + ", qui demande "
  "délibérément beaucoup de mémoire (16 Mio). Le matériel spécialisé et les fermes de cartes graphiques n'ont ainsi "
  "presque aucun avantage sur un téléphone ou un PC. Jusqu'à 64 gagnants sont tirés au sort équitablement par bloc, "
  "chaque gain étant d'au moins 0,2 TOY. Qui calcule davantage a plus de billets ; la récompense attendue est "
  "proportionnelle au travail. L'appareil ne calcule que s'il y est autorisé : sur le chargeur, en Wi-Fi et tant "
  "qu'il reste frais. À titre de comparaison, le réseau Bitcoin consomme en permanence autant d'électricité qu'un pays "
  "entier, comme le montre " + L("l'indice d'électricité de Cambridge", U["cbeci"], "Énergie") + ".")

# ================================================================ 7
H("Se mettre d'accord dans le réseau")
H2("7.1 Comment naît un bloc")
P("Un comité d'au plus 128 validateurs est recomposé à chaque époque. Pour chaque bloc, un hasard vérifiable ("
  + L("VRF", U["vrf"], "Technique") + ") désigne celui qui propose le bloc. Les autres votent en deux tours, selon une "
  "méthode éprouvée de type " + L("Tendermint", U["tendermint"], "Consensus") + ". Dès que deux tiers ont approuvé, "
  "le bloc est <b>définitif</b>. Il ne peut plus être annulé. Avec Bitcoin, on attend en général environ une heure "
  "avant qu'un paiement soit considéré comme sûr. Tous les votes sont regroupés grâce aux "
  + L("signatures BLS", U["bls_wiki"], "Technique") + " en une seule signature de 96 octets seulement, qu'un téléphone "
  "peut vérifier en quelques millisecondes.")
FIG(F + "f_block.png", "Déroulement d'un bloc en cinq étapes. Après l'étape 5, le paiement est définitif.")
H2("7.2 Ordre équitable")
P("Celui qui construit un bloc ne peut pas choisir librement l'ordre des transactions. Il est fixé : selon les frais, "
  "et en cas d'égalité selon un nombre aléatoire connu seulement avec le bloc. Chaque nœud le vérifie. Plus tard, une "
  "obligation d'inclusion garantira que les transactions vues par de nombreux validateurs entrent vraiment dans le "
  "bloc suivant. Comme ToyChain n'a pas de contrats d'échange, le " +
  L("« passage devant » (MEV)", U["eth_mev"], "Consensus") + " connu sur Ethereum disparaît en grande partie.")
H2("7.3 Des pénalités à la mesure du dommage")
P("Celui qui émet deux votes contradictoires pour le même bloc (double signature) perd une partie de son stake. Cela "
  "arrive souvent par erreur, par exemple lorsque la même clé tourne sur deux appareils. C'est pourquoi la pénalité "
  "dépend du nombre de ceux qui le font en même temps. Le logiciel protège aussi contre les incidents : il retient "
  "durablement ce qu'il a signé et reste muet après un changement d'appareil si sa clé est déjà active ailleurs. "
  "Pendant la phase de démarrage, la réduction est limitée à 5 %.")
FIG(F + "f_slashing.png", "Réduction en cas de double signature : max(1 %, 3 × part concernée en même temps), au plus "
                          "100 %.")

# ================================================================ 8
H("La phase de démarrage")
P("Un lancement équitable sans premint pose un problème de l'œuf et de la poule en preuve d'enjeu : au départ, "
  "personne n'a de TOY, donc personne n'a de stake pour sécuriser le réseau. ToyChain résout ce problème de façon "
  "ouverte et limitée dans le temps :")
B(["<b>21 validateurs de départ</b>, gérés par des bénévoles aux opérateurs et lieux aussi variés que possible, "
   "sécurisent le réseau avec un poids égal. Leurs noms et revenus sont publics dans ToyScan.",
   "<b>Rémunération décroissante :</b> au début, ils reçoivent ensemble la moitié de l'émission. Elle baisse de façon "
   "linéaire jour après jour et atteint zéro après 180 jours. Un plafond empêche qu'une émission plus rapide ou des "
   "blocs volontairement lents rapportent davantage. Au total, cela représente au plus environ 2,22 millions de TOY "
   "(environ 2,2 % de l'offre maximale).",
   "<b>Rémunération bloquée :</b> chaque rémunération reste bloquée 365 époques (environ un an) et répond des fautes.",
   "<b>Passation stricte :</b> la phase de démarrage se termine au plus tôt après 90 jours, et seulement lorsqu'au "
   "moins 21 nouveaux validateurs avec 1 000 TOY chacun, bloqués depuis 30 époques, détiennent ensemble les deux tiers "
   "du stake effectif. Ensuite, les validateurs de départ perdent tout statut particulier."])
FIG(F + "f_bootstrap.png", "Rémunération des validateurs de départ par jour et fenêtre de temps pour la passation.")
P("Pour être honnête : la phase de démarrage est la partie où ToyChain a le plus besoin de faire confiance à quelques "
  "personnes. Aucun protocole ne peut prouver que de « nouveaux » validateurs sont vraiment d'autres personnes. Les "
  "règles rendent toutefois une prise de contrôle coûteuse et visible de tous.")

# ================================================================ 9
H("Des nœuds légers pour de vieux appareils")
P("Stocker une chaîne Bitcoin ou Ethereum complète demande aujourd'hui des centaines de gigaoctets. ToyChain se fixe "
  "un objectif de <b>256 Mo par nœud standard</b>. C'est possible parce qu'un nœud ne stocke que les soldes actuels "
  "de tous les comptes actifs et une courte fenêtre des derniers blocs. Des nœuds d'archive bénévoles conservent les "
  "blocs plus anciens. Dans une phase ultérieure, tout l'historique doit être « replié » dans une petite preuve "
  "cryptographique, si bien que les nouveaux nœuds n'auront plus à faire confiance à personne.")
FIG(F + "f_budget.png", "Répartition des 256 Mo. Chaque structure de stockage a son propre test de taille.")
P("Pour que 100 millions de comptes et plus trouvent quand même leur place, il existe des <b>comptes actifs et "
  "dormants</b>. Les comptes inutilisés depuis longtemps sont mis en sommeil hors de la mémoire si nécessaire, mais "
  "<b>jamais supprimés</b>. Leur contenu est garanti par une " + L("preuve de Merkle", U["merkle"]) + " d'un peu moins "
  "de 1 Ko que le wallet conserve lui-même. À la transaction suivante, le compte est réveillé automatiquement, avec "
  "tout son solde.")
FIG(F + "f_dormant.png", "Comptes actifs et dormants. Aucun compte n'est jamais perdu.", 15.0)
P("Pour que personne ne remplisse la mémoire avec des millions de comptes vides, 0,1 TOY est bloqué comme <b>dépôt "
  "d'activation</b> lors de la première réception. Le dépôt reste la propriété de l'utilisateur, n'est jamais réduit "
  "et est intégralement rendu à la clôture du compte. Il ne peut que baisser plus tard, jusqu'à 0,01 TOY au minimum.")

# ================================================================ 10
H("Sécurité pour les utilisateurs")
H2("10.1 Protection volontaire du compte")
P("Chacun peut protéger davantage son propre compte. Ces réglages ne s'appliquent que parce que le propriétaire les a "
  "définis lui-même. Personne d'autre ne peut les définir.")
B(["<b>Délai de coffre-fort :</b> les retraits importants ne prennent effet qu'après un délai d'attente et peuvent "
   "être annulés pendant ce temps. Un voleur ne peut donc pas vider le compte immédiatement.",
   "<b>Clé d'urgence et adresse de sécurité :</b> une seconde clé conservée à part ne peut rien dépenser, mais elle "
   "peut stopper les retraits en attente et mettre tous les fonds à l'abri sur une adresse définie à l'avance.",
   "<b>Auto-blocage :</b> téléphone perdu ? Son propre compte peut être bloqué jusqu'à 30 époques.",
   "<b>Adresse d'héritier :</b> si un compte reste très longtemps sans aucune activité (au moins 180 époques), une "
   "adresse d'héritier choisie par le propriétaire peut le reprendre. Le propriétaire est prévenu plusieurs fois "
   "auparavant et peut annuler à tout moment."])
FIG(F + "f_vault.png", "Délai de coffre-fort : pendant l'attente, le propriétaire peut stopper un retrait suspect.",
    15.0)
H2("10.2 Des règles de protection mesurées")
P("Les attaques doivent être stoppées tôt sans toucher les utilisateurs honnêtes. C'est pourquoi il existe trois "
  "niveaux. La chaîne elle-même ne sanctionne que les infractions <b>clairement prouvables</b>, comme une double "
  "signature ou un bloc falsifié. La conséquence est un blocage qui retire des droits, par exemple le quota gratuit, "
  "mais <b>jamais des fonds</b>. Les récidives doublent le blocage, jusqu'à 64 époques au plus.")
FIG(F + "f_levels.png", "Les trois niveaux des règles de protection. Seul le niveau A agit dans la chaîne, et "
                        "uniquement sur les droits.", 15.0)
H2("10.3 La constitution")
P("Certaines règles sont si fondamentales qu'elles sont fixées dans le code et ne peuvent être modifiées par aucune "
  "mise à jour ni aucun vote futur : le maximum de 100 millions de TOY, le lancement sans solde, la formule "
  "d'émission, l'interdiction de détruire des TOY, la durée minimale d'une époque, l'absence de tokens et de smart "
  "contracts et surtout : <b>aucune règle ne peut prendre, geler ou redistribuer les TOY d'un compte.</b> Pour être "
  "honnête, quiconque diffuse son propre logiciel peut créer une chaîne dissidente. C'est pourquoi le wallet et "
  "ToyScan vérifient eux-mêmes la constitution et avertissent si une scission la viole. Chaque version du logiciel "
  "doit être construite de façon " + L("reproductible", U["repro"], "Technique") + ", afin que chacun puisse vérifier "
  "qu'elle provient bien du code publié.")
H2("10.4 Une cryptographie éprouvée")
P("ToyChain n'invente pas ses propres procédés cryptographiques. Il utilise des standards répandus et bien examinés : "
  "secp256k1 et Keccak-256 comme Ethereum pour les comptes, " + L("BIP-39", U["bip39"]) + " pour la phrase de mots, "
  + L("BLS12-381", U["bls"], "Technique") + " pour les signatures communes des validateurs, BLAKE3 pour les sommes de "
  "contrôle, Argon2id pour le travail de calcul et le chiffrement du wallet sur l'appareil. Chaque signature contient "
  "l'identifiant de la chaîne (" + L("EIP-155", U["eip155"], "Comptes et clés") + "), afin qu'elle ne puisse pas être "
  "réutilisée sur une autre chaîne. Tous les procédés se trouvent derrière des interfaces remplaçables. Une clé "
  "supplémentaire volontaire selon le standard NIST " + L("ML-DSA", U["mldsa"], "Technique") + " doit aussi résister "
  "aux " + L("ordinateurs quantiques", U["pqk"], "Technique") + ". Les règles de calcul les plus importantes sont en "
  "outre vérifiées mathématiquement avec l'outil de preuve " + L("Kani", U["kani"], "Technique") + ".")

# ================================================================ 11
H("Wallet TOYPAY, ToyScan et autres chaînes")
P("Le <b>wallet TOYPAY</b> est à la fois porte-monnaie et nœud. Il fonctionne sur Android à partir d'environ 1 Go de "
  "mémoire vive, sur Windows, Linux et le Raspberry Pi. Un mode énergie règle automatiquement la contribution de "
  "l'appareil selon la batterie, le chargeur, la température et le réseau. L'interface est prévue en 45 langues et "
  "propose quatre modes, du débutant à l'expert. Les avertissements de sécurité apparaissent dans tous les modes.")
P("L'explorateur <b>ToyScan</b> rend tout public : combien de TOY ont déjà été émis, le facteur m actuel, l'état de "
  "la phase de démarrage et chaque blocage avec sa justification. Un feu tricolore (vert, jaune, rouge) résume neuf "
  "critères, par exemple si l'offre maximale est respectée, si le stake est trop concentré ou si la phase de démarrage "
  "dure trop longtemps. ToyScan n'a jamais accès aux clés.")
P("Comme les adresses sont les mêmes que sur Ethereum, le wallet et ToyScan affichent aussi les soldes de la même "
  "adresse sur d'autres chaînes EVM comme Ethereum ou Polygon. Des wallets connus comme MetaMask peuvent "
  + L("ajouter ToyChain comme réseau", U["metamask"], "Wallet") + ". Il n'y a délibérément pas de "
  + L("pont (bridge)", U["eth_bridges"], "Autres chaînes") + " entre les chaînes : en 2022, selon Chainalysis, environ "
  "deux tiers de tous les fonds cryptos volés provenaient " + L("d'attaques contre des ponts", U["bridgehacks"],
                                                                "Autres chaînes") + ". À la place, un "
  + L("échange atomique", U["atomic"], "Autres chaînes") + " est prévu : deux personnes échangent directement, "
  "sécurisées par un " + L("verrou à hachage et délai", U["htlc"], "Autres chaînes") + ". Soit chacune reçoit sa "
  "contrepartie, soit chacune récupère son argent.")
FIG(F + "f_swap.png", "Échange atomique entre des TOY et un coin d'une autre chaîne, sans intermédiaire.")
BOX("<b>Remarque importante sur les tokens homonymes :</b> sur Polygon et Ethereum, il existe d'anciens tokens nommés "
    "ToyCoin avec le symbole TOY. Ils ne sont <b>pas</b> liés au TOY natif de ToyChain et ne peuvent pas être échangés "
    "contre lui. Le wallet et ToyScan le signalent clairement.")

# ================================================================ 12
H("Cas d'usage")
B(["<b>Paiements du quotidien</b> entre particuliers, avec libellé, virement groupé jusqu'à 32 destinataires et "
   "caractère définitif après environ une minute. Les petits montants sont possibles sans frais grâce au quota gratuit.",
   "<b>Participer sans capital :</b> un vieux téléphone ou un Raspberry Pi devient un nœud et peut gagner ses premiers "
   "TOY grâce à la loterie du travail.",
   "<b>Associations, familles et petites entreprises :</b> comptes à plusieurs signatures, séquestre avec arbitre, "
   "autorisations d'abonnement avec plafond fixe et clés de session pour les apps (après le premier testnet).",
   "<b>Prévoyance :</b> délai de coffre-fort, clé d'urgence et adresse d'héritier protègent contre le vol et la perte.",
   "<b>Des dons pour tous :</b> chacun peut donner au pot de remboursement qui paie les transactions gratuites de tous.",
   "<b>Confidentialité si besoin :</b> des adresses de réception à usage unique selon "
   + L("EIP-5564", U["eip5564"], "Comptes et clés") + " cachent qui a été payé (prévu pour le jalon M13).",
   "<b>Apprendre et chercher :</b> code ouvert, testnet en une commande et données publiques font de ToyChain un objet "
   "d'apprentissage pour les écoles, les universités et les curieux.",
   "<b>Envisageable plus tard :</b> intégrations pour commerçants, mode caisse, paiements hors ligne et lien avec le "
   "jeu TOYQUEST. Ces idées ne font explicitement pas partie de la première phase."])

# ================================================================ 13
H("Points communs et différences")
T([["", "Bitcoin", "Ethereum", "ToyChain (prévu)"],
   ["Lancement", "équitable, sans premint", "prévente et part de la fondation", "équitable, genèse sans aucun solde"],
   ["Offre maximale", "environ 21 millions de BTC", "pas de plafond fixe", "100 millions de TOY, immuable"],
   ["Émission", "divisée par deux environ tous les 4 ans", "selon les besoins, une partie des frais est brûlée",
    "baisse régulière, facteur 0,5 à 2,0, jamais brûlé"],
   ["Sécurisé par", "travail de calcul (preuve de travail)", "stake (preuve d'enjeu), 32 ETH par validateur",
    "stake par paliers de 10, 100, 1 000 TOY plus une phase de démarrage ouverte"],
   ["Définitif après", "environ 1 heure (6 confirmations)", "environ 13 minutes", "1 bloc, environ 1 minute"],
   ["Stockage nœud complet", "environ 760 Go", "centaines de Go à plusieurs To", "objectif 256 Mo"],
   ["Programmabilité", "scripts simples", "smart contracts, tokens, NFT", "uniquement des fonctions intégrées"],
   ["Comptes", "modèle UTXO", "modèle de comptes", "modèle de comptes, adresses Ethereum"],
   ["Frais", "toujours, selon le marché", "toujours, frais de base plus pourboire",
    "10 à 100 gratuits par jour, puis frais ou travail"],
   ["Participer sans fonds", "seulement en faisant tourner un nœud", "seulement en faisant tourner un nœud",
    "oui, y compris la loterie du travail"]],
  [3.5, 3.6, 4.2, 5.1])
P("<b>En commun avec Bitcoin</b>, ToyChain a le lancement équitable, l'offre maximale fixe, l'émission décroissante "
  "sur environ 25 ans, l'absence d'autorité centrale et le code ouvert. <b>En commun avec Ethereum</b>, elle a les "
  "adresses, la phrase de mots, le format de signature, la preuve d'enjeu et l'idée de frais de base qui augmentent "
  "sous la charge. <b>Nouvelle</b> est la combinaison de la participation sans fonds, du quota gratuit, de la limite de "
  "stockage de 256 Mo, des comptes dormants, de la protection volontaire des comptes et d'une constitution qui exclut "
  "toute ingérence dans les fonds d'autrui.")

# ================================================================ 14
H("Avantages")
B(["<b>Équité :</b> personne ne reçoit de TOY en cadeau ; toutes les règles sont publiques et les mêmes pour tous.",
   "<b>Accessibilité :</b> participer avec de vieux appareils, sans fonds et sans connaissances spécialisées, en 45 "
   "langues.",
   "<b>Finalité rapide :</b> un paiement est sûr après environ une minute.",
   "<b>Faible consommation d'énergie :</b> pas de course à la puissance de calcul. La loterie du travail ne fonctionne "
   "que lorsque l'électricité est disponible de toute façon et ne récompense le travail que proportionnellement.",
   "<b>La simplicité comme sécurité :</b> sans smart contracts, aucun programme tiers ne peut geler des fonds ou les "
   "perdre à cause d'erreurs.",
   "<b>Protection de la propriété :</b> les blocages ne retirent que des droits, jamais des fonds. Les comptes ne sont "
   "jamais perdus. Les TOY ne sont jamais détruits.",
   "<b>Transparence :</b> ToyScan, des rapports de transparence mensuels dès le testnet et des versions logicielles "
   "reproductibles.",
   "<b>Compatibilité :</b> les adresses Ethereum et les wallets connus fonctionnent."])

# ================================================================ 15
H("Risques et questions ouvertes")
P("Un livre blanc honnête nomme aussi ce qui peut mal tourner. Les principaux risques du point de vue actuel :")
B(["<b>Stade précoce :</b> ToyChain n'a encore jamais fonctionné. Des erreurs de conception ou de code sont probables "
   "et doivent être trouvées sur le testnet. Deux examens indépendants ont déjà révélé des faiblesses, qui ont été "
   "corrigées. D'autres examens et un audit externe sont obligatoires avant tout mainnet.",
   "<b>Petit réseau, faible sécurité :</b> une nouvelle chaîne est vulnérable au début, car peu de participants sont "
   "plus faciles à mettre en minorité. La sécurité ne naît que de nombreux participants indépendants.",
   "<b>Confiance pendant le démarrage :</b> les 21 validateurs de départ portent beaucoup de responsabilités pendant "
   "les premiers mois et reçoivent une rémunération limitée. Reste à voir si la passation à de nouveaux validateurs "
   "réussira.",
   "<b>Limite de stockage :</b> que 256 Mo suffisent sur des années dépend de l'utilisation et de la future couche de "
   "preuves. Sinon, ce sera présenté ouvertement, chiffres à l'appui. Les nœuds plus grands auraient alors besoin de "
   "plus de stockage.",
   "<b>Briques moins éprouvées :</b> pour certains procédés (comme le hasard vérifiable et la bibliothèque de "
   "consensus), il existe moins de bibliothèques Rust bien examinées. Elles seront implémentées strictement selon le "
   "standard et examinées en externe.",
   "<b>Travail de calcul :</b> on mesurera sur le testnet si 16 Mio de mémoire suffisent à tenir le matériel "
   "spécialisé à l'écart.",
   "<b>Droit et réglementation :</b> une cryptomonnaie touche au droit européen (" + L("MiCA", U["mica"], "Droit") +
   "), au droit fiscal et peut-être au droit des jeux d'argent (le terme « loterie »). Un examen juridique est "
   "obligatoire avant tout mainnet. L'adresse d'héritier ne remplace pas le droit des successions.",
   "<b>Erreurs d'utilisateur :</b> qui perd ou divulgue sa phrase de mots perd son argent. La protection du compte et "
   "les avertissements réduisent ce risque, sans le supprimer entièrement.",
   "<b>Valeur :</b> les TOY n'ont aucune valeur garantie. Qu'il existe un jour un marché reste ouvert (section 16). "
   "Personne ne devrait engager de l'argent qu'il ne peut pas se permettre de perdre.",
   "<b>Confusion et fraude :</b> d'anciens tokens homonymes sur d'autres chaînes peuvent semer la confusion ou servir à "
   "des fraudes. L'AMF met en garde de façon générale contre les " + L("risques des crypto-actifs", U["amf_guide"],
                                                                       "Protection des consommateurs") + "."])
MORE(L("Guide AMF : investir dans les crypto-actifs", U["amf_guide"], "Protection des consommateurs"),
     L("INC : que savoir avant d'investir ?", U["inc"], "Protection des consommateurs"))

# ================================================================ 16
H("Chances sur le marché des cryptos : une évaluation")
P("Ce chapitre tente une évaluation sobre. Ce n'est ni une prévision ni une recommandation d'investissement. Les "
  "chiffres du marché datent de mi-octobre 2026 et évoluent vite.")
H2("16.1 La situation de départ")
P("Le 18 octobre 2026, l'ensemble du marché des cryptos valait environ 2 700 milliards de dollars américains, dont "
  "environ 60 % pour Bitcoin. L'ambiance pour les petits coins était morose : sur les 100 plus grands crypto-actifs, "
  "seuls 14 ont progressé ce jour-là (" + L("rapport de marché du 18/10/2026", U["market"], "Marché") + ", mis à jour "
  "en continu sur " + L("CoinGecko", U["cg_charts"], "Marché") + "). En même temps, le marché est inondé de nouveaux "
  "projets. Selon une analyse de CoinGecko, plus de 20 millions de crypto-actifs avaient été recensés fin 2025. Rien "
  "qu'en 2025, plus de 11,6 millions ont échoué, soit au total plus de la moitié de tous ceux jamais recensés ("
  + L("article sur l'étude CoinGecko", U["dead"], "Marché") + "). L'immense majorité sont des tokens éphémères sur "
  "des chaînes existantes, pas des blockchains à part entière. Les chiffres montrent néanmoins à quel point il est "
  "difficile de gagner de l'attention et des utilisateurs durables.")
FIG(F + "f_deadcoins.png", "Crypto-actifs morts par an selon les données de CoinGecko. La plupart des nouveaux "
                           "lancements ne survivent pas.", 15.0)
P("Dans l'UE, le " + L("règlement MiCA", U["esma"], "Droit") + " s'applique sans période transitoire depuis le "
  "1er juillet 2026 : les prestataires de services sur crypto-actifs ont besoin d'un agrément, faute de quoi ils "
  "doivent cesser de servir des clients de l'UE (" + L("communiqué de l'AMF", U["amf"], "Droit") + "). Pour ToyChain, "
  "cela signifie deux choses : les plateformes d'échange et les fournisseurs de wallets qui voudraient plus tard "
  "accueillir le TOY examineront de plus près. Et pour une offre au public ou une admission à la négociation, MiCA "
  "exige en règle générale un livre blanc sur les crypto-actifs selon des règles fixes. Si et comment cela s'applique "
  "à un coin sans émetteur comme le TOY doit être clarifié juridiquement avant tout mainnet. Le présent document "
  "n'est <b>pas</b> un tel livre blanc.")
H2("16.2 Ce qui plaide pour ToyChain")
B(["<b>Une équité crédible :</b> un lancement sans aucun solde est rare. Sur un marché où de nombreux projets "
   "démarrent avec de grandes parts d'initiés, c'est un signe distinctif clair que chacun peut vérifier dans ToyScan.",
   "<b>Une faible barrière d'entrée :</b> participer avec un vieux téléphone, sans rien acheter et sans connaissances "
   "préalables. Cela peut toucher des personnes qui n'auraient jamais fait tourner un nœud et crée beaucoup de petits "
   "participants plutôt que quelques gros.",
   "<b>Une surface d'attaque réduite :</b> sans smart contracts ni ponts, les causes de nombreuses grandes pertes de "
   "ces dernières années disparaissent.",
   "<b>Compatibilité :</b> les adresses Ethereum abaissent la barrière pour les utilisateurs qui ont déjà un wallet.",
   "<b>Ouverture et langues :</b> une licence libre, un journal public des décisions et 45 langues rendent le projet "
   "accessible à une communauté internationale."])
H2("16.3 Ce qui plaide contre")
B(["<b>Pas de capital de départ :</b> sans premint, il n'y a pas de budget pour les développeurs, les audits, la "
   "publicité ou les cotations. Le projet dépend de bénévoles, de dons et de subventions.",
   "<b>Effets de réseau :</b> utilisateurs, commerçants et plateformes se regroupent autour des grandes chaînes. Une "
   "nouvelle chaîne doit offrir une raison concrète de changer.",
   "<b>Volontairement limité :</b> sans smart contracts, il n'y a pas d'écosystème d'apps, de tokens et de plateformes "
   "d'échange décentralisées, qui attire beaucoup d'attention et de capitaux sur d'autres chaînes.",
   "<b>Forte concurrence dans les paiements :</b> pour les paiements du quotidien, beaucoup utilisent désormais des "
   "stablecoins adossés à des devises ou des réseaux établis. Un coin à la valeur fluctuante y est moins attrayant.",
   "<b>Petite équipe, stade précoce :</b> douze jalons, des audits et des clarifications juridiques sont encore "
   "nécessaires avant un mainnet.",
   "<b>Le nom :</b> d'anciens tokens homonymes rendent le projet plus difficile à trouver et comportent un risque de "
   "confusion."])
FIG(F + "f_swot.png", "Forces, faiblesses, opportunités et menaces en un coup d'œil.", 15.0)
H2("16.4 Scénarios")
T([["Scénario", "Description", "Évaluation"],
   ["A · Échec avant le mainnet", "Le testnet révèle des problèmes fondamentaux ou trop peu de contributeurs se "
                                  "manifestent.", "réaliste, comme pour la plupart des projets"],
   ["B · Projet d'apprentissage et communautaire", "Un testnet stable et une petite communauté active ; les TOY sont à "
                                                   "peine échangés.", "le plus probable si le travail se poursuit"],
   ["C · Petit marché réel", "Mainnet après audit, quelques centaines à un millier de nœuds, échanges sur quelques "
                             "plateformes, usage dans des niches (jeu, associations, dons).",
    "possible ; demande communauté, audits et clarté juridique"],
   ["D · Adoption large", "L'un des coins connus avec des millions d'utilisateurs.", "très improbable"]],
  [4.2, 7.6, 4.6])
H2("16.5 Ce qui ferait la différence")
B(["<b>Des validateurs de départ indépendants</b> de différents pays, qui participent publiquement.",
   "<b>Un audit de sécurité externe</b>, financé par des dons ou des programmes de soutien au logiciel libre.",
   "<b>Un cas d'usage concret</b> qui fonctionnerait moins bien sans ToyChain, comme le jeu TOYQUEST, où les joueurs "
   "participent avec leur téléphone et gagnent des TOY.",
   "<b>Une transparence cohérente :</b> rapports mensuels, mesures ouvertes, culture honnête de l'erreur.",
   "<b>La clarté juridique</b> avant chaque étape qui touche à une valeur réelle."])
P("<b>Conclusion de l'évaluation :</b> en tant qu'investissement, ToyChain ne peut pas être évalué aujourd'hui et, "
  "comme presque toute nouvelle cryptomonnaie, comporte un risque de perte très élevé. En tant que projet technique et "
  "communautaire, ToyChain a un profil clair et inhabituel : équitable, léger, compréhensible et prudent. Sa chance "
  "réaliste ne réside pas dans une hausse rapide du cours, mais dans la construction d'une petite communauté "
  "convaincue qui fait elle-même fonctionner le réseau. Si cela réussit, quelque chose de plus grand peut en naître au "
  "fil des années. Rien n'est garanti.")

# ================================================================ 17
H("État et feuille de route")
P("<b>Aujourd'hui (18 octobre 2026) : avant la phase de test.</b> Le concept est consigné dans 113 décisions "
  "documentées. La construction est divisée en 13 jalons. Le jalon M1 est terminé : les règles pour les montants et "
  "pour l'émission de nouveaux TOY sont programmées et vérifiées par 45 tests automatisés. L'implémentation en Rust "
  "donne exactement les mêmes chiffres, jusqu'à la plus petite unité, que la simulation de référence indépendante. Les "
  "tests tournent automatiquement sous Linux, Windows et sur processeurs ARM, et les règles clés de l'offre sont en "
  "outre prouvées mathématiquement avec Kani.")
FIG(F + "f_roadmap.png", "Feuille de route. Il n'y a délibérément aucune date fixe pour un mainnet.")
T([["Phase", "Contenu", "État"],
   ["Concept", "Spécification v0.3, journal des décisions, deux examens", "terminé, mis à jour en continu"],
   ["Phase 1, M1 à M11", "Types de base et émission, cryptographie, comptes et transactions, consensus et phase de "
    "démarrage, stockage, réseau, interfaces, wallet, plateformes, testnet, ToyScan", "M1 terminé, M2 ensuite"],
   ["Testnet 1", "Noyau plus protection des comptes, 45 langues et quatre modes ; d'abord local, puis public", "prévu"],
   ["Phase 1, M12 et M13", "autres types de transaction, échange atomique, noms, adresses à usage unique, clés "
    "post-quantiques", "prévu"],
   ["Phase 2", "Couche de preuves : replier l'historique, nœuds sans confiance dans des points de contrôle", "prévu"],
   ["Ensuite", "Gouvernance, mainnet (seulement après audit, tests de charge et examen juridique, date annoncée "
    "publiquement à l'avance), ponts possibles et paiements hors ligne", "ouvert"]],
  [3.4, 9.4, 3.6])
P("Il n'y a pas de date fixe pour un mainnet. Il ne démarrera que lorsque le testnet fonctionnera de façon stable, que "
  "la sécurité aura été examinée et que les questions juridiques seront réglées. La date de lancement sera annoncée "
  "publiquement à l'avance, afin que tous puissent commencer en même temps.")

# ================================================================ 18
H("Conclusion")
P("ToyChain veut montrer qu'une cryptomonnaie peut démarrer équitablement, se contenter de peu de stockage et rester "
  "rapide et sûre. Chacun doit pouvoir participer, même sans argent et avec un vieil appareil. Pour cela, ToyChain "
  "renonce délibérément à beaucoup de ce que proposent d'autres chaînes et mise sur quelques fonctions intégrées et "
  "faciles à vérifier. Ce n'est pas ce document qui décidera du succès, mais le testnet. Contributions, critiques et "
  "examens sont expressément bienvenus : le code, la spécification et le journal des décisions sont "
  + L("publics sur GitHub", U["repo"]) + ".")

# ================================================================ Annexes
story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Annexe A : principales valeurs de départ", h1))
P("Toutes les valeurs, sauf celles de la constitution, sont des hypothèses pour le testnet et y seront mesurées et "
  "ajustées.")
T([["Domaine", "Valeur de départ"],
   ["Quantité et divisibilité", "100 000 000 TOY, 18 décimales (constitution)"],
   ["Émission par époque", "0,0505 % de la quantité restante fois m ; m entre 0,5 et 2,0, variation d'au plus 0,05 par "
                           "époque"],
   ["Répartition des nouveaux TOY", "50 % consensus, 40 % loterie du travail, 10 % pot de remboursement"],
   ["Temps", "temps de bloc 60 s, au plus tôt 10 s ; époque d'au moins 1 440 blocs et 24 heures"],
   ["Comité et blocs", "au plus 128 validateurs, bloc d'au plus 128 Ko, définitif après 1 bloc"],
   ["Phase de démarrage", "21 validateurs de départ, rémunération de 50 % à 0 % en 180 jours, bloquée 365 époques, "
                          "passation au plus tôt après 90 jours"],
   ["Stake", "10 / 100 / 1 000 TOY ; libération après 21 époques ; plafond 5 %, à partir de 100 validateurs 1 %"],
   ["Pénalité de double signature", "max(1 %, 3 × part concernée en même temps), au plus 100 %, rôle bloqué 7 "
                                    "époques"],
   ["Comptes", "dépôt d'activation 0,1 TOY (peut descendre à 0,01 TOY) ; mise en sommeil des comptes dès 90 % de "
               "remplissage"],
   ["Quota", "10 à 100 transactions gratuites par compte et par époque, à partir d'un âge de compte de 100 blocs"],
   ["Loterie du travail", "Argon2id avec 16 Mio ; jusqu'à 64 gagnants par bloc, gain d'au moins 0,2 TOY"],
   ["Blocages", "uniquement pour des infractions clairement prouvables, doublement en cas de récidive jusqu'à 64 "
                "époques au plus"],
   ["Stockage", "256 Mo par nœud standard : 128 Mo d'état, 48 Mo de fenêtre de blocs, 80 Mo de preuves, index et "
                "réserve"]],
  [4.0, 12.4])

story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Annexe B : liste des liens", h1))
P("Les principales explications pour aller plus loin, classées par thème. Les sources en anglais sont signalées. "
  "Tous les liens ont été vérifiés en octobre 2026.")
T([["Thème", "Lien", "De quoi il s'agit"],
   ["Bases", L("Livre blanc de Bitcoin", U["btc_pdf"]) + " (anglais)", "l'original de 2008, 9 pages"],
   ["Bases", L("Le livre blanc de Bitcoin expliqué (CoinGate)", U["btc_fr"]), "résumé en français"],
   ["Masse monétaire", L("Controlled supply (Bitcoin Wiki)", U["btc_supply"]) + " (anglais)",
    "comment Bitcoin divise son émission"],
   ["Consensus", L("Mécanismes de consensus (ethereum.org)", U["eth_consensus"]), "vue d'ensemble"],
   ["Consensus", L("Preuve d'enjeu (ethereum.org)", U["eth_pos"]), "stake, validateurs, finalité"],
   ["Consensus", L("The latest gossip on BFT consensus", U["tendermint"]) + " (anglais)",
    "article sur la méthode Tendermint"],
   ["Consensus", L("MEV (ethereum.org)", U["eth_mev"]), "le « passage devant » sur Ethereum"],
   ["Technique", L("BLS12-381 For The Rest Of Us", U["bls"]) + " (anglais)", "signatures communes"],
   ["Technique", L("Arbres de Merkle (Bitpanda Academy)", U["merkle"]), "preuves pour les comptes dormants"],
   ["Technique", L("RFC 9106 : Argon2", U["argon2"]) + " (anglais)", "standard du travail de calcul"],
   ["Technique", L("RFC 9381 : VRF", U["vrf"]) + " (anglais)", "hasard vérifiable"],
   ["Technique", L("Dandelion++", U["dandelion"]) + " (anglais)", "protection de l'IP de l'expéditeur"],
   ["Technique", L("libp2p", U["libp2p"]) + " (anglais)", "bibliothèque réseau"],
   ["Technique", L("Cryptographie post-quantique (IEEE Spectrum)", U["pqk"]) + " (anglais)", "introduction"],
   ["Technique", L("NIST FIPS 204 (ML-DSA)", U["mldsa"]) + " (anglais)", "signatures post-quantiques"],
   ["Technique", L("Kani (Ferrous Systems)", U["kani"]) + " (anglais)", "vérification mathématique du code Rust"],
   ["Technique", L("Reproducible Builds", U["repro"]) + " (anglais)", "versions logicielles vérifiables"],
   ["Comptes", L("BIP-39", U["bip39"]) + " (anglais)", "standard de la phrase de 12/24 mots"],
   ["Comptes", L("EIP-155", U["eip155"]) + " (anglais)", "identifiant de chaîne dans chaque signature"],
   ["Comptes", L("EIP-5564 expliqué (QuickNode)", U["eip5564"]) + " (anglais)", "adresses de réception uniques"],
   ["Frais", L("EIP-1559", U["eip1559"]) + " (anglais)", "frais de base sur Ethereum"],
   ["Wallet", L("Ajouter un réseau dans MetaMask", U["metamask"]) + " (anglais)", "réseaux personnalisés"],
   ["Autres chaînes", L("Ponts (ethereum.org)", U["eth_bridges"]), "fonctionnement des ponts, risques"],
   ["Autres chaînes", L("Piratages de ponts 2022 (Decrypt)", U["bridgehacks"]) + " (anglais)",
    "chiffres de Chainalysis sur les pertes"],
   ["Autres chaînes", L("Atomic swap (Blockstream)", U["atomic"]) + " (anglais)", "échange sans intermédiaire"],
   ["Autres chaînes", L("Hashed Timelock Contracts", U["htlc"]) + " (anglais)", "le verrou à délai derrière"],
   ["Énergie", L("Cambridge Bitcoin Electricity Index", U["cbeci"]) + " (anglais)", "consommation de Bitcoin"],
   ["Marché", L("Graphiques CoinGecko", U["cg_charts"]) + " (anglais)", "valeur actuelle du marché"],
   ["Marché", L("Rapport de marché du 18/10/2026 (DailyForex)", U["market"]) + " (anglais)", "état à cette date"],
   ["Marché", L("Étude CoinGecko sur les coins morts (ForkLog)", U["dead"]) + " (anglais)", "chiffres 2021 à 2025"],
   ["Droit", L("Règlement MiCA (EUR-Lex)", U["mica"]), "texte officiel en français"],
   ["Droit", L("Règlement MiCA (ESMA)", U["esma"]) + " (anglais)", "règles et orientations"],
   ["Droit", L("Fin de la transition MiCA (AMF)", U["amf"]) + " (anglais)", "ce qui s'applique dès le 1/7/2026"],
   ["Consommateurs", L("Guide AMF sur les crypto-actifs", U["amf_guide"]), "risques pour les épargnants"],
   ["Consommateurs", L("INC : avant d'investir dans les cryptos", U["inc"]), "conseils aux consommateurs"],
   ["Consommateurs", L("Avertissement des autorités européennes (ESMA)", U["esas_warning"]) + " (anglais)",
    "risques et protection limitée"],
   ["Projet", L("github.com/Safehaigit/toycoin", U["repo"]), "code, spécification, décisions (en allemand)"]],
  [3.0, 7.6, 5.8])

story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Annexe C : sources et standards", h1))
refs = [
    "S. Nakamoto : Bitcoin: A Peer-to-Peer Electronic Cash System, 2008.",
    "E. Buchman, J. Kwon, Z. Milosevic : The latest gossip on BFT consensus (Tendermint), 2018.",
    "BIP-39 : Mnemonic code for generating deterministic keys. EIP-155 : Simple replay attack protection. EIP-1559 : "
    "Fee market change. EIP-2718 : Typed transaction envelope. EIP-5564 : Stealth addresses.",
    "D. Boneh, B. Lynn, H. Shacham : Short signatures from the Weil pairing (BLS), 2001 ; BLS12-381.",
    "RFC 9106 : Argon2 memory-hard function. RFC 9381 : Verifiable Random Functions (VRFs).",
    "G. Fanti et al. : Dandelion++: Lightweight cryptocurrency networking with formal anonymity guarantees, 2018.",
    "NIST FIPS 204 : Module-Lattice-Based Digital Signature Standard (ML-DSA), 2024.",
    "Règlement (UE) 2023/1114 sur les marchés de crypto-actifs (MiCA).",
    "Données de marché : DailyForex, Crypto Overview, 18/10/2026 ; données CoinGecko via ForkLog, janvier 2026 ; "
    "Chainalysis via Decrypt, 2022 ; Spark, Bitcoin Full Node Requirements 2026.",
    "ToyChain : spécification technique v0.3, journal des décisions, plan de construction phase 1, examens 1 et 2, "
    "simulation de référence m-simulation.py ; github.com/Safehaigit/toycoin (en allemand).",
]
for i, r in enumerate(refs, 1):
    story.append(Paragraph(french_typography(f"[{i}] {r}"), small))
story.append(Spacer(1, 6))
story.append(Paragraph(french_typography(
    "<b>Avertissement.</b> Ce document décrit un projet logiciel en cours de développement. Il ne constitue ni un "
    "conseil en investissement, ni un conseil fiscal ou juridique, ni une offre, un prospectus ou un livre blanc sur "
    "les crypto-actifs au sens du règlement MiCA. Toutes les informations sont provisoires et peuvent changer. Les "
    "chiffres du marché reflètent la situation à la date indiquée. En cas de doute, l'édition allemande fait foi."),
    small))

finish("ToyChain-Whitepaper-v0.2-fr.pdf",
       "ToyChain · Livre blanc provisoire · Version 0.2 · avant la phase de test", "Page",
       "ToyChain et ToyCoin (TOY) : livre blanc provisoire", "Version 0.2, avant la phase de test", "fr-FR")
