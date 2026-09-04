use crate::formatting::{percent, whole_currency};
use crate::{
    AccountRecord, AssetRecord, ConnectionRecord, MarketRecord, SecurityEventRecord, SignerRecord,
    TransactionRecord,
};

pub fn assets() -> Vec<AssetRecord> {
    [
        (
            "ETH", "Ethereum", "124.42", 341_228, 42_453_600, 241, 328, "Ethereum", 0,
        ),
        (
            "BTC", "Bitcoin", "4.82", 6_411_200, 30_902_000, 118, 241, "Bitcoin", 1,
        ),
        (
            "USDC", "USD Coin", "240,000", 100, 24_000_000, 0, 187, "Ethereum", 2,
        ),
        (
            "SOL", "Solana", "1,284.65", 14_200, 18_242_000, 202, 142, "Solana", 3,
        ),
        (
            "LINK",
            "Chainlink",
            "7,379.92",
            1_250,
            9_224_900,
            34,
            64,
            "Ethereum",
            4,
        ),
    ]
    .into_iter()
    .map(
        |(symbol, name, balance, price, value, change, allocation, network, tone)| AssetRecord {
            symbol: symbol.into(),
            name: name.into(),
            balance: balance.into(),
            price: whole_currency(price).into(),
            value: whole_currency(value).into(),
            change: percent(change).into(),
            allocation: format!("{}.{:01}%", allocation / 10, allocation % 10).into(),
            allocation_value: allocation,
            network: network.into(),
            tone,
        },
    )
    .collect()
}

fn numeric_magnitude(value: &str) -> u64 {
    value
        .chars()
        .filter(|character| character.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or_default()
}

pub fn sorted_assets(sort_key: i32, ascending: bool) -> Vec<AssetRecord> {
    let mut records = assets();
    records.sort_by(|left, right| match sort_key {
        0 => left.symbol.as_str().cmp(right.symbol.as_str()),
        2 => numeric_magnitude(left.change.as_str()).cmp(&numeric_magnitude(right.change.as_str())),
        _ => numeric_magnitude(left.value.as_str()).cmp(&numeric_magnitude(right.value.as_str())),
    });
    if !ascending {
        records.reverse();
    }
    records
}

pub fn transactions() -> Vec<TransactionRecord> {
    [
        (
            "Send",
            "USDC",
            "24,000.00",
            "TO",
            "Demo Settlement",
            "demo:account:0001",
            "Ethereum",
            "Confirmed",
            "13:42:12",
            "tx_demo_0001",
            0,
        ),
        (
            "Receive",
            "ETH",
            "8.4200",
            "FROM",
            "Demo Custody",
            "demo:account:0002",
            "Ethereum",
            "Confirmed",
            "12:18:04",
            "tx_demo_0002",
            0,
        ),
        (
            "Swap",
            "BTC",
            "0.8240",
            "VIA",
            "Sample Router",
            "demo:account:0003",
            "Bitcoin",
            "Pending",
            "11:54:21",
            "tx_demo_0003",
            1,
        ),
        (
            "Bridge",
            "SOL",
            "420.00",
            "TO",
            "Demo Operations",
            "demo:account:0004",
            "Solana",
            "Confirmed",
            "10:09:36",
            "tx_demo_0004",
            0,
        ),
        (
            "Send",
            "LINK",
            "220.00",
            "TO",
            "Demo Cold Storage",
            "demo:account:0005",
            "Ethereum",
            "Failed",
            "09:48:02",
            "tx_demo_0005",
            2,
        ),
    ]
    .into_iter()
    .map(
        |(kind, asset, amount, direction, party, address, network, status, time, hash, tone)| {
            TransactionRecord {
                kind: kind.into(),
                asset: asset.into(),
                amount: amount.into(),
                direction: direction.into(),
                party: party.into(),
                address: address.into(),
                network: network.into(),
                status: status.into(),
                time: time.into(),
                hash: hash.into(),
                tone,
            }
        },
    )
    .collect()
}

pub fn markets() -> Vec<MarketRecord> {
    [
        (
            "ETH / USD",
            "$3,412.28",
            "+2.41%",
            "$3,448.20",
            "$3,318.04",
            "$18.42B",
            0,
        ),
        (
            "BTC / USD",
            "$64,112.00",
            "+1.18%",
            "$64,882.12",
            "$62,940.40",
            "$28.16B",
            1,
        ),
        (
            "LINK / USD",
            "$12.50",
            "+0.34%",
            "$12.72",
            "$12.18",
            "$428.0M",
            4,
        ),
        (
            "SOL / USD",
            "$142.00",
            "+2.02%",
            "$145.18",
            "$137.84",
            "$3.82B",
            3,
        ),
    ]
    .into_iter()
    .map(
        |(pair, price, change, high, low, volume, tone)| MarketRecord {
            pair: pair.into(),
            price: price.into(),
            change: change.into(),
            high: high.into(),
            low: low.into(),
            volume: volume.into(),
            tone,
        },
    )
    .collect()
}

pub fn accounts() -> Vec<AccountRecord> {
    [
        ("Demo Treasury", "$842,240", "TR—001", "65.6%"),
        ("Operations", "$224,112", "OP—014", "17.4%"),
        ("Custody", "$118,012", "CU—002", "9.2%"),
        ("Settlement", "$100,056", "ST—008", "7.8%"),
    ]
    .into_iter()
    .map(|(name, value, code, share)| AccountRecord {
        name: name.into(),
        value: value.into(),
        code: code.into(),
        share: share.into(),
    })
    .collect()
}

pub fn connections() -> Vec<ConnectionRecord> {
    [
        (
            "Northstar Vault",
            "Demo Custody",
            "Sample Treasury",
            "$842,240",
            "Connected",
            "Just now",
            0,
        ),
        (
            "Harbor Custody",
            "Demo Custody",
            "Sample Cold Storage",
            "$118,012",
            "Connected",
            "2 min ago",
            1,
        ),
        (
            "Meridian Exchange",
            "Demo Exchange",
            "Sample Operations",
            "$224,112",
            "Connected",
            "1 min ago",
            2,
        ),
        (
            "Network Node A",
            "Demo RPC",
            "Sample RPC",
            "—",
            "Connected",
            "8 sec ago",
            2,
        ),
        (
            "Solana RPC B",
            "Demo RPC",
            "Sample Solana Assets",
            "$182,420",
            "Degraded",
            "18 min ago",
            3,
        ),
    ]
    .into_iter()
    .map(
        |(name, category, account, balance, state, sync, tone)| ConnectionRecord {
            name: name.into(),
            category: category.into(),
            account: account.into(),
            balance: balance.into(),
            state: state.into(),
            sync: sync.into(),
            tone,
        },
    )
    .collect()
}

pub fn security_events() -> Vec<SecurityEventRecord> {
    [
        (
            "Sample transfer approved",
            "24,000 USDC · Demo Treasury → Demo Settlement",
            "Primary Administrator · Approval",
            "13:41",
            0,
        ),
        (
            "Demo hardware key registered",
            "Sample security key · fingerprint DEMO:01",
            "Operations Approver · Authentication",
            "12:18",
            1,
        ),
        (
            "Demo destination allowlisted",
            "demo:account:0001 · Demo Settlement",
            "Primary Administrator · Policy",
            "10:52",
            0,
        ),
        (
            "Sample custody session established",
            "Northstar Vault · demo endpoint",
            "Demo System · Access",
            "09:14",
            2,
        ),
    ]
    .into_iter()
    .map(|(title, detail, actor, time, tone)| SecurityEventRecord {
        title: title.into(),
        detail: detail.into(),
        actor: actor.into(),
        time: time.into(),
        tone,
    })
    .collect()
}

pub fn signers() -> Vec<SignerRecord> {
    [
        (
            "Primary Administrator",
            "PA",
            "Demo Treasury Administrator",
            "Sample Hardware Key",
            "Active",
            "Now",
            0,
        ),
        (
            "Operations Approver",
            "OA",
            "Demo Operations Approver",
            "Sample Hardware Key",
            "Active",
            "12:18",
            0,
        ),
        (
            "Finance Approver",
            "FA",
            "Demo Finance Approver",
            "Sample Passkey",
            "Active",
            "Yesterday",
            0,
        ),
        (
            "Recovery Signer",
            "RS",
            "Emergency Recovery",
            "Offline Key",
            "Sealed",
            "Never",
            1,
        ),
    ]
    .into_iter()
    .map(
        |(name, initials, role, method, status, last, tone)| SignerRecord {
            name: name.into(),
            initials: initials.into(),
            role: role.into(),
            method: method.into(),
            status: status.into(),
            last: last.into(),
            tone,
        },
    )
    .collect()
}
