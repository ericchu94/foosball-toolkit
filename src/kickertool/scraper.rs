use scraper::{ElementRef, Html, Selector};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct KickertoolData {
    pub tournament_name: String,
    pub standings: Vec<String>,
    pub tables: Vec<Table>,
    pub next_matches: Vec<Match>,
}

impl KickertoolData {
    pub fn from_playlist<S: AsRef<str>>(html: S) -> Option<KickertoolData> {
        let html = html.as_ref();

        let html = Html::parse_document(html);

        let standings = {
            let selector = Selector::parse(".tournament-right .table-row").unwrap();
            html.select(&selector).map(|row| standing(&row)).collect()
        };

        let tables = {
            let selector = Selector::parse(".playlist-row.active").unwrap();
            html.select(&selector)
                .map(|table| Table::from_playlist_element(&table))
                .collect()
        };

        let next_matches = vec![];

        let tournament_name = String::new();

        Some(KickertoolData {
            tournament_name,
            standings,
            tables,
            next_matches,
        })
    }

    pub fn from_qualification_display<S: AsRef<str>>(html: S) -> Option<KickertoolData> {
        let html = html.as_ref();

        let html = Html::parse_document(html);

        let standings = {
            let selector = Selector::parse(".right-area .table-row").unwrap();
            html.select(&selector).map(|row| standing(&row)).collect()
        };

        let tables = {
            let selector = Selector::parse(".live .table-row").unwrap();
            html.select(&selector)
                .map(|table| Table::from_qualificaition_display_element(&table))
                .collect()
        };

        let next_matches = {
            let selector = Selector::parse(".next.table-row").unwrap();
            html.select(&selector)
                .map(|row| Match::from_element(&row))
                .collect()
        };

        let tournament_name = select_inner_text(&html.root_element(), "h1");

        Some(KickertoolData {
            tournament_name,
            standings,
            tables,
            next_matches,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Table {
    pub number: u8,
    pub r#match: Match,
}

impl Table {
    fn from_playlist_element(element: &ElementRef) -> Self {
        let number = select_inner_text(element, "kt-table-name span")
            .parse()
            .unwrap();
        let r#match = Match::from_element(element);

        Self { number, r#match }
    }

    fn from_qualificaition_display_element(element: &ElementRef) -> Self {
        let number = select_inner_text(element, ".table").parse().unwrap();
        let r#match = Match::from_element(element);

        Self { number, r#match }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Match {
    pub team1: String,
    pub team2: String,
}

impl Match {
    fn from_element(element: &ElementRef) -> Self {
        let team1 = select_inner_text(element, ".left, .team1");
        let team2 = select_inner_text(element, ".right, .team2");

        Self { team1, team2 }
    }
}

fn select_inner_text(element: &ElementRef, selector: &str) -> String {
    let selector = Selector::parse(selector).unwrap();
    let text = element.select(&selector).next().map(inner_text);
    text.unwrap()
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

fn standing(element: &ElementRef) -> String {
    let pos = select_inner_text(element, ".pos");
    let name = select_inner_text(element, ".name");
    format!("{pos} {name}")
}
