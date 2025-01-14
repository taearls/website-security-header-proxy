use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let origin = env.var("ORIGIN")?.to_string();
    let response = worker::Fetch::Request(req).send().await?;

    let mut headers = response.headers().clone();
    headers.set(
        "strict-transport-security",
        "max-age=31536000; includeSubDomains",
    )?;
    headers.set("x-frame-options", "SAMEORIGIN")?;
    headers.set("x-content-type-options", "nosniff")?;
    headers.set("referrer-policy", "no-referrer")?;
    headers.set("permissions-policy", "microphone 'none'")?;
    headers.set(
        "content-security-policy",
        "default-src 'self' cloudflareinsights.com *.cloudflareinsights.com; img-src 'self' *.cloudinary.com",
    )?;
    headers.set("permissions-policy", "microphone=(), geolocation=()")?;
    headers.set("access-control-allow-origin", &origin)?;
    Ok(response.with_headers(headers))
}