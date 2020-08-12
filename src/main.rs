use sawtooth::{protocol::block::BlockBuilder, signing::hash::HashSigner};
use transact::protocol::{
    batch::BatchBuilder,
    transaction::{HashMethod, TransactionBuilder},
};

fn main() {
    let signer = HashSigner::default();

    let txn = TransactionBuilder::new()
        .with_family_name("test".into())
        .with_family_version("0.1".into())
        .with_inputs(vec![])
        .with_outputs(vec![])
        .with_payload_hash_method(HashMethod::SHA512)
        .with_payload(vec![])
        .build(&signer)
        .unwrap();

    let batch = BatchBuilder::new()
        .with_transactions(vec![txn])
        .build(&signer)
        .unwrap();

    BlockBuilder::new()
        .with_block_num(0)
        .with_batches(vec![batch])
        .with_state_root_hash(vec![])
        .with_previous_block_id("".into())
        .build(&signer)
        .unwrap();
}
