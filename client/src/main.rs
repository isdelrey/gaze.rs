mod gaze;

use std::error::Error;
use std::io::{Read, Write};
use std::boxed::Box;
use gaze::Gaze;

fn main() -> Result<(), Box<dyn Error>>  {

    /* Report start: */
    println!("Started");

    /* Connect: */
    let mut gaze = Gaze::connect().unwrap();

    /* Publish: */
    gaze.publish().unwrap();

    Ok(())
}