use base64::prelude::*;
use jwt::{
    header::{HeaderContentType, HeaderType},
    AlgorithmType, JoseHeader, SignWithKey, SigningAlgorithm, Token,
};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::Pair as TraitPair, crypto::Ss58Codec, sr25519::Pair};

use crate::{AgreementId, Timestamp};

const SEPARATOR: &str = ".";

/// JWT header
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "alg")]
    pub algorithm: String,

    #[serde(rename = "typ", skip_serializing_if = "Option::is_none")]
    pub type_: Option<HeaderType>,
}

impl JoseHeader for Header {
    fn algorithm_type(&self) -> AlgorithmType {
        // We use a custom algorithm type
        AlgorithmType::None
    }

    fn key_id(&self) -> Option<&str> {
        None
    }

    fn type_(&self) -> Option<HeaderType> {
        self.type_
    }

    fn content_type(&self) -> Option<HeaderContentType> {
        None
    }
}

/// JWT claims
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    /// The subject of the token (Deitos address)
    sub: String,
    /// Agreement ID
    aud: u32,
    /// Expiration time
    exp: u64,
    /// Issued at
    iat: u64,
}

pub struct TokenSigner {
    pub keypair: Pair,
}

impl TokenSigner {
    fn new(keypair: Pair) -> Self {
        Self { keypair }
    }
}

impl SigningAlgorithm for TokenSigner {
    fn algorithm_type(&self) -> AlgorithmType {
        AlgorithmType::None
    }

    fn sign(&self, header: &str, claims: &str) -> Result<String, jwt::Error> {
        let message = format!("{header}{SEPARATOR}{claims}");
        let signature = self.keypair.sign(message.as_bytes());
        Ok(BASE64_URL_SAFE_NO_PAD.encode(signature))
    }
}

pub fn generate_token(
    keypair: Pair,
    agreement: AgreementId,
    issued: Timestamp,
    expiration: Timestamp,
) -> String {
    let header = Header {
        algorithm: "Sr25519".to_string(),
        type_: Some(HeaderType::JsonWebToken),
    };
    let claims = Claims {
        sub: keypair.public().to_ss58check(),
        aud: agreement,
        exp: expiration,
        iat: issued,
    };

    println!("Token data: {header:?}\n{claims:?}");

    let unsigned_token: Token<Header, Claims, _> = Token::new(header, claims);
    let signed_token = unsigned_token
        .sign_with_key(&TokenSigner::new(keypair))
        .expect("Token should sign successfully");
    signed_token.as_str().to_owned()
}
