// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Blockchain ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _, Wallet};
use node::client::client_sync;
use node::vtype::*;             // All the version specific types.
use node::mtype;

#[test]
fn blockchain__get_best_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json = node.client.get_best_block_hash().expect("rpc");
    let model: Result<mtype::GetBestBlockHash, _> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetBlockVerboseZero = node.client.get_block_verbose_zero(block_hash).expect("getblock verbose=0");
    let model: Result<mtype::GetBlockVerboseZero, _> = json.into_model();
    model.expect("GetBlock into model");

    let json: GetBlockVerboseOne = node.client.get_block_verbose_one(block_hash).expect("getblock verbose=1");
    let model: Result<mtype::GetBlockVerboseOne, GetBlockVerboseOneError> = json.into_model();
    model.expect("GetBlockVerbose into model");

    // TODO: Test getblock 2
    // let json = node.client.get_block_with_verbosity(block_hash, 2).expect("getblock verbosity 2");
    // assert!(json.into_model().is_ok());
}

#[test]
fn blockchain__get_blockchain_info__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBlockchainInfo = node.client.get_blockchain_info().expect("rpc");
    let model: Result<mtype::GetBlockchainInfo, GetBlockchainInfoError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_count__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBlockCount = node.client.get_block_count().unwrap();
    let _: mtype::GetBlockCount = json.into_model();
}

#[test]
#[cfg(not(feature = "v18_and_below"))]
fn blockchain__get_block_filter__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-blockfilterindex"]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetBlockFilter = node.client.get_block_filter(hash).expect("getblockfilter");
    let model: Result<mtype::GetBlockFilter, GetBlockFilterError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v22_and_below"))]
fn blockchain__get_block_from_peer() {
    use bitcoin::hashes::Hash;
    let (node1, _node2, _node3) = integration_test::three_node_network();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;

    // Create a dummy header and submit it
    let mut header = bitcoin::block::Header {
        version: bitcoin::block::Version::from_consensus(0x20000000),
        prev_blockhash: node1.client.best_block_hash().expect("best_block_hash failed"),
        merkle_root: bitcoin::TxMerkleNode::all_zeros(),
        time: now,
        bits: bitcoin::CompactTarget::from_consensus(0x207fffff),
        nonce: 0,
    };
    let target = header.target();
    while header.validate_pow(target).is_err() {
        header.nonce += 1;
    }
    node1.client.submit_header(&header).expect("submit_header failed");

    let hash = header.block_hash();
    let peer_id = node1.client.get_peer_info().expect("getpeerinfo").0[0].id;
    let _: () = node1.client.get_block_from_peer(hash, peer_id).expect("getblockfrompeer");
}

#[test]
fn blockchain__get_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBlockHash = node.client.get_block_hash(0).expect("getblockhash");
    let model: Result<mtype::GetBlockHash, _> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_header__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    // verbose = false
    let json: GetBlockHeader = node.client.get_block_header(&block_hash).expect("getblockheader");
    let model: Result<mtype::GetBlockHeader, GetBlockHeaderError> = json.into_model();
    model.unwrap();

    // verbose = true
    let json:GetBlockHeaderVerbose = node.client.get_block_header_verbose(&block_hash).expect("getblockheader");
    let model: Result<mtype::GetBlockHeaderVerbose, GetBlockHeaderVerboseError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_stats__modelled() {
    // Version 17 and 18 cannot call `getblockstats` if `-txindex` is not enabled.
    #[cfg(not(feature = "v18_and_below"))]
    getblockstats();

    // All versions including 17 and 18 can `getblockstats` if `-txindex` is enabled.
    getblockstats_txindex();
}

#[cfg(not(feature = "v18_and_below"))]
fn getblockstats() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json = node.client.get_block_stats_by_height(1).expect("getblockstats");
    json.into_model().unwrap();

    // No need for explicit types, used explicitly in test below.
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    json.into_model().unwrap();
}

fn getblockstats_txindex() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    // Get block stats by height.
    let json: GetBlockStats = node.client.get_block_stats_by_height(101).expect("getblockstats");
    let model: Result<mtype::GetBlockStats, GetBlockStatsError> = json.into_model();
    model.expect("GetBlockStats into model");

    // Get block stats by block hash.
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    json.into_model().unwrap();
}

#[test]
fn blockchain__get_chain_tips__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetChainTips = node.client.get_chain_tips().expect("getchaintips");
    let model: Result<mtype::GetChainTips, ChainTipsError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_chain_tx_stats__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetChainTxStats = node.client.get_chain_tx_stats().expect("getchaintxstats");
    let model: Result<mtype::GetChainTxStats, GetChainTxStatsError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v22_and_below"))]
fn blockchain__get_deployment_info__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetDeploymentInfo = node.client.get_deployment_info(&block_hash).expect("getdeploymentinfo");
    let model: Result<mtype::GetDeploymentInfo, GetDeploymentInfoError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v28_and_below"))]
fn blockchain__get_descriptor_activity__modelled() {
    let node = Node::with_wallet(Wallet::None, &["-coinstatsindex=1", "-txindex=1"]);

    let json: GetDescriptorActivity = node.client.get_descriptor_activity().expect("getdescriptoractivity");
    let model: Result<mtype::GetDescriptorActivity, GetDescriptorActivityError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_difficulty__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetDifficulty = node.client.get_difficulty().expect("getdifficulty");
    let _: mtype::GetDifficulty = json.into_model();
}

#[test]
#[cfg(feature = "TODO")]
fn blockchain__get_mempool_ancestors__modelled() {
    // We can probably get away with not testing this because it returns the same type as
    // `getmempoolentry` which is tested below (for verbose=true). For verbose=false it
    // just returns a txid.
}

#[test]
#[cfg(feature = "TODO")]
fn blockchain__get_mempool_descendants__modelled() {
    // Same justification as for `blockchain__get_mempool_ancestors__modelled`
}

#[test]
fn blockchain__get_mempool_entry__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();

    let json: GetMempoolEntry = node.client.get_mempool_entry(txid).expect("getmempoolentry");
    let model: Result<mtype::GetMempoolEntry, MempoolEntryError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_mempool_info__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    let json: GetMempoolInfo = node.client.get_mempool_info().expect("getmempoolinfo");
    let model: Result<mtype::GetMempoolInfo, GetMempoolInfoError> = json.clone().into_model();
    let info = model.unwrap();

    // Sanity check.
    assert_eq!(info.size, 1);
}

#[test]
fn blockchain__get_raw_mempool__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // verbose = false
    let json: GetRawMempool = node.client.get_raw_mempool().expect("getrawmempool");
    let model: Result<mtype::GetRawMempool, _> = json.clone().into_model();
    let mempool = model.unwrap();
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);

    // FIXME: Fails: JsonRpc(Json(Error("invalid type: map, expected a sequence", line: 1, column: 0)))
    // verbose = true
    // let json: GetRawMempoolVerbose = node.client.get_raw_mempool_verbose().expect("getrawmempool verbose");
    // let model: Result<mtype::GetRawMempoolVerbose, GetRawMempoolVerboseError> = json.into_model();
    // let mempool = model.unwrap();
    // // Sanity check.
    // assert_eq!(mempool.0.len(), 1);
}

#[test]
fn blockchain__get_tx_out__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    // Test the type and into model conversion code.
    let json: GetTxOut = node.client.get_tx_out(txid, 1).expect("gettxout");
    let model: Result<mtype::GetTxOut, GetTxOutError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_tx_out_proof() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let _ = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");
}

#[test]
fn blockchain__get_tx_out_set_info__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    let json: GetTxOutSetInfo = node.client.get_tx_out_set_info().expect("gettxoutsetinfo");
    let model: Result<mtype::GetTxOutSetInfo, GetTxOutSetInfoError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v23_and_below"))]
fn blockchain__get_tx_spending_prevout__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_address1, txid_1) = node.create_mempool_transaction();
    let (_address2, txid_2) = node.create_mempool_transaction();

    let inputs = vec![
        bitcoin::OutPoint {
            txid: txid_1,
            vout: 0,
        },
        bitcoin::OutPoint {
            txid: txid_2,
            vout: 0,
        },
    ];

    let json: GetTxSpendingPrevout = node.client.get_tx_spending_prevout(&inputs).expect("gettxspendingprevout");
    let model: Result<mtype::GetTxSpendingPrevout, GetTxSpendingPrevoutError> = json.into_model();
    let spending_prevout = model.unwrap();

    assert_eq!(spending_prevout.0.len(), 2);
    assert_eq!(spending_prevout.0[0].outpoint.txid, txid_1);
    assert_eq!(spending_prevout.0[0].outpoint.vout, 0);
}

#[test]
fn blockchain__precious_block() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");
    node.mine_a_block();

    let _: () = node.client.precious_block(hash).expect("preciousblock");
}

#[test]
fn blockchain__prune_blockchain() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &["-prune=550"]);
    let address = node.client.new_address().expect("Failed to get new address");

    let gen_result = node.client.generate_to_address(NBLOCKS, &address).expect("generate_to_address RPC call failed");
    assert_eq!(gen_result.0.len(), NBLOCKS, "generate_to_address did not return the expected number of block hashes");

    let target_height: u64 = 500;

    let _: Result<PruneBlockchain, _> = node.client.prune_blockchain(target_height);
}

#[test]
fn blockchain__savemempool() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_addr, _txid) = node.create_mempool_transaction();

    #[cfg(feature = "v22_and_below")]
    {
        let _: () = node.client.save_mempool().expect("savemempool");
    }

    #[cfg(not(feature = "v22_and_below"))]
    {
        let _: Result<SaveMempool, _> = node.client.save_mempool();
    }
}

#[cfg(not(feature = "v24_and_below"))]
#[test]
fn blockchain__scan_blocks_modelled() {
    let node = Node::with_wallet(Wallet::None, &["-blockfilterindex=1"]);

    // Arbitrary scan descriptor
    let scan_desc = "pkh(022afc20bf379bc96a2f4e9e63ffceb8652b2b6a097f63fbee6ecec2a49a48010e)";

    let json: ScanBlocksStart = node.client.scan_blocks_start(&[&scan_desc]).expect("scanblocks start");
    let model: Result<mtype::ScanBlocksStart, _> = json.into_model();
    model.unwrap();

    let _: Option<ScanBlocksStatus> = node.client.scan_blocks_status().expect("scanblocks status");

    let _: ScanBlocksAbort = node.client.scan_blocks_abort().expect("scanblocks abort");
}

#[test]
fn blockchain__verify_tx_out_proof__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    verify_tx_out_proof(&node).unwrap();
}

#[test]
fn blockchain__get_tx_out_proof__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    verify_tx_out_proof(&node).unwrap();
}

#[test]
fn blockchain__verify_chain() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let _: Result<VerifyChain, _> = node.client.verify_chain();
}

fn verify_tx_out_proof(node: &Node) -> Result<(), client_sync::Error> {
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let proof = node.client.get_tx_out_proof(&[txid])?;

    let json: VerifyTxOutProof = node.client.verify_tx_out_proof(&proof)?;
    let model: Result<mtype::VerifyTxOutProof, _> = json.into_model();
    let txids = model.unwrap();

    // sanity check
    assert_eq!(txids.0.len(), 1);

    Ok(())
}
