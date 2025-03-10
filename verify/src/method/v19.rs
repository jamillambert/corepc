// SPDX-License-Identifier: CC0-1.0

//! JSON RPC methods provided by Bitcoin Core v19.

use super::Method;

/// Data for the JSON RPC methods provided by Bitcoin Core v19.
pub const METHODS: &[Method] = &[
    Method::new_modeled("getbestblockhash", "GetBestBlockHash", "get_best_block_hash"),
    Method::new_modeled("getblock", "GetBlockVerbosityZero", "get_block"), // We only check one of the types.
    Method::new_modeled("getblockchaininfo", "GetBlockchainInfo", "get_blockchain_info"),
    Method::new_modeled("getblockcount", "GetBlockCount", "get_block_count"),
    Method::new_modeled("getblockfilter", "GetBlockFilter", "get_block_filter"),
    Method::new_modeled("getblockhash", "GetBlockHash", "get_block_hash"),
    Method::new_modeled("getblockheader", "GetBlockHeader", "get_block_header"),
    Method::new_modeled("getblockstats", "GetBlockStats", "get_block_stats"),
    Method::new_modeled("getchaintips", "GetChainTips", "get_chain_tips"),
    Method::new_modeled("getchaintxstats", "GetChainTxStats", "get_chain_tx_stats"),
    Method::new_modeled("getdifficulty", "GetDifficulty", "get_difficulty"),
    Method::new_modeled("getmempoolancestors", "GetMempoolAncestors", "get_mempool_ancestors"),
    Method::new_modeled(
        "getmempooldescendants",
        "GetMempoolDescendants",
        "get_mempool_descendants",
    ),
    Method::new_modeled("getmempoolentry", "GetMempoolEntry", "get_mempool_entry"),
    Method::new_modeled("getmempoolinfo", "GetMempoolInfo", "get_mempool_info"),
    Method::new_modeled("getrawmempool", "GetRawMempool", "get_raw_mempool"),
    Method::new_modeled("gettxout", "GetTxOut", "get_tx_out"),
    Method::new_string("gettxoutproof", "get_tx_out_proof"),
    Method::new_modeled("gettxoutsetinfo", "GetTxOutSetInfo", "get_tx_out_set_info"),
    Method::new_nothing("preciousblock", "precious_block"),
    Method::new_numeric("pruneblockchain", "prune_blockchain"),
    Method::new_nothing("savemempool", "save_mempool"),
    Method::new_modeled("scantxoutset", "ScanTxOutSet", "scan_tx_out_set"),
    Method::new_bool("verifychain", "verify_chain"),
    Method::new_modeled("verifytxoutproof", "VerifyTxOutProof", "verify_tx_out_proof"),
    Method::new_no_model("getrpcinfo", "GetRpcInfo", "get_rpc_info"),
    Method::new_no_model("getmemoryinfo", "GetMemoryInfoStats", "get_memory_info"),
    Method::new_string("help", "help"),
    Method::new_no_model("logging", "Logging", "logging"),
    Method::new_nothing("stop", "stop"),
    Method::new_numeric("uptime", "uptime"),
    Method::new_modeled("generatetoaddress", "GenerateToAddress", "generate_to_address"),
    Method::new_nothing("getblocktemplate", "get_block_template"),
    Method::new_nothing("getmininginfo", "get_mining_info"),
    Method::new_nothing("getnetworkhashps", "get_network_hashes_per_second"),
    Method::new_bool("prioritisetransaction", "prioritise_transaction"),
    Method::new_nothing("submitblock", "submit_block"),
    Method::new_nothing("submitheader", "submit_header"),
    Method::new_nothing("addnode", "add_node"),
    Method::new_nothing("clearbanned", "clear_banned"),
    Method::new_nothing("disconnectnode", "disconnect_node"),
    Method::new_no_model("getaddednodeinfo", "GetAddedNodeInfo", "get_added_node_info"),
    Method::new_numeric("getconnectioncount", "get_connection_count"),
    Method::new_no_model("getnettotals", "GetNetTotals", "get_net_totals"),
    Method::new_modeled("getnetworkinfo", "GetNetworkInfo", "get_network_info"),
    Method::new_no_model("getnodeaddresses", "GetNodeAddresses", "get_node_addresses"),
    Method::new_no_model("getpeerinfo", "GetPeerInfo", "get_peer_info"),
    Method::new_string("listbanned", "list_banned"), // v17 docs seem wrong, says no return.
    Method::new_nothing("ping", "ping"),
    Method::new_nothing("setban", "set_ban"),
    Method::new_nothing("setnetworkactive", "set_network_active"),
    Method::new_modeled("analyzepsbt", "AnalyzePsbt", "analyze_psbt"),
    Method::new_nothing("combinepsbt", "combine_psbt"),
    Method::new_nothing("combinerawtransaction", "combine_raw_transaction"),
    Method::new_nothing("converttopsbt", "convert_to_psbt"),
    Method::new_nothing("createpsbt", "create_psbt"),
    Method::new_nothing("createrawtransaction", "create_raw_transaction"),
    Method::new_nothing("decodepsbt", "decode_psbt"),
    Method::new_nothing("decoderawtransaction", "decode_raw_transaction"),
    Method::new_nothing("decodescript", "decode_script"),
    Method::new_nothing("finalizepsbt", "finalize_psbt"),
    Method::new_nothing("fundrawtransaction", "fund_raw_transaciton"),
    Method::new_nothing("getrawtransaction", "get_raw_transaction"),
    Method::new_modeled("joinpsbts", "JoinPsbts", "join_psbts"),
    Method::new_modeled("sendrawtransaction", "SendRawTransaction", "send_raw_transaction"),
    Method::new_nothing("signrawtransactionwithkey", "sign_raw_transaction_with_key"),
    Method::new_nothing("testmempoolaccept", "test_mempool_accept"),
    Method::new_modeled("utxoupdatepsbt", "UtxoUpdatePsbt", "utxo_update_psbt"),
    Method::new_modeled("createmultisig", "CreateMultisig", "create_multisig"),
    Method::new_modeled("deriveaddresses", "DeriveAddresses", "derive_addresses"),
    Method::new_nothing("estimatesmartfee", "estimate_smart_fee"),
    Method::new_no_model("getdescriptorinfo", "GetDescriptorInfo", "get_descriptor_info"),
    Method::new_string("signmessagewithprivkey", "sign_message_with_priv_key"),
    Method::new_modeled("validateaddress", "ValidateAddress", "validate_address"),
    Method::new_bool("verifymessage", "verify_message"),
    Method::new_nothing("abandontransaction", "abandon_transaction"),
    Method::new_nothing("abortrescan", "abort_rescan"),
    Method::new_modeled("addmultisigaddress", "AddMultisigAddress", "add_multisig_address"),
    Method::new_nothing("backupwallet", "backup_wallet"),
    Method::new_modeled("bumpfee", "BumpFee", "bump_fee"),
    Method::new_modeled("createwallet", "CreateWallet", "create_wallet"),
    Method::new_modeled("dumpprivkey", "DumpPrivKey", "dump_priv_key"),
    Method::new_modeled("dumpwallet", "DumpWallet", "dump_wallet"),
    Method::new_nothing("encryptwallet", "encrypt_wallet"),
    Method::new_modeled("getaddressesbylabel", "GetAddressesByLabel", "get_addresses_by_label"),
    Method::new_modeled("getaddressinfo", "GetAddressInfo", "get_address_info"),
    Method::new_modeled("getbalance", "GetBalance", "get_balance"),
    Method::new_modeled("getbalances", "GetBalances", "get_balances"),
    Method::new_modeled("getnewaddress", "GetNewAddress", "get_new_address"),
    Method::new_modeled("getrawchangeaddress", "GetRawChangeAddress", "get_raw_change_address"),
    Method::new_modeled("getreceivedbyaddress", "GetReceivedByAddress", "get_received_by_address"),
    Method::new_modeled("getreceivedbylabel", "GetReceivedByLabel", "get_received_by_label"),
    Method::new_modeled("gettransaction", "GetTransaction", "get_transaction"),
    Method::new_modeled(
        "getunconfirmedbalance",
        "GetUnconfirmedBalance",
        "get_unconfirmed_balance",
    ),
    Method::new_modeled("getwalletinfo", "GetWalletInfo", "get_wallet_info"),
    Method::new_nothing("importaddress", "import_addressss"),
    Method::new_nothing("importmulti", "import_multi"),
    Method::new_nothing("importprivkey", "import_priv_key"),
    Method::new_nothing("importprunedfunds", "import_pruned_funds"),
    Method::new_nothing("importpubkey", "import_pubkey"),
    Method::new_nothing("importwallet", "import_walet"),
    Method::new_nothing("keypoolrefill", "keypool_refill"),
    Method::new_modeled("listaddressgroupings", "ListAddressGroupings", "list_address_groupings"),
    Method::new_modeled("listlabels", "ListLabels", "list_labels"),
    Method::new_modeled("listlockunspent", "ListLockUnspent", "list_lock_unspent"),
    Method::new_modeled(
        "listreceivedbyaddress",
        "ListReceivedByAddress",
        "list_received_by_address",
    ),
    Method::new_modeled("listreceivedbylabel", "ListReceivedByLabel", "list_received_by_label"),
    Method::new_modeled("listsinceblock", "ListSinceBlock", "list_since_block"),
    Method::new_modeled("listtransactions", "ListTransactions", "list_transactions"),
    Method::new_modeled("listunspent", "ListUnspent", "list_unspent"),
    Method::new_no_model("listwalletdir", "ListWalletDir", "list_wallet_dir"),
    Method::new_modeled("listwallets", "ListWallets", "list_wallets"),
    Method::new_modeled("loadwallet", "LoadWallet", "load_wallet"),
    Method::new_bool("lockunspent", "lock_unspent"),
    Method::new_nothing("removeprunedfunds", "remove_pruned_funds"),
    Method::new_modeled("rescanblockchain", "RescanBlockchain", "rescan_blockchain"),
    Method::new_modeled("sendmany", "SendMany", "send_many"),
    Method::new_modeled("sendtoaddress", "SendToAddress", "send_to_address"),
    Method::new_nothing("sethdseed", "set_hd_seed"),
    Method::new_nothing("setlabel", "set_label"),
    Method::new_bool("settxfee", "set_tx_fee"),
    Method::new_modeled("setwalletflag", "SetWalletFlag", "set_wallet_flag"),
    Method::new_modeled("signmessage", "SignMessage", "sign_message"),
    Method::new_modeled(
        "signrawtransactionwithwallet",
        "SignRawTransactionWithWallet",
        "sign_raw_transaction_with_wallet",
    ),
    Method::new_nothing("unloadwallet", "unload_wallet"),
    Method::new_modeled(
        "walletcreatefundedpsbt",
        "WalletCreateFundedPsbt",
        "wallet_create_funded_psbt",
    ),
    Method::new_nothing("walletlock", "wallet_lock"),
    Method::new_nothing("walletpassphrase", "wallet_passphrase"),
    Method::new_nothing("walletpassphrasechange", "wallet_passphrase_change"),
    Method::new_modeled("walletprocesspsbt", "WalletProcessPsbt", "wallet_process_psbt"),
    Method::new_no_model("getzmqnotifications", "GetZmqNotifications", "get_zmq_notifications"),
];
