pub trait IntegerWidth {
    fn width() -> usize;
    fn to_u64(&self) -> u64;
}

impl IntegerWidth for bool {
    fn width() -> usize {
        1
    }

    fn to_u64(&self) -> u64 {
        if *self {
            1
        } else {
            0
        }
    }
}

impl IntegerWidth for u8 {
    fn width() -> usize {
        1
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl IntegerWidth for u16 {
    fn width() -> usize {
        2
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl IntegerWidth for u32 {
    fn width() -> usize {
        4
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl IntegerWidth for u64 {
    fn width() -> usize {
        8
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

