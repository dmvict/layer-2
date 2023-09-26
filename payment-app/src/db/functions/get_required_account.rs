use std::collections::HashMap;
use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Context, Error, Result};
use reqwest::Client;
use service_common::{
  config::models::ServiceVariable, make_request, service_config::ServiceConfig,
};

use crate::db::models::{Account, AccountMonthlyBalance, SwitchMethodWithAccounts};
use crate::{
  db::models::SwitchMethod,
  jcr::{JCRInIntentDb, JCROut},
};

pub async fn get_required_account(
  company_name: String,
  client: Arc<Client>,
  config: Arc<ServiceConfig>,
) -> Result<String> {
  let method_and_account_jcr = JCRInIntentDb::FetchAccountsAndSwitchMethodByName {
    company_name: company_name.clone(),
  };
  let method_and_accounts = match make_request(
    &client,
    &config.get_variable::<String>(&ServiceVariable::IntentDbUrl)?,
    &method_and_account_jcr,
  )
  .await
  .context("Fail to send fetch fetch accounts and switch method by name")?
  {
    JCROut::MethodAndAccounts {
      method_and_accounts,
    } => Ok(method_and_accounts),
    _ => Err(anyhow!("Invalid response from intent-db")),
  }?;
  match SwitchMethod::from_str(&method_and_accounts.switch_method)? {
    SwitchMethod::DEFAULT => method_and_accounts
          .accounts.get(0).ok_or(anyhow!("No available account")).cloned().map(|a|a.id),
    SwitchMethod::ROUNDROBIN => roundrobin(&client, &config, method_and_accounts).await,
    SwitchMethod::LEASTPAID => 
      get_actual_monthly_report(
        &client,
        company_name,
        &config.get_variable::<String>(&ServiceVariable::IntentDbUrl)?,
      )
      .await?
      .into_iter()
      .min_by(|a, b| a.balance.cmp(&b.balance))
      .ok_or(anyhow!("Fail to find required account")).map(|a|a.account_id)
    ,
    SwitchMethod::MANUAL => method_and_accounts
      .accounts
      .into_iter()
      .find(|a| a.picked == 1)
      .map(|a| a.id)
      .ok_or(anyhow!("Fail to find required account")),
    SwitchMethod::CAP { amount } => {
      let combined_accounts =
        get_account_with_balance(company_name, &client, &config, method_and_accounts).await?;
      return if let Some(account) = combined_accounts
        .iter()
        .find(|a| a.account.picked == 1 && a.balance <= amount)
      {
        Ok(account.account.id.clone())
      } else {
        let account = combined_accounts
          .iter()
          .skip_while(|a| a.account.picked != 1)
          .skip(1)
          .skip_while(|a| a.balance > amount)
          .next()
          .unwrap_or(&combined_accounts[0])
          .account
          .clone();
        update_accounts(
          combined_accounts.into_iter().map(|c| c.account).collect(),
          account.id.clone(),
          &client,
          &config.get_variable::<String>(&ServiceVariable::IntentDbUrl)?,
        )
        .await?;
        Ok(account.id)
      };
    }
  }
}

async fn get_account_with_balance(
  company_name: String,
  client: &Arc<Client>,
  config: &Arc<ServiceConfig>,
  method_and_accounts: SwitchMethodWithAccounts,
) -> Result<Vec<CombinedAccount>, Error> {
  let balances: HashMap<String, i64> = get_actual_monthly_report(
    &client,
    company_name,
    &config.get_variable::<String>(&ServiceVariable::IntentDbUrl)?,
  )
  .await?
  .into_iter()
  .map(|b| (b.account_id, b.balance))
  .collect();
  let combined_accounts: Vec<CombinedAccount> = method_and_accounts
    .accounts
    .into_iter()
    .map(|account| {
      let balance = balances
        .get(&account.id)
        .unwrap_or(&0);
      CombinedAccount { account , balance: *balance }
    })
    .collect();
  Ok(combined_accounts)
}


async fn roundrobin(
  client: &Arc<Client>,
  config: &Arc<ServiceConfig>,
  method_and_accounts: SwitchMethodWithAccounts,
) -> Result<String> {
  dbg!(&method_and_accounts.accounts);
  let account = method_and_accounts
    .accounts
    .iter()
    .skip_while(|a| a.picked != 1)
    .skip(1)
    .next()
    .unwrap_or(&method_and_accounts.accounts[0])
    .clone();
  update_accounts(
    method_and_accounts.accounts,
    account.id.clone(),
    client,
    &config.get_variable::<String>(&ServiceVariable::IntentDbUrl)?,
  )
  .await?;
  Ok(account.id)
}

async fn get_actual_monthly_report(
  client: &Client,
  company_name: String,
  intent_db_url: &str,
) -> Result<Vec<AccountMonthlyBalance>> {
  let jcr = JCRInIntentDb::GetActualMonthlyReport { company_name };
  match make_request(&client, intent_db_url, &jcr)
    .await
    .context("Fail to send get actual report request")?
  {
    JCROut::AccountMonthlyBalances { balances } => Ok(balances),
    _ => Err(anyhow!("Invalid response form intent-db")),
  }
}

async fn update_accounts(
  accounts: Vec<Account>,
  account: String,
  client: &Client,
  intent_db_url: &str,
) -> Result<()> {
  for a in accounts {
      if a.id == account{
        let jcr = JCRInIntentDb::PutAccount {
          id: a.id,
          company_name: a.company_name,
          secret: a.secret,
          picked: 1,
          registration_date: a.registration_date,
        };
        match make_request::<_, JCROut>(client, intent_db_url, &jcr)
          .await
          .context("Fail to send put account request")?
          .result()?
        {
          JCROut::Success { .. } => Ok(()),
          _ => Err(anyhow!("Invalid response form intent-db")),
        }?;
      }
      else if a.picked == 1 {
        let jcr = JCRInIntentDb::PutAccount {
          id: a.id,
          company_name: a.company_name,
          secret: a.secret,
          picked: 0,
          registration_date: a.registration_date,
        };
        match make_request::<_, JCROut>(client, intent_db_url, &jcr)
          .await
          .context("Fail to send put account request")?
          .result()?
        {
          JCROut::Success { .. } => Ok(()),
          _ => Err(anyhow!("Invalid response form intent-db")),
        }?;   
      }
  }
  Ok(())
}

struct CombinedAccount {
  account: Account,
  balance: i64,
}
