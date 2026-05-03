use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DataSourceTypes {
    String(String),
    Vec(Vec<String>)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DruidMetric {
    pub value: Option<f64>,
    pub metric: Option<String>,
    pub service: Option<String>,
    pub host: Option<String>,
    #[serde(alias = "dutyGroup")]
    pub duty_group: Option<String>,
    #[serde(alias = "dataSource", default = "default_data_string")]
    pub data_source: DataSourceTypes,
    #[serde(alias = "cpuName")]
    pub cpu_name: Option<String>,
    #[serde(alias = "cpuTime")]
    pub cpu_time: Option<String>,
    pub id: Option<String>,
}

impl DruidMetric {
    pub fn get_string_field(&self, field: &str) -> Result<&Option<String>, String> {
        match field {
            "metric" => Ok(&self.metric),
            "service" => Ok(&self.service),
            "host" => Ok(&self.host),
            "duty_group" => Ok(&self.duty_group),
            "cpu_name" => Ok(&self.cpu_name),
            "cpu_time" => Ok(&self.cpu_time),
            "id" => Ok(&self.id),
            _ => Err(format!("Field {} not found", field)),
        }
    }
}

fn default_data_string() -> DataSourceTypes {
    DataSourceTypes::String(String::default())
}
