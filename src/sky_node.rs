use bevy::prelude::*;

use crate::Sky;
use bevy::core::{AsBytes, Bytes};
use bevy::reflect::TypeUuid;
use bevy::render::mesh::{INDEX_BUFFER_ASSET_INDEX, VERTEX_ATTRIBUTE_BUFFER_ID};
use bevy::render::pass::{
    LoadOp, Operations, PassDescriptor, RenderPassColorAttachmentDescriptor, TextureAttachment,
};
use bevy::render::pipeline::{
    BindGroupDescriptor, BindType, BindingDescriptor, BindingShaderStage, BlendDescriptor,
    BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite, CompareFunction, CullMode,
    DepthStencilStateDescriptor, FrontFace, IndexFormat, InputStepMode, PipelineDescriptor,
    PipelineLayout, PolygonMode, PrimitiveTopology, PushConstantRange,
    RasterizationStateDescriptor, RenderPipeline, StencilStateDescriptor,
    StencilStateFaceDescriptor, UniformProperty, VertexAttributeDescriptor, VertexBufferDescriptor,
    VertexFormat,
};
use bevy::render::render_graph::{
    base, Node, RenderGraph, RenderResourcesNode, ResourceSlotInfo, ResourceSlots, SlotLabel,
    WindowSwapChainNode, WindowTextureNode,
};
use bevy::render::renderer::{
    BindGroup, BufferInfo, BufferUsage, RenderContext, RenderResource, RenderResourceBindings,
    RenderResourceContext, RenderResourceId, RenderResourceIterator, RenderResourceType,
    RenderResources,
};
use bevy::render::shader::{ShaderStage, ShaderStages};
use bevy::render::texture::TextureFormat;

pub const SKY_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x5188c0693b407dd8);
pub const SKY_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 0x5188c0693b407dd9);

pub mod node {
    pub const SKY_PASS: &str = "sky_node";
}

#[derive(Debug)]
struct SkyNode {
    camera_name: String,
    inputs: [ResourceSlotInfo; 1],
}

impl SkyNode {
    const COLOR_ATTACHMENT_SLOT: SlotLabel = SlotLabel::Index(0);
    pub fn new(camera_name: String) -> SkyNode {
        let slot_info = ResourceSlotInfo::new("color_attachment", RenderResourceType::Texture);
        SkyNode {
            camera_name,
            inputs: [slot_info],
        }
    }
}

impl Node for SkyNode {
    fn input(&self) -> &[ResourceSlotInfo] {
        &self.inputs
    }
    fn update(
        &mut self,
        _world: &World,
        resources: &Resources,
        render_context: &mut dyn RenderContext,
        input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        let render_resource_bindings = resources.get::<RenderResourceBindings>().unwrap();
        let mesh_handle: Handle<Mesh> = SKY_MESH_HANDLE.typed();
        let index_buffer_id = render_context
            .resources()
            .get_asset_resource(&mesh_handle, INDEX_BUFFER_ASSET_INDEX)
            .unwrap()
            .get_buffer()
            .unwrap();
        let vertex_buffer_id = render_context
            .resources()
            .get_asset_resource(&mesh_handle, VERTEX_ATTRIBUTE_BUFFER_ID)
            .unwrap()
            .get_buffer()
            .unwrap();
        let pipelines = resources.get::<Assets<PipelineDescriptor>>().unwrap();
        let pipeline_handle: Handle<PipelineDescriptor> = SKY_PIPELINE_HANDLE.typed();
        let pipeline_descriptor: &PipelineDescriptor = pipelines.get(pipeline_handle).unwrap();
        let layout = pipeline_descriptor.get_layout().unwrap();
        let camera_bind_group_descriptor = layout.get_bind_group(0).unwrap();
        let sky = resources.get::<Sky>().unwrap();

        let camera_binding =
            if let Some(camera_binding) = render_resource_bindings.get(self.camera_name.as_str()) {
                camera_binding.clone()
            } else {
                println!("Can't find camera binding!!!");
                return;
            };

        let camera_bind_group_id = if render_context
            .resources()
            .bind_group_descriptor_exists(camera_bind_group_descriptor.id)
        {
            let camera_bind_group = BindGroup::build().add_binding(0, camera_binding).finish();
            render_context
                .resources()
                .create_bind_group(camera_bind_group_descriptor.id, &camera_bind_group);
            camera_bind_group.id
        } else {
            println!("Camera bind group descriptor does not exist!");
            return;
        };
        let color_attachment_texture_id = input
            .get(Self::COLOR_ATTACHMENT_SLOT)
            .expect("SkyNode requires color attachment input!")
            .get_texture()
            .expect("SkyNode requires the input to be a texture!");


        let mut buf: [u8; 128] = [0; 128];
        sky.write_bytes(&mut buf);
        render_context.begin_pass(
            &PassDescriptor {
                color_attachments: vec![RenderPassColorAttachmentDescriptor {
                    attachment: TextureAttachment::Id(color_attachment_texture_id),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::AZURE),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
                sample_count: 1,
            },
            &render_resource_bindings,
            &mut |render_pass| {
                render_pass.set_pipeline(&SKY_PIPELINE_HANDLE.typed());
                render_pass.set_bind_group(
                    0,
                    camera_bind_group_descriptor.id,
                    camera_bind_group_id,
                    None,
                );
                render_pass.set_vertex_buffer(0, vertex_buffer_id, 0);
                render_pass.set_index_buffer(index_buffer_id, 0, IndexFormat::Uint16);
                render_pass.set_push_constants(BindingShaderStage::FRAGMENT, 0, &buf);
                render_pass.draw_indexed(0..14, 0, 0..1);
            },
        )
    }
}

pub(crate) fn setup(
    _commands: &mut Commands,
    _msaa: Res<Msaa>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    sky: Res<Sky>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
) {
    let indices: [u16; 14] = [3, 7, 1, 5, 4, 7, 6, 3, 2, 1, 0, 4, 2, 6];
    let vertices: [[f32; 3]; 8] = [
        [-100., -100., -100.], // 0
        [-100., -100., 100.0], // 1
        [-100., 100.0, -100.], // 2
        [-100., 100.0, 100.0], // 3
        [100.0, -100., -100.], // 4
        [100.0, -100., 100.0], // 5
        [100.0, 100.0, -10.],  // 6
        [100.0, 100.0, 100.0], // 7
    ];
    let mesh_handle: Handle<Mesh> = SKY_MESH_HANDLE.typed();
    render_resource_context.set_asset_resource(
        &mesh_handle,
        RenderResourceId::Buffer(render_resource_context.create_buffer_with_data(
            BufferInfo {
                buffer_usage: BufferUsage::INDEX,
                ..Default::default()
            },
            &indices.as_bytes(),
        )),
        INDEX_BUFFER_ASSET_INDEX,
    );
    render_resource_context.set_asset_resource(
        &mesh_handle,
        RenderResourceId::Buffer(render_resource_context.create_buffer_with_data(
            BufferInfo {
                buffer_usage: BufferUsage::VERTEX,
                ..Default::default()
            },
            &vertices.as_bytes(),
        )),
        VERTEX_ATTRIBUTE_BUFFER_ID,
    );

    pipelines.set_untracked(
        SKY_PIPELINE_HANDLE,
        PipelineDescriptor {
            name: Some("SkyPipeline".into()),
            layout: Some(PipelineLayout {
                bind_groups: vec![BindGroupDescriptor::new(
                    0,
                    vec![BindingDescriptor {
                        name: "Camera".to_string(),
                        index: 0,
                        bind_type: BindType::Uniform {
                            has_dynamic_offset: false,
                            property: UniformProperty::Struct(vec![
                                UniformProperty::Mat4,
                                UniformProperty::Mat4,
                            ]),
                        },
                        shader_stage: BindingShaderStage::VERTEX | BindingShaderStage::FRAGMENT,
                    }],
                )],
                vertex_buffer_descriptors: vec![VertexBufferDescriptor {
                    name: "SkyboxCube".into(),
                    stride: std::mem::size_of::<[f32; 3]>() as u64,
                    step_mode: InputStepMode::Vertex,
                    attributes: vec![VertexAttributeDescriptor {
                        name: "Vertex_Position".into(),
                        offset: 0,
                        format: VertexFormat::Float3,
                        shader_location: 0,
                    }],
                }],
                push_constant_ranges: vec![PushConstantRange {
                    stages: BindingShaderStage::FRAGMENT,
                    range: 0..sky.byte_len() as u32,
                }],
            }),
            shader_stages: ShaderStages {
                vertex: shaders.add(Shader::from_glsl(
                    ShaderStage::Vertex,
                    include_str!("procsky.vert"),
                )),
                fragment: Some(shaders.add(Shader::from_glsl(
                    ShaderStage::Fragment,
                    include_str!("procsky.frag"),
                ))),
            },
            rasterization_state: Some(RasterizationStateDescriptor {
                polygon_mode: PolygonMode::Fill,
                front_face: FrontFace::Cw,
                cull_mode: CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            primitive_topology: PrimitiveTopology::TriangleStrip,
            color_states: vec![ColorStateDescriptor {
                format: TextureFormat::default(),
                color_blend: BlendDescriptor {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha_blend: BlendDescriptor {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                write_mask: ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: Some(IndexFormat::Uint16),
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        },
    );
    let pipeline_handle: Handle<PipelineDescriptor> = SKY_PIPELINE_HANDLE.typed();
    let pipeline = pipelines.get(pipeline_handle.clone()).unwrap();
    render_resource_context.create_render_pipeline(pipeline_handle, pipeline, &*shaders);

    render_graph.add_node(node::SKY_PASS, SkyNode::new("Camera3d".into()));
    render_graph
        .add_node_edge(node::SKY_PASS, base::node::MAIN_PASS)
        .unwrap();

    render_graph
        .add_slot_edge(
            base::node::PRIMARY_SWAP_CHAIN,
            WindowSwapChainNode::OUT_TEXTURE,
            node::SKY_PASS,
            SkyNode::COLOR_ATTACHMENT_SLOT,
        )
        .unwrap();
}
