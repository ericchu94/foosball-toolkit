use scraper::{ElementRef, Html, Selector};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct KickertoolData {
    pub standings: Vec<String>,
    pub tables: Vec<Table>,
}

impl KickertoolData {
    pub fn from_html<S: AsRef<str>>(html: S) -> Option<KickertoolData> {
        let html = html.as_ref();

        let html = Html::parse_document(html);

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

        let tables = {
            let selector = Selector::parse(".playlist-row.active").unwrap();
            html.select(&selector)
                .map(|table| Table::from_element(&table))
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
    fn from_element(table: &ElementRef) -> Self {
        let number = get_table_number(table);
        let team1 = get_team(table, 1);
        let team2 = get_team(table, 2);

        Self {
            number,
            team1,
            team2,
        }
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

fn get_team(table: &ElementRef, number: u8) -> String {
    let selector = Selector::parse(&format!(".team{number}")).unwrap();
    let team = table.select(&selector).next().map(inner_text);
    team.unwrap()
}

fn get_table_number(table: &ElementRef) -> u8 {
    let selector = Selector::parse("kt-table-name span").unwrap();
    let number = table.select(&selector).next().map(inner_text);
    number.unwrap().parse().unwrap()
}
