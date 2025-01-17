use strum::AsRefStr;

#[derive(AsRefStr, Clone)]
pub enum CookieKey {
    #[strum(serialize = "session_id")]
    Id,
    #[strum(serialize = "session_signature")]
    Signature,
}

pub const SESSION_ID: CookieKey = CookieKey::Id;
pub const SESSION_SIGNATURE: CookieKey = CookieKey::Signature;

pub fn get_from_str<'a>(cookies: &'a str, key: &str) -> Option<&'a str> {
    cookies
        .split(';')
        .find_map(|cookie| match cookie.split_once('=') {
            Some((k, v)) => {
                if k.trim() == key {
                    Some(v)
                } else {
                    None
                }
            }
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_cookie() {
        let cookie_str = "session_id=de3da054-5eac-4ea6-959b-7b117188d883; some_other_cookie=test";
        let session_id =
            get_from_str(cookie_str, SESSION_ID.as_ref()).expect("failed to get session_id");
        assert_eq!(session_id, "de3da054-5eac-4ea6-959b-7b117188d883");

        let other =
            get_from_str(cookie_str, "some_other_cookie").expect("failed to get some_other_cookie");
        assert_eq!(other, "test");
    }
}
