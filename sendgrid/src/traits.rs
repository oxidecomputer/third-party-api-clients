#![allow(clippy::field_reassign_with_default)]
use anyhow::{anyhow, Result};

#[async_trait::async_trait]
pub trait MailOps {
    /// Send a plain text email.
    ///
    /// This is a nicer experience than using `post`.
    async fn send_plain_text(
        &self,
        subject: &str,
        message: &str,
        to: &[String],
        cc: &[String],
        bcc: &[String],
        from: &str,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl MailOps for crate::mail_send::MailSend {
    /// Send a plain text email.
    ///
    /// This is a nicer experience than using `post`.
    async fn send_plain_text(
        &self,
        subject: &str,
        message: &str,
        tos: &[String],
        ccs: &[String],
        bccs: &[String],
        from: &str,
    ) -> Result<()> {
        let mut mail: crate::types::PostMailSendRequest = Default::default();
        mail.subject = subject.to_string();
        mail.from = crate::types::FromEmailObject {
            email: from.to_string(),
            name: String::new(),
        };
        mail.content = vec![crate::types::Content {
            value: message.to_string(),
            type_: "text/plain".to_string(),
        }];
        let mut p: crate::types::Personalizations = Default::default();
        p.from = Some(mail.from.clone());
        for to in tos {
            p.to.push(crate::types::ReplyTo {
                email: to.to_string(),
                name: String::new(),
            });
        }
        for cc in ccs {
            p.cc.push(crate::types::CcBccEmailObject {
                email: cc.to_string(),
                name: String::new(),
            });
        }
        for bcc in bccs {
            p.bcc.push(crate::types::CcBccEmailObject {
                email: bcc.to_string(),
                name: String::new(),
            });
        }
        mail.personalizations = vec![p];

        let resp = self
            .client
            .request_raw(
                reqwest::Method::POST,
                "/mail/send",
                Some(reqwest::Body::from(serde_json::to_vec(&mail).unwrap())),
            )
            .await?;

        match resp.status() {
            http::StatusCode::ACCEPTED => Ok(()),
            s => Err(anyhow!("received response status: {:?}", s)),
        }
    }
}
