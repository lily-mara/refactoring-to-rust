use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    pub term: String,
    pub page: isize,
    pub limit: isize,
}

#[wasm_bindgen]
pub async fn paper_search(val: JsValue) -> JsValue{
    let term: Search= serde_wasm_bindgen::from_value(val).unwrap();
    let resp = search(term.term, term.page, term.limit).await.unwrap();
    serde_wasm_bindgen::to_value(&resp).unwrap()
}

#[wasm_bindgen]
pub fn list_component() -> Result<(), JsValue> {
    yew::start_app::<List>();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_search() {
        let res = aw!(search("type".to_string(), 0, 10)).unwrap();
        assert_eq!(res.entry.len(), 10);
        print!("{:?}", res)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub entry: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub updated: String,
    pub published: String,
    pub title: String,
    pub summary: String,
    pub author: Vec<Author>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
}

async fn search(term: String, page: isize, max_results: isize) -> Result<Feed, reqwest::Error> {
    let http_response = reqwest::get(format!("http://export.arxiv.org/api/query?search_query=all:{}&start={}&max_results={}", term, page * max_results, max_results)).await?;
    let b = http_response.text().await?;
    let feed: Feed = serde_xml_rs::from_str(b.as_str()).unwrap();
    return Ok(feed)
}


pub enum Msg {
    IncrementPage,
    SetFeedState(FetchState<Feed>),
    GetSearch(isize),
}

pub enum FetchState<T> {
    Fetching,
    Success(T),
    Failed(reqwest::Error),
}

pub struct List {
    page: isize,
    feed: FetchState<Feed>,
}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetSearch(0));
        Self {
            page: 0,
            feed: FetchState::Fetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFeedState(fetch_state) => {
                self.feed = fetch_state;
                true
            }
            Msg::IncrementPage => {
                self.page += 1;
                ctx.link().send_message(Msg::GetSearch(self.page));
                false
            }
            Msg::GetSearch(page) => {
                ctx.link().send_future(async move {
                    match search("type".to_string(), page, 10).await {
                        Ok(data) => Msg::SetFeedState(FetchState::Success(data)),
                        Err(err) => Msg::SetFeedState(FetchState::Failed(err)),
                    }
                });
                ctx.link().send_message(Msg::SetFeedState(FetchState::Fetching));
                true // Return true to cause the displayed change to update
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.feed {
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(data) => html! { 
                <div>
                    <ul>
                        { for data.entry.iter().map(|e| html!{
                            <li>
                                <a target="_blank" href={e.id.to_string()}>{e.title.to_string()}</a>
                            </li>
                        })}
                    </ul>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::IncrementPage)}>
                        { "More" }
                    </button>

                </div>
               

            },
            FetchState::Failed(err) => html! { err },
        }

    }
}

