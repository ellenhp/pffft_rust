#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct PffftSetup {
    width: usize,
    work: Vec<f32>,
    work_buf_offset: usize,
    input: Vec<f32>,
    input_buf_offset: usize,
    output: Vec<f32>,
    output_buf_offset: usize,
    setup: *mut PFFFT_Setup,
}

fn vec_and_alignment_offset<T: Clone>(val: T, size: usize) -> (Vec<T>, usize) {
    let mut work_buf = Vec::<T>::new();
    // PFFFT requires buffers to be 16-byte aligned.
    work_buf.resize(size + 3, val);
    let work_buf_ptr_alignment = work_buf.as_ptr() as usize % 4;
    return (work_buf, (4 - work_buf_ptr_alignment) % 4);
}

impl PffftSetup {
    pub fn new(width: usize, forward: bool, backward: bool) -> PffftSetup {
        unsafe {
            let (work_buf, work_buf_offset) = vec_and_alignment_offset(0.0f32, width);
            let (input_buf, input_buf_offset) = vec_and_alignment_offset(0.0f32, width);
            let (output_buf, output_buf_offset) = vec_and_alignment_offset(0.0f32, width);

            PffftSetup {
                width: width,
                work: work_buf,
                work_buf_offset: work_buf_offset,
                input: input_buf,
                input_buf_offset: input_buf_offset,
                output: output_buf,
                output_buf_offset: output_buf_offset,
                setup: if forward {
                    crate::pffft_new_setup(width as i32, crate::pffft_transform_t_PFFFT_REAL)
                } else {
                    0usize as *mut PFFFT_Setup
                },
            }
        }
    }

    pub fn forward(&mut self, input: &[f32], output: &mut [f32]) -> bool {
        if self.setup as usize == 0usize {
            return false;
        }
        if output.len() != self.width {
            return false;
        }
        if input.len() != self.width {
            return false;
        }
        self.input[self.input_buf_offset..(self.input_buf_offset + self.width)]
            .copy_from_slice(input);
        unsafe {
            crate::pffft_transform(
                self.setup,
                self.input[self.input_buf_offset..(self.input_buf_offset + self.width)].as_ptr(),
                self.output[self.output_buf_offset..(self.output_buf_offset + self.width)]
                    .as_mut_ptr(),
                self.work[self.work_buf_offset..(self.work_buf_offset + self.width)].as_mut_ptr(),
                crate::pffft_direction_t_PFFFT_FORWARD,
            )
        }
        output.copy_from_slice(
            &self.output[self.input_buf_offset..(self.input_buf_offset + self.width)],
        );
        true
    }

    pub fn backward(&mut self, input: &[f32], output: &mut [f32]) -> bool {
        if self.setup as usize == 0usize {
            return false;
        }
        if output.len() != self.width {
            return false;
        }
        if input.len() != self.width {
            return false;
        }
        self.input[self.input_buf_offset..(self.input_buf_offset + self.width)]
            .copy_from_slice(input);
        unsafe {
            crate::pffft_transform(
                self.setup,
                self.input[self.input_buf_offset..(self.input_buf_offset + self.width)].as_ptr(),
                self.output[self.output_buf_offset..(self.output_buf_offset + self.width)]
                    .as_mut_ptr(),
                self.work[self.work_buf_offset..(self.work_buf_offset + self.width)].as_mut_ptr(),
                crate::pffft_direction_t_PFFFT_BACKWARD,
            )
        }
        output.copy_from_slice(
            &self.output[self.input_buf_offset..(self.input_buf_offset + self.width)],
        );
        true
    }
}

impl Drop for PffftSetup {
    fn drop(&mut self) {
        unsafe {
            if self.setup as usize != 0usize {
                crate::pffft_destroy_setup(self.setup);
                self.setup = 0usize as *mut PFFFT_Setup;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unsafe() {
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

    #[test]
    fn test_forward_safe_no_crash() {
        let mut setup = crate::PffftSetup::new(32, true, false);
        let input: [f32; 32] = [0.0f32; 32];
        let mut output: [f32; 32] = [0.0f32; 32];
        setup.forward(&input, &mut output);
    }

    #[test]
    fn test_backward_safe_no_crash() {
        let mut setup = crate::PffftSetup::new(32, true, false);
        let input: [f32; 32] = [0.0f32; 32];
        let mut output: [f32; 32] = [0.0f32; 32];
        setup.backward(&input, &mut output);
    }

    #[test]
    fn test_forward_safe_dc() {
        let mut setup = crate::PffftSetup::new(32, true, false);
        let input: [f32; 32] = [1.0f32; 32];
        let mut output: [f32; 32] = [0.0f32; 32];
        setup.forward(&input, &mut output);
        assert_eq!(output[0], 32f32);
        for expect_zero_idx in 1..32 {
            assert_eq!(output[expect_zero_idx], 0f32);
        }
    }

    #[test]
    fn test_backward_safe_dc() {
        let mut setup = crate::PffftSetup::new(32, true, false);
        let mut input: [f32; 32] = [0.0f32; 32];
        input[0] = 1f32;
        let mut output: [f32; 32] = [0.0f32; 32];
        setup.backward(&input, &mut output);
        for idx in 0..32 {
            assert_eq!(output[idx], 1f32);
        }
    }
}
