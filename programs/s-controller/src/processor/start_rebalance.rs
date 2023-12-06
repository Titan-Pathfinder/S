use s_controller_interface::{
    start_rebalance_verify_account_keys, start_rebalance_verify_account_privileges,
    SControllerError, StartRebalanceAccounts, StartRebalanceIxArgs, END_REBALANCE_IX_DISCM,
    START_REBALANCE_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    program::{POOL_STATE_BUMP, POOL_STATE_SEED, REBALANCE_RECORD_BUMP, REBALANCE_RECORD_SEED},
    try_lst_state_list, try_pool_state, try_pool_state_mut, try_rebalance_record_mut,
    PoolStateAccount, SrcDstLstIndexes, StartRebalanceFreeArgs, U8BoolMut, REBALANCE_RECORD_SIZE,
};
use sanctum_onchain_utils::{
    system_program::{create_pda, CreateAccountAccounts, CreateAccountArgs},
    token_program::{transfer_tokens_signed, TransferTokensAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program_error::ProgramError,
    sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
};

use crate::{
    account_traits::{DstLstPoolReservesOf, SrcLstPoolReservesOf},
    cpi::SrcDstLstSolValueCalculatorCpis,
    verify::{
        verify_lst_input_not_disabled, verify_not_rebalancing_and_not_disabled,
        verify_src_dst_lst_sol_val_calc_cpis,
    },
};

use super::sync_sol_value_unchecked;

pub fn process_start_rebalance(
    accounts: &[AccountInfo],
    args: StartRebalanceIxArgs,
) -> ProgramResult {
    let (
        accounts,
        SrcDstLstSolValueCalculatorCpis {
            src_lst: src_lst_cpi,
            dst_lst: dst_lst_cpi,
        },
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
    ) = verify_start_rebalance(accounts, &args)?;

    sync_sol_value_unchecked(SrcLstPoolReservesOf(&accounts), src_lst_cpi, src_lst_index)?;
    sync_sol_value_unchecked(DstLstPoolReservesOf(&accounts), dst_lst_cpi, dst_lst_index)?;

    let old_total_sol_value = accounts.pool_state.total_sol_value()?;

    transfer_tokens_signed(
        TransferTokensAccounts {
            token_program: accounts.src_lst_token_program,
            from: accounts.src_pool_reserves,
            to: accounts.withdraw_to,
            authority: accounts.pool_state,
        },
        args.amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    sync_sol_value_unchecked(SrcLstPoolReservesOf(&accounts), src_lst_cpi, src_lst_index)?;

    create_pda(
        CreateAccountAccounts {
            from: accounts.payer,
            to: accounts.rebalance_record,
        },
        CreateAccountArgs {
            space: REBALANCE_RECORD_SIZE,
            owner: s_controller_lib::program::ID,
            lamports: Some(1),
        },
        &[&[REBALANCE_RECORD_SEED, &[REBALANCE_RECORD_BUMP]]],
    )?;

    let mut rebalance_record_data = accounts.rebalance_record.try_borrow_mut_data()?;
    let rebalance_record = try_rebalance_record_mut(&mut rebalance_record_data)?;
    rebalance_record.dst_lst_index = args.dst_lst_index;
    rebalance_record.old_total_sol_value = old_total_sol_value;

    let mut pool_state_data = accounts.pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_data)?;
    U8BoolMut(&mut pool_state.is_rebalancing).set_true();

    Ok(())
}

fn verify_start_rebalance<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    StartRebalanceIxArgs {
        src_lst_calc_accs,
        src_lst_index,
        dst_lst_index,
        ..
    }: &StartRebalanceIxArgs,
) -> Result<
    (
        StartRebalanceAccounts<'a, 'info>,
        SrcDstLstSolValueCalculatorCpis<'a, 'info>,
        SrcDstLstIndexes,
    ),
    ProgramError,
> {
    let actual: StartRebalanceAccounts = load_accounts(accounts)?;

    let free_args = StartRebalanceFreeArgs {
        payer: *actual.payer.key,
        withdraw_to: *actual.withdraw_to.key,
        src_lst_index: *src_lst_index,
        dst_lst_index: *dst_lst_index,
        lst_state_list: actual.lst_state_list,
        pool_state: actual.pool_state,
        src_lst_mint: actual.src_lst_mint,
        dst_lst_mint: actual.dst_lst_mint,
    };
    let expected = free_args.resolve()?;

    start_rebalance_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    start_rebalance_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    // indexes checked in resolve() above
    let src_lst_index: usize = (*src_lst_index).try_into().unwrap();
    let dst_lst_index: usize = (*dst_lst_index).try_into().unwrap();

    let lst_state_list_bytes = actual.lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&lst_state_list_bytes)?;
    let dst_lst_state = lst_state_list[dst_lst_index]; // dst_lst_index checked above
    verify_lst_input_not_disabled(&dst_lst_state)?;

    let accounts_suffix_slice = accounts
        .get(START_REBALANCE_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let src_dst_lst_indexes = SrcDstLstIndexes {
        src_lst_index,
        dst_lst_index,
    };

    let src_dst_lst_cpis = verify_src_dst_lst_sol_val_calc_cpis(
        actual,
        accounts_suffix_slice,
        *src_lst_calc_accs,
        src_dst_lst_indexes,
    )?;

    verify_has_succeeding_end_rebalance_ix(actual.instructions)?;

    Ok((actual, src_dst_lst_cpis, src_dst_lst_indexes))
}

fn verify_has_succeeding_end_rebalance_ix(
    instructions_sysvar: &AccountInfo,
) -> Result<(), ProgramError> {
    let current_idx: usize = load_current_index_checked(instructions_sysvar)?.into();
    let mut next_ix_idx = current_idx + 1;
    loop {
        let next_ix = load_instruction_at_checked(next_ix_idx, instructions_sysvar)
            .map_err(|_| SControllerError::NoSucceedingEndRebalance)?;
        if is_end_rebalance_ix(&next_ix) {
            break;
        }
        next_ix_idx += 1;
    }
    Ok(())
}

fn is_end_rebalance_ix(ix: &Instruction) -> bool {
    let discm = match ix.data.first() {
        Some(d) => d,
        None => return false,
    };
    if *discm != END_REBALANCE_IX_DISCM {
        return false;
    }
    if ix.program_id != s_controller_lib::program::ID {
        return false;
    }
    true
}
