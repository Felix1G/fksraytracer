mod fksray;

use fksray::util::{Color, Vector3, Vector2, Camera, World};
use image::RgbImage;

fn main() {
    let mut world = World::new();
    let id = world.create_texture("test.jpg");
    world.create_plane(
        Vector3::new(-100.0, -70.0, -100.0),
        Vector3::new(100.0, -70.0, -100.0),
        Vector3::new(-100.0, -70.0, 200.0),
        Vector3::new(100.0, -70.0, 200.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Color::new_string("#CFAB23FF"),
        Color::new_string("#FF0000"),
        Color::new_string("#00FF00"),
        Color::new_string("#0000FFFF"),
        [0.0, 0.0, 0.0, 0.0],
        id
    );
    world.create_plane(
        Vector3::new(-100.0, 70.0, -100.0),
        Vector3::new(100.0, 70.0, -100.0),
        Vector3::new(-100.0, 70.0, 200.0),
        Vector3::new(100.0, 70.0, 200.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Color::new_string("#CFAB23FF"),
        Color::new_string("#FF0000"),
        Color::new_string("#00FF00"),
        Color::new_string("#0000FFFF"),
        [0.0, 0.0, 0.0, 0.0],
        id
    );
    world.create_plane(
        Vector3::new(-70.0, -100.0, -100.0),
        Vector3::new(-70.0, 100.0, -100.0),
        Vector3::new(-70.0, -100.0, 200.0),
        Vector3::new(-70.0, 100.0, 200.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Color::new_string("#485783"),
        Color::new_string("#102939"),
        Color::new_string("#FFFFFF"),
        Color::new_string("#DFFFFD"),
        [0.0, 0.0, 0.0, 0.0],
        id
    );
    world.create_plane(
        Vector3::new(70.0, -100.0, -100.0),
        Vector3::new(70.0, 100.0, -100.0),
        Vector3::new(70.0, -100.0, 200.0),
        Vector3::new(70.0, 100.0, 200.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Color::new_string("#485783"),
        Color::new_string("#102939"),
        Color::new_string("#FFFFFF"),
        Color::new_string("#DFFFFD"),
        [0.0, 0.0, 0.0, 0.0],
        id
    );
    world.create_plane(
        Vector3::new(-100.0, -100.0, 200.0),
        Vector3::new(100.0, -100.0, 200.0),
        Vector3::new(-100.0, 100.0, 200.0),
        Vector3::new(100.0, 100.0, 200.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Color::new_string("#000000"),
        Color::new_string("#000000"),
        Color::new_string("#000000"),
        Color::new_string("#000000"),
        [0.5, 0.5, 0.5, 0.5],
        -1
    );
    world.create_plane(
        Vector3::new(-100.0, -100.0, -10.0),
        Vector3::new(100.0, -100.0, -10.0),
        Vector3::new(-100.0, 100.0, -10.0),
        Vector3::new(100.0, 100.0, -10.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Color::new_string("#DFABC3"),
        Color::new_string("#000000"),
        Color::new_string("#BB3399"),
        Color::new_string("#293883"),
        [0.0, 0.0, 0.0, 0.0],
        -1
    );
    world.create_plane(
        Vector3::new(-30.0, -80.0, 110.0),
        Vector3::new(30.0, -80.0, 110.0),
        Vector3::new(-30.0, 0.0, 50.0),
        Vector3::new(30.0, 0.0, 50.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Color::new_string("#000000"),
        Color::new_string("#000000"),
        Color::new_string("#000000"),
        Color::new_string("#000000"),
        [1.0, 1.0, 1.0, 1.0],
        -1
    );
    world.create_point_light(
        Vector3::new(0.0, 50.0, 100.0),
        Color::new_string("#FF0000"),
        15000.0
    );
    world.create_point_light(
        Vector3::new(0.0, -40.0, 100.0),
        Color::new_string("#00FF00"),
        15000.0
    );
    world.create_point_light(
        Vector3::new(30.0, 0.0, 0.0),
        Color::new_string("#0000FF"),
        15000.0
    );
    
    world.compute();

    let mut image = image::open("output.png").unwrap().to_rgb8();//RgbImage::new(512, 512);
    
    fksray::raytracer(
        &mut image,
        &Camera {
            pos: Vector3::new(0.0, 60.0, 0.0),
            rot: Vector3::new(0.0, -40.0, 0.0),
            fov: 100.0,
        },
        &mut world
    );

    image.save("output.png").unwrap();
}
