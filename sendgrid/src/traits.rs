#![allow(clippy::field_reassign_with_default)]
use anyhow::Result;

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
        let mut mail: PostMailSendRequest = Default::default();
        mail.subject = subject.to_string();
        mail.from = crate::types::FromEmailObject {
            email: from.to_string(),
            name: String::new(),
        };
        mail.content = vec![crate::types::Content {
            value: message.to_string(),
            type_: "text/plain".to_string(),
        }];
        let mut p = Default::default();
        p.from = Some(mail.from);
        for to in tos {
            mail.personalizations.to.push(crate::types::ReplyTo {
                email: to.to_string(),
                name: String::new(),
            });
        }
        for cc in ccs {
            mail.personalizations
                .cc
                .push(crate::types::CcBccEmailObject {
                    email: cc.to_string(),
                    name: String::new(),
                });
        }
        for bcc in bccs {
            mail.personalizations
                .bcc
                .push(crate::types::CcBccEmailObject {
                    email: bcc.to_string(),
                    name: String::new(),
                });
        }
        mail.personalizations = vec![p];

        self.post(&mail).await
    }
}
