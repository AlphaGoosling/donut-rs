use std::{process::Command, sync::LazyLock, thread::sleep, time::{Duration, Instant}};

const BUFFER_SIZE: usize = 3885;
const R1: f64 = 2.0; // Minor radius of the torus
const R2: f64 = 3.0; // Major radius
const K1: f64 = 120.0; // zoom in and out (zoom amount)
const K2: f64 = 40.0; // distance of camera away from object (and origin)
const ROT_X: f64 = 0.0; // Rotation speed of donut about the x-axis in degrees per second
const ROT_Y: f64 = 2.0; // Rotation speed of donut about the y-axis in degrees per second
const ONE_COMPLETE_REVOLUTION: f64 = 2.0 * std::f64::consts::PI;
const LIGHT_DIRECTION: (f64,f64,f64) = (0.0, 10.0, 10.0);
const TIME_DELTA: f64 = 20.0; // in milliseconds. Corresponds to 50 fps

fn main() {
    assert!(BUFFER_SIZE % 2 == 1);
    assert!(R2 > R1);
    let mut theta1: f64 = 0.0;
    let mut theta2: f64 = 0.0;
    let mut phi1: f64 = 0.0;
    let mut phi2: f64 = 0.0;
    let mut depth_buffer = [0.0; BUFFER_SIZE];  // 35 x 111
    let mut image_buffer = [' '; BUFFER_SIZE];
    let mut instant = Instant::now();

    loop { 
        phi1 += ROT_X * TIME_DELTA / 1000.0;
        phi2 += ROT_Y * TIME_DELTA / 1000.0; 
        while theta2 < ONE_COMPLETE_REVOLUTION {
            while theta1 < ONE_COMPLETE_REVOLUTION {
                // Precomputing sines and cosine 
                let a = R1 * theta1.sin();
                let b = theta1.cos();
                let c = theta2.sin();
                let d = theta2.cos();
                let e = R2 + R1 * b;
                let f = phi1.cos();
                let g = phi2.cos();

                // x,y,z coordinates of point on torus's surface in 3D space
                let (x, y, z) = ( e * d * g, e * c * f, a * f * g );
                
                // check if point is behind the camera and skip further operations if so
                if (-z + K2) > 0.0 { 
                    let inverse_depth = 1.0 / (-z + K2);
                    // screen pixel coordinates of the torus surface point
                    let (x_s, y_s) = ((x * (2.1*K1) * inverse_depth).floor() as i32, (y * K1 * inverse_depth).floor() as i32);

                    // skip further operations if point outside the margins of the view port
                    let buffer_index = x_s + y_s * 111 + ((BUFFER_SIZE - 1)/2) as i32;
                    if buffer_index >= 0 && buffer_index < BUFFER_SIZE as i32 {
                        // surface normal vector at point (x,y,z)
                        let (n_x, n_y, n_z) = (-R1 * e * b * d * g * f * f,
                                                              -R1 * e * b * c * g * g * f,
                                                              a * e * g * f);
                        let norm_mag = (n_x.powi(2) + n_y.powi(2) + n_z.powi(2)).sqrt();
                        let light_direction_mag = (LIGHT_DIRECTION.0.powi(2) + LIGHT_DIRECTION.1.powi(2) + LIGHT_DIRECTION.2.powi(2)).sqrt();
                        // dot product of normalized light direction and normalized surface normal
                        let brightness_value =   LIGHT_DIRECTION.0 * n_x / (norm_mag*light_direction_mag) 
                                                    + LIGHT_DIRECTION.1 * n_y / (norm_mag*light_direction_mag) 
                                                    + LIGHT_DIRECTION.2 * n_z / (norm_mag*light_direction_mag);

                        if inverse_depth > depth_buffer[buffer_index as usize] {
                            depth_buffer[buffer_index as usize] = inverse_depth;
                            image_buffer[buffer_index as usize] = shade_pixel(brightness_value);
                        } 
                    }
                }
                theta1 += 0.01;   
            }
            theta2 += 0.01;
            theta1 = 0.0;
        }
        theta2 = 0.0;
        //sleep(Duration::from_millis(1000));
        Command::new("clear").status().unwrap(); // clearing the terminal before drawing the next frame
        //Draw image
        for i in 0..BUFFER_SIZE {
            if i % 111 == 0 {
                print!("\n{}", image_buffer[i])
            } else {
                print!("{}", image_buffer[i])
            }
        }
        println!(" ");
        println!("{:?}", Instant::now() - instant);
        instant = Instant::now();
        // clearing the buffers for the next frame
        depth_buffer = [0.0; BUFFER_SIZE];
        image_buffer = [' '; BUFFER_SIZE];
    }
}

fn shade_pixel(brightness: f64) -> char {
    const LM: &str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
    const LUMINOUSITY_RANGES: [f64; 92] = [0.0, 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.185, 0.2183, 0.2417, 0.2571, 0.2852, 0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667, 0.3737, 0.3747, 0.3838, 0.3921, 0.396, 0.3984, 0.3993, 0.4075, 0.4091, 0.4101, 0.42, 0.423, 0.4247, 0.4274, 0.4293, 0.4328, 0.4382, 0.4385, 0.442, 0.4473, 0.4477, 0.4503, 0.4562, 0.458, 0.461, 0.4638, 0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944, 0.4953, 0.4992, 0.5509, 0.5567, 0.5569, 0.5591, 0.5602, 0.5602, 0.565, 0.5776, 0.5777, 0.5818, 0.587, 0.5972, 0.5999, 0.6043, 0.6049, 0.6093, 0.6099, 0.6465, 0.6561, 0.6595, 0.6631, 0.6714, 0.6759, 0.6809, 0.6816, 0.6925, 0.7039, 0.7086, 0.7235, 0.7302, 0.7332, 0.7602, 0.7834, 0.8037, 0.9999];
    static LM_VEC: LazyLock<Vec<char>> = std::sync::LazyLock::new(||{LM.chars().collect()});
    assert!(LM.len() == LUMINOUSITY_RANGES.len());

    if brightness <= LUMINOUSITY_RANGES[0] { 
        return LM_VEC[0]
    } else if brightness >= LUMINOUSITY_RANGES[LUMINOUSITY_RANGES.len()-1] {
        return LM_VEC[LM_VEC.len()-1];
    } else {
        let i = LUMINOUSITY_RANGES.iter().enumerate().find(|&x| *(x.1) > brightness).unwrap().0;
        return LM_VEC[i];
    }
}