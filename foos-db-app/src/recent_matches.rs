use yew::prelude::*;
use rand::prelude::*;

const NAMES: [&str; 10] = [
    "Eric CHU",
    "Lu Dong 鲁东",
    "Ieva KASKELEVICIUTE",
    "Landi CAO",
    "Ren Shaoyi 仁少义",
    "Johan HANNERSTÅL",
    "Renchao SONG",
    "Summer ZHUANG",
    "Hao yang CUI",
    "Anatol BIEGACZ",
];

struct Match {
    team1: Vec<String>,
    team2: Vec<String>,
    // 0 = tie, 1 = team 1, 2 = team 2
    winner: u8,
    timestamp: String,
    tournament: &'static str,
}

impl Match {
    fn new(team1: Vec<&str>, team2: Vec<&str>, winner: u8) -> Self {
        Self {
            team1: team1.into_iter().map(ToOwned::to_owned).collect(),
            team2: team2.into_iter().map(ToOwned::to_owned).collect(),
            winner,
            timestamp: "2022-05-20 16:09".to_owned(),
            tournament: "DaLi Open 2021",
        }
    }
}

#[function_component]
pub fn RecentMatches() -> Html {
    let mut rng = thread_rng();
    let matches = (0..5).map(|_| {
        let count = rng.gen_range(1..=2);
        let team1 = (0..count).map(|_| {
            NAMES[rng.gen_range(0..NAMES.len())]
        }).collect::<Vec<&str>>();
        let team2 = (0..count).map(|_| {
            NAMES[rng.gen_range(0..NAMES.len())]
        }).collect::<Vec<&str>>();
        let winner = rng.gen_range(0..=2);
        Match::new(team1, team2, winner)
    });

    html! {
        <>
            <div class="card">
                <div class="card-header">
                {"Recent Matches"}
                </div>
                <div class="list-group list-group-flush">
                    {matches.into_iter().map(|m| {
                        let class_tie = "bi-emoji-neutral text-warning";
                        let class_win = "bi-emoji-smile text-success";
                        let class_lose = "bi-emoji-frown text-danger";
                        let (classes1, classes2) = match m.winner {
                            0 => (class_tie, class_tie),
                            1 => (class_win, class_lose),
                            2 => (class_lose, class_win),
                            _ => unreachable!(),
                        };

                        html!{
                            <button type="button" class="list-group-item list-group-item-action" aria-current="true">
                                <div class="text-muted fs-6">
                                    <div class="position-absolute start-0 ms-2">
                                        <small>{m.tournament}</small>
                                    </div>
                                    <div class="position-absolute end-0 me-2">
                                        <small>{m.timestamp}</small>
                                    </div>
                                </div>
                                <div class="row">
                                    <div class="col-sm text-sm-end text-center align-self-top">
                                        <i class={classes!(classes1)}></i>
                                        {m.team1.into_iter().map(|name| html! { <div>{name}</div> }).collect::<Html>()}
                                    </div>
                                    <div class="col-sm-1 text-center align-self-center">
                                        <i class="bi-x px-1"></i>
                                    </div>
                                    <div class="col-sm text-sm-start text-center align-self-top">
                                        <i class={classes!(classes2)}></i>
                                        {m.team2.into_iter().map(|name| html! { <div>{name}</div> }).collect::<Html>()}
                                    </div>
                                </div>
                            </button>
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </>
    }
}
