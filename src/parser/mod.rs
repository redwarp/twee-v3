use crate::{error::ParsingError, Story};

use self::story::parse_story;

pub(crate) mod metadata;
pub(crate) mod passage;
pub(crate) mod story;

impl<'a> TryFrom<&'a str> for Story<&'a str> {
    type Error = ParsingError<&'a str>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match parse_story(value) {
            Ok((_, story)) => Ok(story),
            Result::Err(error) => Result::Err(error.into()),
        }
    }
}

impl TryFrom<String> for Story<String> {
    type Error = ParsingError<String>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match parse_story(&value) {
            Ok((_, story)) => {
                let Story {
                    content: _,
                    title,
                    start,
                    passages,
                } = story;

                Ok(Story {
                    content: value,
                    title,
                    start,
                    passages,
                })
            }
            Result::Err(error) => Result::Err(error.into()),
        }
    }
}
