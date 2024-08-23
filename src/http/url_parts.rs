use std::{collections::BTreeMap, fmt::Display};

use crate::{
    constants::URL_PARTS_MAX_CHARS,
    http::result::{H10LibError, H10LibResult},
};

#[derive(Debug, Default)]
pub struct UrlParts2 {
    pub path: Option<String>,
    pub query: Option<BTreeMap<String, String>>,
    pub fragment: Option<String>,
}

impl Display for UrlParts2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();

        if let Some(path) = &self.path {
            output.push_str(path)
        }

        if let Some(queries) = &self.query {
            output.push('?');
            for (key, value) in queries {
                output.push_str(format!("{key}={value}").as_str());
            }
        }

        if let Some(fragment) = &self.fragment {
            output.push('#');
            output.push_str(fragment)
        }

        write!(f, "{output}")
    }
}

impl UrlParts2 {
    pub fn parse(input: &str) -> H10LibResult<UrlParts2> {
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
            Ok(UrlParts2 {
                path: maybe_path,
                query: if queries_string.len() > 0 {
                    Some(queries_string)
                } else {
                    None
                },
                fragment: maybe_fragment,
            })
        } else {
            Err(H10LibError::InvalidInputData(
                "Error: URL max length reached.".into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ok_on_parse_url_parts() -> H10LibResult<()> {
        let url = "/path/to/resource?key1=value1&key2=value2#section1";
        let parts = UrlParts2::parse(url);
        println!("{:#?}", parts);

        assert!(parts.is_ok());
        Ok(())
    }

    #[test]
    fn ok_on_incremental_url_parsing() -> H10LibResult<()> {
        let url = "/path/to/resource?key1=value1&key2=value2#section1";
        let mut idx = 0;
        while let Some(s) = url.split_at_checked(idx).map(|(s, _)| s) {
            idx += 1;
            let parts = UrlParts2::parse(s);
            println!("{:#?}", parts);
            assert!(parts.is_ok())
        }

        Ok(())
    }
}
