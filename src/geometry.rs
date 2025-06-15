use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    /// Calculate Euclidean distance between two points
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Calculate squared distance (faster for comparisons)
    pub fn distance_squared_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosestPairResult {
    pub point1: Point,
    pub point2: Point,
    pub distance: f64,
}

/// Brute force approach to find closest pair of points
/// Time complexity: O(n²)
pub fn closest_pair_brute_force(points: &[Point]) -> Option<ClosestPairResult> {
    if points.len() < 2 {
        return None;
    }
    
    let mut min_distance = f64::INFINITY;
    let mut closest_pair = (points[0], points[1]);
    
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance_to(&points[j]);
            if distance < min_distance {
                min_distance = distance;
                closest_pair = (points[i], points[j]);
            }
        }
    }
    
    Some(ClosestPairResult {
        point1: closest_pair.0,
        point2: closest_pair.1,
        distance: min_distance,
    })
}

/// Divide and conquer approach to find closest pair of points
/// Time complexity: O(n log n)
pub fn closest_pair_divide_conquer(points: &[Point]) -> Option<ClosestPairResult> {
    if points.len() < 2 {
        return None;
    }
    
    // Create sorted copies
    let mut points_x = points.to_vec();
    let mut points_y = points.to_vec();
    
    // Sort by x and y coordinates
    points_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    points_y.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    
    closest_pair_rec(&points_x, &points_y)
}

fn closest_pair_rec(points_x: &[Point], points_y: &[Point]) -> Option<ClosestPairResult> {
    let n = points_x.len();
    
    // Base case: use brute force for small arrays
    if n <= 3 {
        return closest_pair_brute_force(points_x);
    }
    
    // Divide
    let mid = n / 2;
    let midpoint = points_x[mid];
    
    let (left_x, right_x) = points_x.split_at(mid);
    
    // Split points_y into left and right based on x coordinate
    let mut left_y = Vec::new();
    let mut right_y = Vec::new();
    
    for &point in points_y {
        if point.x <= midpoint.x {
            left_y.push(point);
        } else {
            right_y.push(point);
        }
    }
    
    // Conquer
    let left_result = closest_pair_rec(left_x, &left_y);
    let right_result = closest_pair_rec(right_x, &right_y);
    
    // Find minimum distance from both sides
    let mut min_result = match (left_result, right_result) {
        (Some(left), Some(right)) => {
            if left.distance <= right.distance { left } else { right }
        }
        (Some(result), None) | (None, Some(result)) => result,
        (None, None) => return None,
    };
    
    // Check points close to the dividing line
    let mut strip = Vec::new();
    for &point in points_y {
        if (point.x - midpoint.x).abs() < min_result.distance {
            strip.push(point);
        }
    }
    
    // Check closest pair in strip
    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() && (strip[j].y - strip[i].y) < min_result.distance {
            let distance = strip[i].distance_to(&strip[j]);
            if distance < min_result.distance {
                min_result = ClosestPairResult {
                    point1: strip[i],
                    point2: strip[j],
                    distance,
                };
            }
            j += 1;
        }
    }
    
    Some(min_result)
}

/// Find the convex hull using Graham scan algorithm
/// Time complexity: O(n log n)
pub fn convex_hull_graham_scan(points: &[Point]) -> Vec<Point> {
    if points.len() < 3 {
        return points.to_vec();
    }
    
    // Find the bottom-most point (and left-most in case of tie)
    let mut bottom_point = points[0];
    for &point in points.iter().skip(1) {
        if point.y < bottom_point.y || (point.y == bottom_point.y && point.x < bottom_point.x) {
            bottom_point = point;
        }
    }
    
    // Sort points by polar angle with respect to bottom point
    let mut sorted_points: Vec<Point> = points.iter()
        .filter(|&&p| p != bottom_point)
        .cloned()
        .collect();
    
    sorted_points.sort_by(|a, b| {
        let angle_a = polar_angle(&bottom_point, a);
        let angle_b = polar_angle(&bottom_point, b);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
    
    let mut hull = vec![bottom_point];
    
    for point in sorted_points {
        // Remove points that make clockwise turn
        while hull.len() > 1 && cross_product(&hull[hull.len()-2], &hull[hull.len()-1], &point) <= 0.0 {
            hull.pop();
        }
        hull.push(point);
    }
    
    hull
}

fn polar_angle(origin: &Point, point: &Point) -> f64 {
    (point.y - origin.y).atan2(point.x - origin.x)
}

fn cross_product(o: &Point, a: &Point, b: &Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

/// Line segment intersection using divide and conquer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
    
    /// Check if two line segments intersect
    pub fn intersects(&self, other: &LineSegment) -> bool {
        let d1 = direction(&other.start, &other.end, &self.start);
        let d2 = direction(&other.start, &other.end, &self.end);
        let d3 = direction(&self.start, &self.end, &other.start);
        let d4 = direction(&self.start, &self.end, &other.end);
        
        if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0)) &&
           ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0)) {
            return true;
        }
        
        // Check for collinear cases
        if d1 == 0.0 && on_segment(&other.start, &self.start, &other.end) ||
           d2 == 0.0 && on_segment(&other.start, &self.end, &other.end) ||
           d3 == 0.0 && on_segment(&self.start, &other.start, &self.end) ||
           d4 == 0.0 && on_segment(&self.start, &other.end, &self.end) {
            return true;
        }
        
        false
    }
}

fn direction(pi: &Point, pj: &Point, pk: &Point) -> f64 {
    cross_product(pi, pj, pk)
}

fn on_segment(pi: &Point, pj: &Point, pk: &Point) -> bool {
    pj.x <= pi.x.max(pk.x) && pj.x >= pi.x.min(pk.x) &&
    pj.y <= pi.y.max(pk.y) && pj.y >= pi.y.min(pk.y)
}

/// Find all intersecting pairs of line segments using divide and conquer
pub fn find_intersecting_segments(segments: &[LineSegment]) -> Vec<(usize, usize)> {
    let mut intersections = Vec::new();
    
    // Brute force approach for simplicity (can be optimized with sweep line algorithm)
    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if segments[i].intersects(&segments[j]) {
                intersections.push((i, j));
            }
        }
    }
    
    intersections
}

/// K-d tree implementation for efficient nearest neighbor search
#[derive(Debug, Clone)]
pub struct KdTree {
    root: Option<Box<KdNode>>,
}

#[derive(Debug, Clone)]
struct KdNode {
    point: Point,
    left: Option<Box<KdNode>>,
    right: Option<Box<KdNode>>,
    dimension: usize, // 0 for x, 1 for y
}

impl KdTree {
    pub fn new() -> Self {
        Self { root: None }
    }
    
    /// Build k-d tree from points
    pub fn build(points: &[Point]) -> Self {
        let mut tree = Self::new();
        if !points.is_empty() {
            tree.root = Some(Self::build_recursive(points.to_vec(), 0));
        }
        tree
    }
    
    fn build_recursive(mut points: Vec<Point>, depth: usize) -> Box<KdNode> {
        let dimension = depth % 2;
        
        // Sort points by current dimension
        if dimension == 0 {
            points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        } else {
            points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        }
        
        let mid = points.len() / 2;
        let point = points[mid];
        
        let mut node = Box::new(KdNode {
            point,
            left: None,
            right: None,
            dimension,
        });
        
        if mid > 0 {
            node.left = Some(Self::build_recursive(points[..mid].to_vec(), depth + 1));
        }
        
        if mid + 1 < points.len() {
            node.right = Some(Self::build_recursive(points[mid + 1..].to_vec(), depth + 1));
        }
        
        node
    }
    
    /// Find nearest neighbor to a query point
    pub fn nearest_neighbor(&self, query: &Point) -> Option<Point> {
        self.root.as_ref().map(|root| {
            let mut best = root.point;
            let mut best_distance = query.distance_squared_to(&best);
            
            Self::nearest_neighbor_recursive(root, query, &mut best, &mut best_distance);
            best
        })
    }
    
    fn nearest_neighbor_recursive(
        node: &KdNode,
        query: &Point,
        best: &mut Point,
        best_distance: &mut f64,
    ) {
        let distance = query.distance_squared_to(&node.point);
        if distance < *best_distance {
            *best = node.point;
            *best_distance = distance;
        }
        
        let query_coord = if node.dimension == 0 { query.x } else { query.y };
        let node_coord = if node.dimension == 0 { node.point.x } else { node.point.y };
        
        let (near_child, far_child) = if query_coord < node_coord {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };
        
        // Search near child first
        if let Some(child) = near_child {
            Self::nearest_neighbor_recursive(child, query, best, best_distance);
        }
        
        // Check if we need to search far child
        let axis_distance = (query_coord - node_coord).powi(2);
        if axis_distance < *best_distance {
            if let Some(child) = far_child {
                Self::nearest_neighbor_recursive(child, query, best, best_distance);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }
    
    #[test]
    fn test_closest_pair_brute_force() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(5.0, 5.0),
            Point::new(2.0, 2.0),
        ];
        
        let result = closest_pair_brute_force(&points).unwrap();
        assert!((result.distance - 2.0_f64.sqrt()).abs() < 1e-10);
    }
    
    #[test]
    fn test_closest_pair_divide_conquer() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(5.0, 5.0),
            Point::new(2.0, 2.0),
        ];
        
        let result = closest_pair_divide_conquer(&points).unwrap();
        assert!((result.distance - 2.0_f64.sqrt()).abs() < 1e-10);
    }
    
    #[test]
    fn test_line_segment_intersection() {
        let seg1 = LineSegment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
        let seg2 = LineSegment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0));
        assert!(seg1.intersects(&seg2));
        
        let seg3 = LineSegment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let seg4 = LineSegment::new(Point::new(2.0, 2.0), Point::new(3.0, 3.0));
        assert!(!seg3.intersects(&seg4));
    }
    
    #[test]
    fn test_kdtree() {
        let points = vec![
            Point::new(2.0, 3.0),
            Point::new(5.0, 4.0),
            Point::new(9.0, 6.0),
            Point::new(4.0, 7.0),
            Point::new(8.0, 1.0),
            Point::new(7.0, 2.0),
        ];
        
        let tree = KdTree::build(&points);
        let query = Point::new(5.0, 5.0);
        let nearest = tree.nearest_neighbor(&query).unwrap();
        
        // Should find one of the nearby points
        assert!(query.distance_to(&nearest) < 3.0);
    }
}