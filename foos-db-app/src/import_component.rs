use futures::{stream, StreamExt};
use js_sys::Uint8Array;
use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;
use wasm_bindgen::JsCast;
use wasm_streams::ReadableStream;
use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

use crate::hooks::BASE_URL;

fn get_url_for_import(file_name: String) -> String {
    let suffix = if file_name == "init-fast.fast" {
        "/fast-init"
    } else if file_name.ends_with(".fast") {
        "/fast"
    } else if file_name.ends_with(".ktool") {
        "/ktool"
    } else {
        ""
    };

    format!("{BASE_URL}/import{suffix}")
}

#[function_component]
pub fn ImportComponent() -> Html {
    let files = use_state(Option::default);
    let status = use_state(String::default);

    let oninput = {
        let status = status.clone();
        let files = files.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            files.set(input.files());
            let len = input.files().map(|fl| fl.length()).unwrap_or(0);
            let files = (0..len)
                .map(|i| input.files().unwrap().item(i).unwrap().name())
                .collect::<Vec<String>>()
                .join(" ");
            status.set(format!("{len} file(s) selected: {files}"));
        })
    };

    let onsubmit = {
        let status = status.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let files = (*files).clone();

            if files.is_none() {
                status.set(String::from("No files!"));
                return;
            }

            let fl = files.unwrap();

            let len = fl.length();
            if len == 0 {
                status.set(String::from("No files!"));
                return;
            }

            status.set(format!("Preparing to upload {len} files..."));

            let mut files = (0..len).map(|i| fl.item(i).unwrap()).collect::<Vec<File>>();
            if let Some(idx) = files.iter().position(|f| f.name() == "init-fast.fast") {
                files.swap(idx, 0);
            }

            let status = status.clone();
            let status2 = status.clone();

            observable::from_iter(files)
                .flat_map(move |f| {
                    let file_name = f.name();
                    status.set(format!("Reading {file_name}"));
                    let readable_stream = ReadableStream::from_raw(
                        f.stream()
                            .unchecked_into::<wasm_streams::readable::sys::ReadableStream>(),
                    );
                    let data = readable_stream
                        .into_stream()
                        .map(Result::unwrap)
                        .map(|value| value.unchecked_into::<Uint8Array>())
                        .map(|arr| arr.to_vec())
                        .flat_map(stream::iter)
                        .collect::<Vec<u8>>();
                    let status = status.clone();
                    let file_name2 = f.name();
                    observable::from_future(data, LocalSpawner {})
                        .flat_map(move |data| {
                            status.set(format!("Uploading {file_name}"));
                            let part = reqwest::multipart::Part::bytes(data);
                            let form = reqwest::multipart::Form::new().part("file", part);
                            let client = reqwest::Client::new();
                            observable::from_future(
                                client
                                    .post(get_url_for_import(f.name()))
                                    .multipart(form)
                                    .send(),
                                LocalSpawner {},
                            )
                        })
                        .map(Result::unwrap)
                        .map(move |response| (file_name2.clone(), response.status()))
                })
                .subscribe(move |(file_name, code)| {
                    status2.set(format!("Uploaded {file_name}, status: {code}"));
                });
        })
    };

    html! {
        <>
            <form {onsubmit}>
                <div class="mb-3">
                    <label for="import-files" class="form-label">{"Import Files (.ktool, .fast)"}</label>
                    <input class="form-control" type="file" id="import-files" accept=".fast,.ktool" {oninput} multiple={true} />
                </div>
                <div class="mb-3">
                    <button type="submit" class="btn btn-primary mb-3">{"Import"}</button>
                </div>
                <div class="mb-3">
                    {&(*status)}
                </div>
            </form>
        </>
    }
}
