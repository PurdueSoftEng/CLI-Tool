

pub fn calc_responsive_maintainer(average_time_to_response: f64, max_time_to_response: f64) -> f64{
    let responsive_maintainer_ness = ((average_time_to_response / max_time_to_response as f64).abs()).abs();

    responsive_maintainer_ness.max(0.0).min(1.0)
}