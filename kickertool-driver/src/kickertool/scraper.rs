use kickertool_data::*;
use scraper::{ElementRef, Html, Selector};

pub fn kickertool_data_from_qualification_display<S: AsRef<str>>(
    html: S,
) -> Option<KickertoolData> {
    let html = html.as_ref();

    let html = Html::parse_document(html);

    let standings = {
        let selector = Selector::parse(".right-area .table-row").unwrap();
        html.select(&selector).map(|row| standing(&row)).collect()
    };

    let tables = {
        let selector = Selector::parse(".live .table-row").unwrap();
        html.select(&selector)
            .map(|table| table_from_qualificaition_display_element(&table))
            .collect()
    };

    let next_matches = {
        let selector = Selector::parse(".next.table-row").unwrap();
        html.select(&selector)
            .map(|row| match_from_element(&row))
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

fn table_from_qualificaition_display_element(element: &ElementRef) -> Table {
    let number = select_inner_text(element, ".table").parse().unwrap();
    let r#match = match_from_element(element);

    Table { number, r#match }
}

fn match_from_element(element: &ElementRef) -> Match {
    let team1 = select_inner_text(element, ".left, .team1");
    let team2 = select_inner_text(element, ".right, .team2");

    Match { team1, team2 }
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
