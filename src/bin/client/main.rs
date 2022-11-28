use scrap::{Capturer, Display};
use std::io::Write;
use std::io::ErrorKind::WouldBlock;
use std::net::TcpStream;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ScreenShare-sender", about = "Sends Screenshare Stream")]
struct Opt {

    #[structopt(short, long, default_value="0.0.0.0:9090")]
    connect: String,

    #[structopt(long, default_value = "60")]
    fps: u32,

}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let one_second = std::time::Duration::new(1, 0);
    let one_frame = one_second / opt.fps;

    let addr = opt.connect;
    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

    let mut capturer = Capturer::new(d).unwrap();
    loop {
        if let Ok(mut out) = TcpStream::connect(&*addr) {
            out.write_all(format!("{}|{}|", w, h).as_bytes())?;
            loop {
                match capturer.frame() {
                    Ok(frame) => {
                        // Write the frame, removing end-of-row padding.
                        let stride = frame.len() / h;
                        let rowlen = 4 * w;
                        for row in frame.chunks(stride) {
                            let row = &row[..rowlen];
                            out.write_all(row)?;
                        }
                    }
                    Err(ref e) if e.kind() == WouldBlock => {
                        std::thread::sleep(one_frame);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        else {
            println!("Waiting to connect...");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
