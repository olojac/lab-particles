use crate::types::{Line, Position};

pub fn find(
    position: Position,
    next_position: Position,
    bounds: Vec<&Line>,
) -> (Position, f64, &Line) {
    let travel_line = (position, next_position);
    let original_distance = (next_position - position).norm();
    let mut distance = original_distance;
    let mut end_point = next_position;
    let mut selected_bound = bounds[0];

    for bound in bounds {
        if let Some(intersection) = intersection(&travel_line, bound) {
            let distance_to_intersection = (intersection - position).norm();
            if distance_to_intersection < distance {
                end_point = intersection;
                selected_bound = bound;
                distance = distance_to_intersection;
            }
        }
    }

    let distance_quota = distance / original_distance;

    (end_point, distance_quota, selected_bound)
}

pub fn intersection(line_a: &Line, line_b: &Line) -> Option<Position> {
    let p = line_a.0;
    let q = line_b.0;
    let r = line_a.1 - p;
    let s = line_b.1 - q;

    if cross(r, s) == 0.0 {
        return None;
    }

    let t = cross(q - p, s / cross(r, s));
    let u = cross(p - q, r / cross(s, r));

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        Some(p + r * t)
    } else {
        None
    }
}

fn cross(a: Position, b: Position) -> f64 {
    a.x * b.y - a.y * b.x
}
