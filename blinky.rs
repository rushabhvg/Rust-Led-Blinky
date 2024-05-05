//  #include <nuttx/config.h>
//  #include <sys/ioctl.h>
//  #include <stdio.h>
//  #include <fcntl.h>

// For file handling mainly for userleds
use std::fs::File;
use std::fs::OpenOptions;       // For  openinh files

// Rust's equivalent of C's stdio
use std::io::{self, prelude::*, ErrorKind};

use std::thread;
use std::time::Duration;


// I counld'nt find this library in NuttX repo or Apps repo
//  #include <nuttx/leds/userled.h>

//  int main(int argc, FAR char *argv[]) {
// this return type is the Rust standard for declaring void fn
fn main() -> io::Result<()> 
{

//    printf("Hello, World!!\n");
    println!("Hello, World!!");


//    // Open the LED Driver
//    printf("Opening /dev/userleds\n");
    println!("Opening /dev/userleds");


//    int fd = open("/dev/userleds", O_WRONLY);
//    if (fd < 0)
//      {
//        int errcode = errno;
//        printf("ERROR: Failed to open /dev/userleds: %d\n",
//               errcode);
//        return EXIT_FAILURE;
//      }
    let mut file = OpenOptions::new().write(true).open("/dev/userleds")?;
 

//    // Turn on LED
//    puts("Set LED 0 to 1");
    println!("Set LED 0 to 1");


    // This ULEDIOC_SETALL is not used, instead I have used write_all
//    int ret = ioctl(fd, ULEDIOC_SETALL, 1);
//    if (ret < 0)
//      {
//        int errcode = errno;
//        printf("ERROR: ioctl(ULEDIOC_SUPPORTED) failed: %d\n",
//                errcode);
//        return EXIT_FAILURE;
//      }
    let ret = file.write_all(b"1")?;

    // if error handling required, following can be used, after removing the "?" from the above line:
    /*

    match ret {
        Err(why) => panic!("ERROR: write_all(b\"1\") failed: {:?}", why.kind()),
        Ok(_) => (),

     */
 

//    // Sleep a while
//    puts("Waiting...");
    println!("Waiting...");


//    usleep(500 * 1000L);
    thread::sleep(Duration::from_micros(500*1000)); // 500000 microseconds = 500 milliseconds
    /*
    TIME CHART:
    1 second = 1000 milliseconds
    1 millisecond = 1000 microseconds
     */
    /*
    Can also use from_milli function?
    thread::sleep(Duration::from_millis(500));
     */
 

//    // Turn off LED
//    puts("Set LED 0 to 0");
    println!("Set LED 0 to 0");
    

//    ret = ioctl(fd, ULEDIOC_SETALL, 0);
//    if (ret < 0)
//      {
//        int errcode = errno;
//        printf("ERROR: ioctl(ULEDIOC_SUPPORTED) failed: %d\n",
//                errcode);
//        return EXIT_FAILURE;
//      }
    ret = file.write_all(b"0")?;
 

//    // Close the LED Driver
//    close(fd);
    drop(file);
 
 
//    return 0;
    Ok(())
}