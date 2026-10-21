use crate::constants::SESSION_NAME;
use crate::models::admins_model;
use axum::extract::FromRequestParts;
use axum::http::{header::COOKIE, request::Parts, StatusCode, Uri};
use axum::response::{IntoResponse, Redirect, Response};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub admin: admins_model::Admins,
    pub ip: Option<SocketAddr>,
    pub url_path: String, //处理后相对URL路径,不带参数与页数
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Cookie {
    pub id: String,
    pub passwd: String,
    // ip_address -> Inet,
    pub timestamp: String,
    pub userid: i32,
}

impl Cookie {
    pub(crate) fn new() -> Cookie {
        Cookie {
            id: String::new(),
            passwd: String::new(),
            timestamp: String::new(),
            userid: 0,
        }
    }
}

#[derive(Debug)]
pub enum SessionError {
    NoLogin,
    InvalidUserID,
    Logout,
    NoRight,
    InvalidCookie,
    InvalidPassword,
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        log::warn!("Session extraction failed: {:?}", self);
        Redirect::to("/login").into_response()
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = SessionError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let cookie_header = parts
                .headers
                .get(COOKIE)
                .and_then(|value| value.to_str().ok())
                .unwrap_or("");

            let session_cookie = parse_cookie_value(cookie_header, SESSION_NAME);

            let path = parts.uri.path().to_string();
            let url_path = check_full_path(path);
            let ip = parts.extensions.get::<SocketAddr>().cloned();

            let admin = match session_cookie {
                Some(cookie_encode) => {
                    let cookie_decode =
                        base64::decode(cookie_encode).map_err(|_| SessionError::InvalidCookie)?;
                    let cookie_string = String::from_utf8(cookie_decode)
                        .map_err(|_| SessionError::InvalidCookie)?;
                    let s = serde_json::from_str::<Cookie>(&cookie_string)
                        .map_err(|_| SessionError::InvalidCookie)?;

                    if s.userid < 1 {
                        return Err(SessionError::Logout);
                    }

                    let admin =
                        admins_model::find_admin(s.userid).ok_or(SessionError::InvalidUserID)?;
                    if admin.password != s.passwd {
                        return Err(SessionError::InvalidPassword);
                    }
                    admin
                }
                None => return Err(SessionError::NoLogin),
            };

            if crate::models::rights_model::permit(url_path.clone(), &admin) {
                Ok(Session {
                    admin,
                    ip,
                    url_path,
                })
            } else {
                Err(SessionError::NoRight)
            }
        }
    }
}

fn parse_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    cookie_header.split(';').map(str::trim).find_map(|pair| {
        let mut kv = pair.splitn(2, '=');
        let key = kv.next()?.trim();
        if key == name {
            kv.next().map(|value| value.to_string())
        } else {
            None
        }
    })
}

fn check_full_path(full_path: String) -> String {
    if full_path.len() == 1 {
        return full_path;
    }

    let path_vec: Vec<&str> = full_path.trim_matches('/').split('/').collect();
    if path_vec.len() < 2 {
        return path_vec[0].to_string();
    }
    format!("{}/{}", path_vec[0], path_vec[1])
}
