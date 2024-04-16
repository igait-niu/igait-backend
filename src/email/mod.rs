pub fn send_email (
    to:      String,
    subject: String,
    body:    String
) -> Result<(), ureq::Error> {
    ureq::post("https://email-service.igaitniu.workers.dev/")
        .send_form(&[
            ( "API_KEY", &std::env::var("IGAIT_ACCESS_KEY").expect("MISSING IGAIT_ACCESS_KEY!") ),
            ( "to",      &to      ),
            ( "subject", &subject ),
            ( "body",    &body    )
        ])?;
    Ok(())
}