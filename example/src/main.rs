extern crate gl;
extern crate glutin;

#[macro_use]
extern crate vector;

extern crate geometry;
extern crate material;
extern crate shader;

extern crate camera_components;
extern crate transform_components;

extern crate scene_graph;
extern crate scene_renderer;
extern crate gl_renderer_plugin;
extern crate mesh_component;
extern crate mesh_gl_renderer;


use geometry::{Geometry, Attribute};
use material::Material;
use shader::Shader;

use camera_components::{Camera3D, Camera3DManager};
use transform_components::Transform3D;

use scene_graph::{Scene, Entity};
use scene_renderer::SceneRenderer;
use gl_renderer_plugin::GLRendererPlugin;
use mesh_component::Mesh;
use mesh_gl_renderer::MeshGLRenderer;


static VS_SRC: &'static str = "
    #version 140

    in vec3 position;
    in vec3 normal;
    in vec2 uv;

    uniform mat4 projection;
    uniform mat4 model_view;

    varying vec3 v_normal;
    varying vec2 v_uv;

    void main() {
        v_normal = normal;
        v_uv = uv;
        gl_Position = projection * model_view * vec4(position, 1.0);
    }
";
static FS_SRC: &'static str = "
    #version 140

    out vec4 out_color;

    varying vec3 v_normal;
    varying vec2 v_uv;

    void main() {
        out_color = vec4(v_normal, 1.0);
    }
";


fn main() {
    let window = glutin::Window::new().unwrap();

    let mut scene = Scene::new();
    let mut scene_renderer = SceneRenderer::new(scene.clone());

    scene_renderer.add_plugin(GLRendererPlugin::new());
    scene_renderer.add_renderer(MeshGLRenderer::new());


    {
        let mut entity = Entity::new();

        let mut transform = Transform3D::new();
        transform.set_position(&[5f32, 5f32, 5f32]);
        transform.look_at(&[0f32, 0f32, 0f32], &[0f32, 0f32, 1f32]);
        entity.add_component(transform);

        let mut camera3d = Camera3D::new();
        camera3d.set_background(&[0.3, 0.3, 0.3, 1.0]);
        camera3d.set_orthographic_mode(false);
        entity.add_component(camera3d);

        scene.add_entity(&mut entity);
    }
    {
        let mut entity = Entity::new();

        entity.add_component(Transform3D::new());

        let mut geometry = Geometry::new();
        geometry.add_attribute(Attribute::new_f32("position", vector![
            1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0,
            1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
            -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
            1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0
        ], 3, false));
        geometry.add_attribute(Attribute::new_f32("normal", vector![
            0.577349, 0.577349, -0.577349, 0.577349, -0.577349, -0.577349, -0.577349, -0.577349, -0.577349, 0.577349, 0.577349, 0.577349, -0.577349, 0.577349, 0.577349, -0.577349, -0.577349, 0.577349, 0.577349, 0.577349, -0.577349, 0.577349, 0.577349, 0.577349, 0.577349, -0.577349, 0.577349, 0.577349, -0.577349, -0.577349, 0.577349, -0.577349, 0.577349, -0.577349, -0.577349, 0.577349, -0.577349, -0.577349, -0.577349, -0.577349, -0.577349, 0.577349, -0.577349, 0.577349, 0.577349, 0.577349, 0.577349, 0.577349, 0.577349, 0.577349, -0.577349, -0.577349, 0.577349, -0.577349, -0.577349, 0.577349, -0.577349, 0.577349, 0.577349, -0.577349, -0.577349, -0.577349, -0.577349, 0.577349, -0.577349, 0.577349, 0.577349, 0.577349, 0.577349, -0.577349, -0.577349, 0.577349, 0.577349, -0.577349, -0.577349, 0.577349, 0.577349, -0.577349, 0.577349, -0.577349, 0.577349, -0.577349, -0.577349, -0.577349, 0.577349, -0.577349, -0.577349, -0.577349, -0.577349, 0.577349, -0.577349, 0.577349, -0.577349, -0.577349, -0.577349, -0.577349, -0.577349, 0.577349, 0.577349, -0.577349, 0.577349, 0.577349, 0.577349, 0.577349, 0.577349, -0.577349, 0.577349, -0.577349
        ], 3, false));
        geometry.add_attribute(Attribute::new_f32("uv", vector![
            0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0
        ], 2, false));
        geometry.set_index(Attribute::new_u32("index", vector![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35
        ], 1, false));

        let mut material = Material::new();
        material.set_shader(Shader::new(VS_SRC, FS_SRC));

        entity.add_component(Mesh::new(geometry, material));

        scene.add_entity(&mut entity);
    }

    scene.init();

    unsafe {
        match window.make_current() {
            Ok(_) => {
                gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            },
            Err(e) => panic!("{:?}", e),
        }
    }

    scene_renderer.init();

    {
        let mesh_gl_renderer = scene_renderer.get_plugin::<GLRendererPlugin>().unwrap();
        let context = mesh_gl_renderer.get_context();
        println!(
            "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
            context.get_major(), context.get_minor(), context.get_glsl_major(), context.get_glsl_minor()
        );
    }

    let mut playing = true;
    while playing {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => {
                    playing = false;
                },
                glutin::Event::Resized(w, h) => {
                    scene.get_component_manager::<Camera3DManager>()
                        .unwrap().get_active_camera()
                            .unwrap().set(w as usize, h as usize);
                    scene_renderer.get_plugin::<GLRendererPlugin>()
                        .unwrap().get_context_mut().set_viewport(0, 0, w as usize, h as usize);
                },
                _ => (),
            }
        }

        scene.update();
        scene_renderer.render();

        match window.swap_buffers() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    scene.clear();
    scene_renderer.clear();
}
