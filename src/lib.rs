#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe {
            let setup = crate::pffft_new_setup(1024, crate::pffft_direction_t_PFFFT_FORWARD);
            let input: [f32; 1024] = [0.0f32; 1024];
            let mut output: [f32; 1024] = [0.0f32; 1024];
            let mut work: [f32; 1024] = [0.0f32; 1024];
            crate::pffft_transform(
                setup,
                &input[0],
                &mut output[0],
                &mut work[0],
                crate::pffft_direction_t_PFFFT_FORWARD,
            )
        }
    }
}
