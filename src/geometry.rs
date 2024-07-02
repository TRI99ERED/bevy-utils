use bevy::prelude::*;

pub struct UniquePlane {
    pub plane: Plane3d,
    pub origin: Vec3,
}

impl UniquePlane {
    pub fn new(plane: Plane3d, origin: Vec3) -> Self {
        Self { plane, origin }
    }

    pub fn find_closest_point(&self, p: Vec3) -> Vec3 {
        find_closest_point_to_plane(self.origin, *self.plane.normal, p)
    }
}

pub fn find_closest_point_to_plane(origin: Vec3, normal: Vec3, p: Vec3) -> Vec3 {
    let vector_to_point = p - origin;
    let projection = normal.dot(vector_to_point) * normal;

    origin + vector_to_point - projection
}
