use crate::error::{AppError, Code};
use crate::utils::env_var::load_env_var::load_env_var;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::{authentication::Credentials};
use lettre::{Message, SmtpTransport, Transport, Address};

fn get_from() -> Result<Mailbox, AppError> {
    let from_adress = Address::new(load_env_var("MAIL_USER")?, load_env_var("MAIL_DOMAIN")?).map_err(|_| AppError { 
        code: Code::Internal, 
        message: String::from("failed to mount from address on send email") 
    })?;

    Ok(Mailbox::new(Some(load_env_var("MAIL_NAME")?), from_adress))
 }

fn get_transport() -> Result<SmtpTransport, AppError> {
    let smtp_port = load_env_var("MAIL_SMTP_PORT")?
    .parse::<u16>()
    .map_err(|_| AppError { 
        code: Code::Internal, 
        message: String::from("failed to parse value of env MAIL_SMTP_PORT") 
    })?;

    Ok(SmtpTransport::relay(&load_env_var("MAIL_SMTP_SERVER")?)
        .map_err(|_| AppError { 
                code: Code::Internal, 
                message: String::from("failed to build transport") 
            })?
        .port(smtp_port)
        .credentials(Credentials::new(
            load_env_var("MAIL_USER_ADRESS")?, 
            load_env_var("MAIL_USER_PASS")?))
        .build()
    )
 }


pub fn send_email(to_adress: String, subject:&str, body:String) -> Result<String, AppError> {
    let message = Message::builder()
        .from(get_from()?)
        .to(to_adress.parse().map_err(|_| AppError { 
            code: Code::Internal, 
            message: String::from("failed to parse value of recipient address")
        })?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)
        .map_err(|_| AppError { 
            code: Code::Internal, 
            message: String::from("failed to construct the message") 
        })?;

    match get_transport()?.send(&message) {
        Ok(_) => Ok(String::from("E-mail send successfully")),
        Err(error) => {
            println!("{:?}", error);
            Err(AppError { code: Code::Internal, message: String::from("failed to send email") })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_email() {
        dotenv::from_filename(".env.test").ok();
        let response = send_email(load_env_var("MAIL_USER_ADRESS").unwrap(), "email test", String::from("<div>This is a teste</div>")).unwrap();
        
        assert_eq!(response, "E-mail send successfully")
    }
}