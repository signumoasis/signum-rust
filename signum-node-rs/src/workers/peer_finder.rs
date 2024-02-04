use anyhow::Result;
/// This worker finds new peers by querying the existing peers in the database.
/// If no peers exist in the database, it will read from the configuration bootstrap
/// peers list.
pub async fn peer_finder()-> Result<()> {
    todo!()
}
