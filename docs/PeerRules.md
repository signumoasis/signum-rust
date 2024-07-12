## Peer Validation Rules
1. Peer's announcedAddress must resolve to an ip addres
2. Peer must respond to getInfo request
3.
## Peer Removal Rules
To remove a peer the following must be true:

1. Peer version is less than minimum supported version
OR
2. Peer version is supported:
    * Peer is disconnected
    * Peer is NOT blacklisted (to keep track of blacklist)
    * Peers.count greater than maxConnectedPeers (to avoid deleting all peers if internet connection drops)
