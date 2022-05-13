use scraper::{ElementRef, Html, Selector};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct KickertoolData {
    pub team1: Option<String>,
    pub team2: Option<String>,
    pub standings: Vec<String>,
}

impl KickertoolData {
    pub fn from_html<S: AsRef<str>>(html: S) -> Option<KickertoolData> {
        let html = html.as_ref();

        let html = Html::parse_document(html);

        let mut teams = (1..=2)
            .map(|number| {
                let selector = format!(".playlist-row.active .team{number}");
                let selector = Selector::parse(&selector).unwrap();
                let team = html.select(&selector).next().map(inner_text);
                team
            })
            .collect::<Vec<Option<String>>>();

        let standings = {
            let selector = Selector::parse(".tournament-right .table-row").unwrap();
            html.select(&selector)
                .flat_map(|row| {
                    let pos = {
                        let selector = Selector::parse(".pos").unwrap();
                        let el = row.select(&selector).next()?;
                        inner_text(el)
                    };
                    let name = {
                        let selector = Selector::parse(".name").unwrap();
                        let el = row.select(&selector).next()?;
                        inner_text(el)
                    };
                    Some(format!("{pos} {name}"))
                })
                .collect()
        };

        Some(KickertoolData {
            team1: teams[0].take(),
            team2: teams[1].take(),
            standings,
        })
    }
}

fn inner_text(element: ElementRef) -> String {
    element.text().fold(String::new(), |mut acc, s| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(s.trim());
        acc
    })
}
