use crate::cli::Arguments;
use crate::error::UserError;

pub fn ensure_activated(args: &Arguments) -> Result<(), UserError> {
    if args.activated {
        Ok(())
    } else {
        Err(UserError::NotActivated)
    }
}
