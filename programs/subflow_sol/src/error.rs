use anchor_lang::error_code;

#[error_code]
pub enum SubflowError {
    #[msg("Max character length for service name exceeded")]
    MaxServiceNameExceeded,
    #[msg("Max URI length exceeded")]
    MaxURILengthExceeded,
    #[msg("You can't pause for more than the stipulated max pause time")]
    ExceededMaxPauseTime,
    #[msg("Can't perform action, Service is paused")]
    ServicePaused,
    #[msg("Can't unpause service till the pause period elapses")]
    CantUnpauseYet,
    #[msg("Token account has the wrong mint")]
    WrongMint,
    #[msg("This user is not subscribed")]
    UserNeverSubscribed,
}
