 # Computational Geometry Library



##  Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Data Structures](#data-structures)
- [Algorithms](#algorithms)
- [Performance](#performance)
- [Usage Examples](#usage-examples)
- [Testing](#testing)
- [Future Enhancements](#future-enhancements)

##  Overview

This library provides efficient implementations of classical computational geometry algorithms, particularly emphasizing divide-and-conquer techniques for optimal performance. The codebase is designed for both educational purposes and real-world applications requiring geometric computations.

##  Features

### Core Geometric Primitives

- **Point operations** with Euclidean distance calculations
- **Line segment** representation and intersection testing
- **Convex hull** computation using Graham scan
- **K-d tree** for efficient spatial queries

### Algorithm Implementations

- **Closest pair of points** (both brute force and divide-and-conquer)
- **Line segment intersection** detection
- **Nearest neighbor search** using k-d trees
- **Convex hull** construction

### Performance Optimizations

- Distance squared calculations for faster comparisons
- Divide-and-conquer approaches for O(n log n) complexity
- Efficient spatial data structures

## 📊 Data Structures

### Point

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```

**Key Methods:**

- `new(x, y)` - Create a new point
- `distance_to(&other)` - Calculate Euclidean distance
- `distance_squared_to(&other)` - Calculate squared distance (faster for comparisons)

### LineSegment

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}
```

**Key Methods:**

- `new(start, end)` - Create a new line segment
- `intersects(&other)` - Test intersection with another segment

### KdTree

```rust
#[derive(Debug, Clone)]
pub struct KdTree {
    root: Option<Box<KdNode>>,
}
```

**Key Methods:**

- `new()` - Create empty k-d tree
- `build(points)` - Build tree from point array
- `nearest_neighbor(&query)` - Find closest point to query

### ClosestPairResult

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ClosestPairResult {
    pub point1: Point,
    pub point2: Point,
    pub distance: f64,
}
```

##  Algorithms

### 1. Closest Pair of Points

#### Brute Force Approach

- **Time Complexity:** O(n²)
- **Space Complexity:** O(1)
- **Best for:** Small datasets (n < 50)

```rust
pub fn closest_pair_brute_force(points: &[Point]) -> Option<ClosestPairResult>
```

#### Divide and Conquer Approach

- **Time Complexity:** O(n log n)
- **Space Complexity:** O(n)
- **Best for:** Large datasets

```rust
pub fn closest_pair_divide_conquer(points: &[Point]) -> Option<ClosestPairResult>
```

**Algorithm Steps:**

1. Sort points by x and y coordinates
2. Recursively divide the point set
3. Find closest pairs in left and right halves
4. Check points near the dividing line
5. Return the minimum distance pair

### 2. Convex Hull (Graham Scan)

- **Time Complexity:** O(n log n)
- **Space Complexity:** O(n)

```rust
pub fn convex_hull_graham_scan(points: &[Point]) -> Vec<Point>
```

**Algorithm Steps:**

1. Find the bottom-most point (lexicographically smallest)
2. Sort remaining points by polar angle
3. Use stack-based approach to build hull
4. Remove points that create clockwise turns

### 3. Line Segment Intersection

```rust
pub fn find_intersecting_segments(segments: &[LineSegment]) -> Vec<(usize, usize)>
```

**Features:**

- Handles general position intersections
- Manages collinear cases
- Returns all intersecting pairs

### 4. K-d Tree Nearest Neighbor

- **Build Time:** O(n log n)
- **Query Time:** O(log n) average, O(n) worst case
- **Space Complexity:** O(n)

```rust
pub fn nearest_neighbor(&self, query: &Point) -> Option<Point>
```

## ⚡ Performance

|Algorithm                      |Time Complexity|Space Complexity|Best Use Case     |
|-------------------------------|---------------|----------------|------------------|
|Closest Pair (Brute Force)     |O(n²)          |O(1)            |n < 50            |
|Closest Pair (Divide & Conquer)|O(n log n)     |O(n)            |n ≥ 50            |
|Convex Hull (Graham Scan)      |O(n log n)     |O(n)            |Any size          |
|Line Intersection (Brute Force)|O(n²)          |O(1)            |Small segment sets|
|K-d Tree Build                 |O(n log n)     |O(n)            |Multiple queries  |
|K-d Tree Query                 |O(log n) avg   |O(log n)        |Spatial searches  |

##  Usage Examples

### Basic Point Operations

```rust
use geometry::{Point, closest_pair_divide_conquer};

let points = vec![
    Point::new(0.0, 0.0),
    Point::new(1.0, 1.0),
    Point::new(5.0, 5.0),
    Point::new(2.0, 2.0),
];

// Find closest pair
if let Some(result) = closest_pair_divide_conquer(&points) {
    println!("Closest pair: ({:.2}, {:.2}) and ({:.2}, {:.2})", 
             result.point1.x, result.point1.y,
             result.point2.x, result.point2.y);
    println!("Distance: {:.2}", result.distance);
}
```

### Convex Hull Computation

```rust
use geometry::{Point, convex_hull_graham_scan};

let points = vec![
    Point::new(0.0, 0.0),
    Point::new(1.0, 0.0),
    Point::new(0.5, 1.0),
    Point::new(0.0, 1.0),
];

let hull = convex_hull_graham_scan(&points);
println!("Convex hull has {} vertices", hull.len());
```

### Line Segment Intersection

```rust
use geometry::{Point, LineSegment, find_intersecting_segments};

let segments = vec![
    LineSegment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0)),
    LineSegment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0)),
    LineSegment::new(Point::new(3.0, 3.0), Point::new(4.0, 4.0)),
];

let intersections = find_intersecting_segments(&segments);
println!("Found {} intersecting pairs", intersections.len());
```

### K-d Tree Nearest Neighbor Search

```rust
use geometry::{Point, KdTree};

let points = vec![
    Point::new(2.0, 3.0),
    Point::new(5.0, 4.0),
    Point::new(9.0, 6.0),
    Point::new(4.0, 7.0),
];

let tree = KdTree::build(&points);
let query = Point::new(5.0, 5.0);

if let Some(nearest) = tree.nearest_neighbor(&query) {
    println!("Nearest point: ({:.2}, {:.2})", nearest.x, nearest.y);
}
```

# Testing

The library includes comprehensive test coverage:

```bash
cargo test
```

### Test Coverage

- ✅ Point distance calculations
- ✅ Closest pair algorithms (both approaches)
- ✅ Line segment intersection detection
- ✅ K-d tree construction and queries
- ✅ Edge cases and boundary conditions

### Example Test

```rust
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
```

##  Future Enhancements

### Planned Features

- **Sweep Line Algorithm** for O(n log n) line segment intersection
- **Voronoi Diagrams** computation
- **Delaunay Triangulation** implementation
- **3D geometric primitives** and algorithms
- **Polygon operations** (union, intersection, difference)

### Performance Optimizations

- **Parallel processing** for divide-and-conquer algorithms
- **SIMD optimizations** for distance calculations
- **Memory pool allocation** for tree structures
- **Cache-friendly data layouts**

### Additional Data Structures

- **R-trees** for spatial indexing
- **Quadtrees** for 2D spatial partitioning
- **BSP trees** for geometric queries

## Applications

This library is suitable for:

- **CAD/CAM Software** - geometric modeling and analysis
- **Game Development** - collision detection and spatial queries
- **GIS Applications** - geographic data processing
- **Computer Graphics** - rendering and geometric algorithms
- **Robotics** - path planning and obstacle avoidance
- **Machine Learning** - nearest neighbor algorithms
- **Scientific Computing** - computational geometry research


## Unused Functions in geometry.rs and Their Purpose:

### 1. **KdTree-related Functionality**

```rust
impl KdTree {
    pub fn new() -> Self                    // Create empty k-d tree
    pub fn build(points: &[Point]) -> Self  // Build k-d tree from point cloud
    pub fn nearest_neighbor(&self, query: &Point) -> Option<Point>  // Nearest neighbor search
}
```

**Purpose**: Data structure for fast nearest neighbor search
**Use cases**:

- Efficient search for the closest point from a large set (O(log n))
- k-NN classification in machine learning
- Enemy character search in games
- GIS applications

### 2. **Extended Functionality for Point Structure**

```rust
impl Point {
    pub fn new(x: f64, y: f64) -> Self                        // Create point
    pub fn distance_squared_to(&self, other: &Point) -> f64   // Distance squared (faster)
}
```

**Purpose**:

- `new()` → More explicit point creation
- `distance_squared_to()` → Speed optimization by avoiding square root calculation (sufficient for comparisons)

### 3. **LineSegment-related Functions**

```rust
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self              // Create line segment
    pub fn intersects(&self, other: &LineSegment) -> bool     // Line segment intersection test
}
```

**Purpose**: Basic elements of computational geometry
**Use cases**:

- CAD software
- Road intersection detection in map applications
- Collision detection in games
- Path planning in robotics

### 4. **Advanced Geometric Algorithms**

```rust
pub fn find_intersecting_segments(segments: &[LineSegment]) -> Vec<(usize, usize)>
```

**Purpose**: Detect all intersecting pairs among multiple line segments
**Use cases**:

- Wire routing checks in circuit board design
- Interference checking in architectural drawings

## Why They’re Currently Unused:

Current benchmark only uses:

```rust
// Only closest pair problem is used
crate::geometry::closest_pair_divide_conquer(points)
```

**Originally intended extensions**:

- **Performance comparison**: Brute force vs K-d tree for nearest neighbor search
- **Complex geometric problems**: Line intersection, convex hull, Voronoi diagrams, etc.
- **Real-world applications**: Use in GPS navigation, game physics engines

## Additional Unused Functions in geometry.rs:

### 5. **`convex_hull_graham_scan()`**

```rust
pub fn convex_hull_graham_scan(points: &[Point]) -> Vec<Point>
```

**Purpose**: Convex Hull calculation using Graham Scan method
**Use cases**:

- **Game development**: Simplifying character collision detection
- **Image processing**: Boundary detection in object recognition
- **Robotics**: Path planning for obstacle avoidance
- **GIS**: Geographic region boundary calculation

**Algorithm**: Efficient convex hull algorithm with O(n log n) complexity

### 6. **Geometric Helper Functions**

#### **`polar_angle()`**

```rust
fn polar_angle(origin: &Point, point: &Point) -> f64
```

**Purpose**: Calculate polar angle (argument) from origin to specified point
**Usage**: Sort points by angle in Graham Scan

#### **`cross_product()`**

```rust
fn cross_product(o: &Point, a: &Point, b: &Point) -> f64
```

**Purpose**: Cross product calculation of 3 points (orientation determination)
**Use cases**:

- Determine which side of a line segment a point is on
- Polygon area calculation
- Rotation direction determination (clockwise/counterclockwise)

#### **`direction()`**

```rust
fn direction(pi: &Point, pj: &Point, pk: &Point) -> f64
```

**Purpose**: Calculate orientation of 3 points
**Usage**: Geometric primitive that forms the basis for line segment intersection tests

#### **`on_segment()`**

```rust
fn on_segment(pi: &Point, pj: &Point, pk: &Point) -> bool
```

**Purpose**: Determine if a point lies on a line segment
**Usage**: Handle special cases in line intersection (intersection at endpoints)

## Why Such Detailed Implementation?

### 1. **Academic Completeness**

- Comprehensive coverage of basic algorithms found in computational geometry textbooks
- Wanted to include various geometric problems as a “divide and conquer” project

### 2. **Real-world Demand**

- CAD/CAM software
- Physics calculations in game engines
- Path planning for autonomous vehicles
- Design support in architecture and civil engineering

### 3. **Algorithm Comparison Material**

**Originally intended extensions**:

```rust
// Performance comparison of convex hull algorithms
runner.benchmark_convex_hull("Graham Scan", &points);
runner.benchmark_convex_hull("Gift Wrapping", &points);

// Line intersection detection comparison
runner.benchmark_line_intersection("Brute Force", &segments);
runner.benchmark_line_intersection("Sweep Line", &segments);
```

### 7. **`find_intersecting_segments()`**

```rust
pub fn find_intersecting_segments(segments: &[LineSegment]) -> Vec<(usize, usize)>
```

## Purpose and Applications of This Function:

### **Functionality**

- Detect all intersecting pairs among multiple line segments
- Returns index pairs of intersecting line segments

### **Real-world Applications**

#### **1. CAD/Design Software**

```rust
// Wire routing check on circuit boards
let pcb_traces = vec![...];  // Traces on the board
let intersections = find_intersecting_segments(&pcb_traces);
if !intersections.is_empty() {
    println!("Warning: Wire intersections detected: {:?}", intersections);
}
```

#### **2. Architecture/Civil Engineering Design**

```rust
// Interference check for building beams and columns
let structural_elements = vec![...];
let conflicts = find_intersecting_segments(&structural_elements);
```

#### **3. Game Development**

```rust
// Intersection detection for lasers and projectile trajectories
let laser_beams = vec![...];
let collisions = find_intersecting_segments(&laser_beams);
```

#### **4. Maps/GIS**

```rust
// Intersection detection for roads and boundary lines
let roads = vec![...];
let crossroads = find_intersecting_segments(&roads);
```

### **Algorithmic Perspective**

Current implementation: **Brute Force O(n²)**

```rust
// Check all line segment pairs
for i in 0..segments.len() {
    for j in (i + 1)..segments.len() {
        if segments[i].intersects(&segments[j]) {
            intersections.push((i, j));
        }
    }
}
```

**More efficient implementation**: **Sweep Line Algorithm O(n log n)**

- “Sweep” a vertical line from left to right
- Manage only active line segments
- Fast processing even with large numbers of line segments

### **Originally Intended Extensions**

```rust
// Algorithm comparison benchmarks
runner.benchmark_intersection("Brute Force", &segments);
runner.benchmark_intersection("Sweep Line", &segments);
runner.benchmark_intersection("R-Tree", &segments);
```

## Summary of Unused Functions:

1. `convex_hull_graham_scan()` - Convex hull calculation
2. `polar_angle()` - Polar angle calculation
3. `cross_product()` - Cross product calculation
4. `direction()` - Orientation determination
5. `on_segment()` - Check if point lies on line segment
6. `find_intersecting_segments()` - Line segment intersection detection
7. KdTree-related - Fast nearest neighbor search
8. LineSegment-related - Line segment operations

All these functions represent a comprehensive computational geometry library designed for future extensibility and real-world applications.