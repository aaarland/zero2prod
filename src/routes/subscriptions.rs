use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber", 
    skip(form, pool, email_client), fields(subscriber_email = %form.email, subscriber_name = %form.name
    ))
]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_subscriber(&new_subscriber, &pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(new_subscriber, &email_client)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}
impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(name = "Inserting a new subscriber", skip(pool, new_subscriber))]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'confirmed')"#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query. {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(
    name = "Sending a confirmation email",
    skip(new_subscriber, email_client)
)]
pub async fn send_confirmation_email(
    new_subscriber: NewSubscriber,
    email_client: &EmailClient,
) -> Result<(), reqwest::Error> {
    let confirmation_link = "https://api.aswaa.dev/subscriptions/confirm/";
    let plain_body = format!(
        "Welcome to our newsletter! \nVisit the link below to confirm your subscription: \n {confirmation_link}",
    );
    let html_body = format!(
        "Welcome to our newsletter! <br/>\
            Please confirm your subscription by clicking the link below: <br/>\
            Click <a href=\"{confirmation_link}\">here</a> to confirm your subscription",
    );

    email_client
        .send_email(new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}
