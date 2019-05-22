pub fn price_per_cpu_hour(partition: &str) -> f64 {
    return 0.0;
}

pub fn deduct_service_units(account: &str, user_id: u32, expected_cost: f64) -> Result<(), &str> {
    Ok(())
}

pub fn expected_cost(partition: &str, max_cpus: u32, time_limit_minutes: u32) -> f64 {
    return price_per_cpu_hour(partition) * (max_cpus as f64) * ((time_limit_minutes as f64) / 60.0);
}