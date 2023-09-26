use anyhow::{bail, Ok, Result, anyhow};
use std::{collections::HashMap, sync::Mutex};

use crate::config::models::ServiceVariable;

use super::models::ConfigVariable;

pub struct ServiceConfig(Mutex<HashMap<ServiceVariable, ConfigVariable>>);

impl ServiceConfig {
  pub fn new(config: HashMap<ServiceVariable, ConfigVariable>) -> Self {
    Self(Mutex::new(config))
  }

  pub fn get_variable<T: TryFrom<ConfigVariable>>(&self, variable_name: &ServiceVariable) -> Result<T> {
    if let Some(variable) = self.0.lock().unwrap().get(variable_name) {
      return variable.clone().try_into().map_err(|_| anyhow!("Fail to cast value"))
    }
    bail!("Variable with name {:?} not found.", variable_name)
  }

  pub fn update_config(&self, new_config: HashMap<ServiceVariable, ConfigVariable>) -> Result<()> {
    let mut config = self.0.lock().unwrap();
    for (k, v) in new_config.clone() {
      config.insert(k, v);
    }
    Ok(())
  }
}
