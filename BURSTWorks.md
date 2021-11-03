# **How Burstcoin BRS works**

# BlockchainProcessor

## 1. Configuration Properties
Get all the relevant config properties.

## 2. Add Event Listeners
Add 3-4 event listeners, depending on configuration.

* Event.BLOCK_SCANNED
    * log block height to logger
* Event.BLOCK_PUSHED
    * log block height to logger
    * transactionProcessor.revalidateUnconfirmedTransactions
* Event.AFTER_BLOCK_APPLY
    * if trimDerivedTables
        * Every MAX_ROLLBACK blocks, trim derived tables

## 3. Add Genesis Block and Check Database
1. Add the genesis block to the database. This is the only hard-coded and pre-evaluated block.
2. If not configured to skip this, check the db for consistency and log a warning to rollback if not.

## 4. Runnable to Get Blocks
1. Return immediately if configured for offline development.
2. Create a request json object for later use in getting the cumulative difficulty from a peer.
3. While thread is running:
    1. If bool `getMoreBlocks` is false, return immediately.
    2. Syncronize and unlock the download cache.
    3. If download cache is full, return immediately.
    4. Set `peerHasMore` to true for later use.
    5. Get a connected peer or log an error and return immediately.
    6. Request cumulative difficulty from the peer and return immediately if response is null.
    7. Ensure response has a `blockchainHeight` attribute or return "peer has no chainheight" error.
        1. Set `lastBlockchainFeeder` to peer.
        2. Set `lastBlockchainFeederHeight` to value of `blockchainHeight`.
    8. Set `curCumulativeDifficulty` to the `downloadCache`'s cumulative difficulty.
    9. Get the cumulative difficulty from the peer and store it in `peerCumulativeDifficulty`, returning an error if peer has no cumulative difficulty.
    10. Compare `peerCumulativeDifficulty` to `curCumulativeDifficulty` and return early if `peerCumulativeDifficulty` is <= `curCumulativeDifficulty` --- This ends this iteration of the loop because the peer's cumulative difficulty is lower than mine and therefore they have a shorter chain and no additional blocks to download.
    11. If curCumulativeDifficulty > peerCumulativeDifficulty, log to logger and continue.
    12. Find the highest common block between my chain and peer's.
        1. Set `commonBlockId` to genesis block id
        2. set `cacheLastBlockId` to downloadcache's last block id
        3. if `cacheLastBlockId` not equal genesis block id