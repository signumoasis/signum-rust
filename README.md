# signum-rs

## Design
A single executable file that contains all the logic required to be a full node, however it will respond to switches at launch time that can determine whether or not it activates portions of the code, giving the ability to launch the exe similarly to `./signum-rs -blockprocessor` to only launch the block processing module and the portion of the GRPC API required to communicate to that module. Each of the portions of the code will be contained as an actor, designed to only do one particular job.

This design will allow the program to launch as a single executable that can operate fully as a node (allowing people to run it at home with a single click, and only one binary running) while also allowing for scaling in the cloud in an orchestrated environment. The orchestrator just has to launch a service with a particular function enabled.
