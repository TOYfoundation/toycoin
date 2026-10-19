"""ToyChain whitepaper, draft 0.3, Yoruba edition translated directly from the German edition.
Technical terms and links stay English. Run: python3 figures_i18n.py yo && python3 build_yo_de.py"""
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
    repo="https://github.com/Safehaigit/toycoin",
    market="https://dailyforex.com/forex-technical-analysis/2026/10/crypto-overview-18-october-2026/250826",
    cg_charts="https://www.coingecko.com/en/charts",
    dead="https://forklog.com/en/news/coingecko-reports-record-token-mortality-in-2025",
    bridgehacks="https://decrypt.co/106718/cross-chain-crypto-bridge-hacks-hit-2-billion-2022-chainalysis",
)

# ================================================================ Àkọlé
story.append(Spacer(1, 0.3 * cm))
story.append(Paragraph("ToyChain àti ToyCoin (TOY)", title))
story.append(Spacer(1, 4))
story.append(Paragraph("Network ìsanwó tó fúyẹ́, tí a bẹ̀rẹ̀ lọ́nà tó tọ́,<br/>tí ẹnikẹ́ni lè kópa nínú rẹ̀ láìní owó",
                       subtitle))
story.append(Spacer(1, 10))
story.append(Paragraph("Iṣẹ́ àkànṣe ToyChain · " + L("github.com/Safehaigit/toycoin", U["repo"]), center))
story.append(Paragraph("Whitepaper àkọ́kọ́ · Draft 0.3 · 19 October 2026 · ẹ̀dà Yorùbá (láti German)", center))
story.append(Spacer(1, 12))
BOX("<b>Ipò: ṣáájú ìpele ìdánwò (test phase).</b> Títí di báyìí, ToyChain wà gẹ́gẹ́ bí èrò àti gẹ́gẹ́ bí apá àkọ́kọ́ "
    "ti program. Ohun tó ti parí, tí a sì ti dán wò ni àwọn types ìpìlẹ̀ àti òfin bí owó ṣe ń wáyé (milestone M1 nínú "
    "13). Kò tíì sí <b>testnet, kò sí mainnet, kò sì sí TOY tí a lè tà</b>. Gbogbo nọ́ńbà jẹ́ iye ìbẹ̀rẹ̀, tí a ó wọ̀n, "
    "tí a ó sì ṣàtúnṣe lórí testnet. Ìwé yìí kì í ṣe ìmọ̀ràn ìdókòwò, kì í sì ṣe ìpè láti ra ohunkóhun. Owó láti "
    "testnet kò ní iye, a kì yóò sì gbé e lọ sí mainnet ọjọ́ iwájú láé. Àwọn ọ̀rọ̀ aláwọ̀ búlúù tí a fà lá ní ìsàlẹ̀ jẹ́ "
    "link sí àlàyé síwájú sí i (ní èdè Gẹ̀ẹ́sì).")

P("<b>Àkópọ̀.</b> ToyChain jẹ́ blockchain aládàáṣe pẹ̀lú owó ToyCoin (TOY), mílíọ̀nù 100 jùlọ. Ó bẹ̀rẹ̀ lọ́nà tó tọ́ "
  "pátápátá: block àkọ́kọ́ kò ní TOY kankan, kò sí ìpamọ́ fún àwọn olùdásílẹ̀, kò sí presale, kò sì sí ìpín àkànṣe. "
  "Gbogbo TOY ń wáyé díẹ̀díẹ̀ nípasẹ̀ òfin kan tó dúró, tí gbogbo ènìyàn lè ṣàyẹ̀wò, bíi ti "
  + L("Bitcoin", U["btc_pdf"]) + ". Yàtọ̀ sí Bitcoin, ààbò kò wá láti inú lílo iná mànàmáná, ṣùgbọ́n láti ọ̀dọ̀ àwọn "
  "olùkópa tí wọ́n ti TOY pa gẹ́gẹ́ bí ìdógò, tí wọ́n sì ń buwọ́lu àwọn block papọ̀. Block kan ti parí (final) lẹ́yìn "
  "nǹkan bí ìṣẹ́jú kan. Node kọ̀ọ̀kan ń ṣiṣẹ́ pẹ̀lú data 256 MB, ó sì ń ṣiṣẹ́ pẹ̀lú lórí fóònù àtijọ́ àti lórí "
  "Raspberry Pi. Ìkópa ṣeé ṣe láìní owó; ẹni tó bá ń ṣe púpọ̀ sí i ń gba ẹ̀tọ́ púpọ̀ sí i ní ìgbésẹ̀-ìgbésẹ̀. ToyChain "
  "mọ̀ọ́mọ̀ kò lo smart contracts, tokens àti NFTs: iṣẹ́ kọ̀ọ̀kan ni a ti kọ́ sínú rẹ̀ títí, ó rọrùn láti ṣàyẹ̀wò, kò "
  "sì lè fọwọ́ kan owó ẹlòmíràn. Àdírẹ́sì àkáǹtì jẹ́ bákan náà bíi ti Ethereum, nítorí náà àwọn wallet tí a mọ̀ ń bá a "
  "lọ láti ṣiṣẹ́. Ní ìparí, ìgbéyẹ̀wò olóòótọ́ wà nípa àǹfààní nínú ọjà crypto.", abstract)

story.append(Spacer(1, 4))
toc_table(["1 Ìfáárà", "2 ToyChain ní ṣókí", "3 Àwọn ọ̀rọ̀ ìpìlẹ̀ ní ṣókí", "4 Ètò (architecture)",
           "5 Ìbẹ̀rẹ̀ tó tọ́ àti iye owó", "6 Ìkópa: ìpele, quota, lọ́tìrì", "7 Ìfohùnṣọ̀kan nínú network",
           "8 Ìpele ìbẹ̀rẹ̀", "9 Node tó fúyẹ́ fún ẹ̀rọ àtijọ́", "10 Ààbò fún àwọn olùlò",
           "11 Wallet, ToyScan àti chain mìíràn", "12 Ìlò", "13 Ìjọra àti ìyàtọ̀", "14 Àǹfààní",
           "15 Ewu àti ìbéèrè tó ṣì wà", "16 Àǹfààní nínú ọjà crypto", "17 Ipò àti ọ̀nà iwájú", "18 Ìparí",
           "Àfikún: iye, links, orísun, àyípadà"], "Àkóónú")

# ================================================================ 1
H("Ìfáárà")
P("Ní 2008, Bitcoin fi hàn pé àwọn ènìyàn lè fohùnṣọ̀kan láìsí banki lórí ẹni tí owó kọ̀ọ̀kan jẹ́ tirẹ̀. Láti ìgbà náà, "
  "mílíọ̀nù crypto ti wáyé. Nínú ọ̀pọ̀ nínú wọn, àwọn olùdásílẹ̀ àti àwọn olùfowópamọ́ àkọ́kọ́ ti gba ìpín ńlá nínú coin "
  "kí ìbẹ̀rẹ̀ tó dé (premint). Àwọn mìíràn nílò ilé-iṣẹ́ kọ̀ǹpútà ńlá, iná mànàmáná púpọ̀ tàbí disk ọgọ́rọ̀ọ̀rún gigabyte, "
  "tí àwọn ènìyàn lásán kò fi fẹ́rẹ̀ẹ́ lè ṣàyẹ̀wò network fúnra wọn. Bitcoin chain pípé ti tó nǹkan bí 760 GB báyìí. "
  "Ọ̀pọ̀ chain sì ti di onípọ̀-iṣẹ́ tó bẹ́ẹ̀ pẹ̀lú smart contracts tí àṣìṣe nínú code ẹlòmíràn fi máa ń fa àdánù déédéé.")
P("ToyChain ń gbìyànjú láti ṣe èrò ìpìlẹ̀ Bitcoin, ìyẹn iye tó dúró, ìbẹ̀rẹ̀ tó tọ́ àti òfin tó ṣí sílẹ̀, pẹ̀lú ọ̀nà "
  "òde-òní lọ́nà tó fúyẹ́, tó sì wúlò lójoojúmọ́. Àwọn ìbéèrè tó ń darí wa ni: Ṣé ẹnikẹ́ni lè kópa pẹ̀lú ẹ̀rọ àtijọ́? Ṣé "
  "ẹnikẹ́ni lè ṣàyẹ̀wò òfin fúnra rẹ̀? Ṣé owó tìrẹ ń jẹ́ tìrẹ lábẹ́ gbogbo ipò? Whitepaper yìí ṣàlàyé àwọn èrò náà ní "
  "èdè tó rọrùn. Òfin pípé wà nínú technical specification (version 0.3) nínú " + L("repository", U["repo"]) +
  " iṣẹ́ àkànṣe. Ó ṣì jẹ́ private nígbà ìdàgbàsókè, a ó sì ṣí i sílẹ̀ ní ìgbà testnet gbangba ní tuntun jùlọ.")
MORE(L("Bitcoin whitepaper" + EN, U["btc_pdf"]), L("Ìwọ̀n Bitcoin node ní 2026" + EN, U["btc_size"]))

# ================================================================ 2
H("ToyChain ní ṣókí")
T([["Àbùdá", "ToyChain (iye ìbẹ̀rẹ̀ tí a gbèrò)"],
   ["Owó", "ToyCoin, àmì TOY, a lè pín in sí ibi 18 lẹ́yìn àmì ìdá"],
   ["Iye tó pọ̀ jùlọ", "100,000,000 TOY, a kọ ọ́ sínú code gẹ́gẹ́ bí òfin tí kò lè yí padà"],
   ["Ìbẹ̀rẹ̀", "Total fair launch: block àkọ́kọ́ láìsí owó, kò sí premint, kò sí ìpamọ́"],
   ["TOY tuntun", "ní epoch kọ̀ọ̀kan (ó kéré tán wákàtí 24) 0.0505 % nínú iye tó kù, a ń ṣàtúnṣe rẹ̀ pẹ̀lú factor m "
                  "láàrin 0.5 àti 2.0; ọdún kìíní nǹkan bí mílíọ̀nù 16.84, ìdajì lẹ́yìn nǹkan bí ọdún 3.76, 99 % lẹ́yìn "
                  "nǹkan bí ọdún 25"],
   ["Ìfohùnṣọ̀kan (consensus)", "Proof of stake pẹ̀lú committee tó ní validator 128 jùlọ, ìbuwọ́lù àpapọ̀, ó parí "
                                "lẹ́yìn block 1"],
   ["Àkókò block", "ìṣẹ́jú-àáyá 60, ó lè tètè dé tí ìlà ìdúró bá kún, ṣùgbọ́n kò tètè ju ìṣẹ́jú-àáyá 10 lọ"],
   ["Ìpamọ́ fún node", "Èrò: 256 MB, pẹ̀lú lórí ẹ̀rọ Android àtijọ́ àti Raspberry Pi"],
   ["Àkáǹtì", "bíi ti Ethereum: àdírẹ́sì kan náà, gbólóhùn ọ̀rọ̀ 12 tàbí 24 kan náà"],
   ["Iṣẹ́", "àwọn transaction tí a kọ́ sínú rẹ̀ títí nìkan, kò sí smart contracts, kò sí tokens, kò sí NFTs"],
   ["Owó iṣẹ́ (fees)", "transaction ọ̀fẹ́ 10 sí 100 fún àkáǹtì kan ní epoch kan (ó kéré tán ọjọ́ kan), lẹ́yìn náà fee "
                      "kékeré tàbí iṣẹ́ kọ̀ǹpútà"],
   ["Èdè program", "Rust, core kan fún gbogbo platform"],
   ["Licence", L("MIT", U["mit"]) + " (software ọ̀fẹ́)"]],
  [4.0, 12.4])

# ================================================================ 3
H("Àwọn ọ̀rọ̀ ìpìlẹ̀ ní ṣókí")
P("Ẹni tí kò tíì ní nǹkan ṣe pẹ̀lú cryptocurrency rí yóò rí àwọn ọ̀rọ̀ pàtàkì jùlọ níbí. Àwọn apá tó tẹ̀lé e dá lé wọn. "
  "Títẹ ọ̀rọ̀ aláwọ̀ búlúù kan ń mú ọ lọ sí àlàyé tó dára lórí ayélujára.")
T([["Ọ̀rọ̀", "Ní ṣókí"],
   ["Blockchain", "Ìwé àkọsílẹ̀ owó àpapọ̀ tí ọ̀pọ̀ kọ̀ǹpútà ń kọ lẹ́ẹ̀kan náà. A ń so àwọn àkọsílẹ̀ tuntun mọ́ ọn ní "
                  "àpò (block); block kọ̀ọ̀kan ń tọ́ka sí èyí tó ṣáájú rẹ̀ lọ́nà tí a kò lè ṣe ayédèrú."],
   ["Node", "Ẹ̀rọ kan pẹ̀lú software ToyChain. Ó ń ṣàyẹ̀wò fúnra rẹ̀ bóyá gbogbo àkọsílẹ̀ bá òfin mu, kò sì nílò láti "
            "gbẹ́kẹ̀lé ẹnikẹ́ni fún èyí."],
   ["Wallet", "App tó ń tọ́jú àwọn kọ́kọ́rọ́ rẹ, tó sì ń buwọ́lu àwọn ìfiránṣẹ́ owó. Nínú ToyChain, wallet TOYPAY jẹ́ "
              "node lẹ́ẹ̀kan náà."],
   [L("Kọ́kọ́rọ́ àti gbólóhùn", U["bip39"]),
    "Kọ́kọ́rọ́ àṣírí kan fi hàn pé àkáǹtì kan jẹ́ tìrẹ. A lè dá a padà láti inú ọ̀rọ̀ 12 tàbí 24. Ẹni tó bá mọ àwọn ọ̀rọ̀ "
    "náà lè lo owó náà."],
   ["Ìbuwọ́lù (signature)", "Ìbuwọ́lù oní-nọ́ńbà. Ẹnikẹ́ni lè ṣàyẹ̀wò pé ó jẹ́ òótọ́, ṣùgbọ́n kò sí ẹni tó lè ṣe ayédèrú "
                            "rẹ̀ láìsí kọ́kọ́rọ́ náà."],
   [L("Consensus", U["eth_consensus"]), "Ọ̀nà tí gbogbo node fi ń fohùnṣọ̀kan lórí block kan náà tó kàn."],
   [L("Stake", U["eth_pos"]), "TOY tí ènìyàn fi ìfẹ́ ti pa gẹ́gẹ́ bí ìdógò. Ẹni tó bá pa òfin mọ́ ń gba èrè. Ẹni tí a bá "
                              "fi hàn pé ó ṣe màgòmágó ń pàdánù apá kan ìdógò náà."],
   ["Validator", "Node kan pẹ̀lú stake, tó ń dábàá block, tó sì ń buwọ́lù wọ́n pẹ̀lú."],
   [L("Merkle tree", U["merkle"]), "Irú àkóónú kan tí a fi checksum ṣe. Pẹ̀lú proof kékeré, a lè fi hàn pé àkọsílẹ̀ "
                                   "kan jẹ́ apá rẹ̀ láìsí pé a tọ́jú gbogbo rẹ̀."],
   ["Epoch", "Apá chain kan tí ó ní ó kéré tán block 1,440 àti ó kéré tán wákàtí 24. Ọ̀pọ̀ òfin ń kà ní epoch, fún "
             "àpẹẹrẹ ìtújáde TOY tuntun."],
   ["Testnet", "Chain ìdánwò kan tí coin rẹ̀ kò ní iye. Níbẹ̀ ni a ti ń wá àṣìṣe, kí ọ̀rọ̀ tó di ti gidi."]],
  [3.6, 12.8])

# ================================================================ 4
H("Ètò (architecture)")
P("ToyChain ní àwọn layer tí ó wà lórí ara wọn. Ọ̀kọ̀ọ̀kan ní iṣẹ́ tí a ti ṣàlàyé kedere. Gbogbo layer wà nínú program "
  "Rust àpapọ̀ kan, tó ń ṣiṣẹ́ bákan náà lórí Android, Windows, Linux àti Raspberry Pi. Báyìí, kò sí ohun tí a ní láti "
  "kọ lẹ́ẹ̀mejì tàbí ṣàyẹ̀wò lẹ́ẹ̀mejì.")
FIG(F + "f_arch.png", "Àwọn layer ToyChain. Proof layer (ìlà dídá) ń tẹ̀lé e ní phase 2.", 14.8)
MORE(L("libp2p, ìpìlẹ̀ network" + EN, U["libp2p"]), L("Dandelion++: ta ló fi ránṣẹ́?" + EN, U["dandelion"]))

# ================================================================ 5
H("Ìbẹ̀rẹ̀ tó tọ́ àti iye owó")
H2("5.1 Kò sí TOY kankan nínú block àkọ́kọ́")
P("Block àkọ́kọ́ (genesis block) nínú ToyChain kò ní <b>owó kankan rárá</b>. Kò sí ẹnikẹ́ni, kódà àwọn developer, tó ní "
  "TOY ní ìbẹ̀rẹ̀. Kò sí presale, kò sí ìpamọ́, kò sì sí ìpín olùdásílẹ̀. Èyí le ju ti ọ̀pọ̀ iṣẹ́ àkànṣe tuntun lọ, ó sì "
  "bá ẹ̀mí Bitcoin mu. Òfin yìí jẹ́ apá kan “constitution” (apá 10.3), kò sì sí update tó lè yí i padà.")
H2("5.2 Bí TOY tuntun ṣe ń wáyé")
P("Ní epoch kọ̀ọ̀kan, ìpín kan tó dúró nínú iye tí a kò tíì tú jáde ń wáyé: 0.0505 % nínú iye tó kù. Ní epoch àkọ́kọ́, "
  "ìyẹn jẹ́ TOY 50,500 gẹ́lẹ́. Nítorí pé iye tó kù ń dín kù, ìtújáde ń dín kù déédéé, dípò kí ó já sí ìdajì lójijì ní "
  "ọdún mẹ́rin-mẹ́rin bíi ti Bitcoin. Lẹ́yìn nǹkan bí ọdún 3.76, a ti tú ìdajì jáde; lẹ́yìn nǹkan bí ọdún 25, nǹkan bí "
  "99 %. A kì yóò dé iye tó pọ̀ jùlọ láé, a kì yóò sì kọjá rẹ̀ láé. Àwọn ìdánwò ti fi èyí hàn tẹ́lẹ̀: kódà pẹ̀lú "
  "ìyára ìtújáde ìlọ́po méjì fún ọdún 100, àpapọ̀ ṣì wà nísàlẹ̀ rẹ̀.")
FIG(F + "f_emission.png", "Ìpín iye tó pọ̀ jùlọ tí a ti tú jáde bí àkókò ti ń lọ. ToyChain ń dín kù déédéé, Bitcoin "
                          "ń dín kù sí ìdajì ní ìgbésẹ̀. Lẹ́yìn ọdún 25, méjèèjì wà ní nǹkan bí 99 %.")
FIG(F + "f_per_year.png", "TOY tuntun ní ọdún kọ̀ọ̀kan gẹ́gẹ́ bí ètò (m = 1). Ọdún kọ̀ọ̀kan, nǹkan bí 17 % díẹ̀ ló ń "
                          "wáyé ju ti ọdún tó ṣáájú lọ.")
MORE(L("Bí Bitcoin ṣe ń fi ààlà sí iye rẹ̀" + EN, U["btc_supply"]))
H2("5.3 Factor m: a ń ṣàtúnṣe rẹ̀, ṣùgbọ́n pẹ̀lú ìkáwọ́")
P("A ń fi factor m láàrin 0.5 àti 2.0 sọ ìtújáde di púpọ̀. Ó ń ga díẹ̀ tí a bá ń lo network <i>ní tòótọ́</i> "
  "(transaction tí a sanwó fún tàbí tí iṣẹ́ kọ̀ǹpútà bò) àti tí ọ̀pọ̀ validator bá ń buwọ́lù pẹ̀lú déédéé. Transaction "
  "ọ̀fẹ́ kò kà, bí bẹ́ẹ̀ kọ́ a lè fi spam gbé ìtújáde sókè. Apá ètò-àkókò kan ń mú àwọn ìyàtọ̀ padà, m kò sì lè yí ju "
  "0.05 lọ ní epoch kan. Nínú simulation, ìtújáde ọdún kìíní ń yàtọ̀ sí ètò ní nǹkan bí 2.2 % jùlọ, ní ìgbà pípẹ́ ní "
  "nǹkan bí 2.5 % jùlọ. Nítorí náà, m ń yí <i>ìgbà</i> tí TOY ń wáyé padà nìkan, kì í ṣe <i>iye</i> rẹ̀ lápapọ̀.")
H2("5.4 Ta ló ń gba TOY tuntun")
P("A ń pín ìtújáde kọ̀ọ̀kan lọ́nà tó dúró: <b>50 %</b> fún gbogbo àwọn tó ń dáàbò bo network, <b>40 %</b> fún lọ́tìrì "
  "iṣẹ́ kan, tí ẹ̀rọ èyíkéyìí lè kópa nínú rẹ̀ láìní owó (apá 6.3), àti <b>10 %</b> fún àpò ìsanpadà kan, tó ń sanwó "
  "fún transaction ọ̀fẹ́ gbogbo ènìyàn. Ní ìpele ìbẹ̀rẹ̀, ìpín ààbò ń lọ sí àwọn start validators, ó sì ń dín kù títí "
  "dé òdo. Ohun tí wọn kò bá gbà ń ṣàn lọ sí lọ́tìrì. A ń ṣírò pẹ̀lú nọ́ńbà odidi nìkan ní ẹ̀ka tó kéré jùlọ "
  "(10<super>-18</super> TOY). Ìyókù ìṣirò ń lọ sí àpò ìsanpadà, kí ó lè jẹ́ pé ní epoch kọ̀ọ̀kan, iye tí a gbèrò gẹ́lẹ́ "
  "ló ń wáyé.")
FIG(F + "f_split.png", "Pínpín TOY tuntun. Ní ìpele ìbẹ̀rẹ̀, èrè start validators ń lọ díẹ̀díẹ̀ sí lọ́tìrì iṣẹ́; "
                       "lẹ́yìn ìfàlé, gbogbo àwọn tó ní stake ló ń gbà á.")
H2("5.5 A kì í pa TOY run láé")
P("Ọ̀pọ̀ chain ń “jó” owó iṣẹ́ tàbí ìjìyà, fún àpẹẹrẹ Ethereum ń jó apá kan owó iṣẹ́ kọ̀ọ̀kan gẹ́gẹ́ bí "
  + L("EIP-1559", U["eip1559"]) + ". ToyChain kò ṣe bẹ́ẹ̀: ìjìyà àti ẹ̀bùn ń ṣàn lọ sí àpò ìsanpadà, wọ́n sì ń ṣe àwọn "
  "olùlò láǹfààní báyìí. Òfin àyẹ̀wò kan (I0) ń rí i dájú nínú <b>gbogbo</b> block pé àpapọ̀ gbogbo owó dọ́gba gẹ́lẹ́ "
  "pẹ̀lú àpapọ̀ gbogbo TOY tí a ti tú jáde. Block tó bá rú èyí kò wúlò.")

# ================================================================ 6
H("Ìkópa: ìpele, quota àti lọ́tìrì iṣẹ́")
H2("6.1 Ìpele ìkópa")
P("Olúkúlùkù ló ń pinnu fúnra rẹ̀ bí ìkópa rẹ̀ ṣe pọ̀ tó. Wallet ń wọn ẹ̀rọ náà ní ìbẹ̀rẹ̀ àkọ́kọ́, ó sì ń dábàá ìpele "
  "tó yẹ. Ó ń fúnni ní ìpele gíga jù nìkan tí ẹ̀rọ náà bá lè ṣe é.")
FIG(F + "f_tiers.png", "Ìpele ìkópa márùn-ún. Kódà ìpele 0 kò nílò owó.", 15.0)
P("TOY tí a tì pa ṣì jẹ́ ohun ìní olùlò, a sì lè tú u sílẹ̀ padà pẹ̀lú ìdúró epoch 21. Kí ẹnikẹ́ni má baà ní agbára "
  "jù, stake olùkópa kan ń kà títí dé 5 % jùlọ (láti validator 100, 1 %) nínú gbogbo stake. Lẹ́yìn ìpele ìbẹ̀rẹ̀, ìdá "
  "márùn-ún kan nínú ìpín ààbò ń lọ ní ìpín dọ́gba sí ọmọ committee kọ̀ọ̀kan, èyí tó kù ní ìwọ̀n stake lọ́nà linear. "
  "Méjèèjì wà ní ìwọ̀n iṣẹ́ tí a ṣe ní tòótọ́ nìkan: ẹni tó bá buwọ́lu 90 % àwọn block pẹ̀lú ń gba 90 %. Nítorí pé "
  "ìpín ní ìwọ̀n stake jẹ́ linear, pínpín sí ọ̀pọ̀ àkáǹtì kò mú àǹfààní kankan wá.")
MORE(L("Kí ni proof of stake? (ethereum.org)" + EN, U["eth_pos"]))
H2("6.2 Transaction ọ̀fẹ́ àti owó iṣẹ́")
P("Àkáǹtì kọ̀ọ̀kan tí a ti mú ṣiṣẹ́ ní quota ọ̀fẹ́ ní epoch kọ̀ọ̀kan: transaction 100 nígbà tí network bá ṣófo, ó kéré "
  "tán 10 nígbà tí ó bá kún fọ́fọ́. Èyí tó fún ìgbésí ayé ojoojúmọ́. Lẹ́yìn náà, ènìyàn ń san owó iṣẹ́ kékeré kan, tó "
  "ń ga kíákíá tí transaction bá pọ̀ nínú block kan náà, tàbí ènìyàn fi proof of work kún un. Báyìí, network ṣì jẹ́ "
  "ọ̀fẹ́ fún àwọn olùlò lásán, nígbà tí spam ń di olówó ńlá. Transaction ọ̀fẹ́ ń gba ìdajì block kan jùlọ, àpò "
  "ìsanpadà ló sì ń sanwó fún wọn; ẹnikẹ́ni tún lè fi ìfẹ́ fi ẹ̀bùn sí i.")
H2("6.3 Lọ́tìrì iṣẹ́")
P("Kí àwọn ènìyàn tí kò ní TOY pẹ̀lú lè rí coin tuntun, lọ́tìrì kan wà fún iṣẹ́ kọ̀ǹpútà. Ẹ̀rọ náà ń ṣírò tíkẹ́ẹ̀tì "
  "pẹ̀lú ọ̀nà " + L("Argon2id", U["argon2"]) + ", tó mọ̀ọ́mọ̀ nílò ìrántí púpọ̀ (16 MiB). Nítorí èyí, ẹ̀rọ àkànṣe àti "
  "oko graphics card fẹ́rẹ̀ẹ́ má ní àǹfààní kankan lórí fóònù tàbí PC. Ní block kọ̀ọ̀kan, a ń yan olùborí tó tó 64 lọ́nà "
  "tó tọ́, èrè kọ̀ọ̀kan sì kéré tán 0.2 TOY. Ẹni tó bá ṣírò púpọ̀ ní tíkẹ́ẹ̀tì púpọ̀; èrè tí a retí bá iṣẹ́ mu. Èrè "
  "kan ṣoṣo ti tó láti mú àkáǹtì ṣiṣẹ́: a ń ti 0.1 TOY pa gẹ́gẹ́ bí ìdógò (apá 9), èyí tó kù sì wà fún lílò. Ẹ̀rọ ń "
  "ṣírò nìkan tí ó bá gbà á láàyè: lórí charger, lórí Wi-Fi, àti nígbà tí ó bá ṣì tutù. Fún àfiwé: Bitcoin network ń "
  "lo iná mànàmáná tó pọ̀ bíi ti orílẹ̀-èdè kan lápapọ̀ nígbà gbogbo, gẹ́gẹ́ bí "
  + L("Cambridge index" + EN, U["cbeci"]) + " ti fi hàn.")

# ================================================================ 7
H("Ìfohùnṣọ̀kan nínú network")
H2("7.1 Bí block ṣe ń wáyé")
P("Committee kan tó ní validator 128 jùlọ ni a ń tún ṣètò ní epoch kọ̀ọ̀kan. Fún block kọ̀ọ̀kan, a ń yan ẹnìkan pẹ̀lú "
  "àìròtẹ́lẹ̀ tí a lè ṣàyẹ̀wò (" + L("VRF", U["vrf"]) + "), tí yóò dábàá block náà. Àwọn yòókù ń dìbò ní ìpele méjì "
  "(pre-vote àti ìfìdímúlẹ̀), gẹ́gẹ́ bí ọ̀nà tí a ti dán wò tó jọ " + L("Tendermint", U["tendermint"]) + ". Ní kété tí "
  "ìdá méjì nínú mẹ́ta bá ti fọwọ́ sí i, block náà ti <b>parí</b>. A kò lè yí i padà mọ́. Pẹ̀lú Bitcoin, ní ìdàkejì, "
  "ènìyàn sábà máa ń dúró nǹkan bí wákàtí kan, títí ìsanwó kan yóò fi di ààbò. A ń fi "
  + L("BLS signatures", U["bls_wiki"]) + " kó gbogbo ìbò papọ̀ sí ìbuwọ́lù kan ṣoṣo tó jẹ́ byte 96 péré, tí fóònù "
  "pàápàá lè ṣàyẹ̀wò ní milliseconds.")
FIG(F + "f_block.png", "Bí block kan ṣe ń lọ ní ìgbésẹ̀ márùn-ún. Lẹ́yìn ìgbésẹ̀ 5, ìsanwó ti parí.")
H2("7.2 Ètò tó tọ́")
P("Ẹni tó ń kọ́ block kò lè yan ètò àwọn transaction bí ó ti wù ú. A ti fi lélẹ̀: ní ìwọ̀n owó iṣẹ́, tí ó bá dọ́gba, ní "
  "ìwọ̀n nọ́ńbà àìròtẹ́lẹ̀ kan tí a ń mọ̀ pẹ̀lú block náà nìkan. Node kọ̀ọ̀kan ń ṣàyẹ̀wò èyí. Lẹ́yìn náà, òfin ìfisínú "
  "(inclusion duty) yóò rí i dájú pé àwọn transaction tí ọ̀pọ̀ validator rí wọnú block tó kàn ní tòótọ́. Nítorí pé kò "
  "sí exchange contracts lórí ToyChain, " + L("“front-running” (MEV)", U["eth_mev"]) + " tí a mọ̀ lórí Ethereum "
  "fẹ́rẹ̀ẹ́ pòórá.")
H2("7.3 Ìjìyà tó bá ìpalára mu")
P("Ẹni tó bá fúnni ní ìbò méjì tó tako ara wọn fún block kan náà (double signing) ń pàdánù apá kan stake rẹ̀. Èyí "
  "sábà máa ń ṣẹlẹ̀ nípa àṣìṣe, fún àpẹẹrẹ tí kọ́kọ́rọ́ kan náà bá ń ṣiṣẹ́ lórí ẹ̀rọ méjì. Nítorí náà, ìjìyà dá lórí "
  "iye àwọn tó ṣe é lẹ́ẹ̀kan náà. Software náà tún ń dáàbò bo lọ́wọ́ àṣìṣe: ó ń rántí ohun tí ó ti buwọ́lù títí, ó sì "
  "ń dákẹ́ lákọ̀ọ́kọ́ lẹ́yìn yíyí ẹ̀rọ padà, tí kọ́kọ́rọ́ rẹ̀ bá ti ń ṣiṣẹ́ níbòmíràn. Ní ìpele ìbẹ̀rẹ̀, ìyọkúrò ní ààlà "
  "5 %.")
FIG(F + "f_slashing.png", "Ìyọkúrò fún double signing: max(1 %, 3 × ìpín àwọn tó ṣe é lẹ́ẹ̀kan náà), 100 % jùlọ.")

# ================================================================ 8
H("Ìpele ìbẹ̀rẹ̀")
P("Ìbẹ̀rẹ̀ tó tọ́ láìsí premint ní ìṣòro “adìyẹ àti ẹyin” nínú proof of stake: ní ìbẹ̀rẹ̀, kò sí ẹni tó ní TOY, "
  "nítorí náà kò sí ẹni tó ní stake tí ó lè fi dáàbò bo network. ToyChain yanjú èyí ní gbangba, fún àkókò tó ní ààlà:")
B(["<b>Start validators 21</b>, tí àwọn olùyọ̀ǹda ń ṣiṣẹ́, pẹ̀lú olùṣiṣẹ́ àti ibi tó yàtọ̀ síra bí ó ti ṣeé ṣe tó, ń "
   "dáàbò bo network pẹ̀lú agbára dọ́gba. Orúkọ wọn àti owó tí wọ́n ń gbà hàn gbangba nínú ToyScan.",
   "<b>Èrè tó ń dín kù:</b> Ní ìbẹ̀rẹ̀, wọ́n jọ ń gba ìdajì gbogbo TOY tuntun. Ìpín náà ń dín kù déédéé lójoojúmọ́, ó sì "
   "di òdo lẹ́yìn ọjọ́ 180. A ń kà á ní <i>plan time</i>: ìdádúró kọ̀ọ̀kan láàrin block méjì ń kà ní ìṣẹ́jú 10 jùlọ, kí "
   "ìdádúró má baà ti aago síwájú. Ààlà kan ń dènà kí ìtújáde tó yára jù tàbí block tí a mọ̀ọ́mọ̀ fà sẹ́yìn mú èrè púpọ̀ "
   "wá. Lápapọ̀, ó jẹ́ nǹkan bí mílíọ̀nù 2.22 TOY jùlọ (nǹkan bí 2.2 % nínú iye tó pọ̀ jùlọ).",
   "<b>Èrè tí a tì pa:</b> Èrè kọ̀ọ̀kan wà ní títì fún epoch 365 (nǹkan bí ọdún kan), ó sì ń dáhùn fún ìwà àìtọ́ pẹ̀lú.",
   "<b>Ìfàlé tó le:</b> Ìpele ìbẹ̀rẹ̀ ń parí lẹ́yìn ọjọ́ 90 ní kùtùkùtù jùlọ, àti nígbà tí ó kéré tán validator tuntun "
   "21, ọ̀kọ̀ọ̀kan pẹ̀lú TOY 1,000, ti wà ní títì fún epoch 30, tí wọ́n sì jọ ní ìdá méjì nínú mẹ́ta stake tó ń ṣiṣẹ́. "
   "Lẹ́yìn náà, start validators pàdánù gbogbo ipò àkànṣe. Tí ìfàlé kò bá ṣẹlẹ̀ ní ọjọ́ 180, wọ́n ń bá a lọ láti dáàbò "
   "bo network láìsí èrè, títí yóò fi ṣẹlẹ̀."])
FIG(F + "f_bootstrap.png", "Èrè start validators ní ìwọ̀n ọjọ́ àti fèrèsé àkókò fún ìfàlé.")
P("Ní òtítọ́: ìpele ìbẹ̀rẹ̀ ni apá tí ToyChain ti nílò ìgbẹ́kẹ̀lé jùlọ nínú ènìyàn díẹ̀. Kò sí protocol tó lè fi hàn "
  "pé àwọn validator “tuntun” jẹ́ ènìyàn mìíràn ní tòótọ́. Ṣùgbọ́n àwọn òfin ń mú kí gbígba agbára jẹ́ olówó ńlá, kí ó "
  "sì hàn sí gbogbo ènìyàn.")

# ================================================================ 9
H("Node tó fúyẹ́ fún ẹ̀rọ àtijọ́")
P("Láti tọ́jú Bitcoin tàbí Ethereum chain pípé, ènìyàn nílò ọgọ́rọ̀ọ̀rún gigabyte lónìí. ToyChain fi èrò tó dúró kalẹ̀: "
  "<b>256 MB fún node kan</b>. Èyí ń ṣeé ṣe, nítorí pé node kan ń tọ́jú owó lọ́wọ́lọ́wọ́ gbogbo àkáǹtì alààyè àti fèrèsé "
  "kúkúrú ti àwọn block tó kẹ́yìn nìkan. Àwọn archive node olùyọ̀ǹda ń tọ́jú àwọn block àtijọ́. Ní ìpele kan lẹ́yìn "
  "náà, a ó “ká” gbogbo ìtàn sínú proof cryptographic kékeré kan, kí àwọn node tuntun pẹ̀lú má baà nílò láti gbẹ́kẹ̀lé "
  "ohunkóhun mọ́. Títí di ìgbà náà, àwọn node tuntun ń bẹ̀rẹ̀ láti àwọn checkpoint, tí ó kéré tán ẹgbẹ́ mẹ́ta tí kò "
  "gbára lé ara wọn ti buwọ́lù.")
FIG(F + "f_budget.png", "Bí 256 MB ṣe pín. Ètò ìpamọ́ kọ̀ọ̀kan ní ìdánwò ìwọ̀n tirẹ̀.")
P("Kí àkáǹtì mílíọ̀nù 100 àti jù bẹ́ẹ̀ lọ lè rí ààyè síbẹ̀, <b>àkáǹtì alààyè àti àkáǹtì tó ń sùn</b> wà. Àkáǹtì tí a "
  "kò tíì lò fún ìgbà pípẹ́ ni a ń gbé jáde nínú ìrántí tí ó bá pọndandan, ṣùgbọ́n <b>a kì í pa á rẹ́ láé</b>. "
  + L("Merkle proof", U["merkle"]) + " kan tí kò tó 1 KB ló ń dáàbò bo ohun tó wà nínú rẹ̀, wallet sì ń tọ́jú rẹ̀ "
  "fúnra rẹ̀. Pẹ̀lú transaction tó kàn, àkáǹtì náà ń jí padà fúnra rẹ̀, pẹ̀lú gbogbo owó rẹ̀.")
FIG(F + "f_dormant.png", "Àkáǹtì alààyè àti àkáǹtì tó ń sùn. Kò sí àkáǹtì tó ń sọnù.", 15.0)
P("Kí ẹnikẹ́ni má baà fi mílíọ̀nù àkáǹtì òfìfo kún ìrántí, a ń ti 0.1 TOY pa gẹ́gẹ́ bí <b>ìdógò ìmúṣiṣẹ́</b> nígbà "
  "àkọ́kọ́ tí owó bá wọlé. Ìdógò náà ṣì jẹ́ ohun ìní olùlò, a kì í yọ nínú rẹ̀ láé, ó sì ń padà wá ní kíkún nígbà tí a "
  "bá ti àkáǹtì pa. Lẹ́yìn náà, ó lè dín kù nìkan, títí dé 0.01 TOY jùlọ.")

# ================================================================ 10
H("Ààbò fún àwọn olùlò")
H2("10.1 Ààbò àkáǹtì àṣàyàn")
P("Olúkúlùkù lè dáàbò bo àkáǹtì tirẹ̀ sí i. Àwọn ètò wọ̀nyí ń ṣiṣẹ́ nìkan nítorí pé olówó ti ṣètò wọn fúnra rẹ̀. Kò sí "
  "ẹlòmíràn tó lè ṣètò wọn.")
B(["<b>Vault delay:</b> Owó yíyọ tó tóbi ń ṣiṣẹ́ lẹ́yìn àkókò ìdúró nìkan, a sì lè fagilé e nígbà àkókò yìí. Nítorí náà, "
   "olè kò lè sọ àkáǹtì di òfìfo lẹ́sẹ̀kẹsẹ̀.",
   "<b>Emergency key àti àdírẹ́sì ààbò:</b> Kọ́kọ́rọ́ kejì tí a tọ́jú lọ́tọ̀ kò lè ná nǹkankan, ṣùgbọ́n ó lè dá owó yíyọ "
   "tó ń dúró dúró, kí ó sì gbé gbogbo rẹ̀ lọ sí àdírẹ́sì ààbò kan tí a ti yàn tẹ́lẹ̀.",
   "<b>Self-lock:</b> Fóònù sọnù? A lè ti àkáǹtì tìrẹ pa fún epoch 30 jùlọ.",
   "<b>Àdírẹ́sì ajogún:</b> Tí àkáǹtì kan bá wà láìsí ìgbésẹ̀ kankan fún ìgbà pípẹ́ gan-an (ó kéré tán epoch 180), "
   "àdírẹ́sì ajogún kan tí olówó yàn fúnra rẹ̀ lè gbà á. A ń rán olówó létí lọ́pọ̀ ìgbà ṣáájú, ó sì ṣì lè fagilé e fún "
   "epoch 30."])
P("Ní òtítọ́: ẹni tó bá jí fóònù pẹ̀lú kọ́kọ́rọ́ ní ẹ̀tọ́ kan náà bíi ti olówó tí kò bá sí emergency key. Nítorí náà, "
  "wallet ń gbani nímọ̀ràn láti ṣe emergency key lórí ẹ̀rọ kejì tàbí lórí ìwé.")
FIG(F + "f_vault.png", "Vault delay: nígbà àkókò ìdúró, olówó lè dá owó yíyọ tí a fura sí dúró.", 15.0)
H2("10.2 Òfin ààbò pẹ̀lú ìwọ̀ntúnwọ̀nsì")
P("A fẹ́ dá ìkọlù dúró ní kùtùkùtù, láìṣe àwọn olùlò olóòótọ́ ní ìpalára. Ìdí nìyí tí ìpele mẹ́ta fi wà. Chain fúnra "
  "rẹ̀ ń jẹ ẹ̀ṣẹ̀ tí a lè <b>fi hàn kedere</b> níyà nìkan, fún àpẹẹrẹ double signing tàbí block ayédèrú. Àbájáde rẹ̀ jẹ́ "
  "ìdènà kan tó ń gba ẹ̀tọ́ kúrò, fún àpẹẹrẹ quota ọ̀fẹ́, ṣùgbọ́n <b>kì í gba owó láé</b>. Àtúnṣe ẹ̀ṣẹ̀ ń sọ ìdènà di "
  "ìlọ́po méjì, títí dé epoch 64 jùlọ.")
FIG(F + "f_levels.png", "Ìpele mẹ́ta ti òfin ààbò. Ìpele A nìkan ló ń ṣiṣẹ́ nínú chain, lórí ẹ̀tọ́ nìkan.", 15.0)
H2("10.3 Constitution")
P("Àwọn òfin kan jẹ́ ìpìlẹ̀ tó bẹ́ẹ̀ tí wọ́n fi wà nínú code títí, tí update tàbí ìdìbò ọjọ́ iwájú kankan kò sì lè yí "
  "wọn padà: iye tó pọ̀ jùlọ ti mílíọ̀nù 100 TOY, ìbẹ̀rẹ̀ láìsí owó, fọ́múlà ìtújáde, ìkà léèwọ̀ pípa TOY run, àkókò "
  "tó kéré jùlọ fún epoch, àìlo tokens àti smart contracts, àti ju gbogbo rẹ̀ lọ: <b>kò sí òfin tó lè gba TOY láti inú "
  "àkáǹtì kan, dì í, tàbí pín in fún ẹlòmíràn.</b> Ní òtítọ́, ẹnikẹ́ni tó bá ń pín software tirẹ̀ lè ya chain kan "
  "kúrò. Nítorí náà, wallet àti ToyScan ń ṣàyẹ̀wò constitution fúnra wọn, wọ́n sì ń kìlọ̀ tí ìyapa bá rú u. A ó kọ́ "
  "version program kọ̀ọ̀kan lọ́nà " + L("reproducible", U["repro"]) + ", kí ẹnikẹ́ni lè ṣàyẹ̀wò pé ó wá láti inú code "
  "tí a tẹ̀jáde ní tòótọ́.")
H2("10.4 Cryptography tí a ti dán wò")
P("ToyChain kò ṣẹ̀dá ọ̀nà ìfipamọ́ tirẹ̀. A ń lo àwọn standard tí a mọ̀ káàkiri, tí a sì ti ṣàyẹ̀wò: secp256k1 àti "
  "Keccak-256 bíi ti Ethereum fún àkáǹtì, " + L("BIP-39", U["bip39"]) + " fún gbólóhùn ọ̀rọ̀, "
  + L("BLS12-381", U["bls"]) + " fún ìbuwọ́lù àpapọ̀ àwọn validator, BLAKE3 fún checksum, Argon2id fún iṣẹ́ kọ̀ǹpútà àti "
  "fún ìfipamọ́ wallet lórí ẹ̀rọ. Ìbuwọ́lù kọ̀ọ̀kan ní àmì chain (" + L("EIP-155", U["eip155"]) + "), kí a má baà lò "
  "ó padà lórí chain mìíràn. Gbogbo ọ̀nà wà lẹ́yìn interface tí a lè pààrọ̀. Kọ́kọ́rọ́ àfikún àṣàyàn kan gẹ́gẹ́ bí "
  "standard NIST " + L("ML-DSA", U["mldsa"]) + " yóò kojú " + L("quantum computers", U["pqk"]) + " pẹ̀lú. Ní àfikún, a "
  "ń ṣàyẹ̀wò àwọn òfin ìṣirò pàtàkì jùlọ lọ́nà mathematics pẹ̀lú irinṣẹ́ proof " + L("Kani", U["kani"]) + ", ìyẹn kì "
  "í ṣe lórí àpẹẹrẹ nìkan, ṣùgbọ́n fún gbogbo nọ́ńbà tó ṣeé ṣe. A ti fi hàn tẹ́lẹ̀: epoch kan kì í tú ju iye tó kù lọ "
  "láé, àpapọ̀ gbogbo TOY sì máa ń wà ní mílíọ̀nù 100 jùlọ nígbà gbogbo (apá 17).")

# ================================================================ 11
H("Wallet TOYPAY, ToyScan àti chain mìíràn")
P("<b>Wallet TOYPAY</b> jẹ́ àpò owó àti node ní ọ̀kan. Ó ń ṣiṣẹ́ lórí Android láti nǹkan bí 1 GB RAM, lórí Windows, "
  "Linux àti Raspberry Pi. Energy mode kan ń ṣètò fúnra rẹ̀ bí ẹ̀rọ náà ṣe ń kópa tó, gẹ́gẹ́ bí batiri, charger, ìgbóná "
  "àti network ṣe rí. A gbèrò interface ní èdè 45, ó sì ní mode mẹ́rin láti olùbẹ̀rẹ̀ dé ọ̀jọ̀gbọ́n. Ìkìlọ̀ ààbò ń hàn "
  "nínú mode kọ̀ọ̀kan.")
P("Explorer <b>ToyScan</b> ń fi gbogbo nǹkan hàn gbangba: iye TOY tí a ti tú jáde, factor m lọ́wọ́lọ́wọ́, ipò ìpele "
  "ìbẹ̀rẹ̀ àti gbogbo ìdènà pẹ̀lú ìdí wọn. Iná atọ́ka kan (aláwọ̀ ewé, ìyeyè, pupa) ń ṣe àkópọ̀ ìlànà àyẹ̀wò mẹ́sàn-án, "
  "fún àpẹẹrẹ bóyá a ń pa iye tó pọ̀ jùlọ mọ́, bóyá stake ti kórajọ jù, tàbí bóyá ìpele ìbẹ̀rẹ̀ ti pẹ́ jù. ToyScan kò "
  "ní àǹfààní sí kọ́kọ́rọ́ láé.")
P("Nítorí pé àdírẹ́sì jẹ́ bákan náà bíi ti Ethereum, wallet àti ToyScan ń fi owó àdírẹ́sì kan hàn pẹ̀lú lórí EVM chain "
  "mìíràn bí Ethereum tàbí Polygon. Àwọn wallet tí a mọ̀ bí MetaMask lè "
  + L("so ToyChain pọ̀ gẹ́gẹ́ bí network tirẹ̀" + EN, U["metamask"]) + ". A mọ̀ọ́mọ̀ kò ní "
  + L("bridge", U["eth_bridges"]) + " láàrin àwọn chain: ní 2022, gẹ́gẹ́ bí Chainalysis ti sọ, nǹkan bí ìdá méjì nínú "
  "mẹ́ta gbogbo owó crypto tí a jí jẹ́ ti " + L("ìkọlù sí bridge", U["bridgehacks"]) + ". Dípò rẹ̀, a gbèrò "
  + L("atomic swap", U["atomic"]) + ": ènìyàn méjì ń pààrọ̀ tààrà, pẹ̀lú ààbò " + L("hash time lock", U["htlc"]) +
  ". Yálà àwọn méjèèjì gba ohun tí wọ́n fẹ́, tàbí àwọn méjèèjì gba owó wọn padà.")
FIG(F + "f_swap.png", "Atomic swap láàrin TOY àti coin kan lórí chain mìíràn, láìsí alárinà.")
BOX("<b>Àkíyèsí pàtàkì nípa tokens orúkọ kan náà:</b> Lórí Polygon àti Ethereum, àwọn token àtijọ́ wà pẹ̀lú orúkọ "
    "ToyCoin àti àmì TOY. Wọn <b>kò</b> ní ìsopọ̀ pẹ̀lú TOY ti ToyChain, a kò sì lè pààrọ̀ wọn. Wallet àti ToyScan ń "
    "fi èyí hàn kedere.")

# ================================================================ 12
H("Ìlò")
B(["<b>Ìsanwó ojoojúmọ́</b> láàrin ènìyàn, pẹ̀lú ìdí ìsanwó àti ìparí lẹ́yìn nǹkan bí ìṣẹ́jú kan, lẹ́yìn náà pẹ̀lú "
   "gẹ́gẹ́ bí ìsanwó àpapọ̀ sí ẹni 32. Owó kékeré ṣeé ṣe láìsí owó iṣẹ́ nítorí quota ọ̀fẹ́.",
   "<b>Ìkópa láìní owó-ìdókòwò:</b> fóònù àtijọ́ tàbí Raspberry Pi kan di node, ó sì lè jèrè TOY àkọ́kọ́ nípasẹ̀ "
   "lọ́tìrì iṣẹ́.",
   "<b>Ẹgbẹ́, ìdílé àti iṣẹ́ kékeré:</b> àkáǹtì pẹ̀lú ìbuwọ́lù púpọ̀, escrow pẹ̀lú onídàájọ́, ìyọ̀ǹda subscription pẹ̀lú "
   "ààlà tó dúró, àti session keys fún àwọn app (lẹ́yìn testnet àkọ́kọ́).",
   "<b>Ìmúrasílẹ̀:</b> vault delay, emergency key àti àdírẹ́sì ajogún ń dáàbò bo lọ́wọ́ olè àti àdánù.",
   "<b>Ẹ̀bùn fún gbogbo ènìyàn:</b> ẹnikẹ́ni lè fi ẹ̀bùn sí àpò ìsanpadà, tó ń sanwó fún transaction ọ̀fẹ́ gbogbo ènìyàn.",
   "<b>Àṣírí tí ó bá pọndandan:</b> àdírẹ́sì ìgbàwọlé ẹ̀ẹ̀kan gẹ́gẹ́ bí " + L("EIP-5564", U["eip5564"]) + " ń fi ẹni tí "
   "a sanwó fún pamọ́ (a gbèrò rẹ̀ fún milestone M13).",
   "<b>Ẹ̀kọ́ àti ìwádìí:</b> code tó ṣí sílẹ̀, testnet pẹ̀lú àṣẹ kan, àti data gbangba ń sọ ToyChain di ohun ẹ̀kọ́ fún "
   "ilé-ìwé, yunifásítì àti àwọn tó nífẹ̀ẹ́ sí i.",
   "<b>Ó ṣeé ṣe lẹ́yìn náà:</b> ìsopọ̀ fún oníṣòwò, mode ibi ìsanwó, ìsanwó láìsí ayélujára àti ìsopọ̀ pẹ̀lú eré "
   "TOYQUEST. Àwọn èrò wọ̀nyí kì í ṣe apá ìpele àkọ́kọ́ rárá."])

# ================================================================ 13
H("Ìjọra àti ìyàtọ̀")
T([["", "Bitcoin", "Ethereum", "ToyChain (a gbèrò)"],
   ["Ìbẹ̀rẹ̀", "ó tọ́, láìsí premint", "presale àti ìpín foundation", "ó tọ́, genesis láìsí owó kankan"],
   ["Iye tó pọ̀ jùlọ", "nǹkan bí mílíọ̀nù 21 BTC", "kò sí ààlà tó dúró", "mílíọ̀nù 100 TOY, kò lè yí padà"],
   ["Ìtújáde", "ń di ìdajì ní nǹkan bí ọdún 4", "bí ó ti yẹ, a ń jó apá kan owó iṣẹ́",
    "ń dín kù déédéé, factor 0.5 sí 2.0, a kì í jó o láé"],
   ["Ààbò nípasẹ̀", "iṣẹ́ kọ̀ǹpútà (proof of work)", "stake (proof of stake), 32 ETH fún validator kan",
    "stake ní ìpele 10, 100, 1,000 TOY pẹ̀lú ìpele ìbẹ̀rẹ̀ gbangba"],
   ["Ó parí lẹ́yìn", "nǹkan bí wákàtí 1 (6 confirmations)", "nǹkan bí ìṣẹ́jú 13", "block 1, nǹkan bí ìṣẹ́jú 1"],
   ["Ìpamọ́ full node", "nǹkan bí 760 GB", "ọgọ́rọ̀ọ̀rún GB títí dé ọ̀pọ̀ TB", "èrò: 256 MB"],
   ["Programmability", "scripts rírọrùn", "smart contracts, tokens, NFTs", "iṣẹ́ tí a kọ́ sínú rẹ̀ títí nìkan"],
   ["Àkáǹtì", "UTXO model", "account model", "account model, àdírẹ́sì Ethereum"],
   ["Owó iṣẹ́", "nígbà gbogbo, gẹ́gẹ́ bí ipò ọjà", "nígbà gbogbo, base fee pẹ̀lú tip",
    "10 sí 100 ọ̀fẹ́ ní epoch kan (ó kéré tán ọjọ́ 1), lẹ́yìn náà fee tàbí iṣẹ́"],
   ["Ìkópa láìní owó", "ṣíṣe node nìkan", "ṣíṣe node nìkan", "bẹ́ẹ̀ni, pẹ̀lú lọ́tìrì iṣẹ́"]],
  [3.5, 3.6, 4.2, 5.1])
P("<b>Bíi ti Bitcoin</b>, ToyChain ní ìbẹ̀rẹ̀ tó tọ́, iye tó pọ̀ jùlọ tó dúró, ìtújáde tó ń dín kù fún nǹkan bí ọdún "
  "25, àìsí aláṣẹ àárín àti code tó ṣí sílẹ̀. <b>Bíi ti Ethereum</b>, ó ní àdírẹ́sì, gbólóhùn ọ̀rọ̀, ọ̀nà ìbuwọ́lù, "
  "proof of stake àti èrò base fee kan tó ń ga nígbà tí network bá kún. <b>Tuntun</b> ni àkópọ̀ ìkópa láìní owó, quota "
  "ọ̀fẹ́, ààlà ìpamọ́ 256 MB, àkáǹtì tó ń sùn, ààbò àkáǹtì àṣàyàn àti constitution kan tó dènà ìfọwọ́kàn owó ẹlòmíràn.")

# ================================================================ 14
H("Àǹfààní")
B(["<b>Òtítọ́:</b> kò sí ẹni tí a ń fún ní TOY lọ́fẹ̀ẹ́; gbogbo òfin wà ní gbangba, ó sì dọ́gba fún gbogbo ènìyàn.",
   "<b>Ìrọ̀rùn wíwọlé:</b> ìkópa pẹ̀lú ẹ̀rọ àtijọ́, láìní owó àti láìní ìmọ̀ àkànṣe, ní èdè 45.",
   "<b>Ìparí kíákíá:</b> ìsanwó kan di ààbò lẹ́yìn nǹkan bí ìṣẹ́jú kan.",
   "<b>Iná mànàmáná díẹ̀:</b> kò sí ìdíje fún agbára kọ̀ǹpútà. Lọ́tìrì iṣẹ́ ń ṣiṣẹ́ lórí charger nìkan, àti bí olùlò ṣe "
   "gbà láàyè nìkan. Ìṣirò púpọ̀ ń mú tíkẹ́ẹ̀tì púpọ̀ wá ní ìwọ̀n nìkan, kì í ṣe àǹfààní àkànṣe.",
   "<b>Ìrọ̀rùn gẹ́gẹ́ bí ààbò:</b> láìsí smart contracts, kò sí program ẹlòmíràn tó lè dì owó, tàbí sọ ọ́ nù nípa àṣìṣe.",
   "<b>Ààbò ohun ìní:</b> ìdènà ń gba ẹ̀tọ́ kúrò nìkan, kì í ṣe owó láé. Àkáǹtì kì í sọnù. A kì í pa TOY run.",
   "<b>Ìṣípayá:</b> ToyScan, ìròyìn ìṣípayá oṣooṣù láti testnet, àti version program tí a lè tún kọ́.",
   "<b>Ìbámu:</b> àdírẹ́sì Ethereum àti àwọn wallet tí a mọ̀ ń ṣiṣẹ́."])

# ================================================================ 15
H("Ewu àti ìbéèrè tó ṣì wà")
P("Whitepaper olóòótọ́ kan tún ń dárúkọ ohun tó lè lọ ní àìtọ́. Àwọn ewu pàtàkì jùlọ láti ojú ìwòye òní:")
B(["<b>Ìpele ìbẹ̀rẹ̀ pẹ̀pẹ̀:</b> ToyChain kò tíì ṣiṣẹ́ rí. Àṣìṣe nínú èrò tàbí nínú code ṣeé ṣe gan-an, a ó sì wá wọn "
   "lórí testnet. Ìròyìn àyẹ̀wò méjì láti ọwọ́ AI review agent kan, tí kò kópa nínú kíkọ ọ̀rọ̀ náà, ti rí àìlera, a sì "
   "ti ṣàtúnṣe wọn. Àyẹ̀wò láti ọwọ́ ènìyàn ṣì kù. Àyẹ̀wò mìíràn àti audit látìta jẹ́ dandan ṣáájú mainnet kọ̀ọ̀kan.",
   "<b>Network kékeré, ààbò kékeré:</b> chain tuntun kan rọrùn láti kọlù ní ìbẹ̀rẹ̀, nítorí pé ó rọrùn láti borí "
   "olùkópa díẹ̀. Ààbò ń wáyé nípasẹ̀ ọ̀pọ̀ olùkópa tí kò gbára lé ara wọn nìkan.",
   "<b>Ìgbẹ́kẹ̀lé ní ìpele ìbẹ̀rẹ̀:</b> start validators 21 ní ojúṣe púpọ̀ ní oṣù àkọ́kọ́, wọ́n sì ń gba èrè tí ó ní ààlà. "
   "Bóyá ìfàlé sí àwọn validator tuntun yóò yọrí sí rere, a ó rí i.",
   "<b>Ààlà ìpamọ́:</b> bóyá 256 MB yóò tó fún ọ̀pọ̀ ọdún dá lórí ìlò àti lórí proof layer tó ń bọ̀. Tí kò bá tó, a ó fi "
   "nọ́ńbà gbé e kalẹ̀ ní gbangba. Àwọn node ńlá yóò nílò ìpamọ́ púpọ̀ sí i nígbà náà.",
   "<b>Ohun èlò tí a kò tíì dán wò tó:</b> fún àwọn ọ̀nà kan (fún àpẹẹrẹ àìròtẹ́lẹ̀ tí a lè ṣàyẹ̀wò àti consensus "
   "library), Rust library tí a ti ṣàyẹ̀wò kò pọ̀. A ó kọ wọ́n gẹ́gẹ́ bí standard gangan, a ó sì jẹ́ kí àwọn látìta "
   "ṣàyẹ̀wò wọn.",
   "<b>Iṣẹ́ kọ̀ǹpútà:</b> bóyá ìrántí 16 MiB tó láti pa ẹ̀rọ àkànṣe mọ́ sẹ́yìn, a ó wọ̀n ọ́n lórí testnet.",
   "<b>Òfin àti ìlànà:</b> cryptocurrency kan ń kan òfin Europe (" + L("MiCA", U["mica"]) + "), òfin owó-orí, àti bóyá "
   "òfin tẹ́tẹ́ (ọ̀rọ̀ “lọ́tìrì”). Ṣáájú mainnet, àyẹ̀wò òfin jẹ́ dandan. Àdírẹ́sì ajogún kò rọ́pò òfin ogún.",
   "<b>Àṣìṣe olùlò:</b> ẹni tó bá pàdánù gbólóhùn ọ̀rọ̀ rẹ̀ tàbí tó bá sọ ọ́ fún ẹlòmíràn ń pàdánù owó rẹ̀. Ààbò àkáǹtì "
   "àti ìkìlọ̀ ń dín èyí kù, ṣùgbọ́n wọn kò lè dènà rẹ̀ pátápátá.",
   "<b>Iye:</b> TOY kò ní iye tí a ṣèlérí. Bóyá ọjà yóò wà rárá ṣì wà ní ṣíṣí (apá 16). Kò sí ẹni tó gbọ́dọ̀ fi owó tí "
   "kò lè fara da àdánù rẹ̀ sínú rẹ̀.",
   "<b>Ìdàrúdàpọ̀ àti jìbìtì:</b> tokens àtijọ́ tí wọ́n ní orúkọ kan náà lórí chain mìíràn lè da àwọn olùlò rú, tàbí kí "
   "a lò wọ́n fún jìbìtì. Àwọn aláṣẹ EU ń kìlọ̀ ní gbogbogbòò nípa " + L("ewu crypto" + EN, U["esas_warning"]) + "."])

# ================================================================ 16
H("Àǹfààní nínú ọjà crypto: ìgbéyẹ̀wò")
P("Orí yìí ń gbìyànjú ìgbéyẹ̀wò tó dákẹ́ jẹ́jẹ́. Kì í ṣe àsọtẹ́lẹ̀, kì í sì ṣe ìmọ̀ràn ìdókòwò. Nọ́ńbà ọjà wá láti àárín "
  "October 2026, wọ́n sì ń yí padà kíákíá.")
H2("16.1 Ipò ìbẹ̀rẹ̀")
P("Ní 18 October 2026, gbogbo ọjà crypto tó nǹkan bí $2.7 trillion, nǹkan bí 60 % nínú rẹ̀ sì jẹ́ ti Bitcoin. Ìmọ̀lára "
  "fún coin kékeré kò lágbára: nínú crypto 100 tó tóbi jùlọ, 14 péré ló gòkè ní ọjọ́ náà ("
  + L("ìròyìn ọjà 18.10.2026" + EN, U["market"]) + ", ipò lọ́wọ́lọ́wọ́ lórí " + L("CoinGecko", U["cg_charts"]) + "). "
  "Ní àkókò kan náà, iṣẹ́ àkànṣe tuntun ti kún ọjà fọ́fọ́. Gẹ́gẹ́ bí ìwádìí CoinGecko kan, ó ju crypto mílíọ̀nù 20 lọ tí "
  "a ti kà títí di òpin 2025. Ní 2025 nìkan, ó ju mílíọ̀nù 11.6 lọ tó kú, lápapọ̀ ó ju ìdajì gbogbo èyí tí a ti kà rí "
  "(" + L("ìròyìn nípa ìwádìí CoinGecko" + EN, U["dead"]) + "). Èyí tó pọ̀ jùlọ nínú wọn jẹ́ tokens tí kò pẹ́ lórí chain "
  "tó wà tẹ́lẹ̀, kì í ṣe blockchain tiwọn. Ṣùgbọ́n nọ́ńbà wọ̀nyí fi hàn bí ó ti ṣòro tó láti rí àfiyèsí àti olùlò tí "
  "yóò dúró pẹ́.")
FIG(F + "f_deadcoins.png", "Crypto tó kú ní ọdún kọ̀ọ̀kan gẹ́gẹ́ bí data CoinGecko. Èyí tó pọ̀ jùlọ nínú àwọn tuntun kì "
                           "í là á já.", 15.0)
P("Nínú EU, òfin " + L("MiCA", U["esma"]) + " ń ṣiṣẹ́ láìsí àkókò ìyípadà láti 1 July 2026: àwọn olùpèsè iṣẹ́ crypto "
  "nílò ìwé àṣẹ, bí bẹ́ẹ̀ kọ́ wọ́n gbọ́dọ̀ dá iṣẹ́ wọn pẹ̀lú oníbàárà EU dúró (" + L("ìkéde AMF" + EN, U["amf"]) + "). "
  "Fún ToyChain, èyí túmọ̀ sí nǹkan méjì: àwọn exchange àti wallet provider tí wọ́n bá fẹ́ gba TOY lẹ́yìn náà ń ṣàyẹ̀wò "
  "dáadáa sí i. Àti fún títà sí gbogbo ènìyàn tàbí fífi sí ọjà, MiCA sábà máa ń béèrè crypto-asset white paper gẹ́gẹ́ "
  "bí ìlànà tó dúró. Bóyá àti bí èyí ṣe kan coin kan tí kò ní olùgbéjáde bí TOY gbọ́dọ̀ di mímọ̀ lọ́nà òfin ṣáájú "
  "mainnet. Ìwé yìí <b>kì í ṣe</b> irú white paper bẹ́ẹ̀.")
H2("16.2 Ohun tó ń ṣe ToyChain láǹfààní")
B(["<b>Òtítọ́ tí a lè gbẹ́kẹ̀lé:</b> ìbẹ̀rẹ̀ láìsí owó kankan ṣọ̀wọ́n. Nínú ọjà kan tí ọ̀pọ̀ iṣẹ́ àkànṣe ti ń bẹ̀rẹ̀ pẹ̀lú "
   "ìpín ńlá fún àwọn inú, èyí jẹ́ àmì ìyàtọ̀ tó ṣe kedere, tí ẹnikẹ́ni lè wò fúnra rẹ̀ nínú ToyScan.",
   "<b>Wíwọlé tó rọrùn:</b> ìkópa pẹ̀lú fóònù àtijọ́, láìra nǹkankan àti láìní ìmọ̀ tẹ́lẹ̀. Èyí lè dé ọ̀dọ̀ àwọn tí kì "
   "bá tíì ṣe node rí, ó sì ń ṣẹ̀dá ọ̀pọ̀ olùkópa kékeré dípò díẹ̀ ńlá.",
   "<b>Ibi ìkọlù díẹ̀:</b> láìsí smart contracts àti bridges, àwọn ohun tó fa ọ̀pọ̀ àdánù ńlá ní ọdún díẹ̀ sẹ́yìn pòórá.",
   "<b>Ìbámu:</b> àdírẹ́sì Ethereum ń dín ìdíwọ́ kù fún àwọn olùlò tí wọ́n ti ní wallet.",
   "<b>Ìṣípayá àti èdè:</b> licence ọ̀fẹ́, àkọsílẹ̀ ìpinnu tí a lè tọpinpin (gbangba láti testnet) àti èdè 45 ń mú kí "
   "iṣẹ́ àkànṣe yìí ṣí sílẹ̀ fún àwùjọ àgbáyé. Whitepaper yìí ti wà ní èdè German, Gẹ̀ẹ́sì, Faransé àti Yorùbá."])
H2("16.3 Ohun tó ń ṣe é ní ìdíwọ́")
B(["<b>Kò sí owó ìbẹ̀rẹ̀:</b> láìsí premint, kò sí àpò owó fún developer, audit, ìpolówó tàbí exchange listing. Iṣẹ́ "
   "àkànṣe náà gbára lé olùyọ̀ǹda, ẹ̀bùn àti ìrànlọ́wọ́.",
   "<b>Agbára network ńlá:</b> olùlò, oníṣòwò àti exchange ń kórajọ sí chain ńlá. Chain tuntun kan gbọ́dọ̀ fúnni ní "
   "ìdí tó ṣe kedere tí ẹnìkan yóò fi yípadà.",
   "<b>A mọ̀ọ́mọ̀ fi ààlà sí i:</b> láìsí smart contracts, kò sí ecosystem ti app, tokens àti decentralised exchange, "
   "tó ń fa àfiyèsí àti owó púpọ̀ lórí chain mìíràn.",
   "<b>Ìdíje líle nínú ìsanwó:</b> fún ìsanwó ojoojúmọ́, ọ̀pọ̀ ènìyàn ti ń lo stablecoins tí a so mọ́ owó orílẹ̀-èdè, "
   "tàbí network tó ti fìdí múlẹ̀. Coin kan tí iye rẹ̀ ń yí padà kò fà mọ́ra tó fún èyí.",
   "<b>Ẹgbẹ́ kékeré, ìbẹ̀rẹ̀ pẹ̀pẹ̀:</b> títí dé mainnet, milestone méjìlá, audit àti àlàyé òfin ṣì nílò.",
   "<b>Orúkọ:</b> tokens àtijọ́ pẹ̀lú orúkọ kan náà ń mú kí ó ṣòro láti rí iṣẹ́ àkànṣe yìí, wọ́n sì ní ewu ìdàrúdàpọ̀."])
FIG(F + "f_swot.png", "Agbára, àìlera, àǹfààní àti ewu ní ìwòye kan.", 15.0)
H2("16.4 Àwọn ìṣẹ̀lẹ̀ tó ṣeé ṣe")
T([["Ìṣẹ̀lẹ̀", "Àpèjúwe", "Ìgbéyẹ̀wò"],
   ["A · Ìkùnà ṣáájú mainnet", "Testnet fi ìṣòro ìpìlẹ̀ hàn, tàbí olùkópa kò tó.",
    "ó ṣeé ṣe, bíi ti ọ̀pọ̀ iṣẹ́ àkànṣe"],
   ["B · Iṣẹ́ àkànṣe ẹ̀kọ́ àti àwùjọ", "Testnet tó dúró dáadáa àti àwùjọ kékeré tó ń ṣiṣẹ́; a kò fi bẹ́ẹ̀ ta TOY.",
    "ó ṣeé ṣe jùlọ, tí iṣẹ́ bá ń bá a lọ"],
   ["C · Ọjà kékeré ní tòótọ́", "Mainnet lẹ́yìn audit, node ọgọ́rọ̀ọ̀rún sí ẹgbẹ̀rún kan, títà lórí platform díẹ̀, ìlò "
                               "nínú ibi àkànṣe (eré, ẹgbẹ́, ẹ̀bùn).", "ó ṣeé ṣe, ó nílò àwùjọ, audit àti òfin tó ṣe kedere"],
   ["D · Ìlò káàkiri", "Ọ̀kan lára àwọn coin olókìkí pẹ̀lú mílíọ̀nù olùlò.", "kò fi bẹ́ẹ̀ ṣeé ṣe rárá"]],
  [4.2, 7.6, 4.6])
H2("16.5 Ohun tó lè mú ìyàtọ̀ wá")
B(["<b>Start validators tí kò gbára lé ara wọn</b> láti orílẹ̀-èdè ọ̀tọ̀ọ̀tọ̀, tí wọ́n ń kópa ní gbangba.",
   "<b>Audit ààbò látìta kan</b>, tí a fi ẹ̀bùn tàbí ètò ìrànlọ́wọ́ fún software ọ̀fẹ́ sanwó fún.",
   "<b>Ìlò kan tó ṣe kedere</b>, tí kì bá ti dára tó láìsí ToyChain, fún àpẹẹrẹ eré TOYQUEST, níbi tí àwọn olùṣeré ti "
   "ń kópa pẹ̀lú fóònù wọn, tí wọ́n sì ń jèrè TOY.",
   "<b>Ìṣípayá déédéé:</b> ìròyìn oṣooṣù, nọ́ńbà gbangba, àṣà olóòótọ́ nípa àṣìṣe.",
   "<b>Òfin tó ṣe kedere</b> ṣáájú ìgbésẹ̀ kọ̀ọ̀kan tó kan owó gidi."])
P("<b>Ìparí ìgbéyẹ̀wò:</b> gẹ́gẹ́ bí ìdókòwò, a kò lè díwọ̀n ToyChain lónìí, bíi fẹ́rẹ̀ẹ́ gbogbo cryptocurrency tuntun, "
  "ó sì ní ewu àdánù tó ga gan-an. Gẹ́gẹ́ bí iṣẹ́ àkànṣe technical àti ti àwùjọ, ToyChain ní àwòrán tó ṣe kedere tí kò "
  "wọ́pọ̀: ó tọ́, ó fúyẹ́, ó yé ni, ó sì ṣọ́ra. Àǹfààní rẹ̀ gidi kò sí nínú kí iye rẹ̀ gòkè kíákíá, ṣùgbọ́n nínú kíkọ́ "
  "àwùjọ kékeré kan tó gbàgbọ́, tó ń ṣe network náà fúnra rẹ̀. Tí èyí bá yọrí sí rere, ohun tó pọ̀ jù lè wáyé láti inú "
  "rẹ̀ bí ọdún ti ń lọ. Kò sí ìdánilójú.")

# ================================================================ 17
H("Ipò àti ọ̀nà iwájú")
P("<b>Lónìí (19 October 2026): ṣáájú ìpele ìdánwò.</b> A ti kọ èrò náà sílẹ̀ nínú ìpinnu 114 tí a ti ṣàkọsílẹ̀. A pín "
  "iṣẹ́ kíkọ́ sí milestone 13. Milestone M1 ti parí: a ti kọ òfin fún iye owó àti fún ìtújáde TOY tuntun sínú program, a "
  "sì ti fi ìdánwò aládàáṣe 45 ṣàyẹ̀wò wọn. Rust implementation ń fúnni ní nọ́ńbà kan náà gẹ́lẹ́, títí dé ẹ̀ka tó kéré "
  "jùlọ, bíi simulation Python tí a kọ lọ́tọ̀. Àwọn ìdánwò ń ṣiṣẹ́ fúnra wọn ní gbogbo ìyípadà lórí Linux, Windows àti "
  "ARM. Pẹ̀lú irinṣẹ́ proof Kani, a ti fi òfin pàtàkì mẹ́ta hàn lọ́nà mathematics, ọ̀kọ̀ọ̀kan ní ìṣẹ́jú méjì: kò sí "
  "epoch tó ń tú ju iye tó kù lọ, àpapọ̀ ń wà ní mílíọ̀nù 100 TOY jùlọ, pínpín block kan sì péye. Proof mẹ́ta mìíràn fún "
  "pínpín pẹ̀lú ìpín èrè kò tíì parí lẹ́yìn ìṣẹ́jú 30. Nítorí náà, program fúnra rẹ̀ ń ṣàyẹ̀wò pínpín yìí ní block "
  "kọ̀ọ̀kan, àwọn ìdánwò àfikún sì bò ó.")
FIG(F + "f_roadmap.png", "Ọ̀nà iwájú. A mọ̀ọ́mọ̀ kò fi ọjọ́ kan sílẹ̀ fún mainnet.")
T([["Ìpele", "Àkóónú", "Ipò"],
   ["Èrò", "Specification v0.3, àkọsílẹ̀ ìpinnu, ìròyìn àyẹ̀wò méjì", "ó ti parí, a ń tún un ṣe déédéé"],
   ["Phase 1, M1 sí M11", "Types ìpìlẹ̀ àti ìtújáde, cryptography, àkáǹtì àti transaction, consensus àti ìpele "
    "ìbẹ̀rẹ̀, ìpamọ́, network, interfaces, wallet, platform, testnet, ToyScan", "M1 ti parí, M2 ló kàn"],
   ["Testnet 1", "Core pẹ̀lú ààbò àkáǹtì, èdè 45 àti mode mẹ́rin; lákọ̀ọ́kọ́ lórí ẹ̀rọ kan, lẹ́yìn náà gbangba",
    "a ti gbèrò"],
   ["Phase 1, M12 àti M13", "transaction mìíràn, atomic swap, orúkọ, àdírẹ́sì ẹ̀ẹ̀kan, post-quantum keys", "a ti gbèrò"],
   ["Phase 2", "Proof layer: ká ìtàn, node láìsí ìgbẹ́kẹ̀lé checkpoint", "a ti gbèrò"],
   ["Lẹ́yìn náà", "Ìṣàkóso àpapọ̀, mainnet (lẹ́yìn audit, ìdánwò agbára àti àyẹ̀wò òfin nìkan, a ó kéde ọjọ́ ṣáájú), "
    "bridges àti ìsanwó láìsí ayélujára tó ṣeé ṣe", "kò tíì dájú"]],
  [3.4, 9.4, 3.6])
P("Kò sí ọjọ́ tó dúró fún mainnet. Ó ń bẹ̀rẹ̀ nìkan nígbà tí testnet bá ń ṣiṣẹ́ dáadáa, tí a ti ṣàyẹ̀wò ààbò, tí a sì "
  "ti yanjú àwọn ìbéèrè òfin. A ó kéde ọjọ́ ìbẹ̀rẹ̀ ní gbangba ṣáájú, kí gbogbo ènìyàn lè bẹ̀rẹ̀ lẹ́ẹ̀kan náà.")

# ================================================================ 18
H("Ìparí")
P("ToyChain fẹ́ fi hàn pé cryptocurrency kan lè bẹ̀rẹ̀ lọ́nà tó tọ́, ṣiṣẹ́ pẹ̀lú ìpamọ́ díẹ̀, kí ó sì ṣì yára, kí ó sì "
  "ní ààbò. Ẹnikẹ́ni gbọ́dọ̀ lè kópa, kódà láìní owó àti pẹ̀lú ẹ̀rọ àtijọ́. Nítorí èyí, ToyChain mọ̀ọ́mọ̀ fi ọ̀pọ̀ ohun tí "
  "àwọn chain mìíràn ń fúnni sílẹ̀, ó sì gbára lé iṣẹ́ díẹ̀ tí a kọ́ sínú rẹ̀ títí, tí ó rọrùn láti ṣàyẹ̀wò. Bóyá èyí "
  "yóò yọrí sí rere, kì í ṣe ìwé yìí ló máa pinnu, testnet ni. A gbà ìkópa, àríwísí àti àyẹ̀wò tọwọ́tẹsẹ̀. Code, "
  "specification àti àkọsílẹ̀ ìpinnu wà lórí " + L("GitHub", U["repo"]) + ", gbogbo ènìyàn yóò sì lè rí wọn ní ìgbà "
  "testnet gbangba ní tuntun jùlọ.")

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
   ["Iṣẹ́ àkànṣe", L("github.com/Safehaigit/toycoin", U["repo"]), "code, specification (German)"]],
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
    "m-simulation.py; github.com/Safehaigit/toycoin (German).",
]
for i, r in enumerate(refs, 1):
    story.append(Paragraph(f"[{i}] {r}", small))


story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Àfikún D: Àyípadà láti Draft 0.2", h1))
P("Draft 0.3 jẹ́ àyẹ̀wò Draft 0.2 fún ọgbọ́n, ìrọ̀rùn òye àti ìmọ̀ tuntun láti 18 àti 19 October 2026. Èrò fúnra rẹ̀ "
  "kò yí padà.")
T([["Ibi", "Àyípadà", "Ìdí"],
   ["1, 18, Àfikún B", "Repository jẹ́ private lọ́wọ́lọ́wọ́; a ó ṣí i sílẹ̀ ní ìgbà testnet gbangba ní tuntun jùlọ.",
    "Draft 0.2 pè é ní gbangba lọ́nà àṣìṣe."],
   ["2, 13", "Quota ọ̀fẹ́ “ní epoch kan (ó kéré tán ọjọ́ kan)” dípò “ní ọjọ́ kan”.",
    "ó péye jù: epoch kan lè gùn ju wákàtí 24 lọ."],
   ["5.3", "Ìyàtọ̀ sí ètò: ọdún kìíní nǹkan bí 2.2 % jùlọ, ní ìgbà pípẹ́ nǹkan bí 2.5 % jùlọ.",
    "“nǹkan bí 2 % jùlọ” kéré jù (simulation: +2.23 %)."],
   ["6.1", "Èrè lẹ́yìn ìpele ìbẹ̀rẹ̀ ni a ṣàpèjúwe ní kíkún: ìdá márùn-ún kan ní ìpín dọ́gba fún ọmọ committee "
    "kọ̀ọ̀kan, èyí tó kù lọ́nà linear, méjèèjì ní ìwọ̀n iṣẹ́ tí a ṣe ní tòótọ́.", "ó kù, ìpinnu 112"],
   ["6.3", "Èrè lọ́tìrì àkọ́kọ́ kan ti tó fún ìdógò ìmúṣiṣẹ́.",
    "ó dáhùn ìbéèrè bí a ṣe ń rí àkáǹtì àkọ́kọ́ láìní TOY"],
   ["7.1", "“ìpele méjì” (pre-vote àti ìfìdímúlẹ̀) dípò “ìyípo méjì”.",
    "“ìyípo” (round) túmọ̀ sí nǹkan mìíràn nínú ọ̀nà náà."],
   ["8", "A ṣàlàyé plan time; èrè gẹ́gẹ́ bí “ìdajì gbogbo TOY tuntun”; ohun tó ń ṣẹlẹ̀ tí ìfàlé kò bá tíì wáyé lẹ́yìn "
    "ọjọ́ 180.", "Àwòrán 8 lo ọ̀rọ̀ náà láìsí àlàyé; àlàfo nínú ìtẹ̀léra"],
   ["9", "Títí dé phase 2, àwọn node tuntun ń bẹ̀rẹ̀ láti checkpoints pẹ̀lú ìbuwọ́lù mẹ́ta tí kò gbára lé ara wọn.",
    "àlàfo: ta ni node tuntun ń gbẹ́kẹ̀lé títí di ìgbà náà?"],
   ["10.1", "Àkókò ìfagilé àdírẹ́sì ajogún (epoch 30) àti àkíyèsí olóòótọ́ nípa emergency key.",
    "ìpéye àti òtítọ́"],
   ["10.4, 17", "Àbájáde proof Kani: a ti fi òfin pàtàkì mẹ́ta hàn, proof mẹ́ta ṣì wà ní ṣíṣí.",
    "ìmọ̀ tuntun láti 18.10.2026"],
   ["12", "Ìsanwó àpapọ̀ lẹ́yìn testnet àkọ́kọ́ nìkan.", "àkóónú testnet 1 (ìpinnu 109)"],
   ["14", "Iná mànàmáná: lọ́tìrì lórí charger nìkan, àti bí olùlò ṣe gbà láàyè nìkan.",
    "“nígbà tí iná bá wà tẹ́lẹ̀” kò péye."],
   ["15", "Ìròyìn àyẹ̀wò wá láti ọwọ́ AI review agent kan; àyẹ̀wò láti ọwọ́ ènìyàn ṣì kù.",
    "“ìròyìn àyẹ̀wò tí kò gbára lé ẹnikẹ́ni” lè ṣini lọ́nà."],
   ["16.2", "Àkọsílẹ̀ ìpinnu di gbangba láti testnet nìkan; whitepaper ní èdè mẹ́rin (German, Gẹ̀ẹ́sì, Faransé, "
    "Yorùbá).", "àtúnṣe àti ipò tuntun"]],
  [2.4, 8.6, 5.4])

story.append(CondPageBreak(4 * cm))
story.append(Paragraph("Nípa ẹ̀dà yìí", h1))
P("A túmọ̀ ẹ̀dà Yorùbá yìí <b>tààrà láti inú ẹ̀dà German</b> ti Draft 0.3, ìpínrọ̀ sí ìpínrọ̀ àti gbólóhùn sí gbólóhùn, "
  "pẹ̀lú ẹ̀rọ (machine translation). Àwọn ọ̀rọ̀ technical wà ní èdè Gẹ̀ẹ́sì, gbogbo link sì ń tọ́ka sí àlàyé ní èdè "
  "Gẹ̀ẹ́sì. Ẹ̀dà Yorùbá mìíràn wà tí a túmọ̀ láti inú ẹ̀dà Gẹ̀ẹ́sì. Ẹnìkan tí èdè Yorùbá jẹ́ èdè abínibí rẹ̀ gbọ́dọ̀ ka "
  "á kí a tó tẹ̀ ẹ́ jáde.")
story.append(Spacer(1, 6))
story.append(Paragraph("<b>Ìkìlọ̀.</b> Ìwé yìí ṣàpèjúwe iṣẹ́ àkànṣe software tí a ṣì ń kọ́. Kì í ṣe ìmọ̀ràn ìdókòwò, "
                       "owó-orí tàbí òfin, kì í sì ṣe ìpè, prospectus tàbí crypto-asset white paper gẹ́gẹ́ bí òfin "
                       "MiCA. Gbogbo ìsọfúnni jẹ́ ti ìgbà díẹ̀, ó sì lè yí padà. Nọ́ńbà ọjà jẹ́ ipò ọjọ́ tí a sọ. Tí "
                       "iyèméjì bá wà, ẹ̀dà German ló ní àṣẹ.", small))

finish("ToyChain-Whitepaper-v0.3-yo-de.pdf", "ToyChain · Whitepaper àkọ́kọ́ · Draft 0.3 · ṣáájú ìpele ìdánwò",
       "Ojú-ìwé", "ToyChain àti ToyCoin (TOY): Whitepaper àkọ́kọ́", "Draft 0.3 (láti German), ṣáájú ìpele ìdánwò", "yo")
