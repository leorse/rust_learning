extern crate image as im;
extern crate piston_window;

#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

trait OpVector {
    fn times(k: f32, v: &Vector) -> Vector;

    fn times2(&self, k: f32) -> Vector;

    fn minus(v1: Vector, v2: &Vector) -> Vector;

    fn plus(v1: Vector, v2: Vector) -> Vector;

    fn dot(v1: Vector, v2: Vector) -> f32;

    fn mag(v: &Vector) -> f32;

    fn norm(v: Vector) -> Vector;

    fn cross(v1: &Vector, v2: &Vector) -> Vector;
}
impl OpVector for Vector {
    fn times(k: f32, v: &Vector) -> Vector {
        Vector {
            x: k * v.x,
            y: k * v.y,
            z: k * v.z,
        }
    }

    fn times2(&self, k: f32) -> Vector {
        Vector {
            x: k * self.x,
            y: k * self.y,
            z: k * self.z,
        }
    }

    fn minus(v1: Vector, v2: &Vector) -> Vector {
        Vector {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
            z: v1.z - v2.z,
        }
    }

    fn plus(v1: Vector, v2: Vector) -> Vector {
        Vector {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
        }
    }

    fn dot(v1: Vector, v2: Vector) -> f32 {
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }

    fn mag(v: &Vector) -> f32 {
        return ((v.x * v.x + v.y * v.y + v.z * v.z).sqrt()) as f32;
    }

    fn norm(v: Vector) -> Vector {
        let mag: f32 = Vector::mag(&v);
        let div: f32 = {
            if mag == 0.0 {
                f32::INFINITY
            } else {
                1.0 / mag
            }
        };
        return Vector::times(div, &v);
    }

    fn cross(v1: &Vector, v2: &Vector) -> Vector {
        return Vector {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        };
    }
}
struct Camera {
    forward: Vector,
    right: Vector,
    up: Vector,
    pos: Vector,
}

impl Camera {
    fn constructor(pos: Vector, lookAt: Vector) -> Camera {
        let forward: Vector;
        let right: Vector;
        let up: Vector;
        let down: Vector = Vector {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };
        forward = Vector::norm(Vector::minus(lookAt, &pos));
        right = Vector::times(1.5, &Vector::norm(Vector::cross(&forward, &down)));
        up = Vector::times(1.5, &Vector::norm(Vector::cross(&forward, &right)));

        Camera {
            forward: forward,
            right: right,
            up: up,
            pos: pos,
        }
    }
}

struct Ray {
    start: Vector,
    dir: Vector,
}
struct Intersection {
    thing: Thing,
    ray: Ray,
    dist: f32,
}

struct Surface {
    roughness: f32,
}

struct Thing {
    surface: Surface,
}

struct Light {
    pos: Vector,
    color: Color,
}

struct Color {
    //constructor(public r: number, public g: number, public b: number) { }
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    const white: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    const grey: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };
    const black: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    const background: Color = Color::black;
    const defaultColor: Color = Color::black;

    fn scale(k: f32, v: Color) -> Color {
        return Color {
            r: k * v.r,
            g: k * v.g,
            b: k * v.b,
        };
    }

    fn plus(v1: Color, v2: Color) -> Color {
        return Color {
            r: v1.r + v2.r,
            g: v1.g + v2.g,
            b: v1.b + v2.b,
        };
    }

    fn times(v1: Color, v2: Color) -> Color {
        return Color {
            r: v1.r * v2.r,
            g: v1.g * v2.g,
            b: v1.b * v2.b,
        };
    }
}

struct Scene {
    things: Vec<Thing>,
    lights: Vec<Light>,
    camera: Camera,
}

const ROUGE: [u8; 4] = [255, 0, 0, 255];
use piston_window::*;

fn main() {
    let v: Vector = Vector {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    println!("{:?}", v.times2(3.0));
    println!("{:?}", Vector::mag(&v));
    let camera = Camera::constructor(
        Vector {
            x: 3.0,
            y: 2.0,
            z: 4.0,
        },
        Vector {
            x: -1.0,
            y: 0.5,
            z: 0.0,
        },
    );
    println!("{:?},{:?},{:?}", camera.forward, camera.right, camera.up);
    let opengl = OpenGL::V3_2;
    let (width, height) = (300, 300);
    let mut window: PistonWindow = WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    let mut canvas = im::ImageBuffer::new(width, height);
    canvas.put_pixel(15, 15, im::Rgba([0, 0, 0, 255]));

    for i in 0..35 {
        for y in 0..35 {
            canvas.put_pixel(30 + i, 30 + y, im::Rgba(ROUGE));
        }
    }

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }
}
