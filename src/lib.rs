// #![allow(arithmetic_overflow)]
#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand_distr::{Normal, Distribution};
    use rulinalg::{vector::Vector, matrix::{Matrix, BaseMatrix}};
    use std::num::Wrapping;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        print!("libgsw");
        dbg!(Wrapping(u8::MAX) + Wrapping(1u8));
    }

    fn gen_error(alpha: f32, size: usize) -> Vector<Wrapping<u32>> {
        let sigma = alpha / (2.0 * std::f32::consts::PI).sqrt();
        let nml = Normal::new(0.0, sigma).unwrap();
        let x: Vec<Wrapping<u32>> = (0..size).map(|_| {
            let v = nml.sample(&mut rand::thread_rng()) as i16;
            Wrapping(v as u32)
        }).collect();
        Vector::new(x)
    }

    #[test]
    fn lwe_test() {
        let mut rng = rand::thread_rng();

        let n: usize = 10000;
        let m: usize = 14;
        let q: u32 = u32::MAX;//Wrapping(u32::MAX);//65521;
        let A: Matrix<Wrapping<u32>> = Matrix::from_fn(n, m, |_i, _j| Wrapping(rng.gen_range(0..q) as u32));
        let alpha: f32 = 8.0;
        println!("[*] parameters");
        println!("lattice dimensions: nxm = {:?}x{:?}", n, m);
        println!("prime modulus: q = {:?}", q);
        println!("lattice basis: A in Z_{:?}^({:?}x{:?})", q, A.rows(), A.cols());
        // println!("lattice basis: A = {:?}", A);
        println!("error distribution parameter: alpha = {:?}", alpha);
        println!();

        println!("[*] secret key");
        let s: Vector<Wrapping<u32>> = Vector::from_fn(m, |_i| Wrapping(rng.gen_range(0..q) as u32));
        // println!("s = {:?}", s);
        let G: Vector<Wrapping<u32>> = A.clone() * s.clone();
        let e: Vector<Wrapping<u32>> = gen_error(alpha, n);
        // println!("e = {:?}", e);
        let T: Vector<Wrapping<u32>> = G + e;
        println!("[*] public key");
        // println!("T = {:?}", T);
        println!();

        let message_bit: Wrapping<u32> = Wrapping(1);
        println!("[*] message_bit = {:?}", message_bit);
        println!();

        println!("[*] ciphertext");
        let r: Vector<Wrapping<u32>> = gen_error(alpha, n);
        let C1: Vector<Wrapping<u32>> = A.clone().transpose() * r.clone();
        // println!("C1 = {:?}", C1);
        let M: Wrapping<u32> = Wrapping((q - 1) / 2 + 1) * message_bit;
        let C2 = r.clone().dot(&T) - M;
        println!("C2 = {:?}", C2);
        println!();

        let p = C1.dot(&s) - C2;
        let decrypted_bit = Wrapping((Wrapping((q - 3) / 4 + 1) < p) as u32);
        println!("[*] decrypted_bit = {:?}", decrypted_bit);
        assert_eq!(decrypted_bit, message_bit);
    }
}
