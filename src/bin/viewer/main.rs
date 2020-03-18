use std::process::{Command, Stdio};
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::io::{Write, Read};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "ScreenShare-viewer", about="Recieves Screenshare Stream")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long, default_value="0.0.0.0:9090")]
    connect: String,
    
    #[structopt(long, default_value = "60")]
    fps: u32,
    
    #[structopt(long, short)]
    record: Option<String>
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let server = TcpListener::bind(&*opt.connect).unwrap();
    
    if let Some(Ok(mut stream)) = server.incoming().next() {
        let (w, h) = {
            let mut buf = [0; 32];
            stream.read(&mut buf[..])?;
            let s = String::from_utf8_lossy(&buf);
            let mut slices = s.split("|");
            (
                &slices.next()
                    .expect("Expected Width Param")
                    .parse::<usize>()
                    .expect("Expected Int"),
                
                &slices.next()
                    .expect("Expected Height Param")
                    .parse::<usize>()
                    .expect("Expected Int")
            )
        };


        Command::new("ffplay")
            .args(&[
                "-f", "rawvideo",
                "-pixel_format", "bgr0",
                "-video_size", &format!("{}x{}", w, h),
                "-framerate", &format!("{}", opt.fps),
                "-"
            ])
            .stdin(unsafe {
                Stdio::from_raw_fd(stream.as_raw_fd())      
            })
            .spawn()
            .expect("Requires ffplay.")
            .wait()?;
        
        Ok(())
    }
    else {
        Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Failed to accept connection connect"))
    }
}
