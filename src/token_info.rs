use rustc_serialize::{Encodable, Encoder};

static TOKEN_FORMAT: &'static str = "The token should have the format `token-realm-scope1-scope2`.";
static TOKEN_START_ERR: &'static str = "The token does not start with `token-`.";

static TOKEN_MISSING_SCOPES: &'static str = "The token does not have scopes defined.";
static TOKEN_MISSING_REALM: &'static str = "The token does not have a realm defined.";
static TOKEN_MISSING_UID: &'static str = "The token does not have a uid defined.";

pub type Scope = String;

pub type Realm = String;

pub type Uid = String;

#[derive(Debug)]
pub struct TokenInfo {
    scopes: Vec<Scope>,
    realm: Realm,
    uid: Option<Uid>
}

impl Encodable for TokenInfo {

    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("TokenInfo", 1, |encoder| {
            try!(encoder.emit_struct_field( "scope", 0, |encoder| self.scopes.encode(encoder)));
            try!(encoder.emit_struct_field( "realm", 1, |encoder| self.realm.encode(encoder)));
            if self.uid.is_some() {
                try!(encoder.emit_struct_field( "uid", 2, |encoder| self.uid.encode(encoder)));
            }
            Ok(())
        })
    }
}

impl TokenInfo {
    fn new(scopes: Vec<&str>, uid: Option<Uid>, realm: &str) -> TokenInfo {
        let s = scopes.iter().map(|s| s.to_string()).collect();
        TokenInfo { scopes: s, realm: realm.to_string(), uid: uid }
    }

    pub fn from_query_param(param: &str) -> Result<TokenInfo, String> {
         let parts: Vec<&str> = param.split("-").collect();
         if parts[0] != "token" {
             return Err(format!("{} {}", TOKEN_START_ERR, TOKEN_FORMAT));
         }
         let token_info = match parts.len() {
            1 => {
                warn!("{}", TOKEN_MISSING_UID);
                TokenInfo::new(vec![],  None, "")
            },
            2 => {
                warn!("{}", TOKEN_MISSING_REALM);
                TokenInfo::new(vec![], create_uid(parts[1]), "")
            },
            3 =>{
                warn!("{}", TOKEN_MISSING_SCOPES);
                TokenInfo::new(vec![], create_uid(parts[1]), parts[2])
            },
            _ => {
                let v = parts.clone().split_off(3);
                TokenInfo::new(v, create_uid(parts[1]), parts[2])
            }
        };

        Ok(token_info)
    }
}

fn create_uid(s: &str) -> Option<Uid> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::TokenInfo;
    use super::{TOKEN_START_ERR, TOKEN_FORMAT};
    use rustc_serialize::json;

    #[test]
    fn token_info_to_json_test() {
        let token_info = &TokenInfo {
            scopes: vec!["read".to_string(), "write".to_string()],
            realm: "/employees".to_string(),
            uid: Some("username".to_string())
        };

        assert_eq!("{\"scope\":[\"read\",\"write\"],\"realm\":\"/employees\",\"uid\":\"username\"}",
                   json::encode(token_info).unwrap());

       let token_info = &TokenInfo {
           scopes: vec!["read".to_string(), "write".to_string()],
           realm: "/employees".to_string(),
           uid: None
       };

       assert_eq!("{\"scope\":[\"read\",\"write\"],\"realm\":\"/employees\"}",
                  json::encode(token_info).unwrap());
    }

    #[test]
    fn token_info_new_test() {
        let token_info = &TokenInfo::new(vec!["read", "write"], None, "/employees");
        assert_eq!("{\"scope\":[\"read\",\"write\"],\"realm\":\"/employees\"}", json::encode(token_info).unwrap());
    }

    #[test]
    fn token_info_from_token_param_success_test() {
        let token_info = &TokenInfo::from_query_param("token-username-/employees-read-write").unwrap();
        assert_eq!("{\"scope\":[\"read\",\"write\"],\"realm\":\"/employees\",\"uid\":\"username\"}", json::encode(token_info).unwrap());

        let token_info = &TokenInfo::from_query_param("token--/employees-read-write").unwrap();
        assert_eq!("{\"scope\":[\"read\",\"write\"],\"realm\":\"/employees\"}", json::encode(token_info).unwrap());

        let token_info = &TokenInfo::from_query_param("token--/employees").unwrap();
        assert_eq!("{\"scope\":[],\"realm\":\"/employees\"}", json::encode(token_info).unwrap());

        let token_info = &TokenInfo::from_query_param("token-").unwrap();
        assert_eq!("{\"scope\":[],\"realm\":\"\"}", json::encode(token_info).unwrap());
    }

    #[test]
    fn token_info_from_token_param_fail_test() {
        let token_info_err = TokenInfo::from_query_param("bla-/employees-read-write").err().unwrap();
        assert_eq!(format!("{} {}", TOKEN_START_ERR, TOKEN_FORMAT), token_info_err);
    }
}
