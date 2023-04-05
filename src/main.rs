use nannou::color::{Alpha, IntoLinSrgba};
use nannou::draw::properties::ColorScalar;
use nannou::event::ElementState;
use nannou::prelude::*;

use collision2d::core;
use collision2d::core::{are_colliding, Person};

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct Model {
    crowd: Vec<Person>,
    affairs: Vec<usize>,
}

fn model(_app: &App) -> Model {
    let crowd = vec![
        Person::new(50.0, [200.0, 0.0], [5.0, 2.0]),
        Person::new(50.0, [-100.0, 0.0], [-5.0, -1.0]),
        Person::new(50.0, [-150.0, 100.0], [-10.0, -7.5]),
        Person::new(20.0, [-300.0, 100.0], [-10.0, -7.5]),
        Person::new(40.0, [-150.0, 300.0], [10.0, -7.5]),
        Person::new(100.0, [150.0, 150.0], [3.0, -7.5]),
    ];
    let affairs: Vec<usize> = vec![];
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
            let p = Person::new(20.0, [_app.mouse.x, _app.mouse.y], [5.0, -2.3]);
            for i in _model.crowd.iter() {
                if p.peer_collision(&i) {
                    return;
                }
            }
            _model.crowd.push(p)
        }
        Event::DeviceEvent { .. } => (),
        _ => {
            let r = core::naive_have_collided(&_model.crowd);
            let mut prev_crowd = Vec::new();
            for (idx, val) in _model.crowd.iter_mut().enumerate() {
                if _model.affairs.contains(&idx) {
                    prev_crowd.push(val);
                }
            }
            let prev_collided = are_colliding(&mut prev_crowd);
            let boundary = _app.window_rect();
            let colliding_pairs = core::are_colliding_pairs(&mut prev_crowd);
            let mut crowd: Vec<&mut Person> = Vec::new();
            for (idx, val) in _model.crowd.iter_mut().enumerate() {
                if !prev_collided.contains(&idx) && r.contains(&idx) {
                    crowd.push(val);
                } else {
                    if colliding_pairs
                        .iter()
                        .any(|e| e[0] == val.id() || e[1] == val.id())
                    {
                        crowd.push(val);
                    }
                }
            }
            core::collide(&mut crowd);
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
            _model.affairs = r;
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

    draw.background().color(PLUM);

    let _model = &model.crowd;

    for i in _model {
        draw_person(
            &draw,
            Alpha {
                color: STEELBLUE,
                alpha: 0.5,
            },
            i,
        );
    }

    draw.to_frame(app, &frame).unwrap();
}
