pub struct Stake<'info> {
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Collection>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ]
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MetadataAccount>,
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]

    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
}