use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::foos_db_client::FoosDbClient;
use crate::models::*;

type O<N: FnMut(Vec<MatchData>) + 'static> =
    impl Observable<Item = Vec<MatchData>> + SubscribeNext<'static, N>;

pub fn get_match_datas_observable<N: FnMut(Vec<MatchData>) + 'static>(
    client: FoosDbClient,
    limit: i32,
    offset: i32,
) -> O<N> {
    observable::from_future(
        async move { client.match_datas(offset, limit).await },
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_match_datas(limit: i32, offset: i32) -> Vec<MatchData> {
    let client = use_context::<FoosDbClient>().unwrap();
    let item = use_state(Vec::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |&(limit, offset)| {
                let mut s = get_match_datas_observable(client, limit, offset)
                    .subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            (limit, offset),
        );
    }

    (*item).clone()
}
