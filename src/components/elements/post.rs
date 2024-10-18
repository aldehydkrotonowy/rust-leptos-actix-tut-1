use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
struct Post {
    id: usize,
    title: String,
    content: String,
}
#[derive(Clone, PartialEq)]
struct PostId(usize);
impl IntoParam for PostId {
    fn into_param(value: Option<&str>, name: &str) -> Result<Self, ParamsError> {
        match value {
            Some(value) => match value.parse() {
                Ok(id) => Ok(PostId(id)),
                Err(e) => Err(ParamsError::Params(())),
            },
            None => Err(ParamsError::MissingParam(name.to_string())),
        }
    }
}

#[derive(Params, PartialEq)]
struct PostParams {
    id: PostId,
}
#[derive(Debug, Error, Serialize, Deserialize, Clone, PartialEq)]
enum PostError {
    #[error("Id of the post is invalid")]
    InvalidId,
    #[error("Post not found")]
    PostNotFound,
    #[error("Server error")]
    ServerError,
}

#[component]
pub fn Post() -> impl IntoView {
    let query = use_params::<PostParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.clone())
                .map_err(move |_| PostError::InvalidId)
        })
    };
    let _post = create_resource(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(id.0)
                .await
                .map(|post| post.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });

    // let post_view = move || post.with().map();

    view! { <h1>post</h1> }
}

fn generate_posts() -> Vec<Post> {
    vec![Post {
        id: 0,
        title: "Post Title".to_string(),
        content: "Post ontent".to_string(),
    }]
}

#[server(GetPost, "/api")]
async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
    let posts = generate_posts();
    Ok(posts.iter().find(|post| post.id == id).clone())
}
