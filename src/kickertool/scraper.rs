use scraper::{ElementRef, Html, Selector};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct KickertoolData {
    pub standings: Vec<String>,
    pub tables: Vec<Table>,
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

        Some(KickertoolData { standings, tables })
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

        Some(KickertoolData { standings, tables })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Table {
    pub number: u8,
    pub team1: String,
    pub team2: String,
}

impl Table {
    fn from_playlist_element(element: &ElementRef) -> Self {
        let number = select_inner_text(element, "kt-table-name span")
            .parse()
            .unwrap();
        let team1 = select_inner_text(element, ".team1");
        let team2 = select_inner_text(element, ".team2");

        Self {
            number,
            team1,
            team2,
        }
    }

    fn from_qualificaition_display_element(element: &ElementRef) -> Self {
        let number = select_inner_text(element, ".table").parse().unwrap();
        let team1 = select_inner_text(element, ".left");
        let team2 = select_inner_text(element, ".right");

        Self {
            number,
            team1,
            team2,
        }
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
