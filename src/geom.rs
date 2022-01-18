use super::data::Rect;

// Rect-rect intersection test. Returns `Some(Rect)` if there's an intersection, otherwise `None`.
pub fn find_intersection(r1: &Rect, r2: &Rect) -> Option<Rect> {
    let mut intersection = Rect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    // Maybe there's a more elegant way to handle rect-rect intersection, but this'll do for the purpose
    // of this task.
    intersection.x = f32::max(r1.x, r2.x);
    intersection.y = f32::max(r1.y, r2.y);
    intersection.width = f32::min(r1.x + r1.width, r2.x + r2.width) - intersection.x;
    intersection.height = f32::min(r1.y + r1.height, r2.y + r2.height) - intersection.y;

    // If dimensions are negative, we don't have an intersection.
    if intersection.width > 0.0 && intersection.height > 0.0 {
        Some(intersection)
    } else {
        None
    }
}

pub fn compute_rect_area(r: &Rect) -> f32 {
    r.width * r.height
}

#[cfg(test)]
mod test {
    use super::find_intersection;
    use super::Rect;

    #[test]
    pub fn basic_intersection() {
        // `obj1` and `obj2` should intersect, while `obj1` and `obj3` should not.
        let obj1 = Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };

        let obj2 = Rect {
            x: 50.0,
            y: 50.0,
            width: 100.0,
            height: 100.0,
        };

        let obj3 = Rect {
            x: 101.0,
            y: 101.0,
            width: 100.0,
            height: 100.0,
        };

        // Make sure we the results are correct independently of rect order.
        assert_eq!(
            find_intersection(&obj1, &obj2),
            Some(Rect {
                x: 50.0,
                y: 50.0,
                width: 50.0,
                height: 50.0,
            })
        );

        assert_eq!(
            find_intersection(&obj2, &obj1),
            Some(Rect {
                x: 50.0,
                y: 50.0,
                width: 50.0,
                height: 50.0,
            })
        );

        assert_eq!(find_intersection(&obj1, &obj3), None);
        assert_eq!(find_intersection(&obj3, &obj1), None);
    }
}
