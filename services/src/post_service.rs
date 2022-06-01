use crate::markdown_service::markdown_service::{MarkdownService, PostMetadata};
use crate::posts::POSTS;
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use urlencoding::encode;

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub metadata: PostMetadata,
    pub raw_content: &'static str,
    pub desc: String,
    pub modified_time: String,
}

pub struct PostService {
    posts: Vec<Post>,
}

const MAX_DESC_LENGTH: usize = 200;

impl PostService {
    pub fn new() -> PostService {
        let posts = PostService::read_posts_into_memo();

        PostService { posts }
    }

    pub fn get_posts(&self) -> Vec<Post> {
        self.posts.clone()
    }

    pub fn trim_useless_symbol(content: &'static str) -> String {
        Regex::new(r#"[^\u4E00-\u9FFFa-zA-Z]"#)
            .unwrap()
            .replace_all(content, "")
            .into_owned()
    }

    pub fn find_post_by_encoded_title(&self, title: &str) -> Option<&Post> {
        self.posts
            .iter()
            .find(|post| encode(post.metadata.title.as_str()) == title)
    }

    fn read_posts_into_memo() -> Vec<Post> {
        POSTS
            .into_iter()
            .map(|post| {
                let markdown_service = MarkdownService::new(post.content.to_string());
                let metadata = markdown_service.extract_metadata().expect(
                    "Please make sure the post has metadata which declare using block syntax.",
                );
                let parsed_content = PostService::trim_useless_symbol(post.content);
                let parsed_content_length = parsed_content.len();
                let slice_desc_length = if parsed_content_length > MAX_DESC_LENGTH {
                    MAX_DESC_LENGTH
                } else {
                    parsed_content_length
                };
                let desc = parsed_content[..slice_desc_length].to_string();
                let modified_secs = (post.modified_time / 1000) as i64;
                let modified_time = NaiveDateTime::from_timestamp(modified_secs, 0);
                let modified_time = modified_time.format("%a, %b %e %Y").to_string();

                Post {
                    metadata,
                    raw_content: post.content,
                    desc,
                    modified_time,
                }
            })
            .collect::<Vec<Post>>()
    }
}

pub static POST_SERVICE: Lazy<PostService> = Lazy::new(|| PostService::new());
