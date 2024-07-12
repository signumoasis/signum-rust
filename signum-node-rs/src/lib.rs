use actix_web::HttpResponse;
use itertools::Itertools;

pub mod configuration;
pub mod flux_capacitor;
pub mod models;
pub mod peers;
pub mod srs_api;
pub mod telemetry;
pub mod workers;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tracing::instrument(skip(input))]
pub fn statistics_mode<T>(input: impl IntoIterator<Item = T>) -> Option<T>
where
    T: std::fmt::Debug + std::hash::Hash + std::cmp::Eq,
{
    input
        // Get an iterator from the input
        .into_iter()
        // Create a hashmap containing the T as keys and the number of instances of T as values
        .counts()
        // Get an iterator from the new HashMap
        .into_iter()
        // Get the set of maximum numbers of items as a new Vec<(T, i32)>
        .max_set_by_key(|(_, count)| *count)
        // Get an iterator for the Vec
        .into_iter()
        // Return the item if only one exists or an Error
        .exactly_one()
        // Convert the Result into an Option
        .ok()
        // Map the (T, i32) to just T
        .map(|(x, _)| x)
}

#[cfg(test)]
mod test {
    use crate::statistics_mode;

    #[test]
    fn statistics_mode_returns_some_max_value() {
        let test_data = vec![4, 1, 1, 2, 4, 3, 4, 5];

        let result = statistics_mode(test_data);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn statistics_mode_returns_none_if_no_mode() {
        let test_data_no_repeats = vec![1, 2, 3, 4, 5];
        let test_data_multiple_options = vec![1, 1, 2, 2, 3, 4, 5];

        let result = statistics_mode(test_data_no_repeats);
        assert_eq!(result, None, "Failed on no repeats");

        let result = statistics_mode(test_data_multiple_options);
        assert_eq!(result, None, "Failed on multiple options");
    }
}
