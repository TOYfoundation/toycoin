"""ToyChain whitepaper, draft 0.2, English edition.
Run: python3 figures_i18n.py en && python3 build_en.py"""
from wp_common import *  # noqa: F401,F403
from wp_common import story, L, H, H2, P, MORE, B, T, BOX, FIG, toc_table, finish, set_words
from wp_common import title, subtitle, center, abstract, small, h1
from reportlab.platypus import Paragraph, Spacer, CondPageBreak
from reportlab.lib.units import cm

set_words("Figure", "More:")
F = "abbildungen/en/"

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
    eba_warning="https://www.eba.europa.eu/publications-and-media/press-releases/eu-financial-regulators-warn-consumers-risks-crypto-assets",
    bmf="https://www.bundesfinanzministerium.de/Content/DE/Downloads/BMF_Schreiben/Steuerarten/Einkommensteuer/2025-03-06-einzelfragen-kryptowerte.html",
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

# ================================================================ Title
story.append(Spacer(1, 0.3 * cm))
story.append(Paragraph("ToyChain and ToyCoin (TOY)", title))
story.append(Spacer(1, 4))
story.append(Paragraph("A lean, fairly launched payment network<br/>that anyone can join without funds", subtitle))
story.append(Spacer(1, 10))
story.append(Paragraph("ToyChain project · " + L("github.com/TOYfoundation/toycoin", U["repo"]), center))
story.append(Paragraph("Preliminary whitepaper · Draft 0.3 · 19 October 2026 · English edition", center))
story.append(Spacer(1, 12))
BOX("<b>Status: before the test phase.</b> So far ToyChain exists as a concept and as a first software component. "
    "The basic types and the rules for creating new coins are finished and tested (milestone M1 of 13). There is "
    "<b>no testnet yet, no mainnet and no tradable TOY</b>. All figures are starting values that will be measured and "
    "adjusted on the testnet. This document is not investment advice and not an offer to buy anything. Testnet "
    "balances have no value and will never be carried over to a later mainnet. Underlined blue terms are web links to "
    "further explanations. The German edition is the reference version.")

P("<b>Abstract.</b> ToyChain is an independent blockchain with its own currency, ToyCoin (TOY), capped at 100 million "
  "coins. It launches completely fairly: the first block contains not a single TOY, there is no reserve for "
  "founders, no presale and no special allocation. All TOY are created step by step by a fixed, publicly verifiable "
  "rule, similar to " + L("Bitcoin", U["btc_pdf"]) + ". Unlike Bitcoin, security does not come from electricity but "
  "from participants who lock TOY as a deposit and sign blocks together. A block is final after about one minute. "
  "Every node needs only 256 MB of data and also runs on old phones and on a Raspberry Pi. Anyone can join without "
  "funds; whoever contributes more gradually gains more rights. ToyChain deliberately does without smart contracts, "
  "tokens and NFTs: every feature is built in, easy to verify and can never touch other people's funds. Account "
  "addresses are the same as on Ethereum, so well-known wallets keep working. The paper ends with an honest "
  "assessment of the project's chances in the crypto market.", abstract)

story.append(Spacer(1, 4))
toc_table(["1 Introduction", "2 ToyChain at a glance", "3 Basic terms in plain words", "4 Architecture",
           "5 Fair launch and money supply", "6 Taking part: tiers, quota, lottery", "7 Reaching agreement",
           "8 The bootstrap phase", "9 Lean nodes for old devices", "10 Security for users",
           "11 Wallet, ToyScan and other chains", "12 Use cases", "13 Similarities and differences",
           "14 Advantages", "15 Risks and open questions", "16 Chances in the crypto market",
           "17 Status and roadmap", "18 Conclusion", "Appendix: values, links, sources, changes"], "Contents")

# ================================================================ 1
H("Introduction")
P("In 2008 Bitcoin showed that people can agree without a bank on who owns which money. Since then millions of "
  "crypto assets have appeared. In many of them, founders and early investors received a large share of the coins "
  "before launch (premint). Others need large data centres, a lot of electricity or hard drives with hundreds of "
  "gigabytes, so that ordinary people can hardly verify the network themselves. The full Bitcoin chain is now about "
  "760 GB. And many chains have become so versatile with smart contracts that bugs in other people's code regularly "
  "lead to losses.")
P("ToyChain tries to implement the core idea of Bitcoin, namely a fixed supply, a fair launch and open rules, in a "
  "lean and practical way with today's methods. The guiding questions are: Can anyone take part with an old device? "
  "Can anyone verify the rules themselves? Do your own funds stay yours under all circumstances? This whitepaper "
  "explains the ideas in plain language. The exact rules are in the technical specification (version 0.3, German) in "
  "the " + L("project repository", U["repo"]) + ". It is still private during development and is to be opened "
  "by the public testnet at the latest.")
MORE(L("Bitcoin whitepaper", U["btc_pdf"], "Basics"),
     L("Size of a Bitcoin node in 2026", U["btc_size"], "Basics"))

# ================================================================ 2
H("ToyChain at a glance")
T([["Property", "ToyChain (planned starting values)"],
   ["Currency", "ToyCoin, symbol TOY, divisible to 18 decimal places"],
   ["Maximum supply", "100,000,000 TOY, written into the code as an unchangeable rule"],
   ["Launch", "Total fair launch: first block without balances, no premint, no reserve"],
   ["New TOY", "per epoch (at least 24 hours) 0.0505 % of the remaining amount, adjusted by a factor m between 0.5 "
               "and 2.0; first year about 16.84 million, half after about 3.76 years, 99 % after about 25 years"],
   ["Agreement (consensus)", "Proof of stake with a committee of at most 128 validators, one joint signature, "
                             "final after 1 block"],
   ["Block time", "60 seconds, earlier when the waiting area is full, at the earliest after 10 seconds"],
   ["Storage per node", "Target 256 MB, also on old Android devices and the Raspberry Pi"],
   ["Accounts", "as on Ethereum: same addresses, same 12- or 24-word phrase"],
   ["Features", "only built-in transaction types, no smart contracts, no tokens, no NFTs"],
   ["Fees", "10 to 100 free transactions per account and epoch (at least one day), above that a small fee or "
             "computing work"],
   ["Programming language", "Rust, one shared core for all platforms"],
   ["Licence", L("MIT", U["mit"]) + " (free software)"]],
  [4.0, 12.4])

# ================================================================ 3
H("Basic terms in plain words")
P("If you have never dealt with cryptocurrencies, here are the most important terms. The following sections build on "
  "them. Clicking a blue term leads to a good explanation on the web.")
T([["Term", "In plain words"],
   ["Blockchain", "A shared ledger kept by many computers at the same time. New entries are added in packages "
                  "(blocks); every block refers to the previous one in a tamper-proof way."],
   ["Node", "A device running the ToyChain software. It checks by itself whether all entries follow the rules and does "
            "not need to trust anyone for that."],
   ["Wallet", "The app that keeps your keys and signs transfers. On ToyChain the TOYPAY wallet is also a node."],
   [L("Key and phrase", U["bip39"], "Accounts and keys"),
    "A secret key proves that an account belongs to you. It can be restored from 12 or 24 words. Whoever knows the "
    "words can spend the money."],
   ["Signature", "A digital signature. Anyone can check that it is genuine, but nobody can forge it without the key."],
   [L("Consensus", U["eth_consensus"], "Consensus"), "The procedure by which all nodes agree on the same next block."],
   [L("Stake", U["eth_pos"], "Consensus"), "TOY that you voluntarily lock as a deposit. Whoever follows the rules earns "
                                           "rewards. Whoever provably cheats loses part of the deposit."],
   ["Validator", "A node with stake that proposes and co-signs blocks."],
   [L("Merkle tree", U["merkle"], "Storage"), "A kind of table of contents made of checksums. A small proof shows that "
                                              "an entry belongs to it without storing everything."],
   ["Epoch", "A section of the chain of at least 1,440 blocks and at least 24 hours. Many rules count in epochs, such "
             "as the issuance of new TOY."],
   ["Testnet", "A trial chain whose coins are worth nothing. Bugs are searched for there before things get serious."]],
  [3.6, 12.8])

# ================================================================ 4
H("Architecture")
P("ToyChain consists of stacked layers. Each has a clearly defined task. All layers live in one shared Rust program "
  "that works the same on Android, Windows, Linux and the Raspberry Pi. So nothing has to be written and checked twice.")
FIG(F + "f_arch.png", "The layers of ToyChain. The proof layer (dashed) follows in phase 2.", 14.8)
MORE(L("libp2p, the networking foundation", U["libp2p"], "Technology"),
     L("Dandelion++: who sent it?", U["dandelion"], "Technology"))

# ================================================================ 5
H("Fair launch and money supply")
H2("5.1 Not a single TOY in the first block")
P("The first block (genesis block) of ToyChain contains <b>no balances at all</b>. Nobody, not even the developers, "
  "owns TOY at launch. There is no presale, no reserve and no founders' share. That is stricter than most new "
  "projects and follows the spirit of Bitcoin. This rule is part of the “constitution” (section 10.3) and cannot be "
  "changed by any update.")
H2("5.2 How new TOY are created")
P("In every epoch a fixed fraction of the amount not yet issued is created: 0.0505 % of the remaining amount. In the "
  "first epoch that is exactly 50,500 TOY. Because the remaining amount shrinks, issuance falls smoothly instead of "
  "dropping by half every four years as with Bitcoin. After about 3.76 years half is issued, after about 25 years "
  "about 99 %. The maximum is never reached and never exceeded. Tests already prove this: even with twice the "
  "issuance speed over 100 years the total stays below it.")
FIG(F + "f_emission.png", "Issued share of the maximum supply over time. ToyChain declines smoothly, Bitcoin halves "
                          "in steps. After 25 years both are at about 99 %.")
FIG(F + "f_per_year.png", "New TOY per year according to plan (m = 1). Each year about 17 % fewer are created than in "
                          "the year before.")
MORE(L("How Bitcoin limits its supply", U["btc_supply"], "Money supply"))
H2("5.3 The factor m: adjusted, but restrained")
P("Issuance is multiplied by a factor m between 0.5 and 2.0. It rises slightly when the network is <i>really</i> "
  "used (transactions that pay a fee or are backed by computing work) and when many validators reliably co-sign. Free "
  "transactions do not count, otherwise issuance could be pushed up with spam. A schedule component catches up with "
  "deviations, and m may change by at most 0.05 per epoch. In simulations, issuance in the first year deviates from "
  "plan by at most about 2.2 %, and in the long run by at most about 2.5 %. So m only shifts <i>when</i> TOY are created, not <i>how many</i> in total.")
H2("5.4 Who receives the new TOY")
P("Every issuance is split in fixed parts: <b>50 %</b> to everyone securing the network, <b>40 %</b> to a work "
  "lottery that every device can join without funds (section 6.3), and <b>10 %</b> to a refund pool that pays for "
  "everyone's free transactions. In the bootstrap phase the security share goes to the start validators and falls to "
  "zero; what they do not receive flows into the lottery. All calculations use whole numbers in the smallest unit "
  "(10<super>-18</super> TOY). Rounding remainders go to the refund pool, so exactly the planned amount is created in "
  "every epoch.")
FIG(F + "f_split.png", "Split of new TOY. In the bootstrap phase the start validators' reward gradually moves to the "
                       "work lottery; after the handover all stakers receive it.")
H2("5.5 TOY are never destroyed")
P("Many chains “burn” fees or penalties, Ethereum for example part of every fee under "
  + L("EIP-1559", U["eip1559"], "Fees") + ". ToyChain does not: penalties and donations flow into the refund pool and "
  "thus benefit users. A check rule (I0) ensures in <b>every</b> block that the sum of all balances equals exactly the "
  "sum of all TOY issued. A block that violates this is invalid.")

# ================================================================ 6
H("Taking part: tiers, quota and work lottery")
H2("6.1 Participation tiers")
P("Everyone decides how much to contribute. On first start the wallet measures the device and suggests a suitable "
  "tier. It only offers a higher tier if the device can handle it.")
FIG(F + "f_tiers.png", "The five participation tiers. Even tier 0 needs no funds.", 15.0)
P("Locked TOY remain the user's property and can be released again after 21 epochs. So that nobody becomes too "
  "powerful, a participant's stake counts only up to 5 % (from 100 validators on, 1 %) of the total stake. Rewards "
  "After the bootstrap phase, one fifth of the security share goes in equal parts to every committee member, the "
  "rest linearly by stake. Both are paid only in proportion to the work actually done: whoever co-signs 90 % of the "
  "blocks receives 90 %. Because the stake-based part is linear, splitting stake across many accounts brings no "
  "advantage.")
MORE(L("What is proof of stake? (ethereum.org)", U["eth_pos"], "Consensus"))
H2("6.2 Free transactions and fees")
P("Every activated account has a free quota per epoch: 100 transactions when the network is quiet, at least 10 when "
  "it is busy. That is enough for everyday use. Above that you pay a small fee, which rises quickly with many "
  "transactions in the same block, or you attach a proof of work. So the network stays free for normal users while "
  "spam becomes expensive. Free transactions fill at most half a block and are paid from the refund pool, to which "
  "anyone can also donate voluntarily.")
H2("6.3 The work lottery")
P("So that people without TOY can also obtain new coins, there is a lottery for computing work. The device computes "
  "tickets with " + L("Argon2id", U["argon2"], "Technology") + ", which deliberately needs a lot of memory (16 MiB). "
  "This gives special hardware and graphics-card farms hardly any advantage over a phone or PC. Up to 64 winners are "
  "drawn fairly per block, each prize being at least 0.2 TOY. Whoever computes more has more tickets; the expected "
  "reward is proportional to the work. A single prize is enough to activate an account: 0.1 TOY are locked as a "
  "deposit (section 9) and the rest is freely available. The device only computes when allowed: on the charger, on Wi-Fi and while it "
  "stays cool. For comparison, the Bitcoin network continuously uses as much electricity as a whole country, as the "
  + L("Cambridge electricity index", U["cbeci"], "Energy") + " shows.")

# ================================================================ 7
H("Reaching agreement")
H2("7.1 How a block is made")
P("A committee of at most 128 validators is reassembled every epoch. For each block, verifiable randomness ("
  + L("VRF", U["vrf"], "Technology") + ") selects one member to propose the block. The others vote in two phases (pre-vote "
  "and confirmation), "
  "following a proven method in the style of " + L("Tendermint", U["tendermint"], "Consensus") + ". As soon as two "
  "thirds agree, the block is <b>final</b>. It can no longer be reverted. With Bitcoin, by contrast, one usually waits "
  "about an hour before a payment is considered safe. All votes are combined with "
  + L("BLS signatures", U["bls_wiki"], "Technology") + " into a single signature of only 96 bytes, which even a phone "
  "can check in milliseconds.")
FIG(F + "f_block.png", "How a block is made in five steps. After step 5 the payment is final.")
H2("7.2 Fair ordering")
P("Whoever builds a block cannot freely choose the order of transactions. It is fixed: by fee, and in case of a tie "
  "by a random number that is only known with the block. Every node checks this. Later, an inclusion duty will make "
  "sure that transactions seen by many validators really get into the next block. Because ToyChain has no exchange "
  "contracts, the " + L("“front-running” (MEV)", U["eth_mev"], "Consensus") + " known from Ethereum largely "
  "disappears.")
H2("7.3 Penalties that fit the damage")
P("Whoever casts two conflicting votes for the same block (double signing) loses part of their stake. This often "
  "happens by mistake, for example when the same key runs on two devices. That is why the penalty depends on how many "
  "do it at the same time. The software also protects against mishaps: it permanently remembers what it has signed "
  "and stays silent after a device change if its key is already active elsewhere. In the bootstrap phase the cut is "
  "limited to 5 %.")
FIG(F + "f_slashing.png", "Cut for double signing: max(1 %, 3 × share affected at the same time), at most 100 %.")

# ================================================================ 8
H("The bootstrap phase")
P("A fair launch without premint has a chicken-and-egg problem under proof of stake: at the start nobody has TOY, "
  "so nobody has stake to secure the network. ToyChain solves this openly and for a limited time:")
B(["<b>21 start validators</b>, run by volunteers with operators and locations as diverse as possible, secure the "
   "network with equal weight. Their names and earnings are public in ToyScan.",
   "<b>Falling reward:</b> At first they jointly receive half of all new TOY. The share falls evenly day by day and "
   "reaches zero after 180 days. Days are counted in <i>plan time</i>: every pause between two blocks counts at most "
   "10 minutes, so a standstill cannot move the clock forward. A cap prevents faster issuance or deliberately slow blocks from bringing more reward. In "
   "total it is at most about 2.22 million TOY (about 2.2 % of the maximum supply).",
   "<b>Bonded reward:</b> Every reward stays locked for 365 epochs (about one year) and is liable for misconduct.",
   "<b>Strict handover:</b> The bootstrap phase ends no earlier than after 90 days, and only when at least 21 new "
   "validators with 1,000 TOY each have been bonded for 30 epochs and together hold two thirds of the effective stake. "
   "After that the start validators lose any special status. If the handover has not happened after 180 days, they "
   "keep securing the network without reward until it does."])
FIG(F + "f_bootstrap.png", "Start validators' reward by day and the time window for the handover.")
P("To be honest: the bootstrap phase is the part in which ToyChain needs the most trust in a few people. No protocol "
  "can prove that “new” validators really are other people. The rules make a takeover expensive and visible to all, "
  "though.")

# ================================================================ 9
H("Lean nodes for old devices")
P("Storing a full Bitcoin or Ethereum chain today takes hundreds of gigabytes. ToyChain sets a fixed target of "
  "<b>256 MB per standard node</b>. That works because a node stores only the current balances of all active accounts "
  "and a short window of the latest blocks. Volunteer archive nodes keep older blocks. In a later phase the whole "
  "history is to be “folded” into a small cryptographic proof, so that new nodes do not have to trust anyone either. Until "
  "then, new nodes start from snapshots (checkpoints) signed by at least three independent parties.")
FIG(F + "f_budget.png", "How the 256 MB are divided. Every storage structure gets its own size test.")
P("So that 100 million accounts and more still fit, there are <b>active and dormant accounts</b>. Accounts unused for "
  "a long time are moved out of memory when needed, but <b>never deleted</b>. Their content is secured by a "
  + L("Merkle proof", U["merkle"]) + " of just under 1 KB that the wallet keeps itself. With the next transaction the "
  "account is woken up automatically, with its full balance.")
FIG(F + "f_dormant.png", "Active and dormant accounts. No account is ever lost.", 15.0)
P("So that nobody fills the memory with millions of empty accounts, 0.1 TOY are locked as an <b>activation "
  "deposit</b> on the first incoming transfer. The deposit stays the user's property, is never cut and is returned "
  "in full when the account is closed. It may only be lowered later, down to 0.01 TOY at most.")

# ================================================================ 10
H("Security for users")
H2("10.1 Voluntary account protection")
P("Everyone can protect their own account further. These settings apply only because the owner set them. Nobody else "
  "can set them.")
B(["<b>Vault delay:</b> Larger withdrawals only take effect after a waiting time and can be cancelled during it. A "
   "thief therefore cannot empty the account at once.",
   "<b>Emergency key and safety address:</b> A second key kept separately cannot spend anything, but it can stop "
   "pending withdrawals and rescue everything to a safe address set in advance.",
   "<b>Self-lock:</b> Lost your phone? Your own account can be locked for up to 30 epochs.",
   "<b>Heir address:</b> If an account stays without any activity for a very long time (at least 180 epochs), an heir "
   "address chosen by the owner can take it over. The owner is reminded several times beforehand and can still cancel "
   "for 30 epochs."])
P("To be honest: whoever steals the phone together with the key has the same rights as the owner unless there is an "
  "emergency key. The wallet therefore recommends creating an emergency key on a second device or on paper.")
FIG(F + "f_vault.png", "Vault delay: during the waiting time the owner can stop a suspicious withdrawal.", 15.0)
H2("10.2 Protection rules with a sense of proportion")
P("Attacks should be stopped early without hitting honest users. That is why there are three levels. The chain itself "
  "only punishes <b>clearly provable</b> violations, such as double signing or a forged block. The consequence is a "
  "ban that removes rights, for example the free quota, but <b>never funds</b>. Repeated violations double the ban, up "
  "to at most 64 epochs.")
FIG(F + "f_levels.png", "The three levels of protection rules. Only level A acts on the chain, and only on rights.", 15.0)
H2("10.3 The constitution")
P("Some rules are so fundamental that they are fixed in the code and may not be changed by any update or later vote: "
  "the maximum of 100 million TOY, the launch without balances, the issuance formula, the ban on destroying TOY, the "
  "minimum length of an epoch, the absence of tokens and smart contracts and, above all: <b>No rule may take, freeze or "
  "redistribute TOY from an account.</b> To be honest, anyone distributing their own software can split off a chain. "
  "That is why the wallet and ToyScan check the constitution themselves and warn if a split breaks it. Every software "
  "version is to be built " + L("reproducibly", U["repro"], "Technology") + ", so that anyone can check that it really "
  "comes from the published code.")
H2("10.4 Proven cryptography")
P("ToyChain does not invent its own cryptographic methods. It uses widely used, well-reviewed standards: secp256k1 "
  "and Keccak-256 as on Ethereum for accounts, " + L("BIP-39", U["bip39"]) + " for the word phrase, "
  + L("BLS12-381", U["bls"], "Technology") + " for validators' joint signatures, BLAKE3 for checksums, Argon2id for "
  "computing work and for encrypting the wallet on the device. Every signature contains the chain identifier ("
  + L("EIP-155", U["eip155"], "Accounts and keys") + "), so it cannot be reused on another chain. All methods sit "
  "behind replaceable interfaces. A voluntary additional key following the NIST standard "
  + L("ML-DSA", U["mldsa"], "Technology") + " is meant to resist " + L("quantum computers", U["pqk"], "Technology") +
  " as well. The most important calculation rules are also checked mathematically with the proof tool "
  + L("Kani", U["kani"], "Technology") + ", that is not just for examples but for all possible numbers. Already proven: "
  "an epoch never pays out more than the remaining amount, and the total of all TOY always stays at or below 100 "
  "million (section 17).")

# ================================================================ 11
H("TOYPAY wallet, ToyScan and other chains")
P("The <b>TOYPAY wallet</b> is wallet and node in one. It runs on Android from about 1 GB of RAM, on Windows, Linux "
  "and the Raspberry Pi. An energy mode automatically regulates how much the device contributes, depending on battery, "
  "charger, temperature and network. The interface is planned in 45 languages and has four modes from beginner to "
  "expert. Security warnings appear in every mode.")
P("The explorer <b>ToyScan</b> makes everything publicly visible: how many TOY have been issued, the current factor "
  "m, the state of the bootstrap phase and every ban with its reason. A traffic light (green, yellow, red) summarises "
  "nine criteria, such as whether the maximum supply is respected, whether stake is too concentrated or whether the "
  "bootstrap phase is taking too long. ToyScan never has access to keys.")
P("Because the addresses are the same as on Ethereum, the wallet and ToyScan also show balances of the same address "
  "on other EVM chains such as Ethereum or Polygon. Well-known wallets such as MetaMask can "
  + L("add ToyChain as a network", U["metamask"], "Wallet") + ". There is deliberately no "
  + L("bridge", U["eth_bridges"], "Other chains") + " between chains: in 2022, according to Chainalysis, about two "
  "thirds of all stolen crypto funds came from " + L("attacks on bridges", U["bridgehacks"], "Other chains") + ". "
  "Instead an " + L("atomic swap", U["atomic"], "Other chains") + " is planned: two people trade directly, secured by a "
  + L("hash time lock", U["htlc"], "Other chains") + ". Either both receive their counterpart, or both get their money "
  "back.")
FIG(F + "f_swap.png", "Atomic swap between TOY and a coin on another chain, without a middleman.")
BOX("<b>Important note on tokens with the same name:</b> On Polygon and Ethereum there are older tokens named "
    "ToyCoin with the symbol TOY. They are <b>not</b> connected to the native TOY on ToyChain and cannot be exchanged "
    "for it. The wallet and ToyScan mark this clearly.")

# ================================================================ 12
H("Use cases")
B(["<b>Everyday payments</b> between individuals, with a payment reference and finality after about one minute, later "
   "also as batch transfers to up to 32 recipients. Small amounts are possible without fees thanks to the free quota.",
   "<b>Taking part without capital:</b> An old phone or a Raspberry Pi becomes a node and can earn its first TOY "
   "through the work lottery.",
   "<b>Clubs, families and small businesses:</b> accounts with several signatures, escrow with an arbiter, "
   "subscription permissions with a fixed limit and session keys for apps (after the first testnet).",
   "<b>Provision for the future:</b> vault delay, emergency key and heir address protect against theft and loss.",
   "<b>Donations for everyone:</b> anyone can donate to the refund pool that pays for everyone's free transactions.",
   "<b>Privacy when needed:</b> one-time receiving addresses following " + L("EIP-5564", U["eip5564"],
                                                                              "Accounts and keys") +
   " hide who was paid (planned for milestone M13).",
   "<b>Learning and research:</b> open code, a one-command testnet and public data make ToyChain a learning object for "
   "schools, universities and anyone interested.",
   "<b>Conceivable later:</b> merchant integrations, a point-of-sale mode, offline payments and a link to the game "
   "TOYQUEST. These ideas are explicitly not part of the first phase."])

# ================================================================ 13
H("Similarities and differences")
T([["", "Bitcoin", "Ethereum", "ToyChain (planned)"],
   ["Launch", "fair, no premint", "presale and foundation share", "fair, genesis without any balance"],
   ["Maximum supply", "about 21 million BTC", "no fixed cap", "100 million TOY, unchangeable"],
   ["Issuance", "halves about every 4 years", "as needed, part of the fees is burned",
    "declines smoothly, factor 0.5 to 2.0, never burned"],
   ["Secured by", "computing work (proof of work)", "stake (proof of stake), 32 ETH per validator",
    "stake in tiers of 10, 100, 1,000 TOY plus an open bootstrap phase"],
   ["Final after", "about 1 hour (6 confirmations)", "about 13 minutes", "1 block, about 1 minute"],
   ["Full node storage", "about 760 GB", "hundreds of GB to several TB", "target 256 MB"],
   ["Programmability", "simple scripts", "smart contracts, tokens, NFTs", "only built-in features"],
   ["Accounts", "UTXO model", "account model", "account model, Ethereum addresses"],
   ["Fees", "always, by market", "always, base fee plus tip", "10 to 100 free per epoch (min. 1 day), then fee or work"],
   ["Joining without funds", "only running a node", "only running a node", "yes, including the work lottery"]],
  [3.5, 3.6, 4.2, 5.1])
P("<b>In common with Bitcoin</b>, ToyChain has the fair launch, the fixed maximum supply, the declining issuance over "
  "about 25 years, the absence of a central authority and the open code. <b>In common with Ethereum</b> it has the "
  "addresses, word phrase, signature format, proof of stake and the idea of a base fee that rises under load. "
  "<b>New</b> is the combination of joining without funds, a free quota, a 256 MB storage limit, dormant accounts, "
  "voluntary account protection and a constitution that rules out interference with other people's funds.")

# ================================================================ 14
H("Advantages")
B(["<b>Fairness:</b> Nobody is given TOY; all rules are public and the same for everyone.",
   "<b>Accessibility:</b> Join with old devices, without funds and without expert knowledge, in 45 languages.",
   "<b>Fast finality:</b> A payment is safe after about one minute.",
   "<b>Low energy use:</b> No race for computing power. The work lottery only runs on the charger and only as much as "
   "the user allows. Computing more brings proportionally more tickets, no head start.",
   "<b>Simplicity as security:</b> Without smart contracts there are no third-party programs that could freeze funds or "
   "lose them through bugs.",
   "<b>Protection of property:</b> Bans remove only rights, never funds. Accounts are never lost. TOY are never "
   "destroyed.",
   "<b>Transparency:</b> ToyScan, monthly transparency reports from the testnet on, and reproducible software versions.",
   "<b>Compatibility:</b> Ethereum addresses and well-known wallets work."])

# ================================================================ 15
H("Risks and open questions")
P("An honest whitepaper also names what can go wrong. The main risks from today's point of view:")
B(["<b>Early stage:</b> ToyChain has not run yet. Errors in the concept or code are likely and are to be found on the "
   "testnet. Two reviews by an AI review agent not involved in writing the texts have already uncovered weaknesses "
   "that were fixed; a review by humans is still pending. Further reviews and an "
   "external audit are mandatory before any mainnet.",
   "<b>Small network, little security:</b> A new chain is vulnerable at first, because few participants are easier to "
   "outvote. Security only comes from many independent participants.",
   "<b>Trust in the bootstrap phase:</b> The 21 start validators carry a lot of responsibility in the first months and "
   "receive a limited reward. Whether the handover to new validators succeeds remains to be seen.",
   "<b>Storage limit:</b> Whether 256 MB is enough over the years depends on usage and on the later proof layer. If not, "
   "this will be presented openly with numbers. Larger nodes would then need more storage.",
   "<b>Less proven building blocks:</b> For some methods (such as verifiable randomness and the consensus library) "
   "there are fewer well-reviewed Rust libraries. They will be implemented strictly according to the standard and "
   "reviewed externally.",
   "<b>Computing work:</b> Whether 16 MiB of memory is enough to keep special hardware away will be measured on the "
   "testnet.",
   "<b>Law and regulation:</b> A cryptocurrency touches European law (" + L("MiCA", U["mica"], "Law") + "), tax law "
   "(for example the " + L("German tax guidance", U["bmf"], "Law") + ") and possibly gambling law (the term "
   "“lottery”). A legal review is mandatory before any mainnet. The heir address does not replace inheritance law.",
   "<b>User errors:</b> Whoever loses or reveals their word phrase loses their money. Account protection and warnings "
   "reduce this risk but cannot remove it entirely.",
   "<b>Value:</b> TOY have no guaranteed value. Whether there will ever be a market is open (section 16). Nobody should "
   "put in money they cannot afford to lose.",
   "<b>Confusion and fraud:</b> Older tokens with the same name on other chains can confuse users or be used for fraud. "
   "EU regulators warn in general about the " + L("risks of crypto assets", U["esas_warning"], "Consumer protection") +
   "."])
MORE(L("EU supervisors' warning (ESMA)", U["esas_warning"], "Consumer protection"),
     L("EU financial regulators warn consumers (EBA)", U["eba_warning"], "Consumer protection"))

# ================================================================ 16
H("Chances in the crypto market: an assessment")
P("This chapter attempts a sober assessment. It is not a forecast and not an investment recommendation. The market "
  "figures date from mid-October 2026 and change quickly.")
H2("16.1 The starting point")
P("On 18 October 2026 the entire crypto market was worth about USD 2.7 trillion, of which about 60 % was Bitcoin. "
  "Sentiment for smaller coins was weak: of the 100 largest crypto assets only 14 rose that day ("
  + L("market report of 18 Oct 2026", U["market"], "Market") + ", continuously updated at "
  + L("CoinGecko", U["cg_charts"], "Market") + "). At the same time the market is flooded with new projects. According "
  "to a CoinGecko analysis, more than 20 million crypto assets had been tracked by the end of 2025. In 2025 alone more "
  "than 11.6 million failed, more than half of all ever tracked in total ("
  + L("report on the CoinGecko study", U["dead"], "Market") + "). The vast majority of these are short-lived tokens on "
  "existing chains, not blockchains of their own. Still, the numbers show how hard it is to win attention and lasting "
  "users.")
FIG(F + "f_deadcoins.png", "Failed crypto assets per year according to CoinGecko data. Most new launches do not "
                           "survive.", 15.0)
P("In the EU the " + L("MiCA regulation", U["esma"], "Law") + " has applied without a transitional period since "
  "1 July 2026: providers of crypto services need authorisation, otherwise they must stop serving EU customers ("
  + L("notice by the French regulator AMF", U["amf"], "Law") + "). For ToyChain this means two things: exchanges and "
  "wallet providers that may later list TOY will check more closely. And for a public offer or admission to trading, "
  "MiCA usually requires a crypto-asset white paper following fixed rules. Whether and how this applies to a coin "
  "without an issuer such as TOY must be clarified legally before any mainnet. This document is <b>not</b> such a "
  "white paper.")
H2("16.2 What speaks for ToyChain")
B(["<b>Credible fairness:</b> A launch without any balance is rare. In a market where many projects start with large "
   "insider shares, this is a clear distinguishing feature that anyone can check in ToyScan.",
   "<b>Low barrier to entry:</b> Join with an old phone, without buying anything and without prior knowledge. This can "
   "reach people who would never run a node otherwise and creates many small participants instead of a few large ones.",
   "<b>Smaller attack surface:</b> Without smart contracts and bridges, the causes of many large losses of recent years "
   "disappear.",
   "<b>Compatibility:</b> Ethereum addresses lower the barrier for users who already have a wallet.",
   "<b>Openness and language:</b> A free licence, a traceable decision log (public from the testnet on) and 45 languages make the project accessible to "
   "an international community. This whitepaper already exists in German, English, French and Yoruba."])
H2("16.3 What speaks against it")
B(["<b>No start capital:</b> Without premint there is no budget for developers, audits, marketing or exchange listings. "
   "The project depends on volunteers, donations and grants.",
   "<b>Network effects:</b> Users, merchants and exchanges gather around large chains. A new chain must offer a concrete "
   "reason to switch.",
   "<b>Deliberately limited:</b> Without smart contracts there is no ecosystem of apps, tokens and decentralised "
   "exchanges, which attracts a lot of attention and capital on other chains.",
   "<b>Strong competition in payments:</b> For everyday payments many people now use stablecoins pegged to currencies, "
   "or established networks. A coin with a fluctuating value is less attractive for that.",
   "<b>Small team, early stage:</b> Twelve more milestones, audits and legal clarifications are needed before a mainnet.",
   "<b>Name:</b> Older tokens with the same name make the project harder to find and carry a risk of confusion."])
FIG(F + "f_swot.png", "Strengths, weaknesses, opportunities and threats at a glance.", 15.0)
H2("16.4 Scenarios")
T([["Scenario", "Description", "Assessment"],
   ["A · Failure before mainnet", "The testnet reveals fundamental problems or too few contributors are found.",
    "realistic, as for most projects"],
   ["B · Learning and community project", "A stable testnet and a small, active community; TOY are hardly traded.",
    "most likely if the work continues"],
   ["C · Small, real market", "Mainnet after audit, a few hundred to a thousand nodes, trading on a few platforms, use "
                              "in niches (game, clubs, donations).", "possible; needs community, audits and legal "
                                                                    "clarity"],
   ["D · Broad adoption", "One of the well-known coins with millions of users.", "very unlikely"]],
  [4.2, 7.6, 4.6])
H2("16.5 What would make the difference")
B(["<b>Independent start validators</b> from different countries who take part publicly.",
   "<b>An external security audit</b>, funded by donations or grant programmes for free software.",
   "<b>A concrete use case</b> that would work worse without ToyChain, such as the game TOYQUEST, where players take "
   "part with their phone and earn TOY.",
   "<b>Consistent transparency:</b> monthly reports, open measurements, an honest culture of admitting mistakes.",
   "<b>Legal clarity</b> before every step that touches real value."])
P("<b>Conclusion of the assessment:</b> As an investment, ToyChain cannot be valued today and, like almost every new "
  "cryptocurrency, carries a very high risk of loss. As a technical and community project, ToyChain has a clear, "
  "unusual profile: fair, lean, understandable and careful. Its realistic chance lies not in a quick price rise but in "
  "building a small, convinced community that runs the network itself. If that succeeds, more can grow from it over "
  "the years. It is not guaranteed.")

# ================================================================ 17
H("Status and roadmap")
P("<b>Today (19 October 2026): before the test phase.</b> The concept is recorded in 114 documented decisions. "
  "Construction is divided into 13 milestones. Milestone M1 is finished: the rules for amounts and for issuing new TOY "
  "are programmed and checked with 45 automated tests. The Rust implementation produces exactly the same numbers, down "
  "to the smallest unit, as the separately written reference simulation in Python. The tests run automatically on "
  "every change on Linux, Windows and ARM processors. With the proof tool Kani, three core rules are proven "
  "mathematically, each in under two minutes: no epoch pays out more than the remaining amount, the total stays at or "
  "below 100 million TOY, and the split of a block adds up exactly. Three further proofs of the split with a reward "
  "share had not finished after 30 minutes. The program therefore checks this split itself in every block, and "
  "additional tests cover it.")
FIG(F + "f_roadmap.png", "Roadmap. There is deliberately no fixed date for a mainnet.")
T([["Phase", "Content", "Status"],
   ["Concept", "Specification v0.3, decision log, two reviews", "done, maintained continuously"],
   ["Phase 1, M1 to M11", "Basic types and issuance, cryptography, accounts and transactions, consensus and bootstrap "
    "phase, storage, network, interfaces, wallet, platforms, testnet, ToyScan", "M1 done, M2 next"],
   ["Testnet 1", "Core plus account protection, 45 languages and four modes; first local, then public", "planned"],
   ["Phase 1, M12 and M13", "further transaction types, atomic swap, names, one-time addresses, post-quantum keys",
    "planned"],
   ["Phase 2", "Proof layer: fold the history, nodes without trust in checkpoints", "planned"],
   ["Afterwards", "Governance, mainnet (only after audit, load tests and legal review, date announced publicly in "
    "advance), possible bridges and offline payments", "open"]],
  [3.4, 9.4, 3.6])
P("There is no fixed date for a mainnet. It will only start once the testnet runs stably, security has been reviewed "
  "and the legal questions are settled. The start date will be announced publicly in advance so that everyone can "
  "begin at the same time.")

# ================================================================ 18
H("Conclusion")
P("ToyChain wants to show that a cryptocurrency can start fairly, get by with little storage and still be fast and "
  "secure. Everyone should be able to take part, even without money and with an old device. For this, ToyChain "
  "deliberately does without much of what other chains offer and relies on a few built-in, easily verifiable "
  "features. Whether this succeeds will be decided not by this paper but by the testnet. Contributions, criticism and "
  "reviews are explicitly welcome. Code, specification and decision log are on " + L("GitHub", U["repo"]) +
  " and will be visible to everyone by the public testnet at the latest.")

# ================================================================ Appendix
story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Appendix A: Key starting values", h1))
P("All values except those of the constitution are assumptions for the testnet and will be measured and adjusted "
  "there.")
T([["Area", "Starting value"],
   ["Supply and divisibility", "100,000,000 TOY, 18 decimal places (constitution)"],
   ["Issuance per epoch", "0.0505 % of the remaining amount times m; m between 0.5 and 2.0, change at most 0.05 per "
                          "epoch"],
   ["Split of new TOY", "50 % consensus, 40 % work lottery, 10 % refund pool"],
   ["Time", "block time 60 s, at the earliest 10 s; epoch at least 1,440 blocks and 24 hours"],
   ["Committee and blocks", "at most 128 validators, block at most 128 KB, final after 1 block"],
   ["Bootstrap phase", "21 start validators, reward from 50 % to 0 % in 180 days, bonded for 365 epochs, handover no "
                       "earlier than after 90 days"],
   ["Stake", "10 / 100 / 1,000 TOY; release after 21 epochs; cap 5 %, from 100 validators 1 %"],
   ["Double-signing penalty", "max(1 %, 3 × share affected at the same time), at most 100 %, role banned for 7 "
                              "epochs"],
   ["Accounts", "activation deposit 0.1 TOY (may drop to 0.01 TOY); dormant accounts moved out from 90 % fill level"],
   ["Quota", "10 to 100 free transactions per account and epoch, from an account age of 100 blocks"],
   ["Work lottery", "Argon2id with 16 MiB; up to 64 winners per block, prize at least 0.2 TOY"],
   ["Bans", "only for clearly provable violations, doubling on repetition up to at most 64 epochs"],
   ["Storage", "256 MB per standard node: 128 MB state, 48 MB block window, 80 MB proofs, indexes and reserve"]],
  [4.0, 12.4])

story.append(CondPageBreak(6 * cm))
story.append(Paragraph("Appendix B: Link list", h1))
P("The most important explanations for further reading, sorted by topic. Links as of October 2026; the project link works once the repository is public.")
T([["Topic", "Link", "What it covers"],
   ["Basics", L("Bitcoin whitepaper", U["btc_pdf"]), "the original from 2008, 9 pages"],
   ["Basics", L("Bitcoin full node 2026 (Spark)", U["btc_size"]), "storage and cost of a node"],
   ["Money supply", L("Controlled supply (Bitcoin Wiki)", U["btc_supply"]), "how Bitcoin halves its issuance"],
   ["Consensus", L("Consensus mechanisms (ethereum.org)", U["eth_consensus"]), "overview"],
   ["Consensus", L("Proof of stake (ethereum.org)", U["eth_pos"]), "stake, validators, finality"],
   ["Consensus", L("The latest gossip on BFT consensus", U["tendermint"]), "paper on the Tendermint method"],
   ["Consensus", L("MEV (ethereum.org)", U["eth_mev"]), "what “front-running” means on Ethereum"],
   ["Technology", L("BLS12-381 For The Rest Of Us", U["bls"]), "joint signatures"],
   ["Technology", L("Merkle tree (Wikipedia)", U["merkle"]), "proofs for dormant accounts"],
   ["Technology", L("RFC 9106: Argon2", U["argon2"]), "standard for the computing work"],
   ["Technology", L("RFC 9381: VRF", U["vrf"]), "verifiable randomness"],
   ["Technology", L("Dandelion++", U["dandelion"]), "protecting the sender's IP"],
   ["Technology", L("libp2p", U["libp2p"]), "networking library"],
   ["Technology", L("Post-quantum cryptography (IEEE Spectrum)", U["pqk"]), "introduction"],
   ["Technology", L("NIST FIPS 204 (ML-DSA)", U["mldsa"]), "post-quantum signatures"],
   ["Technology", L("Kani (Ferrous Systems)", U["kani"]), "mathematical checking of Rust code"],
   ["Technology", L("Reproducible Builds", U["repro"]), "verifiable software versions"],
   ["Accounts", L("BIP-39", U["bip39"]), "standard for the 12/24-word phrase"],
   ["Accounts", L("EIP-155", U["eip155"]), "chain identifier in every signature"],
   ["Accounts", L("EIP-5564 explained (QuickNode)", U["eip5564"]), "one-time receiving addresses"],
   ["Fees", L("EIP-1559", U["eip1559"]), "base fee on Ethereum"],
   ["Wallet", L("Add a network in MetaMask", U["metamask"]), "adding custom networks"],
   ["Other chains", L("Bridges (ethereum.org)", U["eth_bridges"]), "how bridges work, risks"],
   ["Other chains", L("Bridge hacks 2022 (Decrypt)", U["bridgehacks"]), "Chainalysis figures on losses"],
   ["Other chains", L("Atomic swap (Blockstream)", U["atomic"]), "trading without a middleman"],
   ["Other chains", L("Hashed Timelock Contracts", U["htlc"]), "the time lock behind it"],
   ["Energy", L("Cambridge Bitcoin Electricity Index", U["cbeci"]), "Bitcoin's power consumption"],
   ["Market", L("CoinGecko charts", U["cg_charts"]), "current market value"],
   ["Market", L("Market report 18 Oct 2026 (DailyForex)", U["market"]), "status on that date"],
   ["Market", L("CoinGecko study on failed coins (ForkLog)", U["dead"]), "figures 2021 to 2025"],
   ["Law", L("MiCA regulation (EUR-Lex)", U["mica"]), "official text"],
   ["Law", L("MiCA rulebook (ESMA)", U["esma"]), "rules and guidelines"],
   ["Law", L("End of the MiCA transition (AMF)", U["amf"]), "what applies from 1 July 2026"],
   ["Law", L("German tax guidance on crypto assets (BMF)", U["bmf"]) + " (German)", "taxes in Germany"],
   ["Consumer protection", L("EU supervisors' warning (ESMA)", U["esas_warning"]), "risks and limited protection"],
   ["Consumer protection", L("EU regulators warn consumers (EBA)", U["eba_warning"]), "joint warning"],
   ["Project", L("github.com/TOYfoundation/toycoin", U["repo"]), "code, specification, decisions"]],
  [3.0, 7.6, 5.8])

story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Appendix C: Sources and standards", h1))
refs = [
    "S. Nakamoto: Bitcoin: A Peer-to-Peer Electronic Cash System, 2008.",
    "E. Buchman, J. Kwon, Z. Milosevic: The latest gossip on BFT consensus (Tendermint), 2018.",
    "BIP-39: Mnemonic code for generating deterministic keys. EIP-155: Simple replay attack protection. EIP-1559: Fee "
    "market change. EIP-2718: Typed transaction envelope. EIP-5564: Stealth addresses.",
    "D. Boneh, B. Lynn, H. Shacham: Short signatures from the Weil pairing (BLS), 2001; BLS12-381.",
    "RFC 9106: Argon2 memory-hard function. RFC 9381: Verifiable Random Functions (VRFs).",
    "G. Fanti et al.: Dandelion++: Lightweight cryptocurrency networking with formal anonymity guarantees, 2018.",
    "NIST FIPS 204: Module-Lattice-Based Digital Signature Standard (ML-DSA), 2024.",
    "Regulation (EU) 2023/1114 on markets in crypto-assets (MiCA).",
    "Market data: DailyForex, Crypto Overview, 18 Oct 2026; CoinGecko data via ForkLog, January 2026; Chainalysis via "
    "Decrypt, 2022; Spark, Bitcoin Full Node Requirements 2026.",
    "ToyChain: technical specification v0.3, decision log, build plan phase 1, reviews 1 and 2, reference simulation "
    "m-simulation.py; github.com/TOYfoundation/toycoin (German).",
]
for i, r in enumerate(refs, 1):
    story.append(Paragraph(f"[{i}] {r}", small))
story.append(CondPageBreak(5 * cm))
story.append(Paragraph("Appendix D: Changes since draft 0.2", h1))
P("Draft 0.3 is a review of draft 0.2 for logic, traceability and new findings from 18 and 19 October 2026. The "
  "concept itself has not changed.")
T([["Where", "Change", "Reason"],
   ["1, 18, App. B", "The repository is private for now; it is to be opened by the public testnet at the latest.",
    "Draft 0.2 wrongly called it public."],
   ["2, 13", "Free quota “per epoch (at least one day)” instead of “per day”.", "more precise: an epoch can last longer "
                                                                              "than 24 hours."],
   ["5.3", "Deviation from plan: first year at most about 2.2 %, long run at most about 2.5 %.",
    "“at most about 2 %” was too tight (simulation: +2.23 %)."],
   ["6.1", "Reward after the bootstrap phase fully described: one fifth as an equal base amount per committee member, "
           "the rest linear, both by actual work done.", "was missing, decision 112"],
   ["6.3", "A first lottery prize is enough for the activation deposit.", "answers how to get a first account without TOY"],
   ["7.1", "“two phases” (pre-vote and confirmation) instead of “two rounds”.", "“round” means something else in the "
                                                                              "protocol."],
   ["8", "Plan time explained; reward as “half of all new TOY”; what happens if the handover is still pending after "
         "180 days.", "figure 8 used the term without explanation; gap in the process"],
   ["9", "New nodes start from checkpoints with three independent signatures until phase 2.",
    "gap: whom does a new node trust until then?"],
   ["10.1", "Heir address cancellation period (30 epochs) and an honest note on the emergency key.",
    "completeness and honesty"],
   ["10.4, 17", "Result of the Kani proofs: three core rules proven, three proofs still open.", "new finding, 18 Oct 2026"],
   ["12", "Batch transfers only after the first testnet.", "scope of testnet 1 (decision 109)"],
   ["14", "Energy: lottery only on the charger and only as much as the user allows.",
    "“when power is available anyway” was imprecise."],
   ["15", "The reviews come from an AI review agent; a human review is still pending.",
    "“independent reviews” was misleading."],
   ["16.2", "Decision log public only from the testnet on; whitepaper in four languages.", "correction and new status"]],
  [2.4, 8.6, 5.4])
story.append(Spacer(1, 6))
story.append(Paragraph("<b>Disclaimer.</b> This document describes a software project under development. It is "
                       "neither investment, tax nor legal advice, and not an offer, prospectus or crypto-asset white "
                       "paper within the meaning of the MiCA regulation. All information is preliminary and may change. "
                       "Market figures reflect the status on the stated date. If in doubt, the German edition prevails.",
                       small))

finish("ToyChain-Whitepaper-v0.3-en.pdf", "ToyChain · Preliminary whitepaper · Draft 0.3 · before the test phase",
       "Page", "ToyChain and ToyCoin (TOY): Preliminary whitepaper", "Draft 0.3, before the test phase", "en-GB")
