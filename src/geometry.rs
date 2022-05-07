type Fp32 = I20F12;

// source: https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
fn line_intersection(line0: Vector2<i32>, line0_dir: Vector2<Fp32>, line1: Vector2<i32>, line1_dir: Vector2<Fp32>) -> Option<Vector2<Fp32>>
{
    let line0: Vector2<Fp32> = vec2_cast(line0);
    let line1: Vector2<Fp32> = vec2_cast(line1);

    let line0_end = vec2_add(line0, line0_dir);
    let line1_end = vec2_add(line1, line1_dir);

    let xdiff: Vector2<Fp32> = [-line0_dir[0], -line1_dir[0]];

    todo!();
}

// based on: http://www.cse.yorku.ca/~amana/research/grid.pdf
struct GridTraversalIter {
    cur: Vector2<i32>,
    dir: Vector2<Fp32>,
    /// Distance to next X and Y cell
    max: Vector2<Fp32>,
    /// Distance along X and Y of dir that results in 1
    dlt: Vector2<Fp32>,
}

impl GridTraversalIter {
    fn new(pos: Vector2<i32>, dir: Vector2<Fp32>) -> Self {
        
    }
}

impl Iterator for GridTraversalIter {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {

        Some(self.cur)
    }
}
