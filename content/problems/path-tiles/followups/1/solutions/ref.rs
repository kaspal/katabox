const MOD: i64 = 1_000_003;

fn mul(a: &[[i64; 3]; 3], b: &[[i64; 3]; 3]) -> [[i64; 3]; 3] {
    let mut c = [[0i64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j]) % MOD;
        }
    }
    c
}

pub fn tilings_long(n: i64) -> i64 {
    let mut step = [[1, 1, 1], [1, 0, 0], [0, 1, 0]];
    let mut power = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    let mut e = n;
    while e > 0 {
        if e & 1 == 1 {
            power = mul(&power, &step);
        }
        step = mul(&step, &step);
        e >>= 1;
    }
    power[0][0]
}
