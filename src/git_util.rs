extern crate regex;

use regex::Regex;

pub fn derive_repo_url<S>(addr: S) -> String
    where S: AsRef<str>
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?:https?://|git@)
            (?P<host>[^:/]+)
            (?:[:/])
            (?P<project>[\w/-]+)
            (?:[.]git)?$
            ").unwrap();
    }

    let cap = RE.captures(addr.as_ref()).expect(&format!("invalid git url: {}", addr.as_ref()));
    if let (Some(h), Some(p)) = (cap.name("host"), cap.name("project")) {
        return format!("https://{}/{}", h.as_str(), p.as_str())
    }
    panic!("invalid git address {}", addr.as_ref());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn http_addr() {
        let input = "https://github.skyscannertools.net/Albatross/voyager.git";
        let expect = "https://github.skyscannertools.net/Albatross/voyager";
        let output = derive_repo_url(input);
        assert_eq!(expect, output);
    }

    #[test]
    fn ssh_addr() {
        let input = "git@github.skyscannertools.net:Albatross/voyager.git";
        let expect = "https://github.skyscannertools.net/Albatross/voyager";
        let output = derive_repo_url(input);
        assert_eq!(expect, output);
    }

    #[test]
    fn http_url() {
        let input = "https://github.skyscannertools.net/Albatross/voyager";
        let expect = "https://github.skyscannertools.net/Albatross/voyager";
        let output = derive_repo_url(input);
        assert_eq!(expect, output);
    }
}