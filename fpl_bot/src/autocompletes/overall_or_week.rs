use crate::Context;

pub async fn autocomplete_overall_or_week<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    ["Overall".to_string(), "Current Gameweek".to_string()].into_iter()
}
