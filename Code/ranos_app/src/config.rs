use std::ops::Deref;
use structopt::StructOpt;

#[derive(StructOpt)]
/// Renders some animations through a give LED drawer
pub struct AppOpt {
    #[structopt(short, long, default_value = "64")]
    /// The number of LEDs to draw to. Defaults to 64.
    size: usize,

    #[structopt(short, long)]
    /// The upper limit of the rate of updates to the LED array (e.g. 60,
    /// 29.97).
    ///
    /// If the parameter is omitted, then there will be no upper limit
    /// to the speed and will simply run as fast as the system can support.
    rate: Option<f64>,

    #[structopt(short, long)]
    /// Sets the brightness to use for the LEDs. NOTE: For APA102C and related
    /// LEDs, the minimum possible brightness is 1/31, or approximately 0.0325.
    brightness: f64,

    #[structopt(name = "loop", short, long)]
    /// Enables looping of the animations. To exit the program while looping is
    /// enabled, send the program the `SIGTERM` signal (Ctrl + C in the
    /// terminal) and it will exit at the end of the current loop.
    is_looping: bool,

    #[structopt(subcommand)]
    /// The drawer to use. Possible values are NullDraw, PiDraw, & TermDraw.
    draw: DrawOpt,
}

pub struct DrawOpt {
    draw: DrawVariants,

    anims: 
}

#[derive(StructOpt)]
pub enum DrawVariants {
    NullDraw(NullDrawOpt),
    PiDraw(PiDrawOpt),
    TermDraw(TermDrawOpt),
}

#[derive(StructOpt)]
pub struct NullDrawOpt {
}

#[derive(StructOpt)]
pub struct PiDrawOpt {
}

#[derive(StructOpt)]
pub struct TermDrawOpt {
}

pub struct AnimationOpt {
    
}

#[derive(StructOpt)]
pub enum AnimationVariants {
    Breath,
    Cycle,
    Rainbow,
    Strobe,
}
