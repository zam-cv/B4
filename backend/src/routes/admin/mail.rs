use crate::{config::CONFIG, database::Database, models};
use actix_web::{post, web, HttpResponse, Responder};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use utoipa::ToSchema;

const CONTEXT_PATH: &str = "/api/admin/mail";

#[derive(ToSchema, Deserialize)]
#[serde(tag = "type")]
pub struct Filters {
    pub by_age_range: Option<(i32, i32)>,
    pub by_user_type: Option<models::UserType>,
    pub by_gender: Option<models::Gender>,
    pub by_extension: Option<String>,
}

#[derive(ToSchema, Deserialize)]
pub struct EmailPayload {
    pub title: String,
    pub body: String,
    pub filters: Filters,
}

async fn send_email(email_payload: EmailPayload, database: &Database) -> anyhow::Result<()> {
    let creds = Credentials::new(CONFIG.smtp_username.clone(), CONFIG.smtp_password.clone());

    let mailer = SmtpTransport::starttls_relay(&CONFIG.smtp_host)?
        .port(587)
        .credentials(creds)
        .build();

    let emails = database
        .get_emails_by_user_filter(email_payload.filters)
        .await?;

    for to in emails {
        let email = Message::builder()
            .from(CONFIG.sender.parse()?)
            .to(to.parse()?)
            .subject(email_payload.title.clone())
            .header(ContentType::TEXT_HTML)
            .body(email_payload.body.clone())?;

        mailer.send(&email)?;
    }

    Ok(())
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The emails were sent")
  ),
  request_body = EmailPayload
)]
#[post("")]
pub async fn send_emails(
    email_payload: web::Json<EmailPayload>,
    database: web::Data<Database>,
) -> impl Responder {
    if let Err(e) = send_email(email_payload.into_inner(), &database).await {
        log::error!("Failed to send email: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[utoipa::path(
  context_path = CONTEXT_PATH,
  responses(
    (status = 200, description = "The count of users that match the filter", body = String)
  ),
  request_body = Filters
)]
#[post("/count")]
pub async fn get_user_count_by_user_filter(
    filters: web::Json<Filters>,
    database: web::Data<Database>,
) -> impl Responder {
    match database.get_user_count_by_user_filter(filters.into_inner()).await {
        Ok(count) => HttpResponse::Ok().body(count.to_string()),
        Err(e) => {
            log::error!("Failed to get user count: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}
