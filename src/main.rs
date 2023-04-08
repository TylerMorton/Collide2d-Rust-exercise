use nannou::color::{Alpha, IntoLinSrgba};
use nannou::draw::properties::ColorScalar;
use nannou::event::ElementState;
use nannou::prelude::*;

use std::collections::HashMap;

use collision2d::core;
use collision2d::core::{are_colliding, Person};

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct Model {
    crowd: Vec<Person>,
    affairs: Vec<(u32, u32)>,
}

fn model(_app: &App) -> Model {
    let crowd = vec![
        Person::new(50.0, [200.0, 0.0], [1.0, 0.5]),
        Person::new(50.0, [150.0, 0.0], [-1.0, -1.0]),
        // Person::new(50.0, [-150.0, 100.0], [-1.0, -1.0]),
        // Person::new(20.0, [-300.0, 100.0], [-1.0, -1.0]),
        // Person::new(40.0, [-150.0, 300.0], [1.0, -1.5]),
        // Person::new(30.0, [150.0, 150.0], [1.0, -1.5]),
    ];
    let affairs: Vec<(u32, u32)> = vec![];
    Model { crowd, affairs }
}

use nannou::winit::event::DeviceEvent::Button;

fn event(_app: &App, _model: &mut Model, _event: Event) {
    match _event {
        Event::WindowEvent { .. } => (),
        Event::DeviceEvent(
            _,
            Button {
                button: 0,
                state: ElementState::Pressed,
            },
        ) => {
            let p = Person::new(20.0, [_app.mouse.x, _app.mouse.y], [1.0, -1.3]);
            for i in _model.crowd.iter() {
                if p.peer_collision(&i) {
                    return;
                }
            }
            _model.crowd.push(p)
        }
        Event::DeviceEvent { .. } => (),
        _ => {

            let mut new_affairs:  Vec<(u32, u32)> = Vec::new();
            let mut collisions: HashMap<u32, usize> = HashMap::new();
            for (idx, i) in _model.crowd.iter().enumerate() {
                for j in _model.crowd.iter() {
                    if &j == &i {
                        continue;
                    }
                    if i.peer_collision(&j) {
                        if !_model.affairs.contains(&(std::cmp::min(i.id(), j.id()), std::cmp::max(i.id(), j.id()))) { 
                            collisions.entry(i.id()).or_insert(idx);
                        }
                        new_affairs.push((std::cmp::min(i.id(), j.id()), std::cmp::max(i.id(), j.id())))
                    }
                }
            }
            let r: Vec<usize> = collisions.into_values().collect();
            let boundary = _app.window_rect();
            let mut crowd: Vec<&mut Person> = Vec::new();

            for (idx, val) in _model.crowd.iter_mut().enumerate() {
                if r.contains(&idx) {
                    crowd.push(val);
                }
            }

            // updates velocities after collision
            core::collide(&mut crowd);

            // Boundary bounce
            for x in _model.crowd.iter_mut() {
                if x.position()[0] - x.radius() < boundary.left()
                    || x.position()[0] + x.radius() > boundary.right()
                {
                    x.change_velocity([x.velocity()[0] * -1.0, x.velocity()[1]]);
                }
                if x.position()[1] - x.radius() < boundary.bottom()
                    || x.position()[1] + x.radius() > boundary.top()
                {
                    x.change_velocity([x.velocity()[0], x.velocity()[1] * -1.0]);
                }
                let mut new_position = x.position();
                let v = x.velocity();
                for i in 0..2 {
                    new_position[i] += v[i];
                }
                x.change_position(new_position);
            }
            _model.affairs = new_affairs;
        }
    }
}

fn draw_person<C>(draw: &Draw, color: C, p: &Person)
where
    C: IntoLinSrgba<ColorScalar>,
{
    draw.ellipse()
        .color(color)
        .w(2.0 * p.radius())
        .h(2.0 * p.radius())
        .x_y(p.x(), p.y());
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    draw.background().color(REBECCAPURPLE);

    let _model = &model.crowd;

    for i in _model {
        draw_person(
            &draw,
            Alpha {
                color: ORANGERED,
                alpha: 1.0,
            },
            i,
        );
    }

    draw.to_frame(app, &frame).unwrap();
}
