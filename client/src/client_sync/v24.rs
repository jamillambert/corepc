// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v24`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};

use crate::client_sync::into_json;
use crate::types::v24::*;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::client_sync::v23::AddressType;

crate::define_jsonrpc_minreq_client!("v24");

// == Blockchain ==
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__gettxout!();

// == Control ==
crate::impl_client_v17__stop!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();

// == Network ==
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_check_expected_server_version!({ [240001, 240100, 240200] });

// == Rawtransactions ==
crate::impl_client_v17__sendrawtransaction!();

// == Wallet ==
crate::impl_client_v17__createwallet!();
crate::impl_client_v22__unloadwallet!();
crate::impl_client_v22__loadwallet!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v19__getbalances!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__gettransaction!();
