#[macro_use]
extern crate webplatform;
extern crate nphysics2d as nphys;
extern crate ncollide;
extern crate libc;
extern crate base64;
extern crate pace_files;
extern crate filling;

use nphys::world::World;
use nphys::math::Vector as Vec2;
use nphys::object::{RigidBody, RigidBodyHandle};
use ncollide::shape::{Plane, Ball};
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;
use webplatform::{HtmlNode, Document};
use pace_files::{Array3D, Vec3};
use filling::{draw, blend};

mod interval;

fn add_ball(node: &HtmlNode,
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

    node.html_append(&format!(r#"<div id="ball{:05}" class="ball" style="visibility: hidden;" />"#,
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
    plane.append_translation(&Vec2::new(600.0, 0.0));
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

fn draw_p3s(balls: &[RigidBodyHandle<f32>]) -> (Vec<u8>, Vec<u8>) {
    let mut voxels = Array3D::zeros((600, 600, 1));

    for ball in balls {
        let pos = ball.borrow().position().translation;
        draw::sphere(&mut voxels.view_mut(),
                     Vec3::new(pos.x, pos.y, 0.0),
                     20.0,
                     1.0,
                     1.0,
                     blend::Operation::Max);
    }

    let mut p3s = Vec::new();
    let mut simgeo = Vec::new();

    pace_files::p3s::write(&mut p3s, &mut simgeo, voxels.view()).unwrap();

    (p3s, simgeo)
}

fn download(document: &Document, name: &str, buf: &[u8]) {
    let element = document.element_create("a").unwrap();
    let data = base64::encode(buf);
    element.prop_set_str("href",
                         &format!("data:application/octet-stream;base64,{}", data));
    element.prop_set_str("download", name);

    element.set_style("display: none;");
    let body = document.element_query("body").unwrap();
    body.append(&element);

    element.click();

    body.remove(&element);
}

fn main() {
    let document = webplatform::init();
    let domain = document.element_query(".domain").unwrap();
    let btn_spawn = document.element_query("#spawn").unwrap();
    let btn_download = document.element_query("#download").unwrap();

    let mut balls = Vec::new();

    let mut world = create_world();

    add_ball(&domain, &mut balls, &mut world, 101.0, 350.0);
    add_ball(&domain, &mut balls, &mut world, 102.0, 500.0);
    add_ball(&domain, &mut balls, &mut world, 100.0, 550.0);

    let balls = Rc::new(RefCell::new(balls));
    let world = Rc::new(RefCell::new(world));
    let document = Rc::new(document);

    {
        let balls = balls.clone();
        let world = world.clone();
        let document = document.clone();

        interval::create(Duration::from_millis(1000 / 30), move || {
            run(&document, &mut world.borrow_mut(), &balls.borrow());
        });
    }

    {
        let balls = balls.clone();
        btn_spawn.on("click", move |_| {
            add_ball(&domain,
                     &mut balls.borrow_mut(),
                     &mut world.borrow_mut(),
                     300.0,
                     550.0);
        });
    }

    btn_download.on("click", move |_| {
        let (p3s, simgeo) = draw_p3s(&balls.borrow());
        download(&document, "balls.p3simgeo", &simgeo);
        download(&document, "balls.phi_alpa.p3s", &p3s);
    });

    webplatform::spin();
}
