use retry::delay::Fixed;
use retry::{retry_with_index, OperationResult};
use std::time::Duration;

pub fn retry_rpc<O, R, E, OR>(mut rpc_operation: O) -> Result<R, retry::Error<E>>
where
    O: FnMut() -> OR,
    OR: Into<OperationResult<R, E>>,
{
    let retry_strategy = Fixed::from_millis(100).take(3);
    retry::retry(retry_strategy, rpc_operation)
}
