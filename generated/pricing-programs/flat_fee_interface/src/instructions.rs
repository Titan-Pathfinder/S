use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq)]
pub enum FlatFeeProgramIx {
    PriceExactIn(PriceExactInIxArgs),
    PriceExactOut(PriceExactOutIxArgs),
    PriceLpTokensToMint(PriceLpTokensToMintIxArgs),
    PriceLpTokensToRedeem(PriceLpTokensToRedeemIxArgs),
    SetLpWithdrawalFee(SetLpWithdrawalFeeIxArgs),
    SetLstFee(SetLstFeeIxArgs),
    RemoveLst(RemoveLstIxArgs),
    AddLst(AddLstIxArgs),
    SetManager(SetManagerIxArgs),
    Initialize(InitializeIxArgs),
}
impl BorshSerialize for FlatFeeProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::PriceExactIn(args) => {
                PRICE_EXACT_IN_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PriceExactOut(args) => {
                PRICE_EXACT_OUT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PriceLpTokensToMint(args) => {
                PRICE_LP_TOKENS_TO_MINT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PriceLpTokensToRedeem(args) => {
                PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetLpWithdrawalFee(args) => {
                SET_LP_WITHDRAWAL_FEE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetLstFee(args) => {
                SET_LST_FEE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RemoveLst(args) => {
                REMOVE_LST_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::AddLst(args) => {
                ADD_LST_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetManager(args) => {
                SET_MANAGER_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::Initialize(args) => {
                INITIALIZE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
        }
    }
}
impl FlatFeeProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            PRICE_EXACT_IN_IX_DISCM => {
                Ok(Self::PriceExactIn(PriceExactInIxArgs::deserialize(buf)?))
            }
            PRICE_EXACT_OUT_IX_DISCM => {
                Ok(Self::PriceExactOut(PriceExactOutIxArgs::deserialize(buf)?))
            }
            PRICE_LP_TOKENS_TO_MINT_IX_DISCM => Ok(Self::PriceLpTokensToMint(
                PriceLpTokensToMintIxArgs::deserialize(buf)?,
            )),
            PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM => Ok(Self::PriceLpTokensToRedeem(
                PriceLpTokensToRedeemIxArgs::deserialize(buf)?,
            )),
            SET_LP_WITHDRAWAL_FEE_IX_DISCM => Ok(Self::SetLpWithdrawalFee(
                SetLpWithdrawalFeeIxArgs::deserialize(buf)?,
            )),
            SET_LST_FEE_IX_DISCM => Ok(Self::SetLstFee(SetLstFeeIxArgs::deserialize(buf)?)),
            REMOVE_LST_IX_DISCM => Ok(Self::RemoveLst(RemoveLstIxArgs::deserialize(buf)?)),
            ADD_LST_IX_DISCM => Ok(Self::AddLst(AddLstIxArgs::deserialize(buf)?)),
            SET_MANAGER_IX_DISCM => Ok(Self::SetManager(SetManagerIxArgs::deserialize(buf)?)),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(buf)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const PRICE_EXACT_IN_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInAccounts<'me, 'info> {
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: Pubkey,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: Pubkey,
}
impl From<&PriceExactInAccounts<'_, '_>> for PriceExactInKeys {
    fn from(accounts: &PriceExactInAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
            input_fee_acc: *accounts.input_fee_acc.key,
            output_fee_acc: *accounts.output_fee_acc.key,
        }
    }
}
impl From<&PriceExactInKeys> for [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceExactInKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.input_lst_mint, false),
            AccountMeta::new_readonly(keys.output_lst_mint, false),
            AccountMeta::new_readonly(keys.input_fee_acc, false),
            AccountMeta::new_readonly(keys.output_fee_acc, false),
        ]
    }
}
impl From<[Pubkey; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]> for PriceExactInKeys {
    fn from(pubkeys: [Pubkey; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: pubkeys[0],
            output_lst_mint: pubkeys[1],
            input_fee_acc: pubkeys[2],
            output_fee_acc: pubkeys[3],
        }
    }
}
impl<'info> From<&PriceExactInAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.input_lst_mint.clone(),
            accounts.output_lst_mint.clone(),
            accounts.input_fee_acc.clone(),
            accounts.output_fee_acc.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]>
    for PriceExactInAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: &arr[0],
            output_lst_mint: &arr[1],
            input_fee_acc: &arr[2],
            output_fee_acc: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceExactInIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceExactInIxData(pub PriceExactInIxArgs);
pub const PRICE_EXACT_IN_IX_DISCM: u8 = 0u8;
impl From<PriceExactInIxArgs> for PriceExactInIxData {
    fn from(args: PriceExactInIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceExactInIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_EXACT_IN_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceExactInIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_EXACT_IN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceExactInIxArgs::deserialize(buf)?))
    }
}
pub fn price_exact_in_ix<K: Into<PriceExactInKeys>, A: Into<PriceExactInIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceExactInKeys = accounts.into();
    let metas: [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceExactInIxArgs = args.into();
    let data: PriceExactInIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_exact_in_invoke<'info, A: Into<PriceExactInIxArgs>>(
    accounts: &PriceExactInAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_exact_in_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_exact_in_invoke_signed<'info, A: Into<PriceExactInIxArgs>>(
    accounts: &PriceExactInAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_exact_in_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_exact_in_verify_account_keys(
    accounts: &PriceExactInAccounts<'_, '_>,
    keys: &PriceExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.input_lst_mint.key, &keys.input_lst_mint),
        (accounts.output_lst_mint.key, &keys.output_lst_mint),
        (accounts.input_fee_acc.key, &keys.input_fee_acc),
        (accounts.output_fee_acc.key, &keys.output_fee_acc),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_exact_in_verify_account_privileges<'me, 'info>(
    accounts: &PriceExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const PRICE_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutAccounts<'me, 'info> {
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: Pubkey,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: Pubkey,
}
impl From<&PriceExactOutAccounts<'_, '_>> for PriceExactOutKeys {
    fn from(accounts: &PriceExactOutAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
            input_fee_acc: *accounts.input_fee_acc.key,
            output_fee_acc: *accounts.output_fee_acc.key,
        }
    }
}
impl From<&PriceExactOutKeys> for [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceExactOutKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.input_lst_mint, false),
            AccountMeta::new_readonly(keys.output_lst_mint, false),
            AccountMeta::new_readonly(keys.input_fee_acc, false),
            AccountMeta::new_readonly(keys.output_fee_acc, false),
        ]
    }
}
impl From<[Pubkey; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]> for PriceExactOutKeys {
    fn from(pubkeys: [Pubkey; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: pubkeys[0],
            output_lst_mint: pubkeys[1],
            input_fee_acc: pubkeys[2],
            output_fee_acc: pubkeys[3],
        }
    }
}
impl<'info> From<&PriceExactOutAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.input_lst_mint.clone(),
            accounts.output_lst_mint.clone(),
            accounts.input_fee_acc.clone(),
            accounts.output_fee_acc.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]>
    for PriceExactOutAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: &arr[0],
            output_lst_mint: &arr[1],
            input_fee_acc: &arr[2],
            output_fee_acc: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceExactOutIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceExactOutIxData(pub PriceExactOutIxArgs);
pub const PRICE_EXACT_OUT_IX_DISCM: u8 = 1u8;
impl From<PriceExactOutIxArgs> for PriceExactOutIxData {
    fn from(args: PriceExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceExactOutIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_EXACT_OUT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceExactOutIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_EXACT_OUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceExactOutIxArgs::deserialize(buf)?))
    }
}
pub fn price_exact_out_ix<K: Into<PriceExactOutKeys>, A: Into<PriceExactOutIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceExactOutKeys = accounts.into();
    let metas: [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceExactOutIxArgs = args.into();
    let data: PriceExactOutIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_exact_out_invoke<'info, A: Into<PriceExactOutIxArgs>>(
    accounts: &PriceExactOutAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_exact_out_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_exact_out_invoke_signed<'info, A: Into<PriceExactOutIxArgs>>(
    accounts: &PriceExactOutAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_exact_out_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_exact_out_verify_account_keys(
    accounts: &PriceExactOutAccounts<'_, '_>,
    keys: &PriceExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.input_lst_mint.key, &keys.input_lst_mint),
        (accounts.output_lst_mint.key, &keys.output_lst_mint),
        (accounts.input_fee_acc.key, &keys.input_fee_acc),
        (accounts.output_fee_acc.key, &keys.output_fee_acc),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_exact_out_verify_account_privileges<'me, 'info>(
    accounts: &PriceExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToMintAccounts<'me, 'info> {
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToMintKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
}
impl From<&PriceLpTokensToMintAccounts<'_, '_>> for PriceLpTokensToMintKeys {
    fn from(accounts: &PriceLpTokensToMintAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
        }
    }
}
impl From<&PriceLpTokensToMintKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceLpTokensToMintKeys) -> Self {
        [AccountMeta::new_readonly(keys.input_lst_mint, false)]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]> for PriceLpTokensToMintKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: pubkeys[0],
        }
    }
}
impl<'info> From<&PriceLpTokensToMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceLpTokensToMintAccounts<'_, 'info>) -> Self {
        [accounts.input_lst_mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]>
    for PriceLpTokensToMintAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: &arr[0],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLpTokensToMintIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceLpTokensToMintIxData(pub PriceLpTokensToMintIxArgs);
pub const PRICE_LP_TOKENS_TO_MINT_IX_DISCM: u8 = 2u8;
impl From<PriceLpTokensToMintIxArgs> for PriceLpTokensToMintIxData {
    fn from(args: PriceLpTokensToMintIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceLpTokensToMintIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_LP_TOKENS_TO_MINT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceLpTokensToMintIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_LP_TOKENS_TO_MINT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_LP_TOKENS_TO_MINT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceLpTokensToMintIxArgs::deserialize(buf)?))
    }
}
pub fn price_lp_tokens_to_mint_ix<
    K: Into<PriceLpTokensToMintKeys>,
    A: Into<PriceLpTokensToMintIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceLpTokensToMintKeys = accounts.into();
    let metas: [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceLpTokensToMintIxArgs = args.into();
    let data: PriceLpTokensToMintIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_lp_tokens_to_mint_invoke<'info, A: Into<PriceLpTokensToMintIxArgs>>(
    accounts: &PriceLpTokensToMintAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_lp_tokens_to_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_lp_tokens_to_mint_invoke_signed<'info, A: Into<PriceLpTokensToMintIxArgs>>(
    accounts: &PriceLpTokensToMintAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_lp_tokens_to_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_lp_tokens_to_mint_verify_account_keys(
    accounts: &PriceLpTokensToMintAccounts<'_, '_>,
    keys: &PriceLpTokensToMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.input_lst_mint.key, &keys.input_lst_mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_lp_tokens_to_mint_verify_account_privileges<'me, 'info>(
    accounts: &PriceLpTokensToMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemAccounts<'me, 'info> {
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemKeys {
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
}
impl From<&PriceLpTokensToRedeemAccounts<'_, '_>> for PriceLpTokensToRedeemKeys {
    fn from(accounts: &PriceLpTokensToRedeemAccounts) -> Self {
        Self {
            output_lst_mint: *accounts.output_lst_mint.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&PriceLpTokensToRedeemKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceLpTokensToRedeemKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.output_lst_mint, false),
            AccountMeta::new_readonly(keys.state, false),
        ]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]> for PriceLpTokensToRedeemKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            output_lst_mint: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&PriceLpTokensToRedeemAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceLpTokensToRedeemAccounts<'_, 'info>) -> Self {
        [accounts.output_lst_mint.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]>
    for PriceLpTokensToRedeemAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            output_lst_mint: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLpTokensToRedeemIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceLpTokensToRedeemIxData(pub PriceLpTokensToRedeemIxArgs);
pub const PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM: u8 = 3u8;
impl From<PriceLpTokensToRedeemIxArgs> for PriceLpTokensToRedeemIxData {
    fn from(args: PriceLpTokensToRedeemIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceLpTokensToRedeemIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceLpTokensToRedeemIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceLpTokensToRedeemIxArgs::deserialize(buf)?))
    }
}
pub fn price_lp_tokens_to_redeem_ix<
    K: Into<PriceLpTokensToRedeemKeys>,
    A: Into<PriceLpTokensToRedeemIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceLpTokensToRedeemKeys = accounts.into();
    let metas: [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceLpTokensToRedeemIxArgs = args.into();
    let data: PriceLpTokensToRedeemIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_lp_tokens_to_redeem_invoke<'info, A: Into<PriceLpTokensToRedeemIxArgs>>(
    accounts: &PriceLpTokensToRedeemAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_lp_tokens_to_redeem_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_lp_tokens_to_redeem_invoke_signed<'info, A: Into<PriceLpTokensToRedeemIxArgs>>(
    accounts: &PriceLpTokensToRedeemAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_lp_tokens_to_redeem_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_lp_tokens_to_redeem_verify_account_keys(
    accounts: &PriceLpTokensToRedeemAccounts<'_, '_>,
    keys: &PriceLpTokensToRedeemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.output_lst_mint.key, &keys.output_lst_mint),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_lp_tokens_to_redeem_verify_account_privileges<'me, 'info>(
    accounts: &PriceLpTokensToRedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLpWithdrawalFeeAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetLpWithdrawalFeeKeys {
    ///The program manager
    pub manager: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
}
impl From<&SetLpWithdrawalFeeAccounts<'_, '_>> for SetLpWithdrawalFeeKeys {
    fn from(accounts: &SetLpWithdrawalFeeAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&SetLpWithdrawalFeeKeys> for [AccountMeta; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetLpWithdrawalFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]> for SetLpWithdrawalFeeKeys {
    fn from(pubkeys: [Pubkey; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&SetLpWithdrawalFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetLpWithdrawalFeeAccounts<'_, 'info>) -> Self {
        [accounts.manager.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]>
    for SetLpWithdrawalFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLpWithdrawalFeeIxArgs {
    pub lp_withdrawal_fee_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLpWithdrawalFeeIxData(pub SetLpWithdrawalFeeIxArgs);
pub const SET_LP_WITHDRAWAL_FEE_IX_DISCM: u8 = 250u8;
impl From<SetLpWithdrawalFeeIxArgs> for SetLpWithdrawalFeeIxData {
    fn from(args: SetLpWithdrawalFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetLpWithdrawalFeeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_LP_WITHDRAWAL_FEE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetLpWithdrawalFeeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_LP_WITHDRAWAL_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_LP_WITHDRAWAL_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetLpWithdrawalFeeIxArgs::deserialize(buf)?))
    }
}
pub fn set_lp_withdrawal_fee_ix<
    K: Into<SetLpWithdrawalFeeKeys>,
    A: Into<SetLpWithdrawalFeeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetLpWithdrawalFeeKeys = accounts.into();
    let metas: [AccountMeta; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetLpWithdrawalFeeIxArgs = args.into();
    let data: SetLpWithdrawalFeeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lp_withdrawal_fee_invoke<'info, A: Into<SetLpWithdrawalFeeIxArgs>>(
    accounts: &SetLpWithdrawalFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_lp_withdrawal_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_lp_withdrawal_fee_invoke_signed<'info, A: Into<SetLpWithdrawalFeeIxArgs>>(
    accounts: &SetLpWithdrawalFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_lp_withdrawal_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_lp_withdrawal_fee_verify_account_keys(
    accounts: &SetLpWithdrawalFeeAccounts<'_, '_>,
    keys: &SetLpWithdrawalFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_lp_withdrawal_fee_verify_account_privileges<'me, 'info>(
    accounts: &SetLpWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_LST_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetLstFeeAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///FeeAccount PDA to modify
    pub fee_acc: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetLstFeeKeys {
    ///The program manager
    pub manager: Pubkey,
    ///FeeAccount PDA to modify
    pub fee_acc: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
}
impl From<&SetLstFeeAccounts<'_, '_>> for SetLstFeeKeys {
    fn from(accounts: &SetLstFeeAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fee_acc: *accounts.fee_acc.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&SetLstFeeKeys> for [AccountMeta; SET_LST_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetLstFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new(keys.fee_acc, false),
            AccountMeta::new_readonly(keys.state, false),
        ]
    }
}
impl From<[Pubkey; SET_LST_FEE_IX_ACCOUNTS_LEN]> for SetLstFeeKeys {
    fn from(pubkeys: [Pubkey; SET_LST_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fee_acc: pubkeys[1],
            state: pubkeys[2],
        }
    }
}
impl<'info> From<&SetLstFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetLstFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.fee_acc.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN]>
    for SetLstFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            fee_acc: &arr[1],
            state: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLstFeeIxArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLstFeeIxData(pub SetLstFeeIxArgs);
pub const SET_LST_FEE_IX_DISCM: u8 = 251u8;
impl From<SetLstFeeIxArgs> for SetLstFeeIxData {
    fn from(args: SetLstFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetLstFeeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_LST_FEE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetLstFeeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_LST_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_LST_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetLstFeeIxArgs::deserialize(buf)?))
    }
}
pub fn set_lst_fee_ix<K: Into<SetLstFeeKeys>, A: Into<SetLstFeeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetLstFeeKeys = accounts.into();
    let metas: [AccountMeta; SET_LST_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetLstFeeIxArgs = args.into();
    let data: SetLstFeeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lst_fee_invoke<'info, A: Into<SetLstFeeIxArgs>>(
    accounts: &SetLstFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_lst_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_lst_fee_invoke_signed<'info, A: Into<SetLstFeeIxArgs>>(
    accounts: &SetLstFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_lst_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_lst_fee_verify_account_keys(
    accounts: &SetLstFeeAccounts<'_, '_>,
    keys: &SetLstFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.fee_acc.key, &keys.fee_acc),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_lst_fee_verify_account_privileges<'me, 'info>(
    accounts: &SetLstFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_acc] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_LST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLstAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///Account to refund SOL rent to
    pub refund_rent_to: &'me AccountInfo<'info>,
    ///FeeAccount PDA to be created
    pub fee_acc: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveLstKeys {
    ///The program manager
    pub manager: Pubkey,
    ///Account to refund SOL rent to
    pub refund_rent_to: Pubkey,
    ///FeeAccount PDA to be created
    pub fee_acc: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&RemoveLstAccounts<'_, '_>> for RemoveLstKeys {
    fn from(accounts: &RemoveLstAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            refund_rent_to: *accounts.refund_rent_to.key,
            fee_acc: *accounts.fee_acc.key,
            state: *accounts.state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&RemoveLstKeys> for [AccountMeta; REMOVE_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemoveLstKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new(keys.refund_rent_to, true),
            AccountMeta::new(keys.fee_acc, false),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_LST_IX_ACCOUNTS_LEN]> for RemoveLstKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            refund_rent_to: pubkeys[1],
            fee_acc: pubkeys[2],
            state: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<&RemoveLstAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveLstAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.refund_rent_to.clone(),
            accounts.fee_acc.clone(),
            accounts.state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]>
    for RemoveLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            refund_rent_to: &arr[1],
            fee_acc: &arr[2],
            state: &arr[3],
            system_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLstIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLstIxData(pub RemoveLstIxArgs);
pub const REMOVE_LST_IX_DISCM: u8 = 252u8;
impl From<RemoveLstIxArgs> for RemoveLstIxData {
    fn from(args: RemoveLstIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveLstIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_LST_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl RemoveLstIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REMOVE_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveLstIxArgs::deserialize(buf)?))
    }
}
pub fn remove_lst_ix<K: Into<RemoveLstKeys>, A: Into<RemoveLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveLstKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_LST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveLstIxArgs = args.into();
    let data: RemoveLstIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_lst_invoke<'info, A: Into<RemoveLstIxArgs>>(
    accounts: &RemoveLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_lst_invoke_signed<'info, A: Into<RemoveLstIxArgs>>(
    accounts: &RemoveLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_lst_verify_account_keys(
    accounts: &RemoveLstAccounts<'_, '_>,
    keys: &RemoveLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.refund_rent_to.key, &keys.refund_rent_to),
        (accounts.fee_acc.key, &keys.fee_acc),
        (accounts.state.key, &keys.state),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_lst_verify_account_privileges<'me, 'info>(
    accounts: &RemoveLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.refund_rent_to, accounts.fee_acc] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager, accounts.refund_rent_to] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_LST_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddLstAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///Account paying for FeeAccount's rent
    pub payer: &'me AccountInfo<'info>,
    ///FeeAccount PDA to be created
    pub fee_acc: &'me AccountInfo<'info>,
    ///Mint of the LST
    pub lst_mint: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddLstKeys {
    ///The program manager
    pub manager: Pubkey,
    ///Account paying for FeeAccount's rent
    pub payer: Pubkey,
    ///FeeAccount PDA to be created
    pub fee_acc: Pubkey,
    ///Mint of the LST
    pub lst_mint: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&AddLstAccounts<'_, '_>> for AddLstKeys {
    fn from(accounts: &AddLstAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            payer: *accounts.payer.key,
            fee_acc: *accounts.fee_acc.key,
            lst_mint: *accounts.lst_mint.key,
            state: *accounts.state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&AddLstKeys> for [AccountMeta; ADD_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddLstKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new(keys.fee_acc, false),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; ADD_LST_IX_ACCOUNTS_LEN]> for AddLstKeys {
    fn from(pubkeys: [Pubkey; ADD_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            payer: pubkeys[1],
            fee_acc: pubkeys[2],
            lst_mint: pubkeys[3],
            state: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<&AddLstAccounts<'_, 'info>> for [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: &AddLstAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.payer.clone(),
            accounts.fee_acc.clone(),
            accounts.lst_mint.clone(),
            accounts.state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN]>
    for AddLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            payer: &arr[1],
            fee_acc: &arr[2],
            lst_mint: &arr[3],
            state: &arr[4],
            system_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLstIxArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLstIxData(pub AddLstIxArgs);
pub const ADD_LST_IX_DISCM: u8 = 253u8;
impl From<AddLstIxArgs> for AddLstIxData {
    fn from(args: AddLstIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddLstIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[ADD_LST_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl AddLstIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != ADD_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddLstIxArgs::deserialize(buf)?))
    }
}
pub fn add_lst_ix<K: Into<AddLstKeys>, A: Into<AddLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddLstKeys = accounts.into();
    let metas: [AccountMeta; ADD_LST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddLstIxArgs = args.into();
    let data: AddLstIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_lst_invoke<'info, A: Into<AddLstIxArgs>>(
    accounts: &AddLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_lst_invoke_signed<'info, A: Into<AddLstIxArgs>>(
    accounts: &AddLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_lst_verify_account_keys(
    accounts: &AddLstAccounts<'_, '_>,
    keys: &AddLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.payer.key, &keys.payer),
        (accounts.fee_acc.key, &keys.fee_acc),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.state.key, &keys.state),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_lst_verify_account_privileges<'me, 'info>(
    accounts: &AddLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.fee_acc] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_MANAGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetManagerAccounts<'me, 'info> {
    ///The current program manager
    pub current_manager: &'me AccountInfo<'info>,
    ///The new program manager to set to
    pub new_manager: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetManagerKeys {
    ///The current program manager
    pub current_manager: Pubkey,
    ///The new program manager to set to
    pub new_manager: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
}
impl From<&SetManagerAccounts<'_, '_>> for SetManagerKeys {
    fn from(accounts: &SetManagerAccounts) -> Self {
        Self {
            current_manager: *accounts.current_manager.key,
            new_manager: *accounts.new_manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&SetManagerKeys> for [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetManagerKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.current_manager, true),
            AccountMeta::new_readonly(keys.new_manager, false),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; SET_MANAGER_IX_ACCOUNTS_LEN]> for SetManagerKeys {
    fn from(pubkeys: [Pubkey; SET_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_manager: pubkeys[0],
            new_manager: pubkeys[1],
            state: pubkeys[2],
        }
    }
}
impl<'info> From<&SetManagerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.current_manager.clone(),
            accounts.new_manager.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]>
    for SetManagerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_manager: &arr[0],
            new_manager: &arr[1],
            state: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetManagerIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SetManagerIxData(pub SetManagerIxArgs);
pub const SET_MANAGER_IX_DISCM: u8 = 254u8;
impl From<SetManagerIxArgs> for SetManagerIxData {
    fn from(args: SetManagerIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetManagerIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_MANAGER_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetManagerIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_MANAGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_MANAGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetManagerIxArgs::deserialize(buf)?))
    }
}
pub fn set_manager_ix<K: Into<SetManagerKeys>, A: Into<SetManagerIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetManagerKeys = accounts.into();
    let metas: [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetManagerIxArgs = args.into();
    let data: SetManagerIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_manager_invoke<'info, A: Into<SetManagerIxArgs>>(
    accounts: &SetManagerAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_manager_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_manager_invoke_signed<'info, A: Into<SetManagerIxArgs>>(
    accounts: &SetManagerAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_manager_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_manager_verify_account_keys(
    accounts: &SetManagerAccounts<'_, '_>,
    keys: &SetManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.current_manager.key, &keys.current_manager),
        (accounts.new_manager.key, &keys.new_manager),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_manager_verify_account_privileges<'me, 'info>(
    accounts: &SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.current_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    ///Account paying for ProgramState's rent
    pub payer: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    ///Account paying for ProgramState's rent
    pub payer: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: &InitializeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            state: *accounts.state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<&InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            state: &arr[1],
            system_program: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
pub const INITIALIZE_IX_DISCM: u8 = 255u8;
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(buf)?))
    }
}
pub fn initialize_ix<K: Into<InitializeKeys>, A: Into<InitializeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeIxArgs = args.into();
    let data: InitializeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_invoke<'info, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_invoke_signed<'info, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: &InitializeAccounts<'_, '_>,
    keys: &InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.state.key, &keys.state),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: &InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
