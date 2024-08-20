use std::collections::BTreeMap;

use crate::result::{H10ServerError, ServerResult};

const URL_PARTS_MAX_CHARS: usize = 1024;
#[derive(Debug)]
pub struct UrlParts {
    pub path: Option<String>,
    query: Option<BTreeMap<String, String>>,
    fragment: Option<String>,
}

impl UrlParts {
    pub fn parse(input: &str) -> ServerResult<UrlParts> {
        if input.chars().count() < URL_PARTS_MAX_CHARS {
            let maybe_path = input
                .split('#')
                .next()
                .and_then(|s| s.split('?').next())
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty());

            let mut queries_string = BTreeMap::new();

            let maybe_fragment = input
                .split_once('#')
                .map(|(_, fragment)| fragment.to_string());

            {
                let maybe_query_string =
                    input.split_once('?').map(|(_, query_string)| query_string);

                if let Some(query_string) = maybe_query_string.and_then(|s| s.split("#").next()) {
                    let mut iter = query_string.split("&");
                    while let Some(param) = iter.next() {
                        let mut kv_iter = param.split("=");
                        let opt_k = kv_iter.next();
                        let opt_v = kv_iter.next();

                        match (opt_k, opt_v) {
                            (Some(k), Some(v)) => {
                                queries_string.insert(k.to_string(), v.to_string());
                            }
                            _ => (),
                        }
                    }
                }
            }
            Ok(UrlParts {
                path: maybe_path,
                query: if queries_string.len() > 0 {
                    Some(queries_string)
                } else {
                    None
                },
                fragment: maybe_fragment,
            })
        } else {
            Err(H10ServerError("Error: URL max length reached.".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ok_on_parse_url_parts() -> ServerResult<()> {
        let url = "/path/to/resource?key1=value1&key2=value2#section1";
        let parts = UrlParts::parse(url);
        println!("{:#?}", parts);

        assert!(parts.is_ok());
        Ok(())
    }

    #[test]
    fn ok_on_incremental_url_parsing() -> ServerResult<()> {
        let url = "/path/to/resource?key1=value1&key2=value2#section1";
        let mut idx = 0;
        while let Some(s) = url.split_at_checked(idx).map(|(s, _)| s) {
            idx += 1;
            let parts = UrlParts::parse(s);
            println!("{:#?}", parts);
            assert!(parts.is_ok())
        }

        Ok(())
    }
}
