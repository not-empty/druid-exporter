use actix_web::web;

use crate::types::{app_state::AppState, druid_metrics::DruidMetric};

pub trait DispatcherStrategy {
    async fn send_event(
        &self,
        metrics: &[DruidMetric],
        state: web::Data<AppState>,
    );
}

pub struct DispatcherNavigator<T: DispatcherStrategy> {
    pub strategy: T,
}

impl<T: DispatcherStrategy> DispatcherNavigator<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }

    pub async fn send(
        &self,
        metrics: &[DruidMetric],
        state: web::Data<AppState>,
    ) {
        self.strategy.send_event(
            metrics,
            state,
        ).await
    }
}
