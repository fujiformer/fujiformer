use std::convert::TryFrom;

use fujiformer_geom::{IntRect, Point, Rect, Size};
use log::warn;
use thiserror::Error;

use crate::CelesteMap;

#[derive(Debug, Clone)]
pub struct Filler {
    rect: IntRect,
}

impl Filler {
    pub fn new(rect: IntRect) -> Self {
        Filler { rect }
    }

    pub fn shape(&self) -> IntRect {
        self.rect
    }

    pub fn shape_mut(&mut self) -> &mut IntRect {
        &mut self.rect
    }
}

#[derive(Error, Debug)]
pub enum FillersDecodeError {
    #[error("missing filler node")]
    MissingFillerNode,
    #[error("rect missing x value")]
    MissingX,
    #[error("rect x value not int")]
    XNotInt,
    #[error("rect missing y value")]
    MissingY,
    #[error("rect y value not int")]
    YNotInt,
    #[error("rect missing width value")]
    MissingWidth,
    #[error("rect width not int")]
    WidthNotInt,
    #[error("rect missing height value")]
    MissingHeight,
    #[error("rect height not int")]
    HeightNotInt,
}

pub fn read_fillers(map: &mut CelesteMap) -> Result<(), FillersDecodeError> {
    let mut node = map
        .unread
        .take_child_with_name("Filler")
        .ok_or(FillersDecodeError::MissingFillerNode)?;

    for mut child in node.take_children().into_iter() {
        if child.name() != "rect" {
            warn!("expected \"rect\", got {}", child.name());
        }

        let (mut x, mut y, mut width, mut height) = (None, None, None, None);
        for (key, value) in child.take_properties() {
            match key.as_str() {
                "x" => x = Some(i32::try_from(value).map_err(|_| FillersDecodeError::XNotInt)?),
                "y" => y = Some(i32::try_from(value).map_err(|_| FillersDecodeError::YNotInt)?),
                "w" => {
                    width = Some(u32::try_from(value).map_err(|_| FillersDecodeError::WidthNotInt)?)
                }
                "h" => {
                    height =
                        Some(u32::try_from(value).map_err(|_| FillersDecodeError::HeightNotInt)?)
                }
                _ => (),
            }
        }
        let (x, y, width, height) = (
            x.ok_or(FillersDecodeError::MissingX)?,
            y.ok_or(FillersDecodeError::MissingY)?,
            width.ok_or(FillersDecodeError::MissingWidth)?,
            height.ok_or(FillersDecodeError::MissingHeight)?,
        );
        map.fillers_mut().push(Filler::new(Rect::new(
            Point::new(x, y),
            Size::new(width, height),
        )))
    }

    Ok(())
}
