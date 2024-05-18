use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
    utils::{get_mint_address_for, get_token_transfer},
};

const Deposit: u8 = 3;
const Withdraw: u8 = 4;
// const SwapBaseIn: u8 = 5;
// const SwapBaseOut: u8 = 6;
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

pub fn parse_trade_instruction(
    signer: &String,
    bytes_stream: Vec<u8>,
    accounts: &Vec<String>,
    input_accounts: Vec<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let disc_bytes_arr: [u8; 1] = match disc_bytes.to_vec().try_into() {
        Ok(arr) => arr,
        Err(_) => return None,
    };
    let discriminator: u8 = u8::from_le_bytes(disc_bytes_arr);

    let mut td = TradeData::default();

    match discriminator {
        SwapBaseIn => {
            td.instruction_type = "SwapIn".to_string();
            td.pool = match input_accounts.get(1) {
                Some(pool) => pool.to_string(),
                None => return None,
            };
            td.account_a = match input_accounts.get(15) {
                Some(account) => account.to_string(),
                None => return None,
            };
            td.account_b = match input_accounts.get(16) {
                Some(account) => account.to_string(),
                None => return None,
            };
            td.account_c = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);
            td.mint_c = get_mint_address_for(&td.account_c, post_token_balances, accounts);

            // Filter for SOL pair
            if td.mint_a != SOL_MINT && td.mint_b != SOL_MINT {
                return None;
            }

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_c_amount = get_token_transfer(
                &td.account_c,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );

            return Some(td);
        }
        _ => return None,
    }
}
