use generic_pool_calculator_interface::GenericPoolCalculatorError;
use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateKeys;
use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{
    instruction::AccountMeta,
    pubkey::Pubkey,
    sysvar::{self, clock::Clock},
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};
use spl_calculator_lib::{
    deserialize_sanctum_spl_stake_pool_checked, deserialize_stake_pool_checked,
    resolve_to_account_metas_for_calc, SanctumSplSolValCalc, SplStakePoolCalc,
};
use std::{collections::HashMap, error::Error};

use crate::{
    KnownLstSolValCalc, LstSolValCalc, LstSolValCalcErr, MutableLstSolValCalc, SplLstSolValCalcErr,
    SplLstSolValCalcInitKeys,
};

#[derive(Clone, Debug, Default)]
pub struct SanctumSplLstSolValCalc {
    pub lst_mint: Pubkey,
    pub stake_pool_addr: Pubkey,
    pub calc: Option<SplStakePoolCalc>,
    pub clock: Option<Clock>,
}

impl SanctumSplLstSolValCalc {
    pub fn from_keys(
        SplLstSolValCalcInitKeys {
            lst_mint,
            stake_pool_addr,
        }: SplLstSolValCalcInitKeys,
    ) -> Self {
        Self {
            lst_mint,
            stake_pool_addr,
            calc: None,
            clock: None,
        }
    }

    pub fn from_pool<P: ReadonlyAccountData + ReadonlyAccountPubkey + ReadonlyAccountOwner>(
        pool_acc: P,
    ) -> Result<Self, GenericPoolCalculatorError> {
        let stake_pool_addr = *pool_acc.pubkey();
        let pool = deserialize_sanctum_spl_stake_pool_checked(pool_acc)?;
        Ok(Self {
            lst_mint: pool.pool_mint,
            stake_pool_addr,
            calc: Some(SplStakePoolCalc::from(pool)),
            clock: None,
        })
    }
}

impl MutableLstSolValCalc for SanctumSplLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![sysvar::clock::ID, self.stake_pool_addr]
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(acc) = account_map.get(&sysvar::clock::ID) {
            self.clock = Some(bincode::deserialize::<Clock>(&acc.data())?);
        }
        if let Some(acc) = account_map.get(&self.stake_pool_addr) {
            let pool = deserialize_stake_pool_checked(acc)?;
            if pool.pool_mint != self.lst_mint {
                return Err(SplLstSolValCalcErr::WrongLstMint.into());
            }
            self.calc = Some(SplStakePoolCalc::from(pool));
        }
        Ok(())
    }
}

impl LstSolValCalc for SanctumSplLstSolValCalc {
    fn lst_mint(&self) -> Pubkey {
        self.lst_mint
    }

    fn lst_to_sol(&self, lst_amount: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>> {
        let calc = self.calc.ok_or(SplLstSolValCalcErr::StakePoolNotFetched)?;
        let clock = self
            .clock
            .as_ref()
            .ok_or(SplLstSolValCalcErr::ClockNotFetched)?;
        calc.verify_pool_updated_for_this_epoch(clock)?;
        Ok(calc.calc_lst_to_sol(lst_amount)?)
    }

    fn sol_to_lst(&self, lamports: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>> {
        let calc = self.calc.ok_or(SplLstSolValCalcErr::StakePoolNotFetched)?;
        let clock = self
            .clock
            .as_ref()
            .ok_or(SplLstSolValCalcErr::ClockNotFetched)?;
        calc.verify_pool_updated_for_this_epoch(clock)?;
        Ok(calc.calc_sol_to_lst(lamports)?)
    }

    fn ix_accounts(&self) -> Vec<AccountMeta> {
        Vec::from(resolve_to_account_metas_for_calc::<SanctumSplSolValCalc>(
            LstSolCommonIntermediateKeys {
                lst_mint: self.lst_mint,
                pool_state: self.stake_pool_addr,
            },
        ))
    }
}

impl TryFrom<KnownLstSolValCalc> for SanctumSplLstSolValCalc {
    type Error = LstSolValCalcErr;

    fn try_from(value: KnownLstSolValCalc) -> Result<Self, Self::Error> {
        match value {
            KnownLstSolValCalc::SanctumSpl(s) => Ok(s),
            _ => Err(LstSolValCalcErr::WrongLstSolValCalc),
        }
    }
}
