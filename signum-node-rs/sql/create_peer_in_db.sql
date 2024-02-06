-- SQLite
BEGIN TRANSACTION;
INSERT INTO peers (peer_address, application, version, share_address, network)
VALUES ("p2p.signumoasis.xyz:80", "SignumRust", "0.1.0", false, "Signum");
INSERT INTO peers (peer_address, application, version, share_address, network)
VALUES ("us-east.signum.network:8123", "BRS", "3.8.0", false, "Signum");
COMMIT;
select * from peers;
