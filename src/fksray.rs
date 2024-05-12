pub(crate) mod util;

const INV256: f64 = 1.0 / 256.0;

use image::{Rgb, RgbImage};
use util::{Camera, Color, Vector3, World};

fn triangle_collision(
    origin: &Vector3,
    norm: &Vector3,
    world: &World,
    boxi: usize,
) -> (f64, f64, f64, isize) {
    let bbox = &world.bbox[boxi];
    let left = bbox.left;
    let right = bbox.right;

    let ldata: (f64, f64, f64, isize) = {
        if left == -1 {
            let (t, u, v) = world.triangles[boxi].intersect(&origin, &norm);
            (t, u, v, boxi as isize)
        } else {
            let lbbox = &world.bbox[left as usize];
            if lbbox.intersect(origin, norm) {
                triangle_collision(&origin, &norm, &world, left as usize)
            } else {
                (-1.0f64, 0.0f64, 0.0f64, 0isize)
            }
        }
    };

    let rdata: (f64, f64, f64, isize) = {
        if right == -1 {
            let (t, u, v) = world.triangles[boxi].intersect(&origin, &norm);
            (t, u, v, boxi as isize)
        } else {
            let rbbox = &world.bbox[right as usize];
            if rbbox.intersect(origin, norm) {
                triangle_collision(&origin, &norm, &world, right as usize)
            } else {
                (-1.0f64, 0.0f64, 0.0f64, 0isize)
            }
        }
    };

    if ldata.0 == -1.0 {
        return rdata;
    }

    if rdata.0 == -1.0 {
        return ldata;
    }

    return if ldata.0 < rdata.0 { ldata } else { rdata };
}

fn ray_trace(pos: &Vector3, ray: &Vector3, world: &World, reflect_times: usize) -> Color {
    let data = triangle_collision(pos, ray, world, world.bbox.len() - 1);

    if data.0 != -1.0 {
        let triangle = &world.triangles[data.3 as usize];
        let mut point_pos = triangle.obtain_pos(data.1, data.2);
        
        let mut norm_dot = triangle.norm.dot(&ray);
        let norm = if norm_dot <= 0.0 {
            triangle.norm
        } else {
            norm_dot = triangle.norm_opp.dot(&ray);
            triangle.norm_opp
        };
        
        let norm_rot_x = f64::atan2(norm.x, norm.z);
        let norm_rot_y = f64::asin(norm.y);

        //point light
        let mut lum = Color::new_zero();
        for point in world.point_lights.iter() {

            let mut light_dis = point_pos.sub(&point.pos);
            let dis2 = light_dis.normalize_dis();
            let (tri_dis, _, _, _) =
                triangle_collision(&point.pos, &light_dis, &world, world.bbox.len() - 1);

            //make sure no triangles are blocking
            if !(tri_dis != -1.0 && tri_dis * tri_dis < dis2 - 0.001) {
                let inst = point.lumen / (dis2.abs());
                lum.r += inst * point.color.r;
                lum.g += inst * point.color.g;
                lum.b += inst * point.color.b;
                lum.a += inst * point.color.a;
            }

            //-90 degrees in radians
            let mut rot_x = -1.5708f64;
            let mut rot_y = -1.5708f64;
            if reflect_times < 1 {
                let mut luminosity = Color::new_zero();
                while rot_x <= 1.6 {
                    while rot_y <= 1.6 {
                        let mut direction = Vector3::new(
                            norm_rot_x + rot_x,
                            norm_rot_y + rot_y,
                            0.0
                        );
                        direction.normalize();
                        
                        let reflect_color = ray_trace(&point_pos, &direction,
                                                      world, reflect_times + 1);
                        lum.r = f64::max(lum.r, 0.9 * reflect_color.r);
                        lum.g = f64::max(lum.g, 0.9 * reflect_color.g);
                        lum.b = f64::max(lum.b, 0.9 * reflect_color.b);
                        lum.a = f64::max(lum.a, 0.9 * reflect_color.a);
                        
                        rot_y += 0.25;
                    }
                    rot_x += 0.25;
                }
            }
        }
        
        let mut color = world.obtain_color(data.3 as usize, data.1, data.2);
        color.mul_self(lum.r, lum.g, lum.b, lum.a);
        
        //reflection
        let reflection = triangle.obtain_reflect(data.1, data.2);
        if reflection != 0.0 {
            if reflect_times < 5 {
                let dot_val = 2.0 * norm_dot;
                let reflection_dir = Vector3::new(
                    ray.x - dot_val * norm.x,
                    ray.y - dot_val * norm.y,
                    ray.z - dot_val * norm.z,
                );
                
                let reflect_color = ray_trace(&point_pos, &reflection_dir,
                                              world, reflect_times + 1);
                
                let reflect_opp = 1.0 - reflection;
                color.r = reflect_opp * color.r + reflection * reflect_color.r;
                color.g = reflect_opp * color.g + reflection * reflect_color.g;
                color.b = reflect_opp * color.b + reflection * reflect_color.b;
                color.a = reflect_opp * color.a + reflection * reflect_color.a;
            }
        }

        return color;
    }
    
    return Color::new_zero();
}

pub fn raytracer(image: &mut RgbImage, cam: &Camera, world: &mut World) {
    let mut ray = Vector3::new_zero();
    let mut pos = Vector3::new_zero();
    pos.set(&cam.pos);

    let fov = 0.5 * cam.fov;

    let sinz = cam.rot.z.to_radians().sin();
    let cosz = cam.rot.z.to_radians().cos();

    if !world.validated {
        world.compute();
    }

    if world.triangles.len() == 0 {
        return;
    }

    for x in 400..512i32 {
        for y in 0..512i32 {
            // get ray and pos point in screen
            ray.x = ((x - 256) as f64 - 0.5) * INV256;
            ray.y = ((256 - y) as f64 - 0.5) * INV256;

            // cam.rotate ray to z
            let ray_x = ray.x * cosz - ray.y * sinz;
            let ray_y = ray.x * sinz + ray.y * cosz;
            ray.x = ray_x;
            ray.y = ray_y;

            // fov
            ray.x = ray.x * fov + cam.rot.x;
            ray.y = ray.y * fov + cam.rot.y;

            //draw triangle
            ray.normalize_deg();

            let color = ray_trace(&pos, &ray, &world, 0);
            image.put_pixel(
                x as u32,
                y as u32,
                Rgb([
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                ]),
            );

            println!("Pixel Processed: {x} {y}");
            
            ray.zero();
        }

        if x % 10 == 0 {
            image.save("output.png").unwrap();
        }
    }
}
