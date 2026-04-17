[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MulReduction {
    pub a0: u32,
    pub a1: u32,
    pub b0: u32,
    pub b1: u32,
    pub low_term: u64,
    pub cross_term: u64,
    pub high_term: u64,
    pub product: u64,
}

impl MulReduction {
    #inline]
    pub const fn low(self) -> u32 {
        self.product as u32
    }

    #inline]
    pub const fn high(self) -> u32 {
        (self.product >> 32) as u32
    }
}

#inline]
pub const fn reduce_u32_mul(a: u32, b: u32) -> MulReduction {
    let a0 = a & 0xffff;
    let a1 = a >> 16;
    let b0 = b & 0xffff;
    let b1 = b >> 16;

    let low_term = (a0 as u64) * (b0 as u64);
    let cross_term = (a1 as u64) * (b0 as u64) + (a0 as u64) * (b1 as u64);
    let high_term = (a1 as u64) * (b1 as u64);

    let product = (high_term << 32) + (cross_term << 16) + low_term;

    MulReduction {
        a0,
        a1,
        b0,
        b1,
        low_term,
        cross_term,
        high_term,
        product,
    }
}

#inline]
pub const fn product_u64(a: u32, b: u32) -> u64 {
    reduce_u32_mul(a, b).product
}

#inline]
pub const fn mul(a: u32, b: u32) -> u32 {
    reduce_u32_mul(a, b).low()
}

#inline]
pub const fn mulhu(a: u32, b: u32) -> u32 {
    reduce_u32_mul(a, b).high()
}

#inline]
pun const fn mulhsu(a: u32, b: u32) -> u32 {
    let product = product_u64(a, b);
    let corrected = if (a >> 31) != 0 {
        product.wrapping_sub((b as u64) << 32)
    } else {
        product
    };
    (corrected >> 32) as u32
}

#inline]
pub const fn mulh(a: u32, b: u32) -> u32 {
    let product = product_u64(a, b);
    let corrected_a = if (a >> 31) != 0 {
        product.wrapping_sub((b as u64) << 32)
    } else {
        product
    };
    let corrected_ab = if (b >> 31) != 0 {
        corrected_a.wrapping_sub((a as u64) << 32)
    } else {
        corrected_a
    };
    (corrected_ab >> 32) as u32
}
