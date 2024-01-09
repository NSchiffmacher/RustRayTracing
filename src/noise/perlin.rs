use crate::vector::Vec3;

use rand::Rng;

const PERLIN_POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: [Vec3; PERLIN_POINT_COUNT],

    perm_x: [usize; PERLIN_POINT_COUNT],
    perm_y: [usize; PERLIN_POINT_COUNT],
    perm_z: [usize; PERLIN_POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = [Vec3::zero(); PERLIN_POINT_COUNT];
        for i in 0..PERLIN_POINT_COUNT {
            ranvec[i] = Vec3::random(-1., 1.).normalized();
        }

        Self {
            perm_x: Self::generate_permutation(),
            perm_y: Self::generate_permutation(),
            perm_z: Self::generate_permutation(),
            ranvec,
        }
    }

    fn generate_permutation() -> [usize; PERLIN_POINT_COUNT] {
        let mut rng = rand::thread_rng();

        let mut permutation = [0; PERLIN_POINT_COUNT];
        for i in 0..PERLIN_POINT_COUNT {
            permutation[i] = i;
        }

        for i in (1..PERLIN_POINT_COUNT).rev() {
            let target = rng.gen_range(0..i);
            permutation.swap(i, target);
        }

        permutation
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermitian smoothing
        let uu = u*u*(3. - 2.*u);
        let vv = v*v*(3. - 2.*v);
        let ww = w*w*(3. - 2.*w);

        let mut accum = 0.;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;

                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);

                    accum += (fi*uu + (1.-fi)*(1.-uu))*
                             (fj*vv + (1.-fj)*(1.-vv))*
                             (fk*ww + (1.-fk)*(1.-ww))* c[i][j][k].dot(&weight_v);
                }
            }
        }

        accum
    }

    pub fn turb(&self, point: &Vec3, depth: usize) -> f64 {
        let mut accum = 0.;
        let mut temp_p = *point;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        accum.abs()
    }

    pub fn turb_default(&self, point: &Vec3) -> f64 {
        self.turb(point, 7)
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as isize;
        let j = point.y().floor() as isize;
        let k = point.z().floor() as isize;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[
                        self.perm_x[((i + di) & 255) as usize] ^ self.perm_y[((j + dj) & 255) as usize] ^ self.perm_z[((k + dk) & 255) as usize]
                    ]
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }
}