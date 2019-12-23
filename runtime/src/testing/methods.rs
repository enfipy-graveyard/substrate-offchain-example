use super::*;

use codec::Decode;
use sp_core::offchain::{
    testing::{TestOffchainExt, TestTransactionPoolExt, PoolState},
    OffchainExt, TransactionPoolExt,
};
use sp_runtime::traits::Dispatchable;
use std::sync::Arc;

pub fn exec_with_offchain() -> (sp_io::TestExternalities, Arc<parking_lot::RwLock<PoolState>>) {
    let mut ext = new_test_ext();
    let (offchain, _) = TestOffchainExt::new();
    let (pool, state) = TestTransactionPoolExt::new();
    ext.register_extension(OffchainExt::new(offchain));
    ext.register_extension(TransactionPoolExt::new(pool));
    (ext, state)
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap().into()
}

/// A utility function for our tests. It simulates what the system module does for us (almost
/// analogous to `finalize_block`).
///
/// This function increments the block number and simulates what we have written in
/// `decl_module` as `fn offchain_worker(_now: T::BlockNumber)`: run the offchain logic if the
/// current node is an authority.
///
/// Also, since the offchain code might submit some transactions, it queries the transaction
/// queue and dispatches any submitted transaction. This is also needed because it is a
/// non-runtime logic (transaction queue) which needs to mocked inside a runtime test.
pub fn seal_block(state: Arc<parking_lot::RwLock<PoolState>>) -> Option<usize> {
    let block = System::block_number();
    System::set_block_number(block + 1);
    // run offchain
    Example::offchain().expect("Failed to submit mint");

    // if there are any txs submitted to the queue, dispatch them
    let transactions = &mut state.write().transactions;
    let count = transactions.len();
    while let Some(t) = transactions.pop() {
        let e: Extrinsic = Decode::decode(&mut &*t).unwrap();
        if e.0.is_none() {
            let call = e.1;
            // Todo: Fix with signed transaction
            let _ = call.dispatch(Origin::signed(1)).unwrap();
        }
        // let (who, _) = e.0.expect("0");
    }
    Some(count)
}
