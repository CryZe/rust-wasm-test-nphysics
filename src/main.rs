#[macro_use]
extern crate webplatform;
extern crate nphysics2d as nphys;
extern crate ncollide;
extern crate libc;

use nphys::world::World;
use nphys::math::Vector as Vec2;
use nphys::object::{RigidBody, RigidBodyHandle};
use ncollide::shape::{Plane, Ball};
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;
use webplatform::{HtmlNode, Document};

mod interval;

fn add_ball(body: &HtmlNode,
            balls: &mut Vec<RigidBodyHandle<f32>>,
            world: &mut World<f32>,
            x: f32,
            y: f32) {
    let ball = Ball::new(20.0);
    let mut rigid_body = RigidBody::new_dynamic(ball, 1.0, 0.3, 0.5);
    rigid_body.append_translation(&Vec2::new(x, y));

    let handle = world.add_rigid_body(rigid_body);
    let id = balls.len();
    balls.push(handle);

    body.html_append(&format!(r#"<div id="ball{:05}" class="ball" style="visibility: hidden;" />"#,
                              id));
}

fn create_world() -> World<f32> {
    let mut world = World::new();
    world.set_gravity(Vec2::new(0.0, -98.1));

    let plane = Plane::new(Vec2::new(0.0, 1.0));
    world.add_rigid_body(RigidBody::new_static(plane, 0.3, 0.6));

    let plane = Plane::new(Vec2::new(1.0, 0.0));
    world.add_rigid_body(RigidBody::new_static(plane, 0.3, 0.6));

    let plane = Plane::new(Vec2::new(-1.0, 0.0));
    let mut plane = RigidBody::new_static(plane, 0.3, 0.6);
    plane.append_translation(&Vec2::new(300.0, 0.0));
    world.add_rigid_body(plane);

    world
}

fn run(document: &Document, world: &mut World<f32>, balls: &[RigidBodyHandle<f32>]) {
    world.step(1.0 / 30.0);

    for (id, ball) in balls.iter().enumerate() {
        let node = document.element_query(&format!("#ball{:05}", id)).unwrap();
        let pos = ball.borrow().position().translation;
        node.set_style(&format!("left: {}px; top: {}px;", pos.x - 20.0, 600.0 - pos.y - 20.0));
    }
}

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    let button = document.element_query("button").unwrap();

    let mut balls = Vec::new();

    let mut world = create_world();

    add_ball(&body, &mut balls, &mut world, 101.0, 350.0);
    add_ball(&body, &mut balls, &mut world, 102.0, 500.0);
    add_ball(&body, &mut balls, &mut world, 100.0, 550.0);

    let balls = Rc::new(RefCell::new(balls));
    let world = Rc::new(RefCell::new(world));

    {
        let balls = balls.clone();
        let world = world.clone();
        interval::create(Duration::from_millis(1000 / 30), move || {
            run(&document, &mut world.borrow_mut(), &balls.borrow());
        });
    }

    button.on("click", move |_| {
        add_ball(&body,
                 &mut balls.borrow_mut(),
                 &mut world.borrow_mut(),
                 150.0,
                 550.0);
    });

    webplatform::spin();
}
