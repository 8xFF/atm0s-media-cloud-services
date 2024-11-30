use clerk_rs::{apis::Error, clerk::Clerk};
use reqwest::StatusCode;

#[derive(Clone)]
pub struct ClerkUserService {
    client: Clerk,
}

impl ClerkUserService {
    pub fn new(client: Clerk) -> Self {
        Self { client }
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> anyhow::Result<Option<clerk_rs::models::User>> {
        match clerk_rs::apis::users_api::User::get_user(&self.client, user_id).await {
            Ok(user) => Ok(Some(user)),
            Err(e) => match e {
                Error::ResponseError(e) => match e.status {
                    StatusCode::NOT_FOUND => Ok(None),
                    _ => anyhow::bail!(e.content),
                },
                _ => anyhow::bail!(e),
            },
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> anyhow::Result<Option<clerk_rs::models::User>> {
        match clerk_rs::apis::users_api::User::get_user_list(
            &self.client,
            Some(vec![email.to_string()]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        {
            Ok(users) => Ok(users.into_iter().next()),
            Err(e) => anyhow::bail!(e),
        }
    }

    pub async fn get_user_email(&self, user_id: &str) -> anyhow::Result<String> {
        let user = self.get_user_by_id(user_id).await?;
        match user {
            Some(user) => match user.email_addresses {
                Some(emails) => {
                    if !emails.is_empty() {
                        Ok(emails[0].email_address.clone())
                    } else {
                        anyhow::bail!("User has no email addresses")
                    }
                }
                None => anyhow::bail!("User has no email addresses"),
            },
            None => anyhow::bail!("User not found"),
        }
    }
}
