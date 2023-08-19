use std::{error, fmt};

use core_graphics::{
    event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, ScrollEventUnit},
    event_source::{CGEventSource, CGEventSourceStateID},
    geometry::CGPoint,
};

use crate::types::{keys::Keys, Point};

impl From<CGPoint> for Point {
    #[allow(clippy::cast_possible_truncation)]
    fn from(other: CGPoint) -> Self {
        Self {
            x: other.x as _,
            y: other.y as _,
        }
    }
}

impl From<Point> for CGPoint {
    fn from(value: Point) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}

#[derive(Debug)]
pub enum Error<'a> {
    CGEventNotCreated,
    CGEventSourceStateInvalid,

    InvalidButtonStr(&'a str),
}

impl<'a> error::Error for Error<'a> {}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CGEventNotCreated => write!(f, "CGEvent could not be created"),
            Error::CGEventSourceStateInvalid => write!(f, "invalid CGEventSourceStateID"),

            Error::InvalidButtonStr(button) => write!(f, "invalid button str: {button:?}"),
        }
    }
}

pub struct Mouse;

impl Mouse {
    fn event_source<'a>() -> Result<CGEventSource, Error<'a>> {
        CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
            .or(Err(Error::CGEventSourceStateInvalid))
    }

    pub const fn new() -> Self {
        Self
    }

    pub fn move_to(x: i32, y: i32) -> Result<(), Box<dyn error::Error>> {
        let point = CGPoint::new(x.into(), y.into());

        CGEvent::new_mouse_event(
            Self::event_source()?,
            CGEventType::MouseMoved,
            point,
            CGMouseButton::Left, // ignored
        )
        .or(Err(Error::CGEventNotCreated))?
        .post(CGEventTapLocation::HID);

        Ok(())
    }

    pub fn press<'a>(button: &'a Keys) -> Result<(), Box<dyn error::Error + 'a>> {
        let (event_type, mouse_button) = match button {
            Keys::LEFT => Ok((CGEventType::LeftMouseDown, CGMouseButton::Left)),
            Keys::MIDDLE => Ok((CGEventType::OtherMouseDown, CGMouseButton::Center)),
            Keys::RIGHT => Ok((CGEventType::RightMouseDown, CGMouseButton::Right)),
            _ => Err(Box::new(Error::InvalidButtonStr(""))),
        }?;

        CGEvent::new_mouse_event(
            Self::event_source()?,
            event_type,
            Self::get_position()?.into(),
            mouse_button,
        )
        .or(Err(Error::CGEventNotCreated))?
        .post(CGEventTapLocation::HID);

        Ok(())
    }

    pub fn release<'a>(button: &'a Keys) -> Result<(), Box<dyn error::Error + 'a>> {
        let (event_type, mouse_button) = match button {
            Keys::LEFT => Ok((CGEventType::LeftMouseUp, CGMouseButton::Left)),
            Keys::WHEEL | Keys::MIDDLE => Ok((CGEventType::OtherMouseUp, CGMouseButton::Center)),
            Keys::RIGHT => Ok((CGEventType::RightMouseUp, CGMouseButton::Right)),
            _ => Err(Box::new(Error::InvalidButtonStr(""))),
        }?;

        CGEvent::new_mouse_event(
            Self::event_source()?,
            event_type,
            Self::get_position()?.into(),
            mouse_button,
        )
        .or(Err(Error::CGEventNotCreated))?
        .post(CGEventTapLocation::HID);

        Ok(())
    }

    pub fn get_position() -> Result<Point, Box<dyn error::Error>> {
        Ok(CGEvent::new(Self::event_source()?)
            .or(Err(Error::CGEventNotCreated))?
            .location()
            .into())
    }

    pub fn wheel(delta: i32) -> Result<(), Box<dyn error::Error>> {
        CGEvent::new_scroll_event(Self::event_source()?, ScrollEventUnit::LINE, 1, delta, 0, 0)
            .or(Err(Error::CGEventNotCreated))?
            .post(CGEventTapLocation::HID);

        Ok(())
    }
}
