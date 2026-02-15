use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;

const COMP_DEF_OFFSET_PSI: u32 = comp_def_offset("perform_psi");

declare_id!("2ozAD1iXUUZGAGEp7G5DfBcT9FMACM5cU4Z3VPxBAfUb");

#[arcium_program]
pub mod arcpsi {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    /// [New] Initialize the global registry
    pub fn init_registry(ctx: Context<InitRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.payer.key();
        registry.user_count = 0;
        // Initialize as an empty array
        registry.encrypted_users = [[0u8; 32]; 4];
        Ok(())
    }

    /// [New] Register a user (simulation)
    /// Add the user's hash to the global pool to be discovered by others
    pub fn register_user(
        ctx: Context<RegisterUser>, 
        encrypted_hash: [u8; 32]
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        require!(registry.user_count < 4, PsiError::RegistryFull);
        
        let idx = registry.user_count as usize;
        registry.encrypted_users[idx] = encrypted_hash;
        registry.user_count += 1;
        
        msg!("User registered at index {}. Hash is secret-shared.", idx);
        Ok(())
    }

    /// [Core] Initiate private set intersection
    /// Input: User's local encrypted contact list
    /// Context: Global encrypted user list on-chain
    pub fn discover_contacts(
        ctx: Context<DiscoverContacts>,
        computation_offset: u64,
        user_contacts: [[u8; 32]; 4], // 4 numbers the user wants to query
        pubkey: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        let registry = &ctx.accounts.registry;
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
        
        // Construct MPC parameters: UserContacts + GlobalRegistry
        let mut builder = ArgBuilder::new()
            .x25519_pubkey(pubkey)
            .plaintext_u128(nonce);

        // 1. Inject user query data (UserContacts)
        for contact in &user_contacts {
            builder = builder.encrypted_u64(*contact);
        }

        // 2. Inject global registry data (GlobalRegistry)
        // Arcium nodes will read this on-chain state as the comparison source
        for user in &registry.encrypted_users {
            builder = builder.encrypted_u64(*user);
        }

        queue_computation(
            ctx.accounts,
            computation_offset,
            builder.build(),
            vec![PerformPsiCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[]
            )?],
            1,
            0,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "perform_psi")]
    pub fn perform_psi_callback(
        ctx: Context<PerformPsiCallback>,
        output: SignedComputationOutputs<PerformPsiOutput>,
    ) -> Result<()> {
        let o = match output.verify_output(&ctx.accounts.cluster_account, &ctx.accounts.computation_account) {
            Ok(PerformPsiOutput { field_0 }) => field_0,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        // Parse results: Boolean mask [1, 0, 1, 0]
        // Here, only an event is emitted. The frontend will decrypt to know which ones matched.
        let m0 = u64::from_le_bytes(o.ciphertexts[0][0..8].try_into().unwrap());
        let m1 = u64::from_le_bytes(o.ciphertexts[1][0..8].try_into().unwrap());
        let m2 = u64::from_le_bytes(o.ciphertexts[2][0..8].try_into().unwrap());
        let m3 = u64::from_le_bytes(o.ciphertexts[3][0..8].try_into().unwrap());

        msg!("PSI Execution Complete. Matches found: {:?}", [m0, m1, m2, m3]);
        
        emit!(PsiCompleteEvent {
            user: ctx.accounts.computation_account.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }
}

// --- Accounts ---

#[derive(Accounts)]
pub struct InitRegistry<'info> {
    #[account(
        init, 
        payer = payer, 
        space = 8 + 32 + 1 + (32 * 4) + 100,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, GlobalState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub registry: Account<'info, GlobalState>,
    pub signer: Signer<'info>,
}

#[account]
pub struct GlobalState {
    pub authority: Pubkey,
    pub user_count: u8,
    pub encrypted_users: [[u8; 32]; 4], // Global user pool
}

#[queue_computation_accounts("perform_psi", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct DiscoverContacts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub registry: Account<'info, GlobalState>, // Read source
    
    #[account(init_if_needed, space = 9, payer = payer, seeds = [&SIGN_PDA_SEED], bump, address = derive_sign_pda!())]
    pub sign_pda_account: Account<'info, ArciumSignerAccount>,
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut, address = derive_mempool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: Mempool
    pub mempool_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: Execpool
    pub executing_pool: UncheckedAccount<'info>,
    #[account(mut, address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet))]
    /// CHECK: Comp
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_PSI))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(mut, address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,
    #[account(mut, address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[callback_accounts("perform_psi")]
#[derive(Accounts)]
pub struct PerformPsiCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(COMP_DEF_OFFSET_PSI))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    /// CHECK: Comp
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: Sysvar
    pub instructions_sysvar: AccountInfo<'info>,
}

#[init_computation_definition_accounts("perform_psi", payer)]
#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, address = derive_mxe_pda!())]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: Def
    pub comp_def_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_mxe_lut_pda!(mxe_account.lut_offset_slot))]
    /// CHECK: Lut
    pub address_lookup_table: UncheckedAccount<'info>,
    #[account(address = LUT_PROGRAM_ID)]
    /// CHECK: Lut Prog
    pub lut_program: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct PsiCompleteEvent {
    pub user: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Aborted")] AbortedComputation,
    #[msg("No Cluster")] ClusterNotSet,
}

#[error_code]
pub enum PsiError {
    #[msg("Registry Full")] RegistryFull,
}