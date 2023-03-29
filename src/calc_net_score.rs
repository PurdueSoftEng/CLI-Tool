pub fn calc_net_score(ramp_up_score: f32, correctness_score: f32, bus_factor_score: f32, responsive_maintainer_score: f32, license_score: f32) -> f32 {

    // NetScore= (% Correctness)(0.5) + (% RampUp)(0.2) + (% BusFactor)(0.1) 
    // + (% ResponsiveMaintainer)(0.2) && License
    (((correctness_score * 0.5) + (ramp_up_score * 0.2) + (bus_factor_score * 0.1) + (responsive_maintainer_score * 0.2)) * license_score)
}
