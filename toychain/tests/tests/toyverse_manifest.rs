//! The shared ToyVerse manifest `shared/toyverse.json`.
//!
//! Website, ToyPost, ToySafe, TOYPAY and ToyScan are written in other
//! languages (TypeScript, JavaScript, GDScript, Python, Kotlin) and must not
//! keep their own copies of protocol values. They read this one file. This
//! test builds the manifest from the Rust constants and fails if the
//! committed file differs, so the file can never drift from the node.
//!
//! Regenerate after a deliberate change:
//! `UPDATE_TOYVERSE_JSON=1 cargo test -p toy-integration-tests --test toyverse_manifest`
//!
//! Amounts are decimal strings in smallest units: they exceed 2^53, so
//! JavaScript numbers and Godot integers cannot hold them (use BigInt,
//! `BigInteger` or arbitrary-precision integers).

use std::fmt::Write as _;
use std::path::PathBuf;

use toy_tx::transaction::kind;
use toy_types::params;

/// Old ERC-20 tokens named ToyCoin. They are **not** ToyChain TOY and are
/// never exchanged (decision 63); every app shows them only as a warning.
const LEGACY_TOKENS: [(u64, &str, &str); 2] = [
    (1, "Ethereum", "0xA5B04ce6eDD139C0e58ccdBc0B75E8C67CDA678A"),
    (137, "Polygon", "0x1854b69cd1495c9753c5F1590A35e5DdB556A5BF"),
];

fn repo_root() -> PathBuf {
    // toychain/tests -> repository root
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn language_count() -> usize {
    // A missing file counts as 0 languages and fails the tests below.
    let tsv = std::fs::read_to_string(repo_root().join("i18n/sprachen.tsv")).unwrap_or_default();
    tsv.lines().skip(1).filter(|l| !l.trim().is_empty()).count()
}

/// Builds the manifest. Hand-written JSON keeps the crate free of extra
/// dependencies; the key order is fixed so diffs stay small.
fn manifest() -> String {
    let p = |ppm: toy_types::Ppm| ppm.get();
    let u = |a: toy_types::Amount| format!("\"{}\"", a.units());
    let mut s = String::new();
    let mut line = |text: String| {
        s.push_str(&text);
        s.push('\n');
    };
    line("{".into());
    line("  \"schema\": \"toyverse/1\",".into());
    line(
        "  \"generatedBy\": \"toychain/tests/tests/toyverse_manifest.rs (do not edit by hand)\","
            .into(),
    );
    line("  \"toy\": {".into());
    line("    \"ticker\": \"TOY\",".into());
    line(format!("    \"decimals\": {},", params::DECIMALS));
    line(format!(
        "    \"unitsPerToy\": \"{}\",",
        params::UNITS_PER_TOY
    ));
    line(format!(
        "    \"maxSupplyUnits\": {},",
        u(params::MAX_SUPPLY)
    ));
    line("    \"genesisBalanceUnits\": \"0\"".into());
    line("  },".into());
    line("  \"chainIds\": {".into());
    line(format!("    \"mainnet\": {},", params::CHAIN_ID_MAINNET));
    line(format!("    \"testnet\": {},", params::CHAIN_ID_TESTNET));
    line(format!("    \"devnet\": {}", params::CHAIN_ID_DEVNET));
    line("  },".into());
    line("  \"blocks\": {".into());
    line(format!(
        "    \"targetSeconds\": {},",
        params::TARGET_BLOCK_SECONDS
    ));
    line(format!(
        "    \"minGapSeconds\": {},",
        params::MIN_BLOCK_GAP_SECONDS
    ));
    line(format!("    \"maxBytes\": {},", params::MAX_BLOCK_BYTES));
    line(format!("    \"memoMaxBytes\": {},", params::MEMO_MAX_BYTES));
    line(format!(
        "    \"maxExpiryBlocks\": {},",
        params::MAX_EXPIRY_BLOCKS
    ));
    line(format!("    \"committeeMax\": {}", params::COMMITTEE_MAX));
    line("  },".into());
    line("  \"epoch\": {".into());
    line(format!("    \"minBlocks\": {},", params::EPOCH_MIN_BLOCKS));
    line(format!("    \"minSeconds\": {}", params::EPOCH_MIN_SECONDS));
    line("  },".into());
    line("  \"emission\": {".into());
    line(format!("    \"k0Ppm\": {},", p(params::K0)));
    line(format!("    \"factorMinPpm\": {},", p(params::FACTOR_MIN)));
    line(format!("    \"factorMaxPpm\": {},", p(params::FACTOR_MAX)));
    line(format!(
        "    \"factorMaxStepPpm\": {},",
        p(params::FACTOR_MAX_STEP)
    ));
    line(format!(
        "    \"consensusSharePpm\": {},",
        p(params::CONSENSUS_SHARE)
    ));
    line(format!("    \"workSharePpm\": {},", p(params::WORK_SHARE)));
    line(format!(
        "    \"refundPoolSharePpm\": {}",
        p(params::REFUND_POOL_SHARE)
    ));
    line("  },".into());
    line("  \"fees\": {".into());
    line(format!(
        "    \"lotterySharePpm\": {},",
        p(params::FEE_SHARE_LOTTERY)
    ));
    line(format!(
        "    \"proposerSharePpm\": {},",
        p(params::FEE_SHARE_PROPOSER)
    ));
    line(format!(
        "    \"progressiveStep\": {},",
        params::PROGRESSIVE_FEE_STEP
    ));
    line(format!(
        "    \"progressiveMaxFactor\": {}",
        1u64 << params::PROGRESSIVE_FEE_MAX_DOUBLINGS
    ));
    line("  },".into());
    line("  \"quota\": {".into());
    line(format!("    \"min\": {},", params::QUOTA_MIN));
    line(format!("    \"max\": {},", params::QUOTA_MAX));
    line(format!(
        "    \"minAccountAgeBlocks\": {}",
        params::QUOTA_MIN_ACCOUNT_AGE_BLOCKS
    ));
    line("  },".into());
    line("  \"accounts\": {".into());
    line(format!(
        "    \"activationDepositUnits\": {},",
        u(params::ACTIVATION_DEPOSIT)
    ));
    line(format!(
        "    \"activationDepositFloorUnits\": {},",
        u(params::ACTIVATION_DEPOSIT_FLOOR)
    ));
    line(format!(
        "    \"unbondingEpochs\": {}",
        params::UNBONDING_EPOCHS
    ));
    line("  },".into());
    line("  \"stakeTiersUnits\": {".into());
    line("    \"0\": \"0\",".into());
    line(format!("    \"2\": {},", u(params::STAKE_TIER_2)));
    line(format!("    \"3\": {},", u(params::STAKE_TIER_3)));
    line(format!("    \"4\": {}", u(params::STAKE_TIER_4)));
    line("  },".into());
    line("  \"toyWork\": {".into());
    line("    \"algorithm\": \"argon2id\",".into());
    line(format!(
        "    \"memoryKib\": {},",
        params::TOYWORK_MEMORY_KIB
    ));
    line(format!(
        "    \"iterations\": {},",
        params::TOYWORK_ITERATIONS
    ));
    line(format!(
        "    \"parallelism\": {},",
        params::TOYWORK_PARALLELISM
    ));
    line(format!(
        "    \"lotteryMinPrizeUnits\": {}",
        u(params::LOTTERY_MIN_PRIZE)
    ));
    line("  },".into());
    let kinds = [
        ("transfer", kind::TRANSFER),
        ("bond", kind::BOND),
        ("unbond", kind::UNBOND),
        ("registerValidator", kind::REGISTER_VALIDATOR),
        ("deregisterValidator", kind::DEREGISTER_VALIDATOR),
        ("donate", kind::DONATE),
        ("closeAccount", kind::CLOSE_ACCOUNT),
        ("reportDoubleSign", kind::REPORT_DOUBLE_SIGN),
    ];
    let kinds: Vec<String> = kinds
        .iter()
        .map(|(name, byte)| format!("    \"{name}\": \"0x{byte:02x}\""))
        .collect();
    line("  \"txKinds\": {".into());
    line(kinds.join(",\n"));
    line("  },".into());
    let tokens: Vec<String> = LEGACY_TOKENS
        .iter()
        .map(|(chain_id, chain, address)| {
            format!(
                "    {{ \"chainId\": {chain_id}, \"chain\": \"{chain}\", \"address\": \"{address}\", \"isToyChainToy\": false }}"
            )
        })
        .collect();
    line("  \"legacyTokens\": [".into());
    line(tokens.join(",\n"));
    line("  ],".into());
    line("  \"i18n\": {".into());
    line(format!("    \"languages\": {},", language_count()));
    line("    \"languageFile\": \"i18n/sprachen.tsv\",".into());
    line("    \"glossaryFile\": \"i18n/begriffe.tsv\"".into());
    line("  },".into());
    line("  \"rules\": {".into());
    line("    \"onlyNativeCoin\": true,".into());
    line("    \"burnsToy\": false,".into());
    line("    \"accountsExpire\": false,".into());
    line("    \"premint\": false,".into());
    line("    \"appsNeverAskForSeedPhrase\": true".into());
    line("  }".into());
    line("}".into());
    s
}

#[test]
fn shared_manifest_matches_rust_constants() {
    let path = repo_root().join("shared/toyverse.json");
    let expected = manifest();
    if std::env::var_os("UPDATE_TOYVERSE_JSON").is_some() {
        std::fs::write(&path, &expected).expect("write shared/toyverse.json");
    }
    let committed = std::fs::read_to_string(&path).expect("shared/toyverse.json is missing");
    assert_eq!(
        committed.replace("\r\n", "\n"),
        expected,
        "shared/toyverse.json is out of date; regenerate it with UPDATE_TOYVERSE_JSON=1 (see module docs)"
    );
}

#[test]
fn manifest_values_follow_the_fixed_rules() {
    // Constitution values that every consumer relies on (CLAUDE.md).
    assert_eq!(params::DECIMALS, 18);
    assert_eq!(params::MAX_SUPPLY.units(), 10u128.pow(26));
    assert_eq!(language_count(), 45, "decision 103: 45 languages");
    // Amounts beyond 2^53 must be strings in the manifest.
    let m = manifest();
    assert!(m.contains("\"maxSupplyUnits\": \"100000000000000000000000000\""));
    let mut ids = String::new();
    write!(ids, "{}", params::CHAIN_ID_MAINNET).expect("write");
    assert_eq!(ids, "7630713");
}
