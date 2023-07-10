use rand::Rng; // 0.8.5

fn main() {
    println!("0. In the Chaum-Pederson method, we initially define the values of <g, A, B, AB>");
    let g: u32 = 6;
    let a: u32 = 3;
    let b: u32 = 4;
    let q: u32 = 10009;

    let g_pow_a: u32 = g.pow(a) % q;
    let g_pow_b: u32 = g.pow(b) % q;
    let g_pow_ab: u32 = g.pow(a * b) % q;

    let mut rng = rand::thread_rng();
    let r: u32 = rng.gen_range(2..4); // => need to hide

    let gr: u32 = g.pow(r) % q;
    let gb_pow_r = g_pow_b.pow(r) % q;

    let c: u32 = rng.gen_range(1..4);
    let z: u32 = (r + a * c) % q;
    let g_pow_z: u32 = g.pow(z) % q;

    let y1: u32 = (g_pow_a.pow(c) * gr) % q;
    println!("0. Victor and Peggy agree of (g,g^a, g^b and g^ab) = ({g}, {g_pow_a}, {g_pow_b} and {g_pow_ab})\n");

    println!("1. Peggy generates random number (r): {r} - also known as secret");
    println!("2. Peggy sends y1(g^r, B^r) = ({gr}, {gb_pow_r})\n");
    println!("3. Victor sends a random value (c)={c} challenge Peggy");
    println!("4. Peggy computes z=r+a*s(mod q) = {z}\n");

    println!("5. Victor now checks these are the same");
    println!("5.1 Victor checks g^z = {g_pow_z}");
    println!("5.2 Victor checks A^c y1 (mod q) = {y1}\n");
}
