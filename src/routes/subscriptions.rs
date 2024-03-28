use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let username = SubscriberName::parse(value.username)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(NewSubscriber { email, username })
    }
}

// #[tracing::instrument(skip(form, pool))]
#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool, email_client),
fields(
subscriber_email = %form.email,
subscriber_name = %form.username
)
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_subscriber(&pool, &new_subscriber).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if email_client
        .send_email(
            new_subscriber.email,
            "Welcome!",
            "Welcome to our newsletter!",
            "Welcome to our newsletter!",
        )
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

// #[tracing::instrument(
// skip(pool, new_subscriber),
// fields(
//     subscriber_email = %new_subscriber.email,
//     subscriber_name = %new_subscriber.username.as_ref()
// )
// )]
pub async fn insert_subscriber(
    pool: &PgPool,
    // form: &FormData,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.username.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    username: String,
}
