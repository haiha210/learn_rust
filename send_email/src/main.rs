use dotenv::dotenv;
use lettre::message::{MultiPart, SinglePart};
use std::fs;

use lettre::message::{header::ContentType, Attachment};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    dotenv().ok();
    let sender = "example.gm@gmail.com";
    let receiver = "example.gm@gmail.com";

    send_email(sender, receiver);
}

fn send_email(sender: &str, receiver: &str) {
    let multipart = get_multipart();

    let email = Message::builder()
        .from(
            format!("Sender <{sender}>", sender = sender)
                .parse()
                .unwrap(),
        )
        .to(format!("Receiver <{receiver}>", receiver = receiver)
            .parse()
            .unwrap())
        .subject("Sending email with Rust")
        .multipart(multipart)
        .unwrap();

    let credentials = get_credentials();

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(credentials)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

fn get_credentials() -> Credentials {
    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set.");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set.");
    println!("{}", smtp_username);
    println!("{}", smtp_password);

    Credentials::new(smtp_username, smtp_password)
}

fn get_multipart() -> MultiPart {
    let filename = String::from("sample.pdf");
    let filebody = fs::read("sample.pdf").unwrap();
    let content_type = ContentType::parse("application/pdf").unwrap();
    let attachment = Attachment::new(filename).body(filebody, content_type);

    let image_filename = "image".to_string();
    let image_filebody = std::fs::read("image.png".to_string()).unwrap();
    let image_attachment = Attachment::new_inline(image_filename.to_string())
        .body(image_filebody, ContentType::parse("image/png").unwrap());

    MultiPart::mixed()
        .multipart(
            MultiPart::alternative()
                .multipart(
                    MultiPart::related()
                        .singlepart(SinglePart::html(String::from(
                            "<p><b>Hello</b>, <i>world</i>! <img src=cid:image></p>",
                        )))
                        .singlepart(image_attachment),
                ),
        )
        .singlepart(attachment)
}
