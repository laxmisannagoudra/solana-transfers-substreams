#[allow(unused)]
mod pb;
use pb::sol::transfers::v1 as transfers;
use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const SYSTEM_PROGRAM_ID: [u8; 32] = [0u8; 32];
const TOKEN_PROGRAM_ID: [u8; 32] = b58!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const TOKEN_2022_PROGRAM_ID: [u8; 32] = b58!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

#[substreams::handlers::map]
fn map_transfers(blk: Block) -> transfers::Transfers {
    let mut out: Vec<transfers::Transfer> = Vec::new();

    blk.walk_instructions().for_each(|ix| {
        let data = ix.data();
        let accounts = ix.accounts();
        let is_system = ix.program_id() == SYSTEM_PROGRAM_ID;
        let is_token =
            ix.program_id() == TOKEN_PROGRAM_ID || ix.program_id() == TOKEN_2022_PROGRAM_ID;

        // (from_index, to_index, amount)
        let mut parsed: Option<(usize, usize, u64)> = None;

        if is_system && data.len() == 12 && accounts.len() >= 2 {
            // System Transfer: u32 tag (2) + u64 lamports
            if u32::from_le_bytes(data[0..4].try_into().unwrap()) == 2 {
                parsed = Some((0, 1, u64::from_le_bytes(data[4..12].try_into().unwrap())));
            }
        } else if is_token && data.len() >= 9 {
            if data[0] == 3 && accounts.len() >= 3 {
                // SPL Transfer: [source, destination, authority]
                parsed = Some((0, 1, u64::from_le_bytes(data[1..9].try_into().unwrap())));
            } else if data[0] == 12 && accounts.len() >= 4 {
                // SPL TransferChecked: [source, mint, destination, authority]
                parsed = Some((0, 2, u64::from_le_bytes(data[1..9].try_into().unwrap())));
            }
        }

        if let Some((fi, ti, amount)) = parsed {
            let mut t = transfers::Transfer::default();
            t.from = bs58::encode(&accounts[fi].0).into_string();
            t.to = bs58::encode(&accounts[ti].0).into_string();
            t.amount = amount;
            out.push(t);
        }
    });

    let mut result = transfers::Transfers::default();
    result.transfers = out;
    result
}
