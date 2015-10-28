use simplex_noise::permutations::Permutations;
use simplex_noise::grad::Grad;

pub fn noise2d(x: f32, y: f32) -> f32 {
    let grad3: Vec<Grad> = vec![
        Grad::new(1.0, 1.0, 0.0, 0.0),  Grad::new(-1.0, 1.0, 0.0, 0.0),
        Grad::new(1.0, -1.0, 0.0, 0.0), Grad::new(-1.0, -1.0, 0.0, 0.0),
        Grad::new(1.0, 0.0, 1.0, 0.0),  Grad::new(-1.0, 0.0, 1.0, 0.0),
        Grad::new(1.0, 0.0, -1.0, 0.0), Grad::new(-1.0, 0.0, -1.0, 0.0),
        Grad::new(0.0, 1.0, 1.0, 0.0),  Grad::new(0.0, -1.0, 1.0, 0.0),
        Grad::new(0.0, 1.0, -1.0, 0.0), Grad::new(0.0, -1.0, -1.0, 0.0),
    ];

    // Skewing and unskewing factors for 2 dimensions
    let three_square = (3_f32).sqrt();
    let f2: f32 = 0.5 * (three_square - 1.0);
    let g2: f32 = (3.0 - three_square) / 6.0;

    // Pull in our permutations
    let perms = Permutations::new();

    // Hairy factor for 2D
    let s = (x + y) * f2;

    let i = fast_floor(x + s);
    let j = fast_floor(y + s);

    let t = (i + j) * g2;

    // The x, y distances from the cell origin
    let x0 = x - (i - t);
    let y0 = y - (j - t);

    // For the 2D case, the simplex shape is an equilateral
    // triangle.  Determine which simplex we are in.
    let (i1, j1) = if x0 > y0 { (1.0, 0.0) } else { (0.0, 1.0) };

    // Offsets for middle corner in (x, y) unskewed coords.
    let x1 = x0 - i1 + g2;
    let y1 = y0 - j1 + g2;
    // Offsets from last corner in (x, y) unskewed coords.
    let x2 = x0 - 1.0 + 2.0 * g2;
    let y2 = y0 - 1.0 + 2.0 * g2;

    // Work out the hashed gradient indices of the three simplex corners.
    let ii = (i as i32 & 255) as f32;
    let jj = (j as i32 & 255) as f32;

    let gi0 = perms.perm_mod_12[(ii + perms.perm[jj as usize]) as usize];
    let gi1 = perms.perm_mod_12[(ii + i1 + perms.perm[(jj + j1) as usize]) as usize];
    let gi2 = perms.perm_mod_12[(ii + 1.0 + perms.perm[(jj + 1.0) as usize]) as usize];

    // Calculate the contribution from the three corners
    let n0 = calculate_corner_contribution(x0, y0, &grad3[gi0 as usize]);
    let n1 = calculate_corner_contribution(x1, y1, &grad3[gi1 as usize]);
    let n2 = calculate_corner_contribution(x2, y2, &grad3[gi2 as usize]);

    70.0 * (n0 + n1 + n2)
}

fn calculate_corner_contribution(x: f32, y: f32, grad: &Grad) -> f32 {
    let t = 0.5 - x * x - y * y;
    if t < 0.0 { 0.0 } else { fade(t, x, y, grad) }
}

fn fade(mut t: f32, x: f32, y: f32, grad: &Grad) -> f32 {
    t *= t;
    t * t * dot2d(grad, x, y)
}

fn dot2d(g: &Grad, x: f32, y: f32) -> f32 {
    g.x * x + g.y * y
}

fn fast_floor(x: f32) -> f32 {
    if x > 0.0 { x.floor() } else { (x - 1.0).floor() }
}

// fn dot3d(g: &Grad, x: f32, y: f32, z: f32) -> f32 {
//     g.x * x + g.y * y + g.z * z
// }
