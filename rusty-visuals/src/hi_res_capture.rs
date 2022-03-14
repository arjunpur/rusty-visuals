use nannou::prelude::*;
use super::file_utils;

pub struct HiResCapture {
    // The texture that we will draw to.
    texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    pub draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    renderer: nannou::draw::Renderer,
    // The type used to capture the texture.
    pub texture_capturer: wgpu::TextureCapturer,
    // The type used to resize our texture to the window texture.
    pub texture_reshaper: wgpu::TextureReshaper,
}

impl HiResCapture {

    pub fn new(app: &App, [width, height]: [u32; 2], w_id: WindowId) -> HiResCapture {

        let window = app.window(w_id).unwrap();

        // Retrieve the wgpu device.
        let device = window.device();

        // Create our custom texture.
        let sample_count = window.msaa_samples();
        let texture = wgpu::TextureBuilder::new()
            .size([width, height])
            // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
            // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
            // Use nannou's default multisampling sample count.
            .sample_count(sample_count)
            // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
            .format(wgpu::TextureFormat::Rgba16Float)
            // Build it!
            .build(device);

        // Create our `Draw` instance and a renderer for it.
        let draw = nannou::Draw::new();
        let descriptor = texture.descriptor();
        let renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        // Create the texture capturer.
        let texture_capturer = wgpu::TextureCapturer::default();

        // Create the texture reshaper.
        let texture_view = texture.view().build();
        let texture_sample_type = texture.sample_type();
        let dst_format = Frame::TEXTURE_FORMAT;
        let texture_reshaper = wgpu::TextureReshaper::new(
            device,
            &texture_view,
            sample_count,
            texture_sample_type,
            sample_count,
            dst_format,
        );

        // Create the appropriate output directory if neccessary
        file_utils::create_app_output_dir_all(app);

        HiResCapture {
            texture,
            draw,
            renderer,
            texture_capturer,
            texture_reshaper,
        }
    }

    pub fn update(&mut self, app: &App, save_image: bool) {
        // Render our drawing to the texture.
        let window = app.main_window();
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self 
            .renderer
            .render_to_texture(device, &mut encoder, &self.draw, &self.texture);

        // Take a snapshot of the texture. The capturer will do the following:
        //
        // 1. Resolve the texture to a non-multisampled texture if necessary.
        // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
        // 3. Copy the result to a buffer ready to be mapped for reading.
        let snapshot = self
            .texture_capturer
            .capture(device, &mut encoder, &self.texture);

        // Submit the commands for our drawing and texture capture to the GPU.
        window.queue().submit(Some(encoder.finish()));

        if save_image {
            let path = file_utils::get_timestamp_path_for_output(app);
            println!("Saving image to {}", path.as_path().to_str().unwrap());
            // Submit a function for writing our snapshot to a PNG.
            //
            // NOTE: It is essential that the commands for capturing the snapshot are `submit`ted before we
            // attempt to read the snapshot - otherwise we will read a blank texture!
            snapshot
                .read(move |result| {
                    let image = result.expect("failed to map texture memory").to_owned();
                    image
                        .save(&path)
                        .expect("failed to save texture to png image");
                })
                .unwrap();
        }
    }
}
