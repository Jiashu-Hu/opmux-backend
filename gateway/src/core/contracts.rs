#[derive(Debug, Clone)]
pub struct RoutePlan {
    pub vendor_id: String,
    pub model_id: String,
    pub fallback_plans: Vec<RoutePlan>,
}
