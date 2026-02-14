use rocket::shield::{Feature, Frame, Permission, Shield};

pub fn init() -> Shield {
    let permission = Permission::default()
        .block(Feature::Camera)
        .block(Feature::Microphone)
        .block(Feature::Geolocation)
        .block(Feature::Payment);

    Shield::default()
        .enable(Frame::Deny)
        .enable(permission)
}
