// SPDX-License-Identifier: CC0-1.0

//! RPC methods for the async Bitcoin Core client.
//! All functions return the version nonspecific, strongly typed types.

use bitcoin::{block, Block, BlockHash, OutPoint, Transaction, Txid};
use serde_json::value::RawValue;

use super::error::{decode_v25_or_v29, ParsedVersion};
use super::{
    into_json, Client, GetBestBlockHashError, GetBlockCountError, GetBlockError,
    GetBlockFilterError, GetBlockHashError, GetBlockHeaderError, GetBlockHeaderVerboseError,
    GetBlockVerboseError, GetBlockchainInfoError, GetRawMempoolError, GetRawTransactionError,
    GetTxOutError, ServerVersionError,
};
use crate::types::model::{
    GetBlockFilter, GetBlockHeaderVerbose, GetBlockVerboseOne, GetBlockchainInfo, GetTxOut,
};

/// Template trait for downstream RPC extensions.
// `async fn` in traits produces non-`Send` futures by default; we suppress this lint because
// this trait is intended only for use with concrete types (never `dyn BitcoinRpcs`).
#[allow(async_fn_in_trait)]
pub trait BitcoinRpcs {
    /// Call an RPC `method` with the given JSON `args` and deserialize the result.
    async fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        method: &str,
        args: &[serde_json::Value],
    ) -> super::Result<T>;
}

impl BitcoinRpcs for Client {
    async fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        method: &str,
        args: &[serde_json::Value],
    ) -> super::Result<T> {
        Client::call(self, method, args).await
    }
}

impl Client {
    /// Gets a block by blockhash.
    pub async fn get_block(&self, hash: &BlockHash) -> std::result::Result<Block, GetBlockError> {
        let json: crate::types::v25::GetBlockVerboseZero =
            self.call("getblock", &[into_json(hash)?, into_json(0)?]).await?;
        Ok(json.into_model().map_err(GetBlockError::IntoModel)?.0)
    }

    /// Gets the block count.
    pub async fn get_block_count(&self) -> std::result::Result<u64, GetBlockCountError> {
        let json: crate::types::v25::GetBlockCount = self.call("getblockcount", &[]).await?;
        Ok(json.into_model().0)
    }

    /// Gets the block hash for a height.
    pub async fn get_block_hash(
        &self,
        height: u32,
    ) -> std::result::Result<BlockHash, GetBlockHashError> {
        let json: crate::types::v25::GetBlockHash =
            self.call("getblockhash", &[into_json(height)?]).await?;
        Ok(json.into_model().map_err(GetBlockHashError::IntoModel)?.0)
    }

    /// Gets the hash of the chain tip.
    pub async fn get_best_block_hash(
        &self,
    ) -> std::result::Result<BlockHash, GetBestBlockHashError> {
        let json: crate::types::v25::GetBestBlockHash = self.call("getbestblockhash", &[]).await?;
        Ok(json.into_model().map_err(GetBestBlockHashError::IntoModel)?.0)
    }

    /// Gets the block header by blockhash.
    pub async fn get_block_header(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<block::Header, GetBlockHeaderError> {
        let json: crate::types::v25::GetBlockHeader =
            self.call("getblockheader", &[into_json(hash)?, into_json(false)?]).await?;
        Ok(json.into_model().map_err(GetBlockHeaderError::IntoModel)?.0)
    }

    /// Gets the block header with verbose output.
    pub async fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<GetBlockHeaderVerbose, GetBlockHeaderVerboseError> {
        let raw: Box<RawValue> =
            self.call("getblockheader", &[into_json(hash)?, into_json(true)?]).await?;

        match decode_v25_or_v29::<
            crate::types::v25::GetBlockHeaderVerbose,
            crate::types::v29::GetBlockHeaderVerbose,
        >(&raw)?
        {
            ParsedVersion::V29(json) =>
                Ok(json.into_model().map_err(GetBlockHeaderVerboseError::V29)?),
            ParsedVersion::V25(json) =>
                Ok(json.into_model().map_err(GetBlockHeaderVerboseError::V25)?),
        }
    }

    /// Gets a block by blockhash with verbose set to 1.
    pub async fn get_block_verbose(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<GetBlockVerboseOne, GetBlockVerboseError> {
        let raw: Box<RawValue> = self.call("getblock", &[into_json(hash)?, into_json(1)?]).await?;

        match decode_v25_or_v29::<
            crate::types::v25::GetBlockVerboseOne,
            crate::types::v29::GetBlockVerboseOne,
        >(&raw)?
        {
            ParsedVersion::V29(json) => Ok(json.into_model().map_err(GetBlockVerboseError::V29)?),
            ParsedVersion::V25(json) => Ok(json.into_model().map_err(GetBlockVerboseError::V25)?),
        }
    }

    /// Gets the block filter for a blockhash.
    pub async fn get_block_filter(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<GetBlockFilter, GetBlockFilterError> {
        let json: crate::types::v25::GetBlockFilter =
            self.call("getblockfilter", &[into_json(hash)?]).await?;
        json.into_model().map_err(GetBlockFilterError::IntoModel)
    }

    /// Gets information about the current state of the blockchain.
    pub async fn get_blockchain_info(
        &self,
    ) -> std::result::Result<GetBlockchainInfo, GetBlockchainInfoError> {
        let raw: Box<RawValue> = self.call("getblockchaininfo", &[]).await?;

        if let Ok(json) = serde_json::from_str::<crate::types::v29::GetBlockchainInfo>(raw.get()) {
            Ok(json.into_model().map_err(GetBlockchainInfoError::V29)?)
        } else if let Ok(json) =
            serde_json::from_str::<crate::types::v28::GetBlockchainInfo>(raw.get())
        {
            Ok(json.into_model().map_err(GetBlockchainInfoError::V28)?)
        } else {
            let json: crate::types::v25::GetBlockchainInfo = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(GetBlockchainInfoError::V25)?)
        }
    }

    /// Gets the transaction IDs currently in the mempool.
    pub async fn get_raw_mempool(&self) -> std::result::Result<Vec<Txid>, GetRawMempoolError> {
        let json: crate::types::v25::GetRawMempool = self.call("getrawmempool", &[]).await?;
        Ok(json.into_model().map_err(GetRawMempoolError::IntoModel)?.0)
    }

    /// Gets the raw transaction by txid.
    pub async fn get_raw_transaction(
        &self,
        txid: &Txid,
    ) -> std::result::Result<Transaction, GetRawTransactionError> {
        let json: crate::types::v25::GetRawTransaction =
            self.call("getrawtransaction", &[into_json(txid)?]).await?;
        Ok(json.into_model().map_err(GetRawTransactionError::IntoModel)?.0)
    }

    /// Gets details about an unspent transaction output.
    pub async fn get_tx_out(
        &self,
        outpoint: &OutPoint,
        include_mempool: bool,
    ) -> std::result::Result<Option<GetTxOut>, GetTxOutError> {
        let json: Option<crate::types::v25::GetTxOut> = self
            .call(
                "gettxout",
                &[
                    into_json(outpoint.txid)?,
                    into_json(outpoint.vout)?,
                    into_json(include_mempool)?,
                ],
            )
            .await?;
        match json {
            None => Ok(None),
            Some(json) => Ok(Some(json.into_model().map_err(GetTxOutError::IntoModel)?)),
        }
    }

    /// Returns the version integer reported by the server (e.g. `250200` for v25.2.0).
    pub async fn server_version(&self) -> std::result::Result<usize, ServerVersionError> {
        #[derive(serde::Deserialize)]
        struct NetworkVersion {
            version: usize,
        }

        let json: NetworkVersion = self.call("getnetworkinfo", &[]).await?;
        Ok(json.version)
    }
}
