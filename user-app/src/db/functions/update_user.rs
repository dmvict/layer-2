use std::sync::Arc;

use reqwest::Client;
use service_common::{make_request, models::EmailWebsite, service_config::ServiceConfig};

use crate::{
  http::models::{NewUser, PublicAddress, PublicUser},
  jcr::models::{JCRInForUser, JCROut},
};

use anyhow::{bail, Context, Result};

use service_common::config::models::ServiceVariable;

pub async fn update_user(
  client: Arc<Client>,
  user: EmailWebsite,
  new_user: NewUser,
  config: Arc<ServiceConfig>,
) -> Result<()> {
  //user 8081
  //fetch address
  let old_address = fetch_address(
    &client,
    user.clone(),
    &config.get_variable::<String>(&ServiceVariable::UserDbUrl)?,
  )
  .await?;
  //update address
  update_address(
    &client,
    user.clone(),
    new_user.address,
    &config.get_variable::<String>(&ServiceVariable::UserDbUrl)?,
  )
  .await?;
  //update user
  update_user_(
    &client,
    user,
    new_user.user,
    old_address,
    &config.get_variable::<String>(&ServiceVariable::UserDbUrl)?,
  )
  .await
}

async fn fetch_address(
  client: &Arc<Client>,
  user: EmailWebsite,
  url: &str,
) -> Result<PublicAddress> {
  let address_jcr = JCRInForUser::FetchAddress {
    user_id: None,
    user: Some(user),
  };
  match make_request::<_, JCROut>(client, url, &address_jcr)
    .await
    .context("Failed sending fetch address request")?
    .result()?
  {
    JCROut::ResponseAddress { address } => Ok(address.into()),
    _ => bail!("Invalid response from DB."),
  }
}

async fn update_address(
  client: &Arc<Client>,
  user: EmailWebsite,
  new_address: PublicAddress,
  url: &str,
) -> Result<()> {
  let update_address_jcr = JCRInForUser::UpdateAddress {
    user: user.clone(),
    new_address,
  };
  match make_request::<_, JCROut>(client, url, &update_address_jcr)
    .await
    .context("Failed sending update address request")?
    .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => bail!("Invalid response from DB."),
  }
}

async fn update_user_(
  client: &Arc<Client>,
  user: EmailWebsite,
  new_user: PublicUser,
  rollback_address: PublicAddress,
  url: &str,
) -> Result<()> {
  let update_user_jcr = JCRInForUser::UpdateUser {
    user: user.clone(),
    new_user,
  };
  match make_request::<_, JCROut>(client, url, &update_user_jcr)
    .await
    .context("Failed sending update user request")?
    .result()?
  {
    JCROut::Success { message: _ } => Ok(()),
    _ => {
      update_address(client, user, rollback_address, url)
        .await
        .context("Rollback failed")?;
      bail!("Failed to update User")
    }
  }
}
