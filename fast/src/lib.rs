mod fast;
mod serde;

pub use fast::*;

#[cfg(test)]
mod test {

    use regex::Regex;

    use crate::Fast;

    fn test(input: &str) {
        let fast = quick_xml::de::from_str::<Fast>(input).unwrap();
        let output = quick_xml::se::to_string(&fast).unwrap();

        let regex = Regex::new(r"[\r]?\n\s*").unwrap();

        let input = regex.replace_all(&input[39..], "");
        let output = regex
            .replace_all(&output, "")
            .replace("&quot;", r#"""#)
            .replace("&apos;", "'")
            .replace("/>", " />")
            .replace(
                "<registerUserLogins></registerUserLogins>",
                "<registerUserLogins />",
            )
            .replace("<clubsList></clubsList>", "<clubsList />")
            .replace("<originClub></originClub>", "<originClub />");

        let size = 64;
        for (a, b) in input
            .as_bytes()
            .chunks(size)
            .zip(output.as_bytes().chunks(size))
        {
            match (String::from_utf8(a.to_vec()), String::from_utf8(b.to_vec())) {
                (Ok(a), Ok(b)) => assert_eq!(a, b),
                (a, b) => assert_eq!(a, b),
            }
        }
    }

    #[test]
    fn test_outto() {
        let input = include_str!("outto.xml");
        test(input);
    }

    #[test]
    fn test_outfrom() {
        let input = include_str!("outfrom.xml");
        test(input);
    }
}
