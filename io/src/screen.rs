use std::convert::TryFrom;

use fujiformer_geom::{IntRect, Point, Rect, Size};
use log::warn;
use thiserror::Error;

use crate::{internal::Node, CelesteMap};

#[derive(Debug, Clone)]
pub struct Screen {
    name: String,
    rect: IntRect,
    unread: Node,
}

impl Screen {
    pub fn new(name: String, rect: IntRect) -> Self {
        Screen {
            name,
            rect,
            unread: Node::new("level".into()),
        }
    }

    pub fn shape(&self) -> IntRect {
        self.rect
    }

    pub fn shape_mut(&mut self) -> &mut IntRect {
        &mut self.rect
    }
}

#[derive(Error, Debug)]
pub enum ScreensDecodeError {
    #[error("missing levels node")]
    MissingLevelsNode,
    #[error("level missing name")]
    MissingName,
    #[error("level name not int")]
    NameNotString,
    #[error("level missing x value")]
    MissingX,
    #[error("level x value not int")]
    XNotInt,
    #[error("level missing y value")]
    MissingY,
    #[error("level y value not int")]
    YNotInt,
    #[error("level missing width value")]
    MissingWidth,
    #[error("level width not int")]
    WidthNotInt,
    #[error("level missing height value")]
    MissingHeight,
    #[error("level height not int")]
    HeightNotInt,
}

pub fn decode_screens(map: &mut CelesteMap) -> Result<(), ScreensDecodeError> {
    let mut node = map
        .unread
        .take_child_with_name("levels")
        .ok_or(ScreensDecodeError::MissingLevelsNode)?;

    for mut child in std::mem::take(node.children_mut()).into_iter() {
        if child.name() != "level" {
            warn!("expected \"rect\", got {}", child.name());
        }

        let (mut name, mut x, mut y, mut width, mut height) = (None, None, None, None, None);
        child.properties_mut().retain(|(key, value)| {
            match key.as_str() {
                "name" => {
                    name = Some(
                        String::try_from(value.clone())
                            .map_err(|_| ScreensDecodeError::NameNotString),
                    )
                }
                "x" => x = Some(i32::try_from(value).map_err(|_| ScreensDecodeError::XNotInt)),
                "y" => y = Some(i32::try_from(value).map_err(|_| ScreensDecodeError::YNotInt)),
                "width" => {
                    width = Some(u32::try_from(value).map_err(|_| ScreensDecodeError::WidthNotInt))
                }
                "height" => {
                    height =
                        Some(u32::try_from(value).map_err(|_| ScreensDecodeError::HeightNotInt))
                }
                _ => return true,
            };
            false
        });
        let (name, x, y, width, height) = (
            name.ok_or(ScreensDecodeError::MissingName)??,
            x.ok_or(ScreensDecodeError::MissingX)??,
            y.ok_or(ScreensDecodeError::MissingY)??,
            width.ok_or(ScreensDecodeError::MissingWidth)??,
            height.ok_or(ScreensDecodeError::MissingHeight)??,
        );
        map.screens_mut().push({
            let mut screen =
                Screen::new(name, Rect::new(Point::new(x, y), Size::new(width, height)));
            screen.unread = child;
            screen
        });
    }

    Ok(())
}
