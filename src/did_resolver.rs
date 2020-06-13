use crate::util::XResult;

// curl https://uniresolver.io/1.0/identifiers/did:ccp:3nBPSZU1q6mmxha5Jbg8NcRNGGNt
// curl https://did.baidu.com/v1/did/resolve/did:ccp:3nBPSZU1q6mmxha5Jbg8NcRNGGNt

pub struct DidResolver {
    resolver: String,
}

impl DidResolver {

    pub fn new_baidu() -> Self {
        Self::new("https://did.baidu.com/v1/did/resolve/")
    }

    pub fn new(resolver: &str) -> Self {
        let resolver = if resolver.ends_with('/') {
            resolver.into()
        } else {
            format!("{}/", resolver)
        };
        Self {
            resolver,
        }
    }

    pub async fn resolve(&self, did: &str) -> XResult<String> {
        let resolve_url = format!("{}{}", self.resolver, did);
        Ok(reqwest::get(&resolve_url).await?.text().await?)
    }
}
