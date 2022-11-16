mod movie_clip_title;
mod movie_url;
mod second;

pub use movie_clip_title::MovieClipTtile;
pub use movie_url::MovieUrl;
pub use second::Second;

use crate::ids::Id;
use crate::DomainError::{self, DomainLogicError};

// -------------------------------------------------------------------------------------------------
// # ClipId

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClipIdType;

pub type ClipId = Id<ClipIdType>;

// -------------------------------------------------------------------------------------------------
// # VideoClip

/// VideoClipのエンティティ
#[derive(Debug, Clone)]
pub struct MovieClip {
    title: MovieClipTtile,
    url: MovieUrl,
    start: Second,
    end: Second,
    id: ClipId,
    like: u32,
    dislike: u32,
}

impl MovieClip {
    /// 新しくVideoClipを作成する．
    pub fn create(title: String, url: String, start: u32, end: u32) -> Result<Self, DomainError> {
        if start >= end {
            return Err(DomainLogicError("It is needed start < end".to_string()));
        }
        Ok(Self {
            title: title.try_into()?,
            url: url.try_into()?,
            start: start.into(),
            end: end.into(),
            id: Default::default(),
            like: 0_u32,
            dislike: 0_u32,
        })
    }

    pub fn like_increment(&mut self) {
        self.like += 1;
    }

    pub fn dislike_increment(&mut self) {
        self.dislike += 1;
    }
    pub fn edit_title(&mut self, new_title: String) -> Result<(), DomainError> {
        self.title = new_title.try_into()?;
        Ok(())
    }
    pub fn title(&self) -> &MovieClipTtile {
        &self.title
    }
    pub fn url(&self) -> &MovieUrl {
        &self.url
    }
    pub fn start(&self) -> Second {
        self.start
    }
    pub fn end(&self) -> Second {
        self.end
    }
    pub fn id(&self) -> ClipId {
        self.id
    }
    pub fn like(&self) -> u32 {
        self.like
    }
    pub fn dislike(&self) -> u32 {
        self.dislike
    }
}

#[cfg(test)]
mod test {
    use super::MovieClip;
    #[test]
    fn movie_clip_like_dislike() {
        let mut movie_clip = MovieClip::create(
            "Some Movie Clip".to_string(),
            "https://www.youtube.com/watch?v=SOMEvideoID".to_string(),
            500,
            600,
        )
        .unwrap();

        assert_eq!(movie_clip.like(), 0);
        movie_clip.like_increment();
        assert_eq!(movie_clip.like(), 1);

        assert_eq!(movie_clip.dislike(), 0);
        movie_clip.dislike_increment();
        assert_eq!(movie_clip.dislike(), 1);
    }
}
