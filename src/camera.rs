//! Module for representing a camera in the context of a raytracer.

use crate::color::Color;
use crate::hittable::hittables::HittableList;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval, EMPTY, UNIVERSE};
use crate::ray::Ray;
use crate::vector3d::{Point3D, Vector3D};
use std::io::Write;

/// Represents a camera in a 3D scene.
pub struct Camera {
    aspect_ratio: f64,
    image_width: u16,
    image_height: u16,
    center: Point3D,
    pixel00_loc: Point3D,
    pixel_Δu: Vector3D,
    pixel_Δv: Vector3D,
}

impl Camera {
    /// Creates a new `Camera` with default parameters and initializes its settings.
    ///
    /// The `default` method initializes a `Camera` instance with default settings,
    /// including an aspect ratio of 1.0, an image width of 100 pixels, and all other
    /// parameters set to their default values. It then calls the `initialize` method
    /// to set up the camera for subsequent rendering operations.
    ///
    /// The default camera is ready to be further configured or used for rendering.
    ///
    /// # Returns
    ///
    /// Returns a new `Camera` instance with default parameters and initialized settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_project::camera::Camera;
    ///
    /// // Create a default camera with initialized settings.
    /// let camera = Camera::default();
    /// ```
    pub fn default() -> Self {
        Self::new(1.0, 100)
    }

    /// Creates a new `Camera` with the specified aspect ratio and image width,
    /// and initializes its settings for rendering.
    ///
    /// The `new` method initializes a `Camera` with the provided aspect ratio and image width.
    /// The aspect ratio determines the width-to-height ratio of the resulting image, while
    /// the image width sets the number of pixels along the horizontal axis. After creating
    /// the camera, it calls the `initialize` method to set up the camera for subsequent
    /// rendering operations.
    ///
    /// # Arguments
    ///
    /// * `aspect_ratio` - The aspect ratio of the camera, defining the width-to-height ratio of the image.
    /// * `image_width` - The width of the image in pixels.
    ///
    /// # Returns
    ///
    /// Returns a new `Camera` instance with the specified parameters and initialized settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_project::camera::Camera;
    ///
    /// // Create a camera with a 16:9 aspect ratio and 800 pixels width, with initialized settings.
    /// let camera = Camera::new(16.0 / 9.0, 800);
    /// ```
    pub fn new(aspect_ratio: f64, image_width: u16) -> Self {
        let mut cam: Camera = Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: 0,
            center: Point3D::new(),
            pixel00_loc: Point3D::new(),
            pixel_Δu: Vector3D::new(),
            pixel_Δv: Vector3D::new(),
        };
        cam.initialize();
        cam
    }

    /// Initializes the camera settings based on the aspect ratio and image width.
    fn initialize(&mut self) {
        self.image_height = (f64::from(self.image_width) / self.aspect_ratio) as u16;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Point3D::new();

        // Determine viewport dimensions
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * f64::from(self.image_width) / f64::from(self.image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vector3D = Vector3D::with_values(viewport_width, 0.0, 0.0);
        let viewport_v: Vector3D = Vector3D::with_values(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_Δu = viewport_u / f64::from(self.image_width);
        self.pixel_Δv = viewport_v / f64::from(self.image_height);

        // Calculate the location of the upper left pixel
        let viewport_upper_left: Point3D = self.center
            - Vector3D::with_values(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_Δu + self.pixel_Δv);
    }

    /// Computes the color of a ray using the provided hit record and world geometry.
    fn ray_color(r: &Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal() + Color::with_values(1.0, 1.0, 1.0));
        }

        let unit_direction: &Vector3D = &r.direction().unit_vector();
        let a: f64 = 0.5 * unit_direction.y() + 1.0;
        (1.0 - a) * Color::with_values(1.0, 1.0, 1.0) + a * Color::with_values(0.5, 0.7, 1.0)
    }

    /// Renders the scene using the camera and provided world geometry.
    pub fn render(&mut self, world: &HittableList) {
        let mut buffer = Vec::new();

        // Render
        write!(
            buffer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .expect("Failed to write header");

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            eprint!("\x1B[2J\x1B[1;1H"); // Clear output

            for i in 0..self.image_width {
                let _pixel_center: Point3D = self.pixel00_loc
                    + (f64::from(i) * self.pixel_Δu)
                    + (f64::from(j) * self.pixel_Δv);
                let _ray_direction: Vector3D = _pixel_center - self.center;
                let _r: Ray = Ray::create(self.center, _ray_direction);

                Self::ray_color(&_r, &world)
                    .write(&mut buffer)
                    .expect("Failed to write color");
            }
        }
        println!("{}", String::from_utf8_lossy(&buffer));
        eprintln!("Done.");
    }
}
