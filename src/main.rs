extern crate bufstream;

use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;

use bufstream::BufStream;



// The target host
const HOST: &'static str = "127.0.0.1:8080";

// The default size of the command output read buffer
const CMD_READ_BUFFER_SIZE: usize = 32;



fn main() {
    // Start
    println!("Starting...");

    // let range = Range::new(1, 1000);
    // let mut rng = rand::thread_rng();

    // Create a new pixelflut canvas
    let canvas = PixCanvas::new(HOST, (1000, 1000), 10);

    loop {}
}

/// Create a stream to talk to the pixelflut server.
///
/// The stream is returned as result.
// TODO: Specify a host here!
fn create_stream() -> Result<TcpStream, Error> {
    TcpStream::connect(HOST)
}



/// A pixflut instance 
struct PixCanvas {
    host: &'static str,
    size: (u16, u16),
    painters: Vec<JoinHandle<u32>>,
}

impl PixCanvas {
    /// Create a new pixelflut canvas.
    pub fn new(host: &'static str, size: (u16, u16), painter_count: usize) -> PixCanvas {
        // Initialize the object
        let mut canvas = PixCanvas {
            host,
            size,
            painters: Vec::with_capacity(painter_count),
        };

        // Spawn some painters
        canvas.spawn_painters();

        // Return the canvas
        canvas
    }

    /// Spawn the painters for this canvas
    fn spawn_painters(&mut self) {
        // Spawn some painters
        for i in 0..10 {
            self.spawn_painter(Rect::from(i * 100, 0, 100, 1000));
        }
    }

    /// Spawn a single painter in a thread.
    fn spawn_painter(&mut self, area: Rect) {
        let thread = thread::spawn(move || {
            // Create a new stream
            let stream = create_stream()
                .expect("failed to open stream to pixelflut");

            // Create a new client
            let client = PixClient::new(stream);

            // Create a painter
            let mut painter = Painter::new(client, area);

            // Do some work
            loop {
                painter.work();
            }
        });

        // Add the painter thread to the list
        self.painters.push(thread);
    }
}



struct Painter {
    client: PixClient,
    area: Rect,
}

impl Painter {
    /// Create a new painter.
    pub fn new(client: PixClient, area: Rect) -> Painter {
        Painter {
            client,
            area,
        }
    }

    /// Perform work.
    /// Paint the whole defined area.
    pub fn work(&mut self) {
        // Define the color to draw with
        let color = Color::from(0, 155, 0);

        // Loop through all the pixels, and set their color
        for x in self.area.x..self.area.x + self.area.w {
            for y in self.area.y..self.area.y + self.area.h {
                self.client.write_pixel(x, y, &color);
            }
        }
    }
}



/// A pixelflut client.
/// This client uses a stream to talk to a pixelflut panel.
/// It allows to write pixels to the panel, and read some status.
struct PixClient {
    stream: BufStream<TcpStream>,
}

impl PixClient {
    /// Create a new client instance.
    pub fn new(stream: TcpStream) -> PixClient {
        PixClient {
            stream: BufStream::new(stream),
        }
    }

    /// Write a pixel to the given stream.
    fn write_pixel(&mut self, x: u16, y: u16, color: &Color) {
        // Write the command to set a pixel
        self.write_command(
            format!("PX {} {} {}", x, y, color.as_hex()),
        )
    }

    // /// Read the size of the screen.
    // fn read_screen_size(&mut self) {
    //     // Read the screen size
    //     let size = self
    //         .write_read_command("SIZE".into())
    //         .expect("Failed to read screen size");

    //     // TODO: Remove this after debugging
    //     println!("Read size: {}", size);
    // }

    /// Write the given command to the given stream.
    fn write_command(&mut self, cmd: String) {
        self.stream.write(cmd.as_bytes());
        self.stream.write("\n".as_bytes());
    }

    // /// Write the given command to the given stream, and read the output.
    // fn write_read_command(&mut self, cmd: String) -> Result<String, Error> {
    //     // Write the command
    //     self.write_command(cmd);

    //     // Read the output
    //     let mut buffer = String::with_capacity(CMD_READ_BUFFER_SIZE);
    //     println!("Reading line...");
    //     self.stream.read_line(&mut buffer)?;
    //     println!("Done reading");

    //     // Return the read string
    //     Ok(buffer)
    // }
}



/// Color structure.
#[derive(Copy, Clone)]
struct Color {
    r: u16,
    g: u16,
    b: u16,
}

impl Color {
    /// Create a new color instance
    pub fn from(r: u16, g: u16, b: u16) -> Color {
        Color {
            r,
            g,
            b,
        }
    }

    /// Get a hexadecimal representation of the color,
    /// such as `FFFFFF` for white and `FF0000` for red.
    pub fn as_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}



/// Rectangle struct.
pub struct Rect {
    // TODO: Make these properties private
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn from(x: u16, y: u16, w: u16, h: u16) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
        }
    }
}
