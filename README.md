# signum-rs

## Design
A single executable file that contains all the logic required to be a full node, however it will respond to switches at launch time that can determine whether or not it activates portions of the code, giving the ability to launch the exe similarly to `./signum-rs -blockprocessor` to only launch the block processing module and the portion of the GRPC API required to communicate to that module. Each of the portions of the code will be contained as an actor, designed to only do one particular job.

This design will allow the program to launch as a single executable that can operate fully as a node (allowing people to run it at home with a single click, and only one binary running) while also allowing for scaling in the cloud in an orchestrated environment. The orchestrator just has to launch a service with a particular function enabled.

## Basic Design
Modules available:
    peer_service
        * Retrieves and forwards peers
        * Pushes peers to datastore
        * Checks peers for life
        * Passes peers to other modules on demand
        * Optionally removes old peers
    block_service
        * Downloads blocks
        * Verification of blocks
        * Pushes blocks to datastore
    transaction_service
        * Downloads and verifies transactions
        * Pushes transactions to datastore
        * Handles subscriptions
        * Handles escrow
        * Handles setting signa commitment
    smart_contract_service
        * Compiles, schedules, and computes smart contracts
        * _maybe this gets rolled into transactions_
    market_service
        * Processes data related to the built-in marketplace
    token_service
        * Creates and destroys tokens
        * Maintains a list of tokens
        * Token exchange
    alias_service
        * Creates and deletes aliases
    mining_service
        * Handles generating numbers for miners
        * Handles validating new blocks before block processor takes over?
        * Calculates bonus factor for hdd space
    datastore
        * Persists and retrieving blocks and other data
    UI
        * The user interface, handles user interaction
    api_service
        * Provides programmatic access to the node
        * Aware of active modules
        * Registers endpoints for handlers in each module
        * Forwards requests to api handlers in each module
        * Provides standard user API as well as extended capability on `/api` on port 8125/443 over https
        * Provides peer to peer over http on port 8123 by default
    pool_service
        * Provides pool mining capability
        * Exposes appropriate pool endpoints
        * Integrate with main API if possible
    explorer_service
        * Serves a chain explorer website
        * Integrate with API for the back end