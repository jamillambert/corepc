// SPDX-License-Identifier: CC0-1.0

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v0.17.1`.
//!
//! A `x` marks methods that are implemented _and_ tested.

/// JSON-RPC types by API section.
pub mod blockchain;
pub mod control;
pub mod generating;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod util;
pub mod wallet;
pub mod zmq;

/// == Blockchain ==
///
/// - [x] `getbestblockhash`
/// - [x] `getblock "blockhash" ( verbosity )`
/// - [x] `getblockchaininfo`
/// - [ ] `getblockcount`
/// - [ ] `getblockhash height`
/// - [ ] `getblockheader "hash" ( verbose )`
/// - [ ] `getblockstats hash_or_height ( stats )`
/// - [ ] `getchaintips`
/// - [ ] `getchaintxstats ( nblocks blockhash )`
/// - [ ] `getdifficulty`
/// - [ ] `getmempoolancestors txid (verbose)`
/// - [ ] `getmempooldescendants txid (verbose)`
/// - [ ] `getmempoolentry txid`
/// - [ ] `getmempoolinfo`
/// - [ ] `getrawmempool ( verbose )`
/// - [ ] `gettxout "txid" n ( include_mempool )`
/// - [ ] `gettxoutproof ["txid",...] ( blockhash )`
/// - [ ] `gettxoutsetinfo`
/// - [ ] `preciousblock "blockhash"`
/// - [ ] `pruneblockchain`
/// - [ ] `savemempool`
/// - [ ] `scantxoutset <action> ( <scanobjects> )`
/// - [ ] `verifychain ( checklevel nblocks )`
/// - [ ] `verifytxoutproof "proof"`
#[allow(unused_imports)] // TODO: Remove this.
pub use blockchain::*;

///  == Control ==
///
/// - [ ] `getmemoryinfo ("mode")`
/// - [ ] `help ( "command" )`
/// - [ ] `logging ( <include> <exclude> )`
/// - [ ] `stop`
/// - [ ] `uptime`
#[allow(unused_imports)] // TODO: Remove this.
pub use control::*;

///  == Generating ==
///
/// - [ ] `generate nblocks ( maxtries )`
/// - [ ] `generatetoaddress nblocks address (maxtries)`
#[allow(unused_imports)] // TODO: Remove this.
pub use generating::*;

///  == Mining ==
///
/// - [ ] `getblocktemplate ( TemplateRequest )`
/// - [ ] `getmininginfo`
/// - [ ] `getnetworkhashps ( nblocks height )`
/// - [ ] `prioritisetransaction <txid> <dummy value> <fee delta>`
/// - [ ] `submitblock "hexdata"  ( "dummy" )`
#[allow(unused_imports)] // TODO: Remove this.
pub use mining::*;

///  == Network ==
///
/// - [ ] `addnode "node" "add|remove|onetry"`
/// - [ ] `clearbanned`
/// - [ ] `disconnectnode "\[address\]" \[nodeid\]`
/// - [ ] `getaddednodeinfo ( "node" )`
/// - [ ] `getconnectioncount`
/// - [ ] `getnettotals`
/// - [x] `getnetworkinfo`
/// - [ ] `getpeerinfo`
/// - [ ] `listbanned`
/// - [ ] `ping`
/// - [ ] `setban "subnet" "add|remove" (bantime) (absolute)`
/// - [ ] `setnetworkactive true|false`
#[allow(unused_imports)] // TODO: Remove this.
pub use network::*;

///  == Rawtransactions ==
///
/// - [ ] `combinepsbt ["psbt",...]`
/// - [ ] `combinerawtransaction ["hexstring",...]`
/// - [ ] `converttopsbt "hexstring" ( permitsigdata iswitness )`
/// - [ ] `createpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )`
/// - [ ] `createrawtransaction [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )`
/// - [ ] `decodepsbt "psbt"`
/// - [ ] `decoderawtransaction "hexstring" ( iswitness )`
/// - [ ] `decodescript "hexstring"`
/// - [ ] `finalizepsbt "psbt" ( extract )`
/// - [ ] `fundrawtransaction "hexstring" ( options iswitness )`
/// - [ ] `getrawtransaction "txid" ( verbose "blockhash" )`
/// - [ ] `sendrawtransaction "hexstring" ( allowhighfees )`
/// - [ ] `signrawtransaction "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] ["privatekey1",...] sighashtype )`
/// - [ ] `signrawtransactionwithkey "hexstring" ["privatekey1",...] ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )`
/// - [ ] `testmempoolaccept ["rawtxs"] ( allowhighfees )`
#[allow(unused_imports)] // TODO: Remove this.
pub use raw_transactions::*;

///  == Util ==
///
/// - [ ] `createmultisig nrequired ["key",...] ( "address_type" )`
/// - [ ] `estimatesmartfee conf_target ("estimate_mode")`
/// - [ ] `signmessagewithprivkey "privkey" "message"`
/// - [ ] `validateaddress "address"`
/// - [ ] `verifymessage "address" "signature" "message"`
#[allow(unused_imports)] // TODO: Remove this.
pub use util::*;

///  == Wallet ==
///
/// - [ ] `abandontransaction "txid"`
/// - [ ] `abortrescan`
/// - [ ] `addmultisigaddress nrequired ["key",...] ( "label" "address_type" )`
/// - [ ] `backupwallet "destination"`
/// - [ ] `bumpfee "txid" ( options )`
/// - [x] `createwallet "wallet_name" ( disable_private_keys )`
/// - [ ] `dumpprivkey "address"`
/// - [ ] `dumpwallet "filename"`
/// - [ ] `encryptwallet "passphrase"`
/// - [ ] `getaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `getaccountaddress (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `getaddressbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `getaddressesbylabel "label"`
/// - [ ] `getaddressinfo "address"`
/// - [x] `getbalance ( "(dummy)" minconf include_watchonly )`
/// - [x] `getnewaddress ( "label" "address_type" )`
/// - [ ] `getrawchangeaddress ( "address_type" )`
/// - [ ] `getreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `getreceivedbyaddress "address" ( minconf )`
/// - [ ] `gettransaction "txid" ( include_watchonly )`
/// - [ ] `getunconfirmedbalance`
/// - [ ] `getwalletinfo`
/// - [ ] `importaddress "address" ( "label" rescan p2sh )`
/// - [ ] `importmulti "requests" ( "options" )`
/// - [ ] `importprivkey "privkey" ( "label" ) ( rescan )`
/// - [ ] `importprunedfunds`
/// - [ ] `importpubkey "pubkey" ( "label" rescan )`
/// - [ ] `importwallet "filename"`
/// - [ ] `keypoolrefill ( newsize )`
/// - [ ] `listaccounts (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `listaddressgroupings`
/// - [ ] `listlabels ( "purpose" )`
/// - [ ] `listlockunspent`
/// - [ ] `listreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `listreceivedbyaddress ( minconf include_empty include_watchonly address_filter )`
/// - [ ] `listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )`
/// - [ ] `listtransactions (label count skip include_watchonly)`
/// - [ ] `listunspent ( minconf maxconf  ["addresses",...] [include_unsafe] [query_options])`
/// - [ ] `listwallets`
/// - [x] `loadwallet "filename"`
/// - [ ] `lockunspent unlock ([{"txid":"txid","vout":n},...])`
/// - [ ] `move (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `removeprunedfunds "txid"`
/// - [ ] `rescanblockchain ("start_height") ("stop_height")`
/// - [ ] `sendfrom (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `sendmany "" {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode")`
/// - [x] `sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode")`
/// - [ ] `setaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
/// - [ ] `sethdseed ( "newkeypool" "seed" )`
/// - [ ] `settxfee amount`
/// - [ ] `signmessage "address" "message"`
/// - [ ] `signrawtransactionwithwallet "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )`
/// - [ ] `unloadwallet ( "wallet_name" )`
/// - [ ] `walletcreatefundedpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable ) ( options bip32derivs )`
/// - [ ] `walletlock`
/// - [ ] `walletpassphrase "passphrase" timeout`
/// - [ ] `walletpassphrasechange "oldpassphrase" "newpassphrase"`
/// - [ ] `walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )`
#[allow(unused_imports)] // TODO: Remove this.
pub use wallet::*;

///  == Zmq ==
///
/// - [ ] `getzmqnotifications`
#[allow(unused_imports)] // TODO: Remove this.
pub use zmq::*;
