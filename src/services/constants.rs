use ethers::{prelude::Lazy, types::Address};

pub const MONAD_TESTNET_CHAIN_ID: u64 = 10143;

pub static APRMON_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0xb2f82D0f38dc453D596Ad40A37799446Cc89274A"
        .parse()
        .unwrap()
});

pub static GMON_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0xaEef2f6B429Cb59C9B2D7bB2141ADa993E8571c3"
        .parse()
        .unwrap()
});

pub static GMON_STAKEMANAGER_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x2c9C959516e9AAEdB2C748224a41249202ca8BE7"
        .parse()
        .unwrap()
});

pub static SHMON_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x3a98250F98Dd388C211206983453837C8365BDc1"
        .parse()
        .unwrap()
});
