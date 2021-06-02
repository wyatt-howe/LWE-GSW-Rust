#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand_distr::{Normal, Distribution};
    use rulinalg::{vector::Vector, matrix::{Matrix, BaseMatrix}};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        print!("libgsw")
    }

    fn gen_error(alpha: f32, size: usize, modulus: u32) -> Vector<u32> {
        let sigma = alpha / (2.0 * std::f32::consts::PI).sqrt();
        let nml = Normal::new(0.0, sigma).unwrap();
        let x: Vec<u32> = (0..size).map(|_| {
            let v = nml.sample(&mut rand::thread_rng()) as i32;
            let modulus = modulus as i32;
            let u = (v + modulus) % modulus;
            u as u32
        }).collect();
        Vector::new_bounded(x, modulus)
    }

    #[test]
    fn lwe_test() {
        let mut rng = rand::thread_rng();

        let n: usize = 230;
        let m: usize = 420;
        let q: u32 = 2053;//65521;
        let A: Matrix<u32> = Matrix::from_fn(n, m, |_i, _j| rng.gen_range(0..q) as u32);
        let alpha: f32 = 8.0;
        println!("[*] parameters");
        println!("lattice dimensions: nxm = {:?}x{:?}", n, m);
        println!("prime modulus: q = {:?}", q);
        println!("lattice basis: A in Z_{:?}^({:?}x{:?})", q, A.rows(), A.cols());
        // println!("lattice basis: A = {:?}", A);
        println!("error distribution parameter: alpha = {:?}", alpha);
        println!();

        println!("[*] secret key");
        let s: Vector<u32> = Vector::from_fn(m, |_i| rng.gen_range(0..q) as u32, q);
        // println!("s = {:?}", s);
        let G: Vector<u32> = (A.clone() * s.clone());//.apply(&|G_i| G_i % q);
        let e: Vector<u32> = gen_error(alpha, n, q);
        // println!("e = {:?}", e);
        let T: Vector<u32> = (G
            +
            e);//.apply(&|G_i| G_i % q);
        println!("[*] public key");
        // println!("T = {:?}", T);
        println!();

        let message_bit = 1;
        println!("[*] message_bit = {:?}", message_bit);
        println!();

        println!("[*] ciphertext");
        let r: Vector<u32> = gen_error(alpha, n, q);
        let C1: Vector<u32> = (A.clone().transpose() * r.clone());//.apply(&|C1_i| C1_i % q);
        // println!("C1 = {:?}", C1);
        let M: u32 = ((q + 1) / 2) * message_bit;
        let C2 = (r.clone().dot(&T) - M) % q;
        println!("C2 = {:?}", C2);
        println!();

        let p = (C1.dot(&s) - C2) % q;
        let decrypted_bit = (((q + 1) / 4) < p) as u32;
        println!("[*] decrypted_bit = {:?}", decrypted_bit);
        assert!(decrypted_bit == message_bit);
    }
}
