use crate::models::{Config, Meeting, Role};
use chrono::{DateTime, Local};
use std::collections::HashMap;

pub struct Calculator<'a> {
    config: &'a Config,
}

impl<'a> Calculator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// Calculate cost for a meeting given start and end times.
    pub fn calculate_cost(
        &self,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        attendees: &HashMap<Role, u32>,
    ) -> f64 {
        let duration_minutes = (end_time - start_time).num_minutes();
        if duration_minutes <= 0 {
            return 0.0;
        }
        let hours = duration_minutes as f64 / 60.0;

        let base_cost: f64 = attendees
            .iter()
            .map(|(role, count)| {
                let rate = self.config.get_rate(role);
                rate * (*count as f64) * hours
            })
            .sum();

        // Apply multipliers
        let mut total = base_cost;

        if self.config.include_benefits {
            total *= 1.4; // 40% overhead for benefits
        }

        total *= self.config.context_switch_multiplier;

        total
    }

    /// Calculate cost per minute for real-time display
    pub fn cost_per_minute(&self, attendees: &HashMap<Role, u32>) -> f64 {
        let cost_per_hour: f64 = attendees
            .iter()
            .map(|(role, count)| {
                let rate = self.config.get_rate(role);
                rate * (*count as f64)
            })
            .sum();

        let mut cost = cost_per_hour / 60.0;

        if self.config.include_benefits {
            cost *= 1.4;
        }

        cost *= self.config.context_switch_multiplier;

        cost
    }

    pub fn update_meeting_cost(&self, meeting: &mut Meeting) {
        meeting.cost = self.calculate_cost(meeting.start_time, meeting.end_time, &meeting.attendees);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_calculate_cost() {
        let config = Config::default();
        let calc = Calculator::new(&config);

        let mut attendees = HashMap::new();
        attendees.insert(Role::Engineer, 3);
        attendees.insert(Role::Director, 1);

        let start_time = Local::now();
        let end_time_60 = start_time + Duration::minutes(60);
        let end_time_30 = start_time + Duration::minutes(30);

        // 3 engineers at $100/hr + 1 director at $200/hr = $500/hr
        // For 60 minutes = $500
        let cost = calc.calculate_cost(start_time, end_time_60, &attendees);
        assert_eq!(cost, 500.0);

        // For 30 minutes = $250
        let cost = calc.calculate_cost(start_time, end_time_30, &attendees);
        assert_eq!(cost, 250.0);
    }

    #[test]
    fn test_cost_per_minute() {
        let config = Config::default();
        let calc = Calculator::new(&config);

        let mut attendees = HashMap::new();
        attendees.insert(Role::Engineer, 2);

        // 2 engineers at $100/hr = $200/hr = $3.33.../min
        let cost = calc.cost_per_minute(&attendees);
        assert!((cost - 3.333333).abs() < 0.001);
    }
}
