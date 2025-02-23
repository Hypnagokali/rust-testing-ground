use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub iat: u64,
    pub exp: u64,
}

#[allow(dead_code)]
pub struct JWKS {
    pub kid: String,
    pub kty: String, // should be enum
    pub alg: String, // should be enum too
    pub jwks_use: String,
    pub x5c: Vec<String>,
    pub x5t: String,
    pub n: String,
    pub e: String,
}

impl JWKS {
    pub fn test_without_cert_chain() -> Self {
        Self { 
            kid: "test".to_owned(),
            kty: "RSA".to_owned(),
            alg: "RS256".to_owned(),
            jwks_use: "sig".to_owned(),
            x5c: Vec::new(),
            x5t: "".to_owned(),
            n: "rIyQrizZO1ARTgYAk6ZhY7fSprL6GCScfGLz4XWK_Mgy0XsLz6mxcpnMCI2-Eng9OHuTiU6qjnhm72erfJILxQ0H6HrtrvPhGgQ41sVIvMIk7URZbr5e1PjD1rB1i7AObZWrkwe0vGDbohkayBOgg1BMdPbIGfKqJICXi7vorAiCZmKOQzAziG-2jsfQAadNyfwjUPtGOfOvwvRpZw2YHuJ7JeyZzh2ZBQbbKJgZ2Ip9L69qdJvSqVogdJTPMDXAU7JUc6dsWpDVSJMveEeVlzg4qfczT1RfI09zNyVFjpzqOgVXdqvXXvDaO_klT0u80nv0MYInqUia28dL98uFhQ".to_owned(),
            e: "AQAB".to_owned() 
        }
    }
}