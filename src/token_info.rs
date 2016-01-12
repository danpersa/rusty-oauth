
static TOKEN_FORMAT: &'static str = "The token should have the format `token-realm-scope1-scope2`.";
static TOKEN_START_ERR: &'static str = "The token does not start with `token-`.";

static TOKEN_MISSING_SCOPES: &'static str = "The token does not have scopes defined.";
static TOKEN_MISSING_REALM: &'static str = "The token does not have a realm defined.";

pub type Scope = String;

pub type Realm = String;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct TokenInfo {
    scope: Vec<Scope>,
    realm: Realm
}

impl TokenInfo {
    fn new(scopes: Vec<&str>, realm: &str) -> TokenInfo {
        let s = scopes.iter().map(|s| s.to_string()).collect();
        TokenInfo { scope: s, realm: realm.to_string() }
    }

    pub fn from_query_param(param: &str) -> Result<TokenInfo, String> {
         let parts: Vec<&str> = param.split("-").collect();
         if parts[0] != "token" {
             return Err(format!("{} {}", TOKEN_START_ERR, TOKEN_FORMAT));
         }
         let token_info = match parts.len() {
             1 => {
                warn!("{}", TOKEN_MISSING_REALM);
                TokenInfo::new(vec![], "")
             },
             2 =>{
                 warn!("{}", TOKEN_MISSING_SCOPES);
                 TokenInfo::new(vec![], parts[1])
             },
             _ => {
                let v = parts.clone().split_off(2);
                TokenInfo::new(v, parts[1])
            }
         };

         return Ok(token_info);
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
            realm: "/employees".to_string()
        };
        assert_eq!("{\"scopes\":[\"read\",\"write\"],\"realm\":\"/employees\"}", json::encode(token_info).unwrap());
    }

    #[test]
    fn token_info_new_test() {
        let token_info = &TokenInfo::new(vec!["read", "write"], "/employees");
        assert_eq!("{\"scopes\":[\"read\",\"write\"],\"realm\":\"/employees\"}", json::encode(token_info).unwrap());
    }

    #[test]
    fn token_info_from_token_param_success_test() {
        let token_info = &TokenInfo::from_query_param("token-/employees-read-write").unwrap();
        assert_eq!("{\"scopes\":[\"read\",\"write\"],\"realm\":\"/employees\"}", json::encode(token_info).unwrap());
        let token_info = &TokenInfo::from_query_param("token-/employees").unwrap();
        assert_eq!("{\"scopes\":[],\"realm\":\"/employees\"}", json::encode(token_info).unwrap());
        let token_info = &TokenInfo::from_query_param("token-").unwrap();
        assert_eq!("{\"scopes\":[],\"realm\":\"\"}", json::encode(token_info).unwrap());
    }

    #[test]
    fn token_info_from_token_param_fail_test() {
        let token_info_err = TokenInfo::from_query_param("bla-/employees-read-write").err().unwrap();
        assert_eq!(format!("{} {}", TOKEN_START_ERR, TOKEN_FORMAT), token_info_err);
    }
}
