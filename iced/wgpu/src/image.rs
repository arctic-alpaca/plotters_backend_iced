mod atlas;

#[cfg(feature = "image")]
mod raster;

#[cfg(feature = "svg")]
mod vector;

use crate::Transformation;
use atlas::Atlas;

use iced_graphics::layer;
use iced_native::Rectangle;
use std::cell::RefCell;
use std::mem;
use zerocopy::AsBytes;

#[cfg(feature = "image")]
use iced_native::image;

#[cfg(feature = "svg")]
use iced_native::svg;

#[derive(Debug)]
pub struct Pipeline {
    #[cfg(feature = "image")]
    raster_cache: RefCell<raster::Cache>,
    #[cfg(feature = "svg")]
    vector_cache: RefCell<vector::Cache>,

    pipeline: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    instances: wgpu::Buffer,
    constants: wgpu::BindGroup,
    texture: wgpu::BindGroup,
    texture_version: usize,
    texture_layout: wgpu::BindGroupLayout,
    texture_atlas: Atlas,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        use wgpu::util::DeviceExt;

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let constant_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("iced_wgpu::image constants layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer {
                            dynamic: false,
                            min_binding_size: wgpu::BufferSize::new(
                                mem::size_of::<Uniforms>() as u64,
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                        count: None,
                    },
                ],
            });

        let uniforms_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("iced_wgpu::image uniforms buffer"),
            size: mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let constant_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("iced_wgpu::image constants bind group"),
                layout: &constant_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            uniforms_buffer.slice(..),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

        let texture_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("iced_wgpu::image texture atlas layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Float,
                        multisampled: false,
                    },
                    count: None,
                }],
            });

        let layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("iced_wgpu::image pipeline layout"),
                push_constant_ranges: &[],
                bind_group_layouts: &[&constant_layout, &texture_layout],
            });

        let vs_module = device.create_shader_module(wgpu::include_spirv!(
            "shader/image.vert.spv"
        ));

        let fs_module = device.create_shader_module(wgpu::include_spirv!(
            "shader/image.frag.spv"
        ));

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("iced_wgpu::image pipeline"),
                layout: Some(&layout),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: wgpu::CullMode::None,
                    ..Default::default()
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor {
                    format,
                    color_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[
                        wgpu::VertexBufferDescriptor {
                            stride: mem::size_of::<Vertex>() as u64,
                            step_mode: wgpu::InputStepMode::Vertex,
                            attributes: &[wgpu::VertexAttributeDescriptor {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float2,
                                offset: 0,
                            }],
                        },
                        wgpu::VertexBufferDescriptor {
                            stride: mem::size_of::<Instance>() as u64,
                            step_mode: wgpu::InputStepMode::Instance,
                            attributes: &[
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 0,
                                },
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 2,
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 4 * 2,
                                },
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 3,
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 4 * 4,
                                },
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 4,
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 4 * 6,
                                },
                                wgpu::VertexAttributeDescriptor {
                                    shader_location: 5,
                                    format: wgpu::VertexFormat::Uint,
                                    offset: 4 * 8,
                                },
                            ],
                        },
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let vertices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("iced_wgpu::image vertex buffer"),
                contents: QUAD_VERTS.as_bytes(),
                usage: wgpu::BufferUsage::VERTEX,
            });

        let indices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("iced_wgpu::image index buffer"),
                contents: QUAD_INDICES.as_bytes(),
                usage: wgpu::BufferUsage::INDEX,
            });

        let instances = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("iced_wgpu::image instance buffer"),
            size: mem::size_of::<Instance>() as u64 * Instance::MAX as u64,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_atlas = Atlas::new(device);

        let texture = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("iced_wgpu::image texture atlas bind group"),
            layout: &texture_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(
                    &texture_atlas.view(),
                ),
            }],
        });

        Pipeline {
            #[cfg(feature = "image")]
            raster_cache: RefCell::new(raster::Cache::new()),

            #[cfg(feature = "svg")]
            vector_cache: RefCell::new(vector::Cache::new()),

            pipeline,
            uniforms: uniforms_buffer,
            vertices,
            indices,
            instances,
            constants: constant_bind_group,
            texture,
            texture_version: texture_atlas.layer_count(),
            texture_layout,
            texture_atlas,
        }
    }

    #[cfg(feature = "image")]
    pub fn dimensions(&self, handle: &image::Handle) -> (u32, u32) {
        let mut cache = self.raster_cache.borrow_mut();
        let memory = cache.load(&handle);

        memory.dimensions()
    }

    #[cfg(feature = "svg")]
    pub fn viewport_dimensions(&self, handle: &svg::Handle) -> (u32, u32) {
        let mut cache = self.vector_cache.borrow_mut();
        let svg = cache.load(&handle);

        svg.viewport_dimensions()
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        images: &[layer::Image],
        transformation: Transformation,
        bounds: Rectangle<u32>,
        target: &wgpu::TextureView,
        _scale: f32,
    ) {
        let instances: &mut Vec<Instance> = &mut Vec::new();

        #[cfg(feature = "image")]
        let mut raster_cache = self.raster_cache.borrow_mut();

        #[cfg(feature = "svg")]
        let mut vector_cache = self.vector_cache.borrow_mut();

        for image in images {
            match &image {
                #[cfg(feature = "image")]
                layer::Image::Raster { handle, bounds } => {
                    if let Some(atlas_entry) = raster_cache.upload(
                        handle,
                        device,
                        encoder,
                        &mut self.texture_atlas,
                    ) {
                        add_instances(
                            [bounds.x, bounds.y],
                            [bounds.width, bounds.height],
                            atlas_entry,
                            instances,
                        );
                    }
                }
                #[cfg(not(feature = "image"))]
                layer::Image::Raster { .. } => {}

                #[cfg(feature = "svg")]
                layer::Image::Vector { handle, bounds } => {
                    let size = [bounds.width, bounds.height];

                    if let Some(atlas_entry) = vector_cache.upload(
                        handle,
                        size,
                        _scale,
                        device,
                        encoder,
                        &mut self.texture_atlas,
                    ) {
                        add_instances(
                            [bounds.x, bounds.y],
                            size,
                            atlas_entry,
                            instances,
                        );
                    }
                }
                #[cfg(not(feature = "svg"))]
                layer::Image::Vector { .. } => {}
            }
        }

        if instances.is_empty() {
            return;
        }

        let texture_version = self.texture_atlas.layer_count();

        if self.texture_version != texture_version {
            log::info!("Atlas has grown. Recreating bind group...");

            self.texture =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("iced_wgpu::image texture atlas bind group"),
                    layout: &self.texture_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &self.texture_atlas.view(),
                        ),
                    }],
                });

            self.texture_version = texture_version;
        }

        {
            let mut uniforms_buffer = staging_belt.write_buffer(
                encoder,
                &self.uniforms,
                0,
                wgpu::BufferSize::new(mem::size_of::<Uniforms>() as u64)
                    .unwrap(),
                device,
            );

            uniforms_buffer.copy_from_slice(
                Uniforms {
                    transform: transformation.into(),
                }
                .as_bytes(),
            );
        }

        let mut i = 0;
        let total = instances.len();

        while i < total {
            let end = (i + Instance::MAX).min(total);
            let amount = end - i;

            let mut instances_buffer = staging_belt.write_buffer(
                encoder,
                &self.instances,
                0,
                wgpu::BufferSize::new(
                    (amount * std::mem::size_of::<Instance>()) as u64,
                )
                .unwrap(),
                device,
            );

            instances_buffer
                .copy_from_slice(instances[i..i + amount].as_bytes());

            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: target,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            },
                        },
                    ],
                    depth_stencil_attachment: None,
                });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.constants, &[]);
            render_pass.set_bind_group(1, &self.texture, &[]);
            render_pass.set_index_buffer(self.indices.slice(..));
            render_pass.set_vertex_buffer(0, self.vertices.slice(..));
            render_pass.set_vertex_buffer(1, self.instances.slice(..));

            render_pass.set_scissor_rect(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            );

            render_pass.draw_indexed(
                0..QUAD_INDICES.len() as u32,
                0,
                0..amount as u32,
            );

            i += Instance::MAX;
        }
    }

    pub fn trim_cache(&mut self) {
        #[cfg(feature = "image")]
        self.raster_cache.borrow_mut().trim(&mut self.texture_atlas);

        #[cfg(feature = "svg")]
        self.vector_cache.borrow_mut().trim(&mut self.texture_atlas);
    }
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes)]
pub struct Vertex {
    _position: [f32; 2],
}

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        _position: [0.0, 0.0],
    },
    Vertex {
        _position: [1.0, 0.0],
    },
    Vertex {
        _position: [1.0, 1.0],
    },
    Vertex {
        _position: [0.0, 1.0],
    },
];

#[repr(C)]
#[derive(Debug, Clone, Copy, AsBytes)]
struct Instance {
    _position: [f32; 2],
    _size: [f32; 2],
    _position_in_atlas: [f32; 2],
    _size_in_atlas: [f32; 2],
    _layer: u32,
}

impl Instance {
    pub const MAX: usize = 1_000;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, AsBytes)]
struct Uniforms {
    transform: [f32; 16],
}

fn add_instances(
    image_position: [f32; 2],
    image_size: [f32; 2],
    entry: &atlas::Entry,
    instances: &mut Vec<Instance>,
) {
    match entry {
        atlas::Entry::Contiguous(allocation) => {
            add_instance(image_position, image_size, allocation, instances);
        }
        atlas::Entry::Fragmented { fragments, size } => {
            let scaling_x = image_size[0] / size.0 as f32;
            let scaling_y = image_size[1] / size.1 as f32;

            for fragment in fragments {
                let allocation = &fragment.allocation;

                let [x, y] = image_position;
                let (fragment_x, fragment_y) = fragment.position;
                let (fragment_width, fragment_height) = allocation.size();

                let position = [
                    x + fragment_x as f32 * scaling_x,
                    y + fragment_y as f32 * scaling_y,
                ];

                let size = [
                    fragment_width as f32 * scaling_x,
                    fragment_height as f32 * scaling_y,
                ];

                add_instance(position, size, allocation, instances);
            }
        }
    }
}

#[inline]
fn add_instance(
    position: [f32; 2],
    size: [f32; 2],
    allocation: &atlas::Allocation,
    instances: &mut Vec<Instance>,
) {
    let (x, y) = allocation.position();
    let (width, height) = allocation.size();
    let layer = allocation.layer();

    let instance = Instance {
        _position: position,
        _size: size,
        _position_in_atlas: [
            (x as f32 + 0.5) / atlas::SIZE as f32,
            (y as f32 + 0.5) / atlas::SIZE as f32,
        ],
        _size_in_atlas: [
            (width as f32 - 1.0) / atlas::SIZE as f32,
            (height as f32 - 1.0) / atlas::SIZE as f32,
        ],
        _layer: layer as u32,
    };

    instances.push(instance);
}
