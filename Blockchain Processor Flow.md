## First, only do work if:
##   * more blocks are needed
##   * the `downloadCache` can be write-lastBlockchainFeeder
##   * the `downloadCache` is not full.

If `getMoreBlocks` false return

Write-lock `downloadCache`

If `downloadCache` full return


## Connect to a random, connected peer and get its cumulative difficulty and blockchain height. Cache them.
## Then compare the local downloadCache cumulative difficulty to the peer and give up if local is bigger.
## If the peer is bigger, cache the peer and its chainheight.

Set `peerHasMore` true

Get random connected peer
    If no connected peers return

Get peer's cumulative difficulty
    If no reponse return
    If response has no `blockchainHeight` return

Store `lastBlockchainFeeder` as current peer
Store `lastBlockchainFeederHeight` as response's `blockchainHeight`

Store `curCumulativeDifficulty` from `downloadCache`
Store `peerCumulativeDifficulty` from peer's response field `cumulativeDifficulty`
    If `peerCumulativeDifficulty` is null return

If `peerCumulativeDifficulty is <= `curCumulativeDifficulty` return



Store `commonBlockId` as genesis block id
Store `cacheLastBlockId` as last block id in `downloadCache`

If `cacheLastBlockId` is not the genesis block
    Get the latest milestone block in common with the peer

### NOTE: If the last block in our chain is not the common milestone block with the peer, then a fork is being downloaded.
### This may or may not be worth it. `canBeFork` will check if the block is too far back to be worth it.

Store `saveInCache` as true

If `commonBlockId` != `cacheLastBlockId` and `canBeFork` is true      <------ Expand `canBeFork` into flow
    Store `commonBlockId` as more precise common block id
        If peer has no more blocks or gives id as 0 then return
    Store `saveInCache` as false
    Reset the fork block cache
Else blacklist the peer and return


Request blocks from peer starting at `commonBlockId`
    Store `nextBlocks` as an array of blocks received from peer
    If no blocks received return

Store `lastBlock` as the `downloadCache`'s `commonBlockId`
    If `lastBlock` is null return



## Here the process loops over the received blocks and ensures they fit in the chain

FOR EACH block IN nextBlocks
    Store `height` as `lastBlock`'s height + 1
    TRY
    CATCH blockOutOfOrderException
