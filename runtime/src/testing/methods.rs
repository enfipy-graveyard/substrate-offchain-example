use super::*;

use codec::Decode;
use sp_core::offchain::{
    testing::{TestOffchainExt, TestTransactionPoolExt, PoolState},
    OffchainExt, TransactionPoolExt,
};
use sp_runtime::traits::Dispatchable;
use std::sync::Arc;

pub fn exec_with_offchain() -> (sp_io::TestExternalities, Arc<parking_lot::RwLock<PoolState>>) {
    // What authorities will be available during tests
    let local_keys = vec![42.into()];
    let mut ext = new_test_ext(local_keys);
    let (offchain, _) = TestOffchainExt::new();
    let (pool, state) = TestTransactionPoolExt::new();
    ext.register_extension(OffchainExt::new(offchain));
    ext.register_extension(TransactionPoolExt::new(pool));
    (ext, state)
}

pub fn new_test_ext(local_keys: Vec<UintAuthorityId>) -> sp_io::TestExternalities {
    UintAuthorityId::set_all_keys(local_keys.clone());

    let mut t = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
    crate::example::GenesisConfig::<TestRuntime> { authorities: local_keys }.assimilate_storage(&mut t).unwrap();
    t.into()
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
    if let Some(_) = Example::authority_id() {
        // Run offchain logic
        Example::offchain();
        // if there are any txs submitted to the queue, dispatch them
        let transactions = &mut state.write().transactions;
        let count = transactions.len();
        while let Some(t) = transactions.pop() {
            let e: Extrinsic = Decode::decode(&mut &*t).unwrap();
            let (who, _) = e.0.expect("Invalid transaction origin");
            let call = e.1;
            let _ = call.dispatch(Origin::signed(who.into())).unwrap();
        }
        Some(count)
    } else {
        None
    }
}
