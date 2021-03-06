use super::derive::*;
use super::media::*;
use super::parser::*;
use super::UntrimmedString;

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCard {
    kind: PostCardType,
    #[html(selector = "div.card--header a.card-meta--row")]
    author: Option<Author>,
    #[html(selector = "div.card--body > p", attr = "inner")]
    body: Option<UntrimmedString>,
    #[html(selector = "div.ch--avatar--badge--wrapper > img", attr = "src")]
    badge: Option<UntrimmedString>,
    #[html(
        selector = "span.card-meta--row span.post--timestamp",
        attr = "inner"
    )]
    timestamp: Option<UntrimmedString>,
    #[html(
        selector = "div.eb--timestamp span.reblock",
        attr = "inner"
    )]
    echo_timestamp: Option<UntrimmedString>,
    #[html(
        selector = "span.card-meta--row span.impressions--wrapper span.impressions--count",
        attr = "inner"
    )]
    impression_count: Option<i64>,
    #[html(selector = ":scope > div.card--body
       ")]
    media_container: Option<MediaContainer>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
#[html(selector = "div.post--card--wrapper")]
pub struct ParlerPost {
    #[html(
        selector = "div.card--post-container span.post,div.card--post-container span.echo--parent, div.card--post-container span.echo--root, div.card--post-container div.echo-byline--wrapper"
    )]
    cards: Vec<PostCard>,
    #[html(selector = "div.comments-list--container")]
    comments: Vec<Comment>,
    #[html(selector = "div.card--body > p a.at", attr = "inner")]
    mentions: Option<Vec<String>>,
    #[html(selector = "div.card--footer div.post--actions")]
    engagements: Option<PostCounts>,
}

impl ParlerPost {
    pub fn get_card(&mut self, kind: PostCardType) -> Option<&mut PostCard> {
        self.cards.iter_mut().find(|c| c.kind == kind)
    }
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct Author {
    #[html(selector = "span.author--name", attr = "inner")]
    pub name: Option<String>,
    #[html(selector = "span.author--username", attr = "inner")]
    pub username: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PostCardType {
    Post,
    EchoParent,
    EchoRoot,
}

impl FromHtml for PostCardType {
    fn from_elements(select: ElemIter) -> unhtml::Result<Self> {
        let elem = select.next().ok_or(())?.value();
        if elem.has_class("echo--parent", CaseSensitivity::AsciiCaseInsensitive) {
            Ok(PostCardType::EchoParent)
        } else if elem.has_class("echo--root", CaseSensitivity::AsciiCaseInsensitive) {
            Ok(PostCardType::EchoRoot)
        } else {
            Ok(PostCardType::Post)
        }
    }
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCounts {
    #[html(
        selector = ".pa--item--wrapper:nth-child(1) span.pa--item--count",
        attr = "inner"
    )]
    comment_count: Option<i64>,
    #[html(
        selector = ".pa--item--wrapper:nth-child(2) span.pa--item--count",
        attr = "inner"
    )]
    echo_count: Option<i64>,
    #[html(
        selector = ".pa--item--wrapper:nth-child(3) span.pa--item--count",
        attr = "inner"
    )]
    upvote_count: Option<i64>,
}

#[derive(FromHtml, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    #[html(selector = "div.card--header div.ch--meta-col")]
    author: Option<Author>,
    #[html(selector = "div.card--body p", attr = "inner")]
    body: Option<String>,
}
