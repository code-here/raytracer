use std::cmp::Ordering;

pub mod light;
pub mod material;
pub mod sphere;

pub trait Object {}

#[derive(Debug, Clone)]
pub struct Intersection<T: Object + Clone> {
    pub distance: f64,
    pub object: T,
}

impl<T: Object + Clone> Intersection<T> {
    pub fn new(distance: f64, object: T) -> Self {
        Self { distance, object }
    }
    pub fn hits(intersections: &[Intersection<T>]) -> Option<Intersection<T>> {
        // if distance is negative than the object is behind the ray so exclude those intersections in hits
        let mut positive_intersections = intersections
            .iter()
            .filter(|&i| i.distance >= 0.0)
            .collect::<Vec<&Intersection<T>>>();
        if positive_intersections.is_empty() {
            // if objects are not in front of ray
            None
        } else {
            positive_intersections.sort();
            Some(positive_intersections[0].clone())
        }
    }
}

impl<T: Object + Clone> Ord for Intersection<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance == other.distance {
            Ordering::Equal
        } else if self.distance < other.distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl<T: Object + Clone> PartialOrd for Intersection<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.distance == other.distance {
            Some(Ordering::Equal)
        } else if self.distance < other.distance {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<T: Object + Clone> PartialEq for Intersection<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T: Object + Clone> Eq for Intersection<T> {}
