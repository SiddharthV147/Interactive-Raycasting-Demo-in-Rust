extern crate sdl2;
use std::f64::consts::PI;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

const MAX_RAY_LENGTH: f64 = 1000.0;

struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}

struct Ray {
    x_start: i32,
    y_start: i32,
    angle: i32,
}

struct Coord {
    x: i32,
    y: i32,
}

impl Circle {
    fn contains_point(&self, px: i32, py: i32) -> bool {
        let dx = px - self.x;
        let dy = py - self.y;
        dx*dx + dy*dy <= self.radius * self.radius
    }
}

fn draw_light_circle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, circle: &Circle, object: &Circle) -> Result<(), String> {

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    for x in (circle.x - circle.radius)..=(circle.x + circle.radius) {
        for y in (circle.y - circle.radius)..=(circle.y + circle.radius) {
           
            let dx = x - circle.x;
            let dy = y - circle.y;
    
            if dx*dx + dy*dy == circle.radius * circle.radius {
                canvas.draw_point(Point::new(x, y))?;
                draw_ligth_rays(canvas, circle,x, y, object);
            }
        }
    }
    Ok(())
}

fn draw_shadow_circle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, circle: &Circle) -> Result<(), String> {

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    for x in (circle.x - circle.radius)..=(circle.x + circle.radius) {
        for y in (circle.y - circle.radius)..=(circle.y + circle.radius) {
            let dx = x - circle.x;
            let dy = y - circle.y;
            if dx*dx + dy*dy <= circle.radius * circle.radius {
                canvas.draw_point(Point::new(x, y))?;
            }
        }
    }
    Ok(())
}

fn detect_collision(ptx: f64, pty: f64, obj: &Circle) -> bool {
    // println!("{}", obj.radius);
    let dx = ptx as i32 - obj.x;
    let dy = pty as i32 - obj.y;
    let dist = dx*dx + dy*dy;
    dist <= obj.radius*obj.radius 
}

fn draw_ligth_rays(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, circle:& Circle, x: i32, y: i32, object: &Circle) -> Result<(), String> {
    
    let num_rays = 360;
    let radius = MAX_RAY_LENGTH;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    for i in 0..num_rays {
        // let mut points: Vec<Coord> = Vec::new();
        let angle = 2.0 * PI * (i as f64) / (num_rays as f64);
        // println!("{}", angle);
        let dirx = angle.cos();
        let diry = angle.sin();
        let mut px = circle.x as f64;
        let mut py = circle.y as f64;    
        
        while px>=0.0 && px < MAX_RAY_LENGTH && py >= 0.0 && py < MAX_RAY_LENGTH {
            let dx = px as i32 - circle.x;
            let dy = py as i32 - circle.y;
            let dist = dx*dx + dy*dy;

            if dist <= circle.radius*circle.radius {
                canvas.set_draw_color(Color::RGB(255, 255, 0));
            }
            
            else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            
            if detect_collision(px, py, object) {
                break;
            }
            
            canvas.draw_point(Point::new(px as i32, py as i32))?;
            px += dirx;
            py += diry;
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Ray Tracing", 1000, 1200)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut x = 0;
    let mut y = 400;

    let mut circle = Circle {
        x: 100,
        y: 100,
        radius: 50,
    };

    let mut shadow_circle = Circle {
        x: 200,
        y: 200,
        radius: 100,
    };

    // let mut shadow_circle2 = Circle {
    //     x: 400,
    //     y: 400,
    //     radius: 100,
    // };
    //

    let mut dragging = false;
    let mut dragging_shadow = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                    Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    }

                Event::MouseButtonDown { x:mouse_x, y: mouse_y, mouse_btn: MouseButton::Left,..} => {
                    if circle.contains_point(mouse_x, mouse_y) {
                        dragging = true;
                    }
                    if shadow_circle.contains_point(mouse_x, mouse_y) {
                        dragging_shadow = true;
                    }
                }
                Event::MouseButtonUp {mouse_btn: MouseButton::Left, ..} => {
                    dragging = false;
                    dragging_shadow = false;
                }
                Event::MouseMotion { x, y, ..} => {
                    if dragging {
                        circle.x = x;
                        circle.y = y;
                    }
                    if dragging_shadow {
                        shadow_circle.x = x;
                        shadow_circle.y = y;
                    }
                }
                _ => {}
            }
        }
         
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        
        // draw_light_circle(&mut canvas, &shadow_circle2);
        draw_light_circle(&mut canvas, &circle, & shadow_circle);
        draw_shadow_circle(&mut canvas, &shadow_circle);
        
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
