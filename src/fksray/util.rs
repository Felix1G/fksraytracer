use std::collections::HashMap;
use image::RgbImage;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

impl Color {
    const COLOR_MAX_INV: f64 = 1.0 / 255.0;
    
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        return Color { r, g, b, a };
    }

    pub fn new_color(other: &Color) -> Color {
        return Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a
        }
    }

    pub fn new_zero() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    }

    pub fn new_string(s: &str) -> Color {
        if s.len() != 7 && s.len() != 9 {
            panic!("Invalid color string");
        }

        let r = Self::COLOR_MAX_INV * (u8::from_str_radix(&s[1..3], 16).unwrap() as f64);
        let g = Self::COLOR_MAX_INV * (u8::from_str_radix(&s[3..5], 16).unwrap() as f64);
        let b = Self::COLOR_MAX_INV * (u8::from_str_radix(&s[5..7], 16).unwrap() as f64);
        
        let mut a = 1.0;
        if s.len() == 9 {
            a = Self::COLOR_MAX_INV * (u8::from_str_radix(&s[7..9], 16).unwrap() as f64);
        }
        
        return Color { r, g, b, a };
    }

    pub fn mul_self(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.r *= r;
        self.g *= g;
        self.b *= b;
        self.a *= a;
    }
}

struct Texture {
    width: u32,
    height: u32,
    img: RgbImage
}

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Vector3 { x, y, z };
    }

    pub fn cpy(vec: &Vector3) -> Self {
        return Self::new(vec.x, vec.y, vec.z);
    }

    pub fn new_zero() -> Self {
        return Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };
    }

    pub fn zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x +
            self.y * other.y +
            self.z * other.z;
    }

    pub fn cross(&self, other: &Self) -> Self {
        return Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize_deg(&mut self) {
        let xr = self.x.to_radians();
        let yr = self.y.to_radians();
        let yrcos = yr.cos();
        
        self.x = xr.sin() * yrcos;
        self.y = yr.sin();
        self.z = xr.cos() * yrcos;
    }

    pub fn normalize(&mut self) {
        let yrcos = self.y.cos();

        self.z = self.x.cos() * yrcos;
        self.x = self.x.sin() * yrcos;
        self.y = self.y.sin();
    }
    
    pub fn normalize_dis(&mut self) -> f64{
        let dis2 = self.distance2();
        let dis_inv = 1.0 / dis2.sqrt();
        
        self.x *= dis_inv;
        self.y *= dis_inv;
        self.z *= dis_inv;

        return dis2;
    }

    pub fn distance2(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn set(&mut self, vec: &Self) {
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;
    }

    pub fn sub(&self, other: &Self) -> Self {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    
    pub fn add(&self, other: &Self) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn inverse(&self) -> Self {
        return Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        return Vector2 { x, y };
    }
}

pub(crate) struct Triangle {
    pub(crate) p1: Vector3,
    pub(crate) p2: Vector3,
    pub(crate) p3: Vector3,
    pub(crate) e1: Vector3,
    pub(crate) e2: Vector3,
    pub(crate) uv1: Vector2,
    pub(crate) uv2: Vector2,
    pub(crate) uv3: Vector2,
    pub(crate) c1: Color,
    pub(crate) c2: Color,
    pub(crate) c3: Color,
    pub(crate) reflect: [f64; 3],
    pub(crate) mid: f64,
    pub(crate) norm: Vector3,
    pub(crate) norm_opp: Vector3,
    pub(crate) tex: isize
}

impl Triangle {
    const EPSILON: f64 = 0.0001;
    
    pub fn max_box(&self) -> Vector3 {
        return Vector3 {
            x: f64::max(self.p1.x, f64::max(self.p2.x, self.p3.x)),
            y: f64::max(self.p1.y, f64::max(self.p2.y, self.p3.y)),
            z: f64::max(self.p1.z, f64::max(self.p2.z, self.p3.z)),
        };
    }
    
    pub fn min_box(&self) -> Vector3 {
        return Vector3 {
            x: f64::min(self.p1.x, f64::min(self.p2.x, self.p3.x)),
            y: f64::min(self.p1.y, f64::min(self.p2.y, self.p3.y)),
            z: f64::min(self.p1.z, f64::min(self.p2.z, self.p3.z)),
        };
    }
    
    //returns t, u, and v
    pub fn intersect(&self, ori: &Vector3, dir: &Vector3) -> (f64, f64, f64) {
        let pvec = dir.cross(&self.e2);
        let det = pvec.dot(&self.e1);

        if det.abs() < Self::EPSILON {
            return (-1.0, 0.0, 0.0);
        }

        let inv_det = 1.0 / det;
        
        let tvec = ori.sub(&self.p1);

        let u = tvec.dot(&pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return (-1.0, 0.0, 0.0);
        }

        let qvec = tvec.cross(&self.e1);
        let v = dir.dot(&qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return (-1.0, 0.0, 0.0);
        }

        let t = self.e2.dot(&qvec) * inv_det;
        return if t < 0.0 || t.abs() < Self::EPSILON {
            (-1.0, 0.0, 0.0)
        } else {
            (t, u, v)
        }
    }

    pub fn obtain_uv(&self, u: f64, v: f64) -> Vector2 {
        let subuv = 1.0 - u - v;
        return Vector2::new(
            self.uv1.x * subuv + self.uv2.x * u + self.uv3.x * v,
            self.uv1.y * subuv + self.uv2.y * u + self.uv3.y * v);
    }

    pub fn obtain_reflect(&self, u: f64, v: f64) -> f64 {
        return
            self.reflect[0] * (1.0 - u - v) +
            self.reflect[1] * u +
            self.reflect[2] * v;
    }

    pub fn obtain_pos(&self, u: f64, v: f64) -> Vector3 {
        let subuv = 1.0 - u - v;
        return Vector3::new(
            self.p1.x * subuv + self.p2.x * u + self.p3.x * v,
            self.p1.y * subuv + self.p2.y * u + self.p3.y * v,
            self.p1.z * subuv + self.p2.z * u + self.p3.z * v);
    }
    
    pub fn obtain_color(&self, u: f64, v: f64) -> Color {
        let subuv = 1.0 - u - v;
        return Color::new(
            self.c1.r * subuv + self.c2.r * u + self.c3.r * v,
            self.c1.g * subuv + self.c2.g * u + self.c3.g * v,
            self.c1.b * subuv + self.c2.b * u + self.c3.b * v,
            self.c1.a * subuv + self.c2.a * u + self.c3.a * v,);
    }
}

// structure:
// when the bounding box is less than the triangles size
// that index becomes the triangle's index
// 
// otherwise, the bounding box should be composed of
// two other bounding boxes
//
// the last bbox contains the root node.
#[derive(Debug)]
pub(crate) struct BoundingBox {
    pub(crate) m1: Vector3,
    pub(crate) m2: Vector3,
    pub(crate) left: isize,
    pub(crate) right: isize
}

impl BoundingBox {
    pub fn intersect(&self, ori: &Vector3, norm: &Vector3) -> bool {
        let norm_inv = norm.inverse();
        
        //slab test
        //x
        let mut t1 = norm_inv.x * (self.m1.x - ori.x);
        let mut t2 = norm_inv.x * (self.m2.x - ori.x);
        let mut tmin = t1.min(t2);
        let mut tmax = t2.max(t1);

        //y
        t1 = norm_inv.y * (self.m1.y - ori.y);
        t2 = norm_inv.y * (self.m2.y - ori.y);
        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t2.max(t1));
        
        //z
        t1 = norm_inv.z * (self.m1.z - ori.z);
        t2 = norm_inv.z * (self.m2.z - ori.z);
        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t2.max(t1));

        return tmin <= tmax;
    }
}

pub(crate) struct PointLight {
    pub(crate) pos: Vector3,
    pub(crate) color: Color,
    pub(crate) lumen: f64
}

pub struct World {
    pub(crate) triangles: Vec<Triangle>,
    pub(crate) bbox: Vec<BoundingBox>,
    textures: HashMap<usize, Texture>,
    texture_count: usize,
    pub(crate) point_lights: Vec<PointLight>,
    pub(crate) validated: bool
}

impl World {
    const INV255: f64 = 1.0 / 255.0;
    
    pub fn new() -> Self {
        return Self {
            triangles: Vec::<Triangle>::new(),
            bbox: Vec::<BoundingBox>::new(),
            textures: HashMap::<usize, Texture>::new(),
            texture_count: 0,
            point_lights: Vec::<PointLight>::new(),
            validated: true
        }
    }

    pub fn create_point_light(&mut self, pos: Vector3, color: Color, lumen: f64) {
        self.point_lights.push(PointLight {
            pos, color, lumen
        });
    }

    pub fn create_texture(&mut self, tex: &str) -> isize {
        let img = image::open(tex).unwrap().to_rgb8();
        let width = img.width();
        let height = img.height();
        
        self.textures.insert(self.texture_count, Texture {
            width, height, img
        });

        let id = self.texture_count;
        self.texture_count += 1;
        return id as isize;
    }

    pub fn remove_texture(&mut self, id: usize) {
        self.textures.remove(&id);
    }
    
    pub fn create_triangle(&mut self,
                           p1: Vector3, p2: Vector3, p3: Vector3,
                           uv1: Vector2, uv2: Vector2, uv3: Vector2,
                           c1: Color, c2: Color, c3: Color,
                           reflect: [f64; 3], tex: isize) {
        let midvec = Vector3::new(
                0.5 * (f64::max(p1.x, f64::max(p2.x, p3.x)) +
                       f64::min(p1.x, f64::min(p2.x, p3.x))),
                0.5 * (f64::max(p1.y, f64::max(p2.y, p3.y)) +
                       f64::min(p1.y, f64::min(p2.y, p3.y))),
                0.5 * (f64::max(p1.z, f64::max(p2.z, p3.z)) +
                       f64::min(p1.z, f64::min(p2.z, p3.z))));
        
        let e1 = p2.sub(&p1);
        let e2 = p3.sub(&p1);
        let mut norm = e1.cross(&e2);
        norm.normalize_dis();
        let norm_opp = Vector3::new(-norm.x, -norm.y, -norm.z);
        
        self.triangles.push(Triangle {
            p1, p2, p3, e1, e2, uv1, uv2, uv3, c1, c2, c3,
            mid: midvec.x + midvec.y + midvec.z,
            norm, norm_opp, reflect, tex
        });
        
        self.validated = false;
    }

    pub fn create_plane(&mut self,
                        p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3,
                        uv1: Vector2, uv2: Vector2, uv3: Vector2, uv4: Vector2,
                        c1: Color, c2: Color, c3: Color, c4: Color, 
                        reflect: [f64; 4], tex: isize) {
        self.create_triangle(p1, p2, p4, uv1, uv2, uv4, c1, c2, c4,
                             [reflect[0], reflect[1], reflect[2]], tex);
        self.create_triangle(p1, p3, p4, uv1, uv3, uv4, c1, c3, c4,
                             [reflect[0], reflect[2], reflect[3]], tex);
    }

    pub fn compute(&mut self) {
        self.triangles.sort_by(|a, b| a.mid.partial_cmp(&b.mid).unwrap());

        self.bbox = Vec::<BoundingBox>::with_capacity(2 * self.triangles.len() - 1);

        //initialise the first boxes
        let mut left = 0;
        for triangle in self.triangles.iter() {
            self.bbox.push(BoundingBox {
                m1: triangle.min_box(),
                m2: triangle.max_box(),
                left: -1,
                right: -1
            })
        }

        //loop until there is only one singular bounding box
        let mut boxes = self.triangles.len() >> 1;
        let mut start = 0;
        while boxes >= 1 {
            let mut i = 0;
            
            while i < boxes {
                //combine bounding box
                let mut j = start + i;

                let lbox = &self.bbox[j];
                let rbox = &self.bbox[j + 1];
                
                self.bbox.push(BoundingBox {
                    m1: Vector3::new(
                        lbox.m1.x.min(rbox.m1.x),
                        lbox.m1.y.min(rbox.m1.y),
                        lbox.m1.z.min(rbox.m1.z)),
                    m2: Vector3::new(
                        lbox.m2.x.max(rbox.m2.x),
                        lbox.m2.y.max(rbox.m2.y),
                        lbox.m2.z.max(rbox.m2.z)),
                    left: j as isize,
                    right: j as isize + 1
                });
                
                i += 2;
            }

            start += i;
            
            boxes = (self.bbox.len() - start) >> 1;
        }

        self.validated = true;
        //println!("{:?}", self.bbox);
    }

    pub fn obtain_color(&self, tri_id: usize,
                        u: f64, v: f64) -> Color {
        if self.point_lights.len() == 0 {
            return Color::new_zero();
        }
        
        let triangle = &self.triangles[tri_id];
        let mut color = triangle.obtain_color(u, v);

        if triangle.tex != -1 {
            let uv = triangle.obtain_uv(u, v);
            let tex = &self.textures[&(triangle.tex as usize)];
            
            let x = uv.x * (tex.width as f64);
            let y = (1.0 - uv.y) * (tex.height as f64);
            
            let pix = tex.img.get_pixel(
                (x as u32) % tex.width,
                (y as u32) % tex.height);
            
            color.mul_self(
                pix.0[0] as f64 * Self::INV255,
                pix.0[1] as f64 * Self::INV255,
                pix.0[2] as f64 * Self::INV255,
                1.0);
        }

        return color;
    }
}

pub struct Camera {
    pub pos: Vector3,
    pub rot: Vector3,
    pub fov: f64
}