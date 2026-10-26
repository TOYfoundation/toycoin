"""ToyChain whitepaper, draft 0.3, Yoruba edition (technical terms in English where no clear Yoruba term exists).
Run: python3 figures_i18n.py yo && python3 build_yo.py"""
from wp_common import story, L, H, H2, P, MORE, B, T, BOX, FIG, toc_table, finish, set_words
from wp_common import title, subtitle, center, abstract, small, h1
from reportlab.platypus import Paragraph, Spacer, CondPageBreak
from reportlab.lib.units import cm

set_words("Àwòrán", "Ka sí i:")
F = "abbildungen/yo/"
EN = " (Gẹ̀ẹ́sì)"

U = dict(
    btc_pdf="https://bitcoin.org/bitcoin.pdf",
    btc_supply="https://en.bitcoin.it/wiki/Controlled_supply",
    btc_size="https://www.spark.money/research/bitcoin-full-node-cost-requirements-2026",
    cbeci="https://ccaf.io/cbnsi/about/cbeci",
    eth_consensus="https://ethereum.org/en/developers/docs/consensus-mechanisms/",
    eth_pos="https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/",
    eth_mev="https://ethereum.org/en/developers/docs/mev/",
    eth_bridges="https://ethereum.org/en/bridges/",
    eip1559="https://eips.ethereum.org/EIPS/eip-1559",
    eip155="https://eips.ethereum.org/EIPS/eip-155",
    eip5564="https://www.quicknode.com/guides/ethereum-development/wallets/how-to-use-stealth-addresses-on-ethereum-eip-5564",
    bip39="https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki",
    tendermint="https://arxiv.org/abs/1807.04938",
    bls="https://hackmd.io/@benjaminion/bls12-381",
    bls_wiki="https://en.wikipedia.org/wiki/BLS_digital_signature",
    merkle="https://en.wikipedia.org/wiki/Merkle_tree",
    argon2="https://www.rfc-editor.org/rfc/rfc9106",
    vrf="https://www.rfc-editor.org/rfc/rfc9381",
    dandelion="https://arxiv.org/abs/1805.11060",
    mldsa="https://csrc.nist.gov/pubs/fips/204/final",
    pqk="https://spectrum.ieee.org/post-quantum-cryptography-nist",
    atomic="https://glossary.blockstream.com/atomic-swap",
    htlc="https://en.bitcoin.it/wiki/Hashed_Timelock_Contracts",
    mica="https://eur-lex.europa.eu/eli/reg/2023/1114/oj?locale=en",
    esma="https://www.esma.europa.eu/publications-and-data/interactive-single-rulebook/mica",
    amf="https://www.amf-france.org/en/news-publications/news/end-mica-transitional-period-esma-sets-out-its-expectations-professionals-and-warns-retail-investors",
    esas_warning="https://esma.europa.eu/press-news/esma-news/eu-supervisory-authorities-warn-consumers-risks-and-limited-protection-certain",
    metamask="https://support.metamask.io/configure/networks/how-to-add-a-custom-network-rpc",
    repro="https://reproducible-builds.org/",
    kani="https://rust-training.ferrous-systems.com/latest/book/kani",
    libp2p="https://docs.libp2p.io/",
    mit="https://opensource.org/license/mit",
    repo="https://github.com/TOYfoundation/toycoin",
    market="https://dailyforex.com/forex-technical-analysis/2026/10/crypto-overview-18-october-2026/250826",
    cg_charts="https://www.coingecko.com/en/charts",
    dead="https://forklog.com/en/news/coingecko-reports-record-token-mortality-in-2025",
    bridgehacks="https://decrypt.co/106718/cross-chain-crypto-bridge-hacks-hit-2-billion-2022-chainalysis",
)

# ================================================================ Àkọlé
story.append(Spacer(1, 0.3 * cm))
story.append(Paragraph("ToyChain àti ToyCoin (TOY)", title))
story.append(Spacer(1, 4))
story.append(Paragraph("Network ìsanwó tó fúyẹ́, tí a bẹ̀rẹ̀ lọ́nà tó tọ́,<br/>tí ẹnikẹ́ni lè darapọ̀ mọ́ láìní owó",
                       subtitle))
story.append(Spacer(1, 10))
story.append(Paragraph("Iṣẹ́ àkànṣe ToyChain · " + L("github.com/TOYfoundation/toycoin", U["repo"]), center))
story.append(Paragraph("Whitepaper àkọ́kọ́ · Draft 0.3 · 19 October 2026 · ẹ̀dà Yorùbá", center))
story.append(Spacer(1, 12))
BOX("<b>Ipò: ṣáájú ìpele ìdánwò (test phase).</b> Lọ́wọ́lọ́wọ́, ToyChain jẹ́ èrò àti apá àkọ́kọ́ ti software nìkan. "
    "Àwọn types ìpìlẹ̀ àti òfin bí a ṣe ń ṣẹ̀dá owó tuntun ti parí, a sì ti dán wọn wò (milestone M1 nínú 13). "
    "<b>Kò tíì sí testnet, kò sí mainnet, kò sì sí TOY tí a lè tà</b>. Gbogbo nọ́ńbà jẹ́ iye ìbẹ̀rẹ̀ tí a ó wọ̀n, tí a ó "
    "sì ṣàtúnṣe lórí testnet. Ìwé yìí kì í ṣe ìmọ̀ràn ìdókòwò, kì í sì ṣe ìpè láti ra ohunkóhun. Owó testnet kò ní "
    "iye kankan, a kì yóò sì gbé e lọ sí mainnet láé. Àwọn ọ̀rọ̀ aláwọ̀ búlúù tí a fà lá ní ìsàlẹ̀ jẹ́ link sí àlàyé "
    "síwájú sí i. Ẹ̀dà German ni ẹ̀dà àkọ́kọ́ tó ní àṣẹ. Níbi tí ọ̀rọ̀ Yorùbá kò bá ti ṣe kedere, a lo ọ̀rọ̀ Gẹ̀ẹ́sì.")

P("<b>Àkópọ̀.</b> ToyChain jẹ́ blockchain aládàáṣe pẹ̀lú owó tirẹ̀, ToyCoin (TOY). Iye tó pọ̀ jùlọ jẹ́ mílíọ̀nù "
  "100. Ìbẹ̀rẹ̀ rẹ̀ tọ́ pátápátá (total fair launch): block àkọ́kọ́ kò ní TOY kankan, kò sí ìpamọ́ fún àwọn olùdásílẹ̀, "
  "kò sí presale, kò sì sí ìpín àkànṣe. Gbogbo TOY ni a ń ṣẹ̀dá díẹ̀díẹ̀ pẹ̀lú òfin kan tó dúró, tí gbogbo ènìyàn lè "
  "ṣàyẹ̀wò, bí ti " + L("Bitcoin", U["btc_pdf"]) + ". Ṣùgbọ́n ààbò kò wá láti inú iná mànàmáná bí ti Bitcoin, ó wá "
  "láti ọ̀dọ̀ àwọn olùkópa tí wọ́n ti TOY pa gẹ́gẹ́ bí ìdógò (stake), tí wọ́n sì ń buwọ́lu àwọn block papọ̀. Block "
  "kan ti parí (final) lẹ́yìn nǹkan bí ìṣẹ́jú kan. Node kọ̀ọ̀kan nílò data 256 MB nìkan, ó sì ń ṣiṣẹ́ lórí fóònù "
  "àtijọ́ àti Raspberry Pi. Ẹnikẹ́ni lè darapọ̀ láìní owó; ẹni tó bá ṣe púpọ̀ sí i ń gba ẹ̀tọ́ púpọ̀ sí i. ToyChain "
  "mọ̀ọ́mọ̀ kò lo smart contracts, tokens tàbí NFTs: gbogbo iṣẹ́ ni a kọ́ sínú rẹ̀, ó rọrùn láti ṣàyẹ̀wò, kò sì lè "
  "fọwọ́ kan owó ẹlòmíràn láé. Àdírẹ́sì àkáǹtì jẹ́ bákan náà pẹ̀lú ti Ethereum, nítorí náà àwọn wallet tí a mọ̀ "
  "ń ṣiṣẹ́. Ìwé yìí parí pẹ̀lú ìgbéyẹ̀wò òtítọ́ nípa àǹfààní iṣẹ́ àkànṣe yìí nínú ọjà crypto.", abstract)

story.append(Spacer(1, 4))
toc_table(["1 Ìfáárà", "2 ToyChain ní ṣókí", "3 Àwọn ọ̀rọ̀ ìpìlẹ̀", "4 Ètò (architecture)",
           "5 Ìbẹ̀rẹ̀ tó tọ́ àti iye owó", "6 Ìkópa: ìpele, quota, lọ́tìrì", "7 Ìfohùnṣọ̀kan (consensus)",
           "8 Ìpele ìbẹ̀rẹ̀ (bootstrap)", "9 Node tó fúyẹ́ fún ẹ̀rọ àtijọ́", "10 Ààbò fún àwọn olùlò",
           "11 Wallet, ToyScan àti chain mìíràn", "12 Ìlò", "13 Ìjọra àti ìyàtọ̀", "14 Àǹfààní",
           "15 Ewu àti ìbéèrè tó ṣì wà", "16 Àǹfààní nínú ọjà crypto", "17 Ipò àti ọ̀nà iwájú", "18 Ìparí",
           "Àfikún: iye, links, orísun, àyípadà"], "Àkóónú")

# ================================================================ 1
H("Ìfáárà")
P("Ní 2008, Bitcoin fi hàn pé àwọn ènìyàn lè fohùnṣọ̀kan láìsí banki lórí ẹni tó ni owó wo. Láti ìgbà náà, "
  "mílíọ̀nù crypto ló ti jáde. Nínú ọ̀pọ̀ nínú wọn, àwọn olùdásílẹ̀ àti àwọn olùdókòwò àkọ́kọ́ gba ìpín ńlá nínú coin "
  "kí wọ́n tó bẹ̀rẹ̀ (premint). Àwọn mìíràn nílò ilé-iṣẹ́ kọ̀ǹpútà ńlá, iná mànàmáná púpọ̀ tàbí disk ọgọ́rọ̀ọ̀rún "
  "gigabyte, tí àwọn ènìyàn lásán kò fi lè ṣàyẹ̀wò network fúnra wọn. Bitcoin chain pípé ti tó nǹkan bí 760 GB báyìí. "
  "Àwọn chain kan sì ti di onípọ̀-iṣẹ́ tó bẹ́ẹ̀ pẹ̀lú smart contracts tí àṣìṣe nínú code ẹlòmíràn fi máa ń fa àdánù.")
P("ToyChain ń gbìyànjú láti ṣe èrò pàtàkì Bitcoin, ìyẹn iye tó dúró, ìbẹ̀rẹ̀ tó tọ́ àti òfin tó ṣí sílẹ̀, lọ́nà tó "
  "fúyẹ́ tó sì wúlò lójoojúmọ́. Àwọn ìbéèrè pàtàkì ni: Ṣé ẹnikẹ́ni lè kópa pẹ̀lú ẹ̀rọ àtijọ́? Ṣé ẹnikẹ́ni lè ṣàyẹ̀wò "
  "òfin fúnra rẹ̀? Ṣé owó rẹ yóò jẹ́ tìrẹ nígbà gbogbo? Whitepaper yìí ṣàlàyé àwọn èrò náà ní èdè tó rọrùn. Òfin "
  "pípé wà nínú technical specification (version 0.3, ní èdè German) nínú " + L("repository", U["repo"]) + ". Ó ṣì jẹ́ "
  "private nígbà ìdàgbàsókè, a ó sì ṣí i sílẹ̀ fún gbogbo ènìyàn ní ìgbà testnet gbangba ní tuntun jùlọ.")
MORE(L("Bitcoin whitepaper" + EN, U["btc_pdf"]), L("Bitcoin node ní 2026" + EN, U["btc_size"]))

# ================================================================ 2
H("ToyChain ní ṣókí")
T([["Àbùdá", "ToyChain (iye ìbẹ̀rẹ̀ tí a gbèrò)"],
   ["Owó", "ToyCoin, àmì TOY, a lè pín in sí ibi 18 lẹ́yìn àmì ìdá"],
   ["Iye tó pọ̀ jùlọ", "100,000,000 TOY, a kọ ọ́ sínú code gẹ́gẹ́ bí òfin tí kò lè yí padà"],
   ["Ìbẹ̀rẹ̀", "Total fair launch: block àkọ́kọ́ láìsí owó kankan, kò sí premint, kò sí ìpamọ́"],
   ["TOY tuntun", "ní epoch kọ̀ọ̀kan (ó kéré tán wákàtí 24) 0.0505 % nínú iye tó kù, pẹ̀lú factor m láàrin 0.5 àti "
                  "2.0; ọdún kìíní nǹkan bí mílíọ̀nù 16.84, ìdajì lẹ́yìn nǹkan bí ọdún 3.76, 99 % lẹ́yìn nǹkan bí ọdún 25"],
   ["Ìfohùnṣọ̀kan", "Proof of stake pẹ̀lú committee tó ní validator 128 jùlọ, ìbuwọ́lù kan papọ̀, ó parí lẹ́yìn block 1"],
   ["Àkókò block", "ìṣẹ́jú-àáyá 60; ó lè yára ju bẹ́ẹ̀ lọ tí ìlà ìdúró bá kún, ṣùgbọ́n kò yára ju ìṣẹ́jú-àáyá 10"],
   ["Ààyè fún node", "Èrò: 256 MB, pẹ̀lú lórí Android àtijọ́ àti Raspberry Pi"],
   ["Àkáǹtì", "bí ti Ethereum: àdírẹ́sì kan náà, gbólóhùn ọ̀rọ̀ 12 tàbí 24 kan náà"],
   ["Iṣẹ́", "àwọn transaction tí a kọ́ sínú rẹ̀ nìkan; kò sí smart contracts, tokens tàbí NFTs"],
   ["Owó iṣẹ́ (fees)", "transaction 10 sí 100 lọ́fẹ̀ẹ́ fún àkáǹtì kọ̀ọ̀kan ní epoch kọ̀ọ̀kan (ó kéré tán ọjọ́ kan); "
                      "lẹ́yìn náà owó kékeré tàbí iṣẹ́ kọ̀ǹpútà"],
   ["Èdè software", "Rust, core kan fún gbogbo platform"],
   ["Licence", L("MIT", U["mit"]) + " (software ọ̀fẹ́)"]],
  [4.0, 12.4])

# ================================================================ 3
H("Àwọn ọ̀rọ̀ ìpìlẹ̀")
P("Tí o kò bá tíì lo cryptocurrency rí, àwọn ọ̀rọ̀ pàtàkì nìwọ̀nyí. Àwọn apá tó kàn dá lé wọn. Ọ̀rọ̀ aláwọ̀ búlúù "
  "kọ̀ọ̀kan ń mú ọ lọ sí àlàyé tó dára lórí ayélujára (ní èdè Gẹ̀ẹ́sì).")
T([["Ọ̀rọ̀", "Ní ṣókí"],
   ["Blockchain", "Ìwé àkọsílẹ̀ owó kan tí ọ̀pọ̀ kọ̀ǹpútà ń tọ́jú lẹ́ẹ̀kan náà. A ń fi àwọn àkọsílẹ̀ tuntun kún un ní "
                  "àpò (block); block kọ̀ọ̀kan tọ́ka sí èyí tó ṣáájú rẹ̀ lọ́nà tí a kò lè yí padà."],
   ["Node", "Ẹ̀rọ tó ń ṣiṣẹ́ pẹ̀lú software ToyChain. Ó ń ṣàyẹ̀wò fúnra rẹ̀ bóyá gbogbo àkọsílẹ̀ tẹ̀lé òfin, kò sì "
            "nílò láti gbẹ́kẹ̀lé ẹnikẹ́ni."],
   ["Wallet", "App tó ń tọ́jú àwọn kọ́kọ́rọ́ rẹ, tó sì ń buwọ́lu ìfiránṣẹ́ owó. Lórí ToyChain, wallet TOYPAY tún jẹ́ node."],
   [L("Kọ́kọ́rọ́ àti gbólóhùn", U["bip39"]),
    "Kọ́kọ́rọ́ àṣírí fi hàn pé àkáǹtì jẹ́ tìrẹ. A lè dá a padà láti ọ̀rọ̀ 12 tàbí 24. Ẹni tó bá mọ àwọn ọ̀rọ̀ náà lè "
    "ná owó náà."],
   ["Ìbuwọ́lù (signature)", "Ìbuwọ́lù oní-nọ́ńbà. Ẹnikẹ́ni lè ṣàyẹ̀wò pé ó jẹ́ òótọ́, ṣùgbọ́n kò sí ẹni tó lè ṣe ayédèrú "
                            "rẹ̀ láìsí kọ́kọ́rọ́."],
   [L("Consensus", U["eth_consensus"]), "Ọ̀nà tí gbogbo node fi ń fohùnṣọ̀kan lórí block tó kàn."],
   [L("Stake", U["eth_pos"]), "TOY tí o fi ìfẹ́ ti pa gẹ́gẹ́ bí ìdógò. Ẹni tó bá tẹ̀lé òfin ń gba èrè. Ẹni tí a bá "
                              "fi hàn pé ó ṣe màgòmágó yóò pàdánù apá kan ìdógò náà."],
   ["Validator", "Node tó ní stake, tó ń dábàá block, tó sì ń buwọ́lù wọ́n."],
   [L("Merkle tree", U["merkle"]), "Irú àkóónú tí a fi checksum ṣe. Proof kékeré fi hàn pé àkọsílẹ̀ kan wà nínú rẹ̀ "
                                   "láìsí pé a tọ́jú gbogbo rẹ̀."],
   ["Epoch", "Apá chain tó ní ó kéré tán block 1,440 àti ó kéré tán wákàtí 24. Ọ̀pọ̀ òfin ló ń kà ní epoch, bí ìṣẹ̀dá "
             "TOY tuntun."],
   ["Testnet", "Chain ìdánwò tí coin rẹ̀ kò ní iye. Níbẹ̀ la ti ń wá àṣìṣe kí ohun tó ṣe pàtàkì tó bẹ̀rẹ̀."]],
  [3.6, 12.8])

# ================================================================ 4
H("Ètò (architecture)")
P("ToyChain ní àwọn layer tí a tò lé ara wọn. Ọ̀kọ̀ọ̀kan ní iṣẹ́ tó ṣe kedere. Gbogbo layer wà nínú program Rust "
  "kan náà, tó ń ṣiṣẹ́ bákan náà lórí Android, Windows, Linux àti Raspberry Pi. Nítorí náà, kò sí ohun tí a ní láti "
  "kọ tàbí ṣàyẹ̀wò lẹ́ẹ̀mejì.")
FIG(F + "f_arch.png", "Àwọn layer ToyChain. Proof layer (ìlà dídá) yóò wá ní phase 2.", 14.8)
MORE(L("libp2p" + EN, U["libp2p"]), L("Dandelion++" + EN, U["dandelion"]))

# ================================================================ 5
H("Ìbẹ̀rẹ̀ tó tọ́ àti iye owó")
H2("5.1 Kò sí TOY kankan nínú block àkọ́kọ́")
P("Block àkọ́kọ́ (genesis block) ToyChain <b>kò ní owó kankan</b>. Kò sí ẹnikẹ́ni, kódà àwọn developer, tó ní TOY "
  "ní ìbẹ̀rẹ̀. Kò sí presale, kò sí ìpamọ́, kò sì sí ìpín olùdásílẹ̀. Èyí le ju ti ọ̀pọ̀ iṣẹ́ àkànṣe tuntun lọ, ó sì "
  "tẹ̀lé ẹ̀mí Bitcoin. Òfin yìí jẹ́ apá kan “constitution” (apá 10.3), kò sì sí update tó lè yí i padà.")
H2("5.2 Bí a ṣe ń ṣẹ̀dá TOY tuntun")
P("Ní epoch kọ̀ọ̀kan, a ń ṣẹ̀dá ìpín kan tó dúró nínú iye tí a kò tíì ṣẹ̀dá: 0.0505 % nínú iye tó kù. Ní epoch "
  "àkọ́kọ́, ìyẹn jẹ́ TOY 50,500 gẹ́lẹ́. Nítorí pé iye tó kù ń dín kù, ìṣẹ̀dá ń dín kù díẹ̀díẹ̀, dípò kí ó já sí ìdajì "
  "lójijì ní ọdún mẹ́rin mẹ́rin bí ti Bitcoin. Lẹ́yìn nǹkan bí ọdún 3.76, ìdajì ti jáde; lẹ́yìn nǹkan bí ọdún 25, nǹkan "
  "bí 99 %. A kì yóò dé iye tó pọ̀ jùlọ, a kì yóò sì kọjá rẹ̀ láé. Àwọn ìdánwò ti fi èyí hàn: kódà pẹ̀lú ìṣẹ̀dá tó "
  "yára ní ìlọ́po méjì fún ọdún 100, àpapọ̀ ṣì wà nísàlẹ̀ rẹ̀.")
FIG(F + "f_emission.png", "Ìpín iye tó pọ̀ jùlọ tí a ti ṣẹ̀dá bí àkókò ti ń lọ. ToyChain ń dín kù díẹ̀díẹ̀, Bitcoin ń "
                          "dín kù ní ìgbésẹ̀. Lẹ́yìn ọdún 25, méjèèjì wà ní nǹkan bí 99 %.")
FIG(F + "f_per_year.png", "TOY tuntun ní ọdún kọ̀ọ̀kan gẹ́gẹ́ bí ètò (m = 1). Ọdún kọ̀ọ̀kan, a ń ṣẹ̀dá nǹkan bí 17 % "
                          "díẹ̀ ju ti ọdún tó ṣáájú lọ.")
MORE(L("Bí Bitcoin ṣe ń dín iye rẹ̀ kù" + EN, U["btc_supply"]))
H2("5.3 Factor m: a ń ṣàtúnṣe rẹ̀, ṣùgbọ́n pẹ̀lú ààlà")
P("A ń fi factor m láàrin 0.5 àti 2.0 sọ ìṣẹ̀dá di púpọ̀. Ó ń ga díẹ̀ nígbà tí a bá ń lo network <i>ní tòótọ́</i> "
  "(transaction tí a sanwó fún tàbí tí iṣẹ́ kọ̀ǹpútà bò) àti nígbà tí ọ̀pọ̀ validator bá ń buwọ́lù déédéé. Transaction "
  "ọ̀fẹ́ kò kà, bí bẹ́ẹ̀ kọ́ spam lè gbé ìṣẹ̀dá sókè. Apá ètò-àkókò ń mú àwọn ìyàtọ̀ padà sí ètò, m kò sì lè yí ju "
  "0.05 lọ ní epoch kan. Nínú simulation, ìṣẹ̀dá ọdún kìíní yàtọ̀ sí ètò ní nǹkan bí 2.2 % jùlọ, ní ìgbà pípẹ́ ní nǹkan "
  "bí 2.5 % jùlọ. Nítorí náà, m ń yí <i>ìgbà</i> tí a ń ṣẹ̀dá TOY padà nìkan, kì í ṣe <i>iye</i> rẹ̀ lápapọ̀.")
H2("5.4 Ta ló ń gba TOY tuntun")
P("A ń pín ìṣẹ̀dá kọ̀ọ̀kan lọ́nà tó dúró: <b>50 %</b> fún gbogbo àwọn tó ń dáàbò bo network, <b>40 %</b> fún lọ́tìrì "
  "iṣẹ́ tí ẹ̀rọ èyíkéyìí lè kópa nínú rẹ̀ láìní owó (apá 6.3), àti <b>10 %</b> fún àpò ìsanpadà (refund pool) tó ń "
  "sanwó fún transaction ọ̀fẹ́ gbogbo ènìyàn. Ní ìpele ìbẹ̀rẹ̀, ìpín ààbò ń lọ sí àwọn start validators, ó sì ń dín kù "
  "títí dé òdo; ohun tí wọn kò bá gbà ń lọ sí lọ́tìrì. Gbogbo ìṣirò ni a ń ṣe pẹ̀lú nọ́ńbà odidi ní ẹ̀ka tó kéré jùlọ "
  "(10<super>-18</super> TOY). Ìyókù ìṣirò ń lọ sí àpò ìsanpadà, kí epoch kọ̀ọ̀kan lè ṣẹ̀dá iye tí a gbèrò gẹ́lẹ́.")
FIG(F + "f_split.png", "Pínpín TOY tuntun. Ní ìpele ìbẹ̀rẹ̀, èrè start validators ń lọ díẹ̀díẹ̀ sí lọ́tìrì iṣẹ́; "
                       "lẹ́yìn ìfàlé (handover), gbogbo staker ló ń gbà á.")
H2("5.5 A kì í pa TOY run láé")
P("Ọ̀pọ̀ chain ń “jó” owó iṣẹ́ tàbí ìjìyà, bí Ethereum ṣe ń jó apá kan owó iṣẹ́ gẹ́gẹ́ bí "
  + L("EIP-1559", U["eip1559"]) + ". ToyChain kò ṣe bẹ́ẹ̀: ìjìyà àti ẹ̀bùn ń lọ sí àpò ìsanpadà, wọ́n sì ń ṣe àwọn "
  "olùlò láǹfààní. Òfin àyẹ̀wò kan (I0) ń rí i dájú ní <b>gbogbo</b> block pé àpapọ̀ gbogbo owó dọ́gba gẹ́lẹ́ pẹ̀lú "
  "àpapọ̀ gbogbo TOY tí a ti ṣẹ̀dá. Block tó bá rú òfin yìí kò wúlò.")

# ================================================================ 6
H("Ìkópa: ìpele, quota àti lọ́tìrì iṣẹ́")
H2("6.1 Ìpele ìkópa")
P("Olúkúlùkù ló ń pinnu bí ìkópa rẹ̀ ṣe pọ̀ tó. Ní ìbẹ̀rẹ̀ àkọ́kọ́, wallet ń wọn ẹ̀rọ náà, ó sì ń dábàá ìpele tó "
  "yẹ. Ó ń fúnni ní ìpele gíga nìkan tí ẹ̀rọ náà bá lè ṣe é.")
FIG(F + "f_tiers.png", "Ìpele ìkópa márùn-ún. Kódà ìpele 0 kò nílò owó.", 15.0)
P("TOY tí a tì pa ṣì jẹ́ ti olùlò, a sì lè tú u sílẹ̀ lẹ́yìn epoch 21. Kí ẹnikẹ́ni má baà ní agbára jù, stake "
  "olùkópa kan ń kà títí dé 5 % nìkan (láti validator 100 sókè, 1 %) nínú gbogbo stake. Lẹ́yìn ìpele ìbẹ̀rẹ̀, ìdá "
  "márùn-ún kan nínú ìpín ààbò ń lọ ní ìpín dọ́gba sí ọmọ committee kọ̀ọ̀kan, èyí tó kù sì ń lọ ní ìwọ̀n stake. A ń "
  "san méjèèjì ní ìwọ̀n iṣẹ́ tí a ṣe ní tòótọ́: ẹni tó bá buwọ́lu 90 % àwọn block ń gba 90 %. Nítorí pé apá stake "
  "jẹ́ linear, pínpín stake sí ọ̀pọ̀ àkáǹtì kò mú àǹfààní kankan wá.")
MORE(L("Kí ni proof of stake?" + EN, U["eth_pos"]))
H2("6.2 Transaction ọ̀fẹ́ àti owó iṣẹ́")
P("Àkáǹtì kọ̀ọ̀kan tí a ti mú ṣiṣẹ́ ní quota ọ̀fẹ́ ní epoch kọ̀ọ̀kan: transaction 100 nígbà tí network bá dákẹ́, ó "
  "kéré tán 10 nígbà tí ó bá kún. Èyí tó fún lílò ojoojúmọ́. Lẹ́yìn náà, o ń san owó iṣẹ́ kékeré tó ń ga kíákíá tí "
  "transaction bá pọ̀ nínú block kan, tàbí o fi proof of work kún un. Báyìí, network ṣì jẹ́ ọ̀fẹ́ fún àwọn olùlò "
  "lásán, spam sì di olówó ńlá. Transaction ọ̀fẹ́ kò lè gba ju ìdajì block lọ, àpò ìsanpadà ló sì ń sanwó fún wọn; "
  "ẹnikẹ́ni lè fi ẹ̀bùn sí àpò yìí.")
H2("6.3 Lọ́tìrì iṣẹ́")
P("Kí àwọn tí kò ní TOY lè rí coin tuntun gbà, lọ́tìrì wà fún iṣẹ́ kọ̀ǹpútà. Ẹ̀rọ náà ń ṣírò tíkẹ́ẹ̀tì pẹ̀lú "
  + L("Argon2id", U["argon2"]) + ", tó mọ̀ọ́mọ̀ nílò ìrántí púpọ̀ (16 MiB). Nítorí èyí, ẹ̀rọ àkànṣe àti oko graphics "
  "card kò ní àǹfààní púpọ̀ lórí fóònù tàbí PC. Títí dé olùborí 64 ni a ń yàn lọ́nà tó tọ́ ní block kọ̀ọ̀kan, èrè "
  "kọ̀ọ̀kan sì kéré tán 0.2 TOY. Ẹni tó bá ṣírò púpọ̀ ní tíkẹ́ẹ̀tì púpọ̀; èrè tí a retí bá iṣẹ́ mu. Èrè kan ṣoṣo tó "
  "láti mú àkáǹtì ṣiṣẹ́: a ń ti 0.1 TOY pa gẹ́gẹ́ bí ìdógò (apá 9), èyí tó kù sì wà fún lílò. Ẹ̀rọ ń ṣírò nìkan "
  "nígbà tí a bá gbà á láàyè: lórí charger, lórí Wi-Fi, àti nígbà tí ó bá tutù. Fún àfiwé, Bitcoin network ń lo iná "
  "mànàmáná tó pọ̀ bíi ti orílẹ̀-èdè kan, gẹ́gẹ́ bí " + L("Cambridge index" + EN, U["cbeci"]) + " ti fi hàn.")

# ================================================================ 7
H("Ìfohùnṣọ̀kan (consensus)")
H2("7.1 Bí a ṣe ń ṣe block")
P("Committee tó ní validator 128 jùlọ ni a ń tún ṣètò ní epoch kọ̀ọ̀kan. Fún block kọ̀ọ̀kan, àìròtẹ́lẹ̀ tí a lè "
  "ṣàyẹ̀wò (" + L("VRF", U["vrf"]) + ") ń yan ẹnìkan tí yóò dábàá block náà. Àwọn yòókù ń dìbò ní ìpele méjì "
  "(pre-vote àti ìfìdímúlẹ̀), gẹ́gẹ́ bí ọ̀nà tí a ti dán wò tó jọ " + L("Tendermint", U["tendermint"]) + ". Ní kété tí "
  "ìdá méjì nínú mẹ́ta bá fọwọ́ sí i, block náà ti <b>parí</b>. A kò lè yí i padà mọ́. Pẹ̀lú Bitcoin, a sábà máa ń "
  "dúró nǹkan bí wákàtí kan kí ìsanwó tó di ìdánilójú. Gbogbo ìbò ni a ń kó papọ̀ pẹ̀lú "
  + L("BLS signatures", U["bls_wiki"]) + " sí ìbuwọ́lù kan ṣoṣo tó jẹ́ byte 96 péré, tí fóònù pàápàá lè ṣàyẹ̀wò ní "
  "milliseconds.")
FIG(F + "f_block.png", "Bí block ṣe ń ṣẹlẹ̀ ní ìgbésẹ̀ márùn-ún. Lẹ́yìn ìgbésẹ̀ 5, ìsanwó ti parí.")
H2("7.2 Ètò tó tọ́")
P("Ẹni tó ń kọ́ block kò lè yan ètò transaction bí ó ti wù ú. Ó ti dúró: ní ìwọ̀n owó iṣẹ́, tí ó bá sì dọ́gba, ní "
  "ìwọ̀n nọ́ńbà àìròtẹ́lẹ̀ tí a mọ̀ pẹ̀lú block náà nìkan. Gbogbo node ló ń ṣàyẹ̀wò èyí. Nígbà tó bá yá, òfin ìfisínú "
  "(inclusion duty) yóò rí i dájú pé transaction tí ọ̀pọ̀ validator rí wọnú block tó kàn. Nítorí pé ToyChain kò ní "
  "exchange contracts, " + L("“front-running” (MEV)", U["eth_mev"]) + " tí a mọ̀ lórí Ethereum fẹ́rẹ̀ẹ́ pòórá.")
H2("7.3 Ìjìyà tó bá ìpalára mu")
P("Ẹni tó bá dìbò lọ́nà méjì tó tako ara wọn fún block kan náà (double signing) ń pàdánù apá kan stake rẹ̀. Èyí "
  "sábà máa ń ṣẹlẹ̀ nípa àṣìṣe, fún àpẹẹrẹ tí kọ́kọ́rọ́ kan náà bá ń ṣiṣẹ́ lórí ẹ̀rọ méjì. Nítorí náà, ìjìyà dá lórí "
  "iye àwọn tó ṣe é lẹ́ẹ̀kan náà. Software náà tún ń dáàbò bo lọ́wọ́ àṣìṣe: ó ń rántí ohun tí ó ti buwọ́lù, ó sì ń "
  "dákẹ́ lẹ́yìn yíyí ẹ̀rọ padà tí kọ́kọ́rọ́ rẹ̀ bá ti ń ṣiṣẹ́ níbòmíràn. Ní ìpele ìbẹ̀rẹ̀, ìyọkúrò kò lè ju 5 % lọ.")
FIG(F + "f_slashing.png", "Ìyọkúrò fún double signing: max(1 %, 3 × ìpín àwọn tó ṣe é lẹ́ẹ̀kan náà), 100 % jùlọ.")

# ================================================================ 8
H("Ìpele ìbẹ̀rẹ̀ (bootstrap)")
P("Ìbẹ̀rẹ̀ tó tọ́ láìsí premint ní ìṣòro “adìyẹ àti ẹyin” nínú proof of stake: ní ìbẹ̀rẹ̀, kò sí ẹni tó ní TOY, "
  "nítorí náà kò sí ẹni tó ní stake láti dáàbò bo network. ToyChain yanjú èyí ní gbangba fún àkókò díẹ̀:")
B(["<b>Start validators 21</b>, tí àwọn olùyọ̀ǹda láti ibi àti olùṣiṣẹ́ tó yàtọ̀ síra ń ṣiṣẹ́, ń dáàbò bo network "
   "pẹ̀lú agbára dọ́gba. Orúkọ wọn àti owó tí wọ́n ń gbà hàn gbangba nínú ToyScan.",
   "<b>Èrè tó ń dín kù:</b> Ní ìbẹ̀rẹ̀, wọ́n jọ ń gba ìdajì gbogbo TOY tuntun. Ìpín náà ń dín kù déédéé lójoojúmọ́, ó sì "
   "di òdo lẹ́yìn ọjọ́ 180. A ń ka ọjọ́ ní <i>plan time</i>: ìdádúró kọ̀ọ̀kan láàrin block méjì ń kà ní ìṣẹ́jú 10 jùlọ, "
   "kí ìdádúró pípẹ́ má baà ti aago síwájú. Ààlà kan ń dènà kí ìṣẹ̀dá tó yára tàbí block tí a mọ̀ọ́mọ̀ fà sẹ́yìn mú èrè "
   "púpọ̀ wá. Lápapọ̀ ó jẹ́ nǹkan bí mílíọ̀nù 2.22 TOY jùlọ (nǹkan bí 2.2 % nínú iye tó pọ̀ jùlọ).",
   "<b>Èrè tí a tì pa:</b> Èrè kọ̀ọ̀kan wà ní títì fún epoch 365 (nǹkan bí ọdún kan), ó sì lè jẹ́ ìjìyà fún ìwà àìtọ́.",
   "<b>Ìfàlé tó le:</b> Ìpele ìbẹ̀rẹ̀ ń parí lẹ́yìn ọjọ́ 90 ní kùtùkùtù jùlọ, àti nígbà tí validator tuntun 21, ọ̀kọ̀ọ̀kan "
   "pẹ̀lú TOY 1,000 tí a ti tì pa fún epoch 30, bá jọ ní ìdá méjì nínú mẹ́ta stake tó ń ṣiṣẹ́. Lẹ́yìn náà, start "
   "validators pàdánù ipò àkànṣe wọn. Tí ìfàlé kò bá tíì ṣẹlẹ̀ lẹ́yìn ọjọ́ 180, wọ́n ń bá a lọ láti dáàbò bo network "
   "láìsí èrè títí yóò fi ṣẹlẹ̀."])
FIG(F + "f_bootstrap.png", "Èrè start validators ní ìwọ̀n ọjọ́ àti àkókò fún ìfàlé.")
P("Ní òtítọ́: ìpele ìbẹ̀rẹ̀ ni apá tí ToyChain ti nílò ìgbẹ́kẹ̀lé púpọ̀ jùlọ nínú ènìyàn díẹ̀. Kò sí protocol tó lè "
  "fi hàn pé validator “tuntun” jẹ́ ènìyàn mìíràn ní tòótọ́. Ṣùgbọ́n àwọn òfin mú kí gbígba agbára olówó ńlá, kí ó sì "
  "hàn sí gbogbo ènìyàn.")

# ================================================================ 9
H("Node tó fúyẹ́ fún ẹ̀rọ àtijọ́")
P("Láti tọ́jú Bitcoin tàbí Ethereum chain pípé nílò ọgọ́rọ̀ọ̀rún gigabyte lónìí. ToyChain ní èrò tó dúró: <b>256 MB "
  "fún node kan</b>. Èyí ṣeé ṣe nítorí pé node ń tọ́jú owó lọ́wọ́lọ́wọ́ gbogbo àkáǹtì alààyè àti fèrèsé kúkúrú àwọn "
  "block tó kẹ́yìn nìkan. Àwọn archive node olùyọ̀ǹda ń tọ́jú block àtijọ́. Ní ìpele tó ń bọ̀, a ó “ká” gbogbo ìtàn "
  "sínú proof cryptographic kékeré kan, kí node tuntun má baà nílò láti gbẹ́kẹ̀lé ẹnikẹ́ni. Títí di ìgbà náà, node "
  "tuntun ń bẹ̀rẹ̀ láti checkpoint tí ó kéré tán ẹgbẹ́ mẹ́ta tí kò gbára lé ara wọn ti buwọ́lù.")
FIG(F + "f_budget.png", "Bí a ṣe pín 256 MB. Ètò ìpamọ́ kọ̀ọ̀kan ní ìdánwò ìwọ̀n tirẹ̀.")
P("Kí àkáǹtì mílíọ̀nù 100 tàbí jù bẹ́ẹ̀ lọ lè rí ààyè, a ní <b>àkáǹtì alààyè àti àkáǹtì tó ń sùn</b>. Àkáǹtì tí a "
  "kò tíì lò fún ìgbà pípẹ́ ni a ń gbé jáde nínú ìrántí nígbà tí ó bá yẹ, ṣùgbọ́n <b>a kì í pa á rẹ́ láé</b>. "
  + L("Merkle proof", U["merkle"]) + " tí kò tó 1 KB ló ń dáàbò bo ohun tó wà nínú rẹ̀, wallet sì ń tọ́jú rẹ̀ fúnra "
  "rẹ̀. Pẹ̀lú transaction tó kàn, àkáǹtì náà ń jí fúnra rẹ̀, pẹ̀lú gbogbo owó rẹ̀.")
FIG(F + "f_dormant.png", "Àkáǹtì alààyè àti àkáǹtì tó ń sùn. Kò sí àkáǹtì tó ń sọnù.", 15.0)
P("Kí ẹnikẹ́ni má baà fi mílíọ̀nù àkáǹtì òfìfo kún ìrántí, a ń ti 0.1 TOY pa gẹ́gẹ́ bí <b>ìdógò ìmúṣiṣẹ́</b> ní "
  "ìgbà àkọ́kọ́ tí owó bá wọlé. Ìdógò náà ṣì jẹ́ ti olùlò, a kì í yọ nínú rẹ̀, a sì ń dá a padà ní kíkún nígbà tí a "
  "bá ti àkáǹtì pa. Ó lè dín kù nìkan lọ́jọ́ iwájú, títí dé 0.01 TOY.")

# ================================================================ 10
H("Ààbò fún àwọn olùlò")
H2("10.1 Ààbò àkáǹtì tí o yàn")
P("Olúkúlùkù lè dáàbò bo àkáǹtì tirẹ̀ sí i. Àwọn ètò wọ̀nyí ń ṣiṣẹ́ nìkan nítorí pé olówó ló ṣètò wọn. Kò sí ẹlòmíràn "
  "tó lè ṣètò wọn.")
B(["<b>Vault delay:</b> Owó yíyọ ńlá ń ṣẹlẹ̀ lẹ́yìn àkókò ìdúró nìkan, a sì lè fagilé e nígbà náà. Nítorí náà, olè kò "
   "lè sọ àkáǹtì di òfìfo lẹ́sẹ̀kẹsẹ̀.",
   "<b>Emergency key àti àdírẹ́sì ààbò:</b> Kọ́kọ́rọ́ kejì tí a tọ́jú lọ́tọ̀ kò lè ná nǹkankan, ṣùgbọ́n ó lè dá owó "
   "yíyọ tó ń dúró dúró, kí ó sì gbé gbogbo owó lọ sí àdírẹ́sì ààbò tí a ti yàn tẹ́lẹ̀.",
   "<b>Self-lock:</b> Fóònù sọnù? O lè ti àkáǹtì rẹ pa fún epoch 30 jùlọ.",
   "<b>Àdírẹ́sì ajogún:</b> Tí àkáǹtì kan bá wà láìsí ìgbésẹ̀ kankan fún ìgbà pípẹ́ gan-an (ó kéré tán epoch 180), "
   "àdírẹ́sì ajogún tí olówó yàn lè gbà á. A ń rán olówó létí lọ́pọ̀ ìgbà ṣáájú, ó sì ṣì lè fagilé e fún epoch 30."])
P("Ní òtítọ́: láìsí emergency key, ẹni tó jí fóònù pẹ̀lú kọ́kọ́rọ́ ní ẹ̀tọ́ kan náà bí olówó. Nítorí náà, wallet "
  "gbani nímọ̀ràn láti ṣe emergency key lórí ẹ̀rọ kejì tàbí lórí ìwé.")
FIG(F + "f_vault.png", "Vault delay: nígbà àkókò ìdúró, olówó lè dá owó yíyọ tí a fura sí dúró.", 15.0)
H2("10.2 Òfin ààbò pẹ̀lú ìwọ̀ntúnwọ̀nsì")
P("A fẹ́ dá ìkọlù dúró ní kùtùkùtù láìṣe àwọn olùlò olóòótọ́ ní ìpalára. Ìdí nìyí tí ìpele mẹ́ta fi wà. Chain fúnra "
  "rẹ̀ ń jẹ ẹ̀ṣẹ̀ tí a lè <b>fi hàn kedere</b> nìkan níyà, bí double signing tàbí block ayédèrú. Àbájáde rẹ̀ jẹ́ ìdènà "
  "tó ń gba ẹ̀tọ́ kúrò, fún àpẹẹrẹ quota ọ̀fẹ́, ṣùgbọ́n <b>kì í gba owó láé</b>. Àtúnṣe ẹ̀ṣẹ̀ ń sọ ìdènà di ìlọ́po "
  "méjì, títí dé epoch 64 jùlọ.")
FIG(F + "f_levels.png", "Ìpele mẹ́ta òfin ààbò. Ìpele A nìkan ló ń ṣiṣẹ́ nínú chain, lórí ẹ̀tọ́ nìkan.", 15.0)
H2("10.3 Constitution")
P("Àwọn òfin kan ṣe pàtàkì tó bẹ́ẹ̀ tí wọ́n fi wà nínú code títí láé, tí kò sí update tàbí ìdìbò ọjọ́ iwájú tó lè "
  "yí wọn padà: iye tó pọ̀ jùlọ ti mílíọ̀nù 100 TOY, ìbẹ̀rẹ̀ láìsí owó, fọ́múlà ìṣẹ̀dá, ìkà léèwọ̀ pípa TOY run, "
  "àkókò tó kéré jùlọ fún epoch, àìsí tokens àti smart contracts, àti ju gbogbo rẹ̀ lọ: <b>kò sí òfin tó lè gba, "
  "dì, tàbí pín TOY láti inú àkáǹtì kan.</b> Ní òtítọ́, ẹnikẹ́ni tó bá pín software tirẹ̀ lè ya chain kúrò. Ìdí nìyí "
  "tí wallet àti ToyScan fi ń ṣàyẹ̀wò constitution fúnra wọn, tí wọ́n sì ń kìlọ̀ tí ìyapa bá rú u. A ó kọ́ version "
  "software kọ̀ọ̀kan lọ́nà " + L("reproducible", U["repro"]) + ", kí ẹnikẹ́ni lè ṣàyẹ̀wò pé ó wá láti inú code tí a tẹ̀jáde.")
H2("10.4 Cryptography tí a ti dán wò")
P("ToyChain kò ṣẹ̀dá ọ̀nà cryptography tirẹ̀. Ó ń lo àwọn standard tí a mọ̀ dáadáa: secp256k1 àti Keccak-256 bí ti "
  "Ethereum fún àkáǹtì, " + L("BIP-39", U["bip39"]) + " fún gbólóhùn ọ̀rọ̀, " + L("BLS12-381", U["bls"]) + " fún "
  "ìbuwọ́lù àpapọ̀ àwọn validator, BLAKE3 fún checksum, Argon2id fún iṣẹ́ kọ̀ǹpútà àti fún dídáàbò bo wallet lórí "
  "ẹ̀rọ. Ìbuwọ́lù kọ̀ọ̀kan ní àmì chain (" + L("EIP-155", U["eip155"]) + "), kí a má baà lò ó lórí chain mìíràn. "
  "Gbogbo ọ̀nà wà lẹ́yìn interface tí a lè pààrọ̀. Kọ́kọ́rọ́ àfikún àṣàyàn gẹ́gẹ́ bí standard NIST "
  + L("ML-DSA", U["mldsa"]) + " yóò kojú " + L("quantum computers", U["pqk"]) + " pẹ̀lú. A tún ń ṣàyẹ̀wò àwọn òfin "
  "ìṣirò pàtàkì jùlọ lọ́nà mathematics pẹ̀lú " + L("Kani", U["kani"]) + ", kì í ṣe lórí àpẹẹrẹ nìkan, ṣùgbọ́n fún "
  "gbogbo nọ́ńbà tó ṣeé ṣe. A ti fi hàn tẹ́lẹ̀: epoch kan kì í san ju iye tó kù lọ láé, àpapọ̀ gbogbo TOY sì máa ń "
  "wà ní mílíọ̀nù 100 jùlọ nígbà gbogbo (apá 17).")

# ================================================================ 11
H("Wallet TOYPAY, ToyScan àti chain mìíràn")
P("<b>Wallet TOYPAY</b> jẹ́ wallet àti node papọ̀. Ó ń ṣiṣẹ́ lórí Android láti nǹkan bí 1 GB RAM, lórí Windows, "
  "Linux àti Raspberry Pi. Energy mode kan ń ṣètò fúnra rẹ̀ bí ẹ̀rọ ṣe ń kópa tó, gẹ́gẹ́ bí batiri, charger, ìgbóná "
  "àti network. A gbèrò interface ní èdè 45, pẹ̀lú mode mẹ́rin láti olùbẹ̀rẹ̀ dé ọ̀jọ̀gbọ́n. Ìkìlọ̀ ààbò ń hàn nínú "
  "gbogbo mode.")
P("Explorer <b>ToyScan</b> ń fi gbogbo nǹkan hàn gbangba: iye TOY tí a ti ṣẹ̀dá, factor m lọ́wọ́lọ́wọ́, ipò ìpele "
  "ìbẹ̀rẹ̀ àti ìdènà kọ̀ọ̀kan pẹ̀lú ìdí rẹ̀. Iná atọ́ka (aláwọ̀ ewé, ìyeyè, pupa) ń ṣe àkópọ̀ ìlànà mẹ́sàn-án, bí "
  "bóyá a ń bọ̀wọ̀ fún iye tó pọ̀ jùlọ, bóyá stake ti kórajọ jù, tàbí bóyá ìpele ìbẹ̀rẹ̀ ti pẹ́ jù. ToyScan kò ní "
  "àǹfààní sí kọ́kọ́rọ́ kankan láé.")
P("Nítorí pé àdírẹ́sì jẹ́ bákan náà pẹ̀lú ti Ethereum, wallet àti ToyScan tún ń fi owó àdírẹ́sì kan náà hàn lórí "
  "EVM chain mìíràn bí Ethereum tàbí Polygon. Àwọn wallet tí a mọ̀ bí MetaMask lè "
  + L("fi ToyChain kún un gẹ́gẹ́ bí network" + EN, U["metamask"]) + ". A mọ̀ọ́mọ̀ kò ní "
  + L("bridge", U["eth_bridges"]) + " láàrin chain: ní 2022, gẹ́gẹ́ bí Chainalysis ti sọ, nǹkan bí ìdá méjì nínú mẹ́ta "
  "gbogbo owó crypto tí a jí wá láti " + L("ìkọlù sí bridge", U["bridgehacks"]) + ". Dípò rẹ̀, a gbèrò "
  + L("atomic swap", U["atomic"]) + ": ènìyàn méjì ń pààrọ̀ tààrà, pẹ̀lú ààbò " + L("hash time lock", U["htlc"]) +
  ". Yálà àwọn méjèèjì gba ohun tí wọ́n fẹ́, tàbí àwọn méjèèjì gba owó wọn padà.")
FIG(F + "f_swap.png", "Atomic swap láàrin TOY àti coin lórí chain mìíràn, láìsí alárinà.")
BOX("<b>Àkíyèsí pàtàkì nípa tokens orúkọ kan náà:</b> Lórí Polygon àti Ethereum, àwọn token àtijọ́ wà tí orúkọ wọn "
    "jẹ́ ToyCoin pẹ̀lú àmì TOY. Wọn <b>kò</b> ní ìsopọ̀ kankan pẹ̀lú TOY ti ToyChain, a kò sì lè pààrọ̀ wọn. Wallet "
    "àti ToyScan ń fi èyí hàn kedere.")

# ================================================================ 12
H("Ìlò")
B(["<b>Ìsanwó ojoojúmọ́</b> láàrin ènìyàn, pẹ̀lú àkọsílẹ̀ ìdí ìsanwó àti ìparí lẹ́yìn nǹkan bí ìṣẹ́jú kan; nígbà tó bá "
   "yá, ìsanwó sí ẹni 32 lẹ́ẹ̀kan náà. Owó kékeré ṣeé ṣe láìsí owó iṣẹ́ nítorí quota ọ̀fẹ́.",
   "<b>Ìkópa láìní owó:</b> fóònù àtijọ́ tàbí Raspberry Pi di node, ó sì lè jèrè TOY àkọ́kọ́ nípasẹ̀ lọ́tìrì iṣẹ́.",
   "<b>Ẹgbẹ́, ìdílé àti iṣẹ́ kékeré:</b> àkáǹtì pẹ̀lú ìbuwọ́lù púpọ̀, escrow pẹ̀lú onídàájọ́, ìyọ̀ǹda subscription pẹ̀lú "
   "ààlà, àti session keys fún app (lẹ́yìn testnet àkọ́kọ́).",
   "<b>Ìmúrasílẹ̀ fún ọjọ́ iwájú:</b> vault delay, emergency key àti àdírẹ́sì ajogún ń dáàbò bo lọ́wọ́ olè àti àdánù.",
   "<b>Ẹ̀bùn fún gbogbo ènìyàn:</b> ẹnikẹ́ni lè fi ẹ̀bùn sí àpò ìsanpadà tó ń sanwó fún transaction ọ̀fẹ́ gbogbo ènìyàn.",
   "<b>Àṣírí nígbà tó bá yẹ:</b> àdírẹ́sì ìgbàwọlé ẹ̀ẹ̀kan gẹ́gẹ́ bí " + L("EIP-5564", U["eip5564"]) + " ń fi ẹni tí a "
   "sanwó fún pamọ́ (a gbèrò rẹ̀ fún milestone M13).",
   "<b>Ẹ̀kọ́ àti ìwádìí:</b> code tó ṣí sílẹ̀, testnet pẹ̀lú àṣẹ kan, àti data gbangba mú kí ToyChain jẹ́ ohun ẹ̀kọ́ fún "
   "ilé-ìwé, yunifásítì àti ẹnikẹ́ni tó nífẹ̀ẹ́ sí i.",
   "<b>Lọ́jọ́ iwájú:</b> ìsopọ̀ fún oníṣòwò, mode ibi ìsanwó, ìsanwó láìsí ayélujára àti ìsopọ̀ pẹ̀lú eré TOYQUEST. Èyí kì "
   "í ṣe apá ìpele àkọ́kọ́."])

# ================================================================ 13
H("Ìjọra àti ìyàtọ̀")
T([["", "Bitcoin", "Ethereum", "ToyChain (a gbèrò)"],
   ["Ìbẹ̀rẹ̀", "ó tọ́, kò sí premint", "presale àti ìpín foundation", "ó tọ́, genesis láìsí owó kankan"],
   ["Iye tó pọ̀ jùlọ", "nǹkan bí mílíọ̀nù 21 BTC", "kò sí ààlà tó dúró", "mílíọ̀nù 100 TOY, kò lè yí padà"],
   ["Ìṣẹ̀dá", "ń já sí ìdajì ní nǹkan bí ọdún 4", "bí ó ti yẹ, a ń jó apá kan owó iṣẹ́",
    "ń dín kù déédéé, factor 0.5 sí 2.0, a kì í jó o"],
   ["Ààbò láti", "iṣẹ́ kọ̀ǹpútà (proof of work)", "stake (proof of stake), 32 ETH fún validator kan",
    "stake ní ìpele 10, 100, 1,000 TOY àti ìpele ìbẹ̀rẹ̀ gbangba"],
   ["Ó parí lẹ́yìn", "nǹkan bí wákàtí 1 (6 confirmations)", "nǹkan bí ìṣẹ́jú 13", "block 1, nǹkan bí ìṣẹ́jú 1"],
   ["Ààyè full node", "nǹkan bí 760 GB", "ọgọ́rọ̀ọ̀rún GB sí ọ̀pọ̀ TB", "èrò: 256 MB"],
   ["Programmability", "scripts rírọrùn", "smart contracts, tokens, NFTs", "iṣẹ́ tí a kọ́ sínú rẹ̀ nìkan"],
   ["Àkáǹtì", "UTXO model", "account model", "account model, àdírẹ́sì Ethereum"],
   ["Owó iṣẹ́", "nígbà gbogbo, gẹ́gẹ́ bí ọjà", "nígbà gbogbo, base fee àti tip",
    "10 sí 100 ọ̀fẹ́ ní epoch kan (ó kéré tán ọjọ́ 1), lẹ́yìn náà fee tàbí iṣẹ́"],
   ["Ìkópa láìní owó", "ṣíṣe node nìkan", "ṣíṣe node nìkan", "bẹ́ẹ̀ni, pẹ̀lú lọ́tìrì iṣẹ́"]],
  [3.5, 3.6, 4.2, 5.1])
P("<b>Bí ti Bitcoin</b>, ToyChain ní ìbẹ̀rẹ̀ tó tọ́, iye tó pọ̀ jùlọ tó dúró, ìṣẹ̀dá tó ń dín kù fún nǹkan bí ọdún "
  "25, àìsí aláṣẹ àárín àti code tó ṣí sílẹ̀. <b>Bí ti Ethereum</b>, ó ní àdírẹ́sì, gbólóhùn ọ̀rọ̀, ọ̀nà ìbuwọ́lù, "
  "proof of stake àti èrò base fee tó ń ga nígbà tí network bá kún. <b>Ohun tuntun</b> ni àkópọ̀ ìkópa láìní owó, quota "
  "ọ̀fẹ́, ààlà ìpamọ́ 256 MB, àkáǹtì tó ń sùn, ààbò àkáǹtì àṣàyàn, àti constitution tó dènà ìfọwọ́kàn owó ẹlòmíràn.")

# ================================================================ 14
H("Àǹfààní")
B(["<b>Òtítọ́:</b> kò sí ẹni tí a fún ní TOY lọ́fẹ̀ẹ́; gbogbo òfin ló wà ní gbangba, ó sì jẹ́ bákan náà fún gbogbo ènìyàn.",
   "<b>Ìrọ̀rùn wíwọlé:</b> ìkópa pẹ̀lú ẹ̀rọ àtijọ́, láìní owó àti láìní ìmọ̀ àkànṣe, ní èdè 45.",
   "<b>Ìparí kíákíá:</b> ìsanwó di ìdánilójú lẹ́yìn nǹkan bí ìṣẹ́jú kan.",
   "<b>Iná mànàmáná díẹ̀:</b> kò sí ìdíje agbára kọ̀ǹpútà. Lọ́tìrì iṣẹ́ ń ṣiṣẹ́ lórí charger nìkan, àti bí olùlò ṣe gbà "
   "láàyè nìkan. Ìṣirò púpọ̀ ń mú tíkẹ́ẹ̀tì púpọ̀ wá ní ìwọ̀n, kò sí àǹfààní àkànṣe.",
   "<b>Ìrọ̀rùn gẹ́gẹ́ bí ààbò:</b> láìsí smart contracts, kò sí program ẹlòmíràn tó lè dì owó tàbí sọ ọ́ nù nípa àṣìṣe.",
   "<b>Ààbò ohun ìní:</b> ìdènà ń gba ẹ̀tọ́ nìkan, kì í ṣe owó. Àkáǹtì kì í sọnù. A kì í pa TOY run.",
   "<b>Ìṣípayá:</b> ToyScan, ìròyìn oṣooṣù láti testnet, àti version software tí a lè tún kọ́.",
   "<b>Ìbámu:</b> àdírẹ́sì Ethereum àti àwọn wallet tí a mọ̀ ń ṣiṣẹ́."])

# ================================================================ 15
H("Ewu àti ìbéèrè tó ṣì wà")
P("Whitepaper olóòótọ́ tún ń dárúkọ ohun tó lè lọ ní àìtọ́. Àwọn ewu pàtàkì lónìí:")
B(["<b>Ìpele ìbẹ̀rẹ̀ pẹ̀pẹ̀:</b> ToyChain kò tíì ṣiṣẹ́ rí. Ó ṣeé ṣe kí àṣìṣe wà nínú èrò tàbí code, a ó sì wá wọn lórí "
   "testnet. Àyẹ̀wò méjì láti ọwọ́ AI review agent tí kò kópa nínú kíkọ ọ̀rọ̀ náà ti rí àìlera, a sì ti ṣàtúnṣe wọn; "
   "àyẹ̀wò láti ọwọ́ ènìyàn kò tíì ṣẹlẹ̀. Àyẹ̀wò mìíràn àti audit látìta jẹ́ dandan kí mainnet tó bẹ̀rẹ̀.",
   "<b>Network kékeré, ààbò kékeré:</b> chain tuntun rọrùn láti kọlù ní ìbẹ̀rẹ̀, nítorí pé ó rọrùn láti borí olùkópa "
   "díẹ̀. Ààbò ń wá láti ọ̀pọ̀ olùkópa tí kò gbára lé ara wọn nìkan.",
   "<b>Ìgbẹ́kẹ̀lé ní ìpele ìbẹ̀rẹ̀:</b> start validators 21 ní ojúṣe ńlá ní oṣù àkọ́kọ́, wọ́n sì ń gba èrè tí ó ní ààlà. "
   "A ó rí bóyá ìfàlé sí validator tuntun yóò yọrí sí rere.",
   "<b>Ààlà ìpamọ́:</b> bóyá 256 MB yóò tó fún ọ̀pọ̀ ọdún dá lórí ìlò àti proof layer tó ń bọ̀. Tí kò bá tó, a ó sọ ọ́ "
   "ní gbangba pẹ̀lú nọ́ńbà. Node ńlá yóò nílò ìpamọ́ púpọ̀ sí i nígbà náà.",
   "<b>Ohun èlò tí a kò tíì dán wò tó:</b> fún àwọn ọ̀nà kan (bí VRF àti consensus library), Rust library tí a ti "
   "ṣàyẹ̀wò dáadáa kò pọ̀. A ó kọ wọ́n gẹ́gẹ́ bí standard, àwọn ògbógi látìta yóò sì ṣàyẹ̀wò wọn.",
   "<b>Iṣẹ́ kọ̀ǹpútà:</b> a ó wọ̀n lórí testnet bóyá ìrántí 16 MiB tó láti dènà ẹ̀rọ àkànṣe.",
   "<b>Òfin:</b> cryptocurrency kan ń kan òfin Europe (" + L("MiCA", U["mica"]) + "), òfin owó-orí, àti bóyá òfin "
   "tẹ́tẹ́ (nítorí ọ̀rọ̀ “lọ́tìrì”). Àyẹ̀wò òfin jẹ́ dandan kí mainnet tó bẹ̀rẹ̀. Àdírẹ́sì ajogún kò rọ́pò òfin ogún.",
   "<b>Àṣìṣe olùlò:</b> ẹni tó bá pàdánù tàbí sọ gbólóhùn ọ̀rọ̀ rẹ̀ fún ẹlòmíràn yóò pàdánù owó rẹ̀. Ààbò àkáǹtì àti "
   "ìkìlọ̀ ń dín ewu yìí kù, ṣùgbọ́n wọn kò lè mú un kúrò pátápátá.",
   "<b>Iye:</b> TOY kò ní iye tí a ṣèlérí. Bóyá ọjà yóò wà rárá kò tíì dájú (apá 16). Kò sí ẹni tó gbọ́dọ̀ fi owó tí "
   "kò lè pàdánù sínú rẹ̀.",
   "<b>Ìdàrúdàpọ̀ àti jìbìtì:</b> tokens àtijọ́ tí wọ́n ní orúkọ kan náà lórí chain mìíràn lè da àwọn olùlò rú tàbí "
   "kí a lò wọ́n fún jìbìtì. Àwọn aláṣẹ EU ń kìlọ̀ ní gbogbogbòò nípa " + L("ewu crypto" + EN, U["esas_warning"]) + "."])

# ================================================================ 16
H("Àǹfààní nínú ọjà crypto: ìgbéyẹ̀wò")
P("Orí yìí gbìyànjú ìgbéyẹ̀wò tó dákẹ́ jẹ́jẹ́. Kì í ṣe àsọtẹ́lẹ̀, kì í sì ṣe ìmọ̀ràn ìdókòwò. Nọ́ńbà ọjà jẹ́ ti àárín "
  "October 2026, wọ́n sì ń yí padà kíákíá.")
H2("16.1 Ipò ìbẹ̀rẹ̀")
P("Ní 18 October 2026, gbogbo ọjà crypto tó nǹkan bí $2.7 trillion, nǹkan bí 60 % nínú rẹ̀ sì jẹ́ Bitcoin. Ọjà fún "
  "coin kékeré kò dára: nínú crypto 100 tó tóbi jùlọ, 14 péré ló gòkè lọ́jọ́ náà (" + L("ìròyìn ọjà" + EN, U["market"])
  + "; ipò lọ́wọ́lọ́wọ́ wà lórí " + L("CoinGecko", U["cg_charts"]) + "). Ní àkókò kan náà, iṣẹ́ àkànṣe tuntun pọ̀ jù "
  "nínú ọjà. Gẹ́gẹ́ bí ìwádìí CoinGecko, ó ju crypto mílíọ̀nù 20 lọ tí a ti kà títí di òpin 2025. Ní 2025 nìkan, ó ju "
  "mílíọ̀nù 11.6 lọ tó kú, ìyẹn ju ìdajì gbogbo èyí tí a ti kà rí lápapọ̀ (" + L("ìròyìn" + EN, U["dead"]) + "). Èyí "
  "tó pọ̀ jùlọ nínú wọn jẹ́ tokens tí kò pẹ́ lórí chain tó wà tẹ́lẹ̀, kì í ṣe blockchain tiwọn. Síbẹ̀, nọ́ńbà wọ̀nyí "
  "fi hàn bí ó ti ṣòro tó láti rí àfiyèsí àti olùlò tí yóò dúró.")
FIG(F + "f_deadcoins.png", "Crypto tó kú ní ọdún kọ̀ọ̀kan gẹ́gẹ́ bí data CoinGecko. Èyí tó pọ̀ jùlọ nínú àwọn tuntun "
                           "kì í là á já.", 15.0)
P("Nínú EU, òfin " + L("MiCA", U["esma"]) + " ń ṣiṣẹ́ láìsí àkókò ìyípadà láti 1 July 2026: àwọn olùpèsè iṣẹ́ crypto "
  "nílò ìwé àṣẹ, bí bẹ́ẹ̀ kọ́ wọ́n gbọ́dọ̀ dáwọ́ dúró láti sìn àwọn oníbàárà EU (" + L("AMF" + EN, U["amf"]) + "). Fún "
  "ToyChain, èyí túmọ̀ sí nǹkan méjì: àwọn exchange àti wallet provider tí wọ́n bá fẹ́ fi TOY kún un lọ́jọ́ iwájú yóò "
  "ṣàyẹ̀wò dáadáa. Àti fún títà sí gbogbo ènìyàn tàbí fífi sí ọjà, MiCA sábà máa ń béèrè crypto-asset white paper "
  "gẹ́gẹ́ bí òfin tó dúró. Bóyá àti bí èyí ṣe kan coin tí kò ní olùgbéjáde bí TOY gbọ́dọ̀ di mímọ̀ lọ́nà òfin kí mainnet "
  "tó bẹ̀rẹ̀. Ìwé yìí <b>kì í ṣe</b> irú white paper bẹ́ẹ̀.")
H2("16.2 Ohun tó ń ṣe ToyChain láǹfààní")
B(["<b>Òtítọ́ tí a lè gbẹ́kẹ̀lé:</b> ìbẹ̀rẹ̀ láìsí owó kankan ṣọ̀wọ́n. Nínú ọjà tí ọ̀pọ̀ iṣẹ́ àkànṣe ti ń bẹ̀rẹ̀ pẹ̀lú ìpín "
   "ńlá fún àwọn inú, èyí jẹ́ àmì ìyàtọ̀ tí ẹnikẹ́ni lè ṣàyẹ̀wò nínú ToyScan.",
   "<b>Wíwọlé tó rọrùn:</b> ìkópa pẹ̀lú fóònù àtijọ́, láìra nǹkankan, láìní ìmọ̀ tẹ́lẹ̀. Èyí lè dé ọ̀dọ̀ àwọn tí kì bá "
   "tíì ṣe node rí, ó sì ń ṣẹ̀dá ọ̀pọ̀ olùkópa kékeré dípò díẹ̀ ńlá.",
   "<b>Ibi ìkọlù díẹ̀:</b> láìsí smart contracts àti bridges, ọ̀pọ̀ ohun tó fa àdánù ńlá ní ọdún díẹ̀ sẹ́yìn pòórá.",
   "<b>Ìbámu:</b> àdírẹ́sì Ethereum mú kí ó rọrùn fún àwọn tó ti ní wallet.",
   "<b>Ìṣípayá àti èdè:</b> licence ọ̀fẹ́, àkọsílẹ̀ ìpinnu tí a lè tọpinpin (gbangba láti testnet) àti èdè 45 mú kí "
   "iṣẹ́ àkànṣe yìí ṣí sílẹ̀ fún àwùjọ àgbáyé. Whitepaper yìí ti wà ní èdè German, Gẹ̀ẹ́sì, Faransé àti Yorùbá."])
H2("16.3 Ohun tó ń ṣe é ní ìdíwọ́")
B(["<b>Kò sí owó ìbẹ̀rẹ̀:</b> láìsí premint, kò sí owó fún developer, audit, ìpolówó tàbí exchange listing. Iṣẹ́ àkànṣe "
   "náà gbára lé olùyọ̀ǹda, ẹ̀bùn àti ìrànlọ́wọ́.",
   "<b>Agbára network ńlá:</b> olùlò, oníṣòwò àti exchange ń kórajọ sí chain ńlá. Chain tuntun gbọ́dọ̀ fúnni ní ìdí "
   "tó ṣe kedere láti yípadà.",
   "<b>A mọ̀ọ́mọ̀ fi ààlà sí i:</b> láìsí smart contracts, kò sí ecosystem app, tokens àti decentralised exchange tó ń "
   "fa àfiyèsí àti owó púpọ̀ lórí chain mìíràn.",
   "<b>Ìdíje líle nínú ìsanwó:</b> fún ìsanwó ojoojúmọ́, ọ̀pọ̀ ènìyàn ń lo stablecoins tàbí network tó ti wà. Coin tí "
   "iye rẹ̀ ń yí padà kò fà mọ́ra tó fún èyí.",
   "<b>Ẹgbẹ́ kékeré, ìbẹ̀rẹ̀ pẹ̀pẹ̀:</b> milestone méjìlá, audit àti àlàyé òfin ṣì kù kí mainnet tó bẹ̀rẹ̀.",
   "<b>Orúkọ:</b> tokens àtijọ́ pẹ̀lú orúkọ kan náà mú kí ó ṣòro láti rí iṣẹ́ àkànṣe yìí, ó sì lè fa ìdàrúdàpọ̀."])
FIG(F + "f_swot.png", "Agbára, àìlera, àǹfààní àti ewu ní ṣókí.", 15.0)
H2("16.4 Àwọn ìṣẹ̀lẹ̀ tó ṣeé ṣe")
T([["Ìṣẹ̀lẹ̀", "Àpèjúwe", "Ìgbéyẹ̀wò"],
   ["A · Ìkùnà ṣáájú mainnet", "Testnet fi ìṣòro ìpìlẹ̀ hàn, tàbí olùkópa kò tó.",
    "ó ṣeé ṣe, bí ti ọ̀pọ̀ iṣẹ́ àkànṣe"],
   ["B · Iṣẹ́ àkànṣe ẹ̀kọ́ àti àwùjọ", "Testnet tó dúró dáadáa àti àwùjọ kékeré tó ń ṣiṣẹ́; a kò fi bẹ́ẹ̀ ta TOY.",
    "ó ṣeé ṣe jùlọ tí iṣẹ́ bá ń bá a lọ"],
   ["C · Ọjà kékeré ní tòótọ́", "Mainnet lẹ́yìn audit, node ọgọ́rọ̀ọ̀rún sí ẹgbẹ̀rún kan, títà lórí platform díẹ̀, ìlò "
                               "nínú ibi àkànṣe (eré, ẹgbẹ́, ẹ̀bùn).", "ó ṣeé ṣe; ó nílò àwùjọ, audit àti òfin tó ṣe kedere"],
   ["D · Ìlò káàkiri", "Ọ̀kan lára coin olókìkí pẹ̀lú mílíọ̀nù olùlò.", "kò fi bẹ́ẹ̀ ṣeé ṣe rárá"]],
  [4.2, 7.6, 4.6])
H2("16.5 Ohun tó lè mú ìyàtọ̀ wá")
B(["<b>Start validators tí kò gbára lé ara wọn</b> láti orílẹ̀-èdè ọ̀tọ̀ọ̀tọ̀, tí wọ́n ń kópa ní gbangba.",
   "<b>Audit ààbò látìta</b>, tí a fi ẹ̀bùn tàbí ètò ìrànlọ́wọ́ fún software ọ̀fẹ́ sanwó fún.",
   "<b>Ìlò kan tó ṣe kedere</b> tí kò ní dára tó láìsí ToyChain, bí eré TOYQUEST, níbi tí àwọn olùṣeré ti ń kópa pẹ̀lú "
   "fóònù wọn, tí wọ́n sì ń jèrè TOY.",
   "<b>Ìṣípayá déédéé:</b> ìròyìn oṣooṣù, nọ́ńbà gbangba, àṣà olóòótọ́ nípa àṣìṣe.",
   "<b>Òfin tó ṣe kedere</b> ṣáájú ìgbésẹ̀ kọ̀ọ̀kan tó kan owó gidi."])
P("<b>Ìparí ìgbéyẹ̀wò:</b> gẹ́gẹ́ bí ìdókòwò, a kò lè díwọ̀n ToyChain lónìí, bí ọ̀pọ̀ cryptocurrency tuntun, ó sì ní ewu "
  "àdánù tó ga gan-an. Gẹ́gẹ́ bí iṣẹ́ àkànṣe technical àti ti àwùjọ, ToyChain ní àwòrán tó ṣe kedere tí kò wọ́pọ̀: ó tọ́, "
  "ó fúyẹ́, ó yé ni, ó sì ṣọ́ra. Àǹfààní rẹ̀ gidi kò sí nínú kí iye rẹ̀ gòkè kíákíá, ṣùgbọ́n nínú kíkọ́ àwùjọ kékeré tó "
  "gbàgbọ́, tó ń ṣe network náà fúnra rẹ̀. Tí èyí bá yọrí sí rere, ohun tó tóbi jù lè dàgbà láti inú rẹ̀ bí ọdún ti ń "
  "lọ. Kò sí ìdánilójú.")

# ================================================================ 17
H("Ipò àti ọ̀nà iwájú")
P("<b>Lónìí (19 October 2026): ṣáájú ìpele ìdánwò.</b> A ti kọ èrò náà sílẹ̀ nínú ìpinnu 114. A pín iṣẹ́ kíkọ́ sí "
  "milestone 13. Milestone M1 ti parí: òfin fún iye owó àti fún ṣíṣẹ̀dá TOY tuntun ti wà nínú program, a sì ti fi "
  "ìdánwò aládàáṣe 45 ṣàyẹ̀wò wọn. Rust implementation ń fúnni ní nọ́ńbà kan náà gẹ́lẹ́, títí dé ẹ̀ka tó kéré jùlọ, bí "
  "simulation Python tí a kọ lọ́tọ̀. Àwọn ìdánwò ń ṣiṣẹ́ fúnra wọn ní gbogbo ìyípadà lórí Linux, Windows àti ARM. Pẹ̀lú "
  "Kani, a ti fi òfin pàtàkì mẹ́ta hàn lọ́nà mathematics, ọ̀kọ̀ọ̀kan ní ìṣẹ́jú méjì: kò sí epoch tó ń san ju iye tó kù "
  "lọ, àpapọ̀ wà ní mílíọ̀nù 100 TOY jùlọ, pínpín block kan sì péye. Proof mẹ́ta mìíràn fún pínpín pẹ̀lú ìpín èrè kò "
  "tíì parí lẹ́yìn ìṣẹ́jú 30. Nítorí náà, program fúnra rẹ̀ ń ṣàyẹ̀wò pínpín yìí ní block kọ̀ọ̀kan, àwọn ìdánwò mìíràn "
  "sì bò ó.")
FIG(F + "f_roadmap.png", "Ọ̀nà iwájú. A mọ̀ọ́mọ̀ kò fi ọjọ́ kan sí mainnet.")
T([["Ìpele", "Àkóónú", "Ipò"],
   ["Èrò", "Specification v0.3, àkọsílẹ̀ ìpinnu, àyẹ̀wò méjì", "ó ti parí, a ń tún un ṣe"],
   ["Phase 1, M1 sí M11", "Types ìpìlẹ̀ àti ìṣẹ̀dá, cryptography, àkáǹtì àti transaction, consensus àti ìpele ìbẹ̀rẹ̀, "
    "ìpamọ́, network, interfaces, wallet, platform, testnet, ToyScan", "M1 ti parí, M2 ló kàn"],
   ["Testnet 1", "Core pẹ̀lú ààbò àkáǹtì, èdè 45 àti mode mẹ́rin; lákọ̀ọ́kọ́ lórí ẹ̀rọ kan, lẹ́yìn náà gbangba",
    "a ti gbèrò"],
   ["Phase 1, M12 àti M13", "transaction mìíràn, atomic swap, orúkọ, àdírẹ́sì ẹ̀ẹ̀kan, post-quantum keys", "a ti gbèrò"],
   ["Phase 2", "Proof layer: ká ìtàn, node láìsí ìgbẹ́kẹ̀lé checkpoint", "a ti gbèrò"],
   ["Lẹ́yìn náà", "Ìṣàkóso, mainnet (lẹ́yìn audit, ìdánwò agbára àti àyẹ̀wò òfin nìkan, a ó kéde ọjọ́ ṣáájú), bridges "
    "àti ìsanwó láìsí ayélujára tó ṣeé ṣe", "kò tíì dájú"]],
  [3.4, 9.4, 3.6])
P("Kò sí ọjọ́ tó dúró fún mainnet. Yóò bẹ̀rẹ̀ nìkan nígbà tí testnet bá ń ṣiṣẹ́ dáadáa, tí a ti ṣàyẹ̀wò ààbò, tí a sì "
  "ti yanjú ìbéèrè òfin. A ó kéde ọjọ́ ìbẹ̀rẹ̀ ní gbangba ṣáájú, kí gbogbo ènìyàn lè bẹ̀rẹ̀ lẹ́ẹ̀kan náà.")

# ================================================================ 18
H("Ìparí")
P("ToyChain fẹ́ fi hàn pé cryptocurrency lè bẹ̀rẹ̀ lọ́nà tó tọ́, lo ìpamọ́ díẹ̀, kí ó sì ṣì yára, kí ó sì ní ààbò. Ẹnikẹ́ni "
  "gbọ́dọ̀ lè kópa, kódà láìní owó àti pẹ̀lú ẹ̀rọ àtijọ́. Nítorí èyí, ToyChain mọ̀ọ́mọ̀ fi ọ̀pọ̀ ohun tí àwọn chain mìíràn "
  "ń fúnni sílẹ̀, ó sì gbára lé iṣẹ́ díẹ̀ tí a kọ́ sínú rẹ̀, tí ó rọrùn láti ṣàyẹ̀wò. Kì í ṣe ìwé yìí ló máa pinnu "
  "bóyá yóò yọrí sí rere, testnet ni. A gbà ìkópa, àríwísí àti àyẹ̀wò tọwọ́tẹsẹ̀. Code, specification àti àkọsílẹ̀ "
  "ìpinnu wà lórí " + L("GitHub", U["repo"]) + ", gbogbo ènìyàn yóò sì rí wọn ní ìgbà testnet gbangba ní tuntun jùlọ.")

# ================================================================ Àfikún
story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Àfikún A: Iye ìbẹ̀rẹ̀ pàtàkì", h1))
P("Gbogbo iye, àyàfi ti constitution, jẹ́ àbá fún testnet, a ó sì wọ̀n wọ́n, a ó sì ṣàtúnṣe wọn níbẹ̀.")
T([["Apá", "Iye ìbẹ̀rẹ̀"],
   ["Iye àti pínpín", "100,000,000 TOY, ibi 18 lẹ́yìn àmì ìdá (constitution)"],
   ["Ìṣẹ̀dá ní epoch kan", "0.0505 % nínú iye tó kù × m; m láàrin 0.5 àti 2.0, ìyípadà 0.05 jùlọ ní epoch kan"],
   ["Pínpín TOY tuntun", "50 % consensus, 40 % lọ́tìrì iṣẹ́, 10 % àpò ìsanpadà"],
   ["Àkókò", "block: ìṣẹ́jú-àáyá 60, ó kéré tán 10; epoch: ó kéré tán block 1,440 àti wákàtí 24"],
   ["Committee àti block", "validator 128 jùlọ, block 128 KB jùlọ, ó parí lẹ́yìn block 1"],
   ["Ìpele ìbẹ̀rẹ̀", "start validators 21, èrè láti 50 % dé 0 % ní ọjọ́ 180, títì fún epoch 365, ìfàlé lẹ́yìn ọjọ́ 90 "
                    "ní kùtùkùtù jùlọ"],
   ["Stake", "10 / 100 / 1,000 TOY; títú lẹ́yìn epoch 21; ààlà 5 %, láti validator 100, 1 %"],
   ["Ìjìyà double signing", "max(1 %, 3 × ìpín àwọn tó ṣe é lẹ́ẹ̀kan náà), 100 % jùlọ, ìdènà ipa fún epoch 7"],
   ["Àkáǹtì", "ìdógò ìmúṣiṣẹ́ 0.1 TOY (ó lè dín kù dé 0.01 TOY); gbígbé àkáǹtì tó ń sùn jáde láti 90 % ìkún"],
   ["Quota", "transaction ọ̀fẹ́ 10 sí 100 fún àkáǹtì kan ní epoch kan, láti ọjọ́ orí àkáǹtì block 100"],
   ["Lọ́tìrì iṣẹ́", "Argon2id pẹ̀lú 16 MiB; olùborí 64 jùlọ ní block kan, èrè ó kéré tán 0.2 TOY"],
   ["Ìdènà", "fún ẹ̀ṣẹ̀ tí a lè fi hàn kedere nìkan, ìlọ́po méjì fún àtúnṣe, títí dé epoch 64 jùlọ"],
   ["Ìpamọ́", "256 MB fún node kan: 128 MB state, 48 MB block window, 80 MB proofs, indexes àti ìpamọ́"]],
  [4.0, 12.4])

story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Àfikún B: Àwọn link", h1))
P("Àlàyé pàtàkì fún kíkà síwájú, ní ìtòlẹ́sẹẹsẹ ọ̀rọ̀. Gbogbo wọn jẹ́ èdè Gẹ̀ẹ́sì àyàfi tí a bá sọ. Ipò link: October "
  "2026; link iṣẹ́ àkànṣe yóò ṣiṣẹ́ nígbà tí repository bá di gbangba.")
T([["Ọ̀rọ̀", "Link", "Nípa kí ni"],
   ["Ìpìlẹ̀", L("Bitcoin whitepaper", U["btc_pdf"]), "ìwé àkọ́kọ́ ti 2008"],
   ["Ìpìlẹ̀", L("Bitcoin full node 2026 (Spark)", U["btc_size"]), "ìpamọ́ àti iye owó node"],
   ["Iye owó", L("Controlled supply (Bitcoin Wiki)", U["btc_supply"]), "bí Bitcoin ṣe ń dín ìṣẹ̀dá kù"],
   ["Consensus", L("Consensus mechanisms (ethereum.org)", U["eth_consensus"]), "àkópọ̀"],
   ["Consensus", L("Proof of stake (ethereum.org)", U["eth_pos"]), "stake, validators, ìparí"],
   ["Consensus", L("The latest gossip on BFT consensus", U["tendermint"]), "ìwé nípa Tendermint"],
   ["Consensus", L("MEV (ethereum.org)", U["eth_mev"]), "“front-running” lórí Ethereum"],
   ["Technical", L("BLS12-381 For The Rest Of Us", U["bls"]), "ìbuwọ́lù àpapọ̀"],
   ["Technical", L("Merkle tree (Wikipedia)", U["merkle"]), "proof fún àkáǹtì tó ń sùn"],
   ["Technical", L("RFC 9106: Argon2", U["argon2"]), "standard iṣẹ́ kọ̀ǹpútà"],
   ["Technical", L("RFC 9381: VRF", U["vrf"]), "àìròtẹ́lẹ̀ tí a lè ṣàyẹ̀wò"],
   ["Technical", L("Dandelion++", U["dandelion"]), "ààbò IP ẹni tó fi ránṣẹ́"],
   ["Technical", L("libp2p", U["libp2p"]), "network library"],
   ["Technical", L("Post-quantum cryptography (IEEE Spectrum)", U["pqk"]), "ìfáárà"],
   ["Technical", L("NIST FIPS 204 (ML-DSA)", U["mldsa"]), "post-quantum signatures"],
   ["Technical", L("Kani (Ferrous Systems)", U["kani"]), "àyẹ̀wò mathematics fún Rust code"],
   ["Technical", L("Reproducible Builds", U["repro"]), "version software tí a lè ṣàyẹ̀wò"],
   ["Àkáǹtì", L("BIP-39", U["bip39"]), "standard gbólóhùn ọ̀rọ̀ 12/24"],
   ["Àkáǹtì", L("EIP-155", U["eip155"]), "àmì chain nínú ìbuwọ́lù kọ̀ọ̀kan"],
   ["Àkáǹtì", L("EIP-5564 (QuickNode)", U["eip5564"]), "àdírẹ́sì ìgbàwọlé ẹ̀ẹ̀kan"],
   ["Owó iṣẹ́", L("EIP-1559", U["eip1559"]), "base fee lórí Ethereum"],
   ["Wallet", L("Add a network in MetaMask", U["metamask"]), "fífi network kún un"],
   ["Chain mìíràn", L("Bridges (ethereum.org)", U["eth_bridges"]), "bí bridge ṣe ń ṣiṣẹ́, ewu"],
   ["Chain mìíràn", L("Bridge hacks 2022 (Decrypt)", U["bridgehacks"]), "nọ́ńbà Chainalysis"],
   ["Chain mìíràn", L("Atomic swap (Blockstream)", U["atomic"]), "pàṣípààrọ̀ láìsí alárinà"],
   ["Chain mìíràn", L("Hashed Timelock Contracts", U["htlc"]), "títì àkókò lẹ́yìn rẹ̀"],
   ["Iná mànàmáná", L("Cambridge Bitcoin Electricity Index", U["cbeci"]), "lílo iná Bitcoin"],
   ["Ọjà", L("CoinGecko charts", U["cg_charts"]), "iye ọjà lọ́wọ́lọ́wọ́"],
   ["Ọjà", L("Market report 18 Oct 2026 (DailyForex)", U["market"]), "ipò ní ọjọ́ náà"],
   ["Ọjà", L("CoinGecko study on failed coins (ForkLog)", U["dead"]), "nọ́ńbà 2021 sí 2025"],
   ["Òfin", L("MiCA regulation (EUR-Lex)", U["mica"]), "ọ̀rọ̀ òfin"],
   ["Òfin", L("MiCA rulebook (ESMA)", U["esma"]), "òfin àti ìtọ́sọ́nà"],
   ["Òfin", L("End of the MiCA transition (AMF)", U["amf"]), "ohun tó ń ṣiṣẹ́ láti 1/7/2026"],
   ["Ààbò oníbàárà", L("EU supervisors' warning (ESMA)", U["esas_warning"]), "ewu àti ààbò tó ní ààlà"],
   ["Iṣẹ́ àkànṣe", L("github.com/TOYfoundation/toycoin", U["repo"]), "code, specification (German)"]],
  [3.0, 7.6, 5.8])

story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Àfikún C: Orísun àti standard", h1))
refs = [
    "S. Nakamoto: Bitcoin: A Peer-to-Peer Electronic Cash System, 2008.",
    "E. Buchman, J. Kwon, Z. Milosevic: The latest gossip on BFT consensus (Tendermint), 2018.",
    "BIP-39, EIP-155, EIP-1559, EIP-2718, EIP-5564.",
    "D. Boneh, B. Lynn, H. Shacham: Short signatures from the Weil pairing (BLS), 2001; BLS12-381.",
    "RFC 9106: Argon2. RFC 9381: Verifiable Random Functions (VRFs).",
    "G. Fanti et al.: Dandelion++, 2018.",
    "NIST FIPS 204: ML-DSA, 2024.",
    "Regulation (EU) 2023/1114 (MiCA).",
    "Data ọjà: DailyForex, 18 Oct 2026; data CoinGecko nípasẹ̀ ForkLog, January 2026; Chainalysis nípasẹ̀ Decrypt, 2022; "
    "Spark, Bitcoin Full Node Requirements 2026.",
    "ToyChain: technical specification v0.3, àkọsílẹ̀ ìpinnu, ètò ìkọ́lé phase 1, àyẹ̀wò 1 àti 2, simulation "
    "m-simulation.py; github.com/TOYfoundation/toycoin (German).",
]
for i, r in enumerate(refs, 1):
    story.append(Paragraph(f"[{i}] {r}", small))

story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Àfikún D: Nípa ẹ̀dà yìí", h1))
P("Ẹ̀dà Yorùbá yìí tẹ̀lé Draft 0.3 ti German, èyí tí a ti ṣàyẹ̀wò fún ìrọ̀rùn òye àti òtítọ́ (wo Àfikún D nínú ẹ̀dà "
  "German fún àtòjọ àwọn àyípadà). A túmọ̀ rẹ̀ pẹ̀lú ẹ̀rọ (machine translation). Àwọn ọ̀rọ̀ technical tí kò ní "
  "orúkọ Yorùbá tó ṣe kedere wà ní èdè Gẹ̀ẹ́sì. Ẹnìkan tí èdè Yorùbá jẹ́ èdè abínibí rẹ̀ gbọ́dọ̀ ka á kí a tó tẹ̀ ẹ́ jáde.")
story.append(Spacer(1, 6))
story.append(Paragraph("<b>Ìkìlọ̀.</b> Ìwé yìí ṣàpèjúwe iṣẹ́ àkànṣe software tí a ṣì ń kọ́. Kì í ṣe ìmọ̀ràn ìdókòwò, "
                       "owó-orí tàbí òfin, kì í sì ṣe ìpè, prospectus tàbí crypto-asset white paper gẹ́gẹ́ bí òfin "
                       "MiCA. Gbogbo ìsọfúnni jẹ́ ti ìgbà díẹ̀, ó sì lè yí padà. Nọ́ńbà ọjà jẹ́ ipò ọjọ́ tí a sọ. Tí "
                       "iyèméjì bá wà, ẹ̀dà German ló ní àṣẹ.", small))

finish("ToyChain-Whitepaper-v0.3-yo.pdf", "ToyChain · Whitepaper àkọ́kọ́ · Draft 0.3 · ṣáájú ìpele ìdánwò",
       "Ojú-ìwé", "ToyChain àti ToyCoin (TOY): Whitepaper àkọ́kọ́", "Draft 0.3, ṣáájú ìpele ìdánwò", "yo")
