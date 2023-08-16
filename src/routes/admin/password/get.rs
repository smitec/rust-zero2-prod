use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    }

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Change Password</title>
            </head>
            <body>
                {msg_html}
                <form action="/admin/password" method="post">
                    <label>Current password
                    <input
                        type="password"
                        placeholder="Current Password"
                        name="current_password"
                    >
                    </label>
                    <br>
                    <label>New Password
                    <input
                        type="password"
                        placeholder="Current Password"
                        name="new_password"
                    >
                    </label>
                    <br>
                    <label>New Password Again
                    <input
                        type="password"
                        placeholder="New Password"
                        name="new_password_check"
                    >
                    </label>
                    <br>
                    <button type="submit">Change Password</button>
                </form>
                <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>
        </html>"#
        )))
}
