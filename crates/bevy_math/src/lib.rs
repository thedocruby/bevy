mod geometry;

pub use geometry::*;
pub use glam::*;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        BVec2, BVec3, BVec4, EulerRot, IVec2, IVec3, IVec4, Mat3, Mat4, Quat, Rect, UVec2, UVec3,
        UVec4, Vec2, Vec3, Vec4,
    };
}
