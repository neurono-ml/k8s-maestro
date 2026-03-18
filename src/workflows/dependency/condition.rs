use crate::steps::result::StepResult;
use std::sync::Arc;

pub type ConditionFn = Arc<dyn Fn(&Vec<StepResult>) -> bool + Send + Sync>;

pub struct ConditionBuilder;

impl ConditionBuilder {
    pub fn all_success() -> ConditionFn {
        Arc::new(|deps| deps.iter().all(|r| r.is_success()))
    }

    pub fn any_success() -> ConditionFn {
        Arc::new(|deps| deps.iter().any(|r| r.is_success()))
    }

    pub fn all_failure() -> ConditionFn {
        Arc::new(|deps| deps.iter().all(|r| r.is_failure()))
    }

    pub fn any_failure() -> ConditionFn {
        Arc::new(|deps| deps.iter().any(|r| r.is_failure()))
    }

    pub fn custom<F>(condition: F) -> ConditionFn
    where
        F: Fn(&Vec<StepResult>) -> bool + Send + Sync + 'static,
    {
        Arc::new(condition)
    }

    pub fn output_greater_than(key: &str, threshold: i64) -> ConditionFn {
        let key = key.to_string();
        Arc::new(move |deps| {
            deps.iter()
                .any(|r| r.get_output(&key).and_then(|v| v.as_i64()).unwrap_or(0) > threshold)
        })
    }

    pub fn output_equals(key: &str, value: serde_json::Value) -> ConditionFn {
        let key = key.to_string();
        Arc::new(move |deps| deps.iter().any(|r| r.get_output(&key) == Some(&value)))
    }

    pub fn exit_code_equals(code: i32) -> ConditionFn {
        Arc::new(move |deps| deps.iter().any(|r| r.exit_code == code))
    }

    pub fn always_execute() -> ConditionFn {
        Arc::new(|_deps| true)
    }

    pub fn never_execute() -> ConditionFn {
        Arc::new(|_deps| false)
    }

    pub fn and(conditions: Vec<ConditionFn>) -> ConditionFn {
        Arc::new(move |deps| conditions.iter().all(|c| c(deps)))
    }

    pub fn or(conditions: Vec<ConditionFn>) -> ConditionFn {
        Arc::new(move |deps| conditions.iter().any(|c| c(deps)))
    }

    pub fn not(condition: ConditionFn) -> ConditionFn {
        Arc::new(move |deps| !condition(deps))
    }
}

impl StepResult {
    pub fn get_output_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.outputs.get(key)
    }

    pub fn get_output(&self, key: &str) -> Option<&serde_json::Value> {
        self.get_output_value(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::steps::result::StepStatus;
    use serde_json::json;

    fn create_result(step_id: &str, status: StepStatus) -> StepResult {
        StepResult::new(step_id).with_status(status)
    }

    fn create_result_with_output(step_id: &str, key: &str, value: serde_json::Value) -> StepResult {
        StepResult::new(step_id).with_output(key, value)
    }

    #[allow(dead_code)]
    fn create_result_with_exit_code(step_id: &str, exit_code: i32) -> StepResult {
        StepResult::new(step_id).with_exit_code(exit_code)
    }

    #[test]
    fn test_all_success_all_succeed() {
        let deps = vec![
            create_result("A", StepStatus::Success),
            create_result("B", StepStatus::Success),
        ];
        let condition = ConditionBuilder::all_success();
        assert!(condition(&deps));
    }

    #[test]
    fn test_any_success_one_succeeds() {
        let deps = vec![
            create_result("A", StepStatus::Success),
            create_result("B", StepStatus::Failure),
        ];
        let condition = ConditionBuilder::any_success();
        assert!(condition(&deps));
    }

    #[test]
    fn test_output_greater_than_passes() {
        let deps = vec![create_result_with_output("A", "data_size", json!(1500))];
        let condition = ConditionBuilder::output_greater_than("data_size", 1000);
        assert!(condition(&deps));
    }

    #[test]
    fn test_custom_condition() {
        let deps = vec![
            create_result("A", StepStatus::Success),
            create_result("B", StepStatus::Success),
        ];
        let condition = ConditionBuilder::custom(|deps| deps.len() > 1);
        assert!(condition(&deps));
    }
}
