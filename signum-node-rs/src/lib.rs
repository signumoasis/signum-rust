use std::collections::HashMap;

use actix_web::HttpResponse;
use itertools::Itertools;

pub mod configuration;
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
    let frequencies = input.into_iter().counts();

    let mut frequencies = frequencies.into_iter().max_set_by_key(|(_, count)| *count);
    if frequencies.len() > 1 || frequencies.is_empty() {
        return None;
    };

    let (thing, _) = frequencies.remove(0);

    Some(thing)
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
