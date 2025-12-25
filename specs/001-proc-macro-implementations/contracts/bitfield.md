# Contract: Bitfield Macro

**Project**: `bitfield/`  
**Macro Type**: Attribute macro + Derive macro  
**Entry Points**:
- `#[proc_macro_attribute] fn bitfield` (in `bitfield-impl`)
- `#[proc_macro_derive(BitfieldSpecifier)]` (in `bitfield-impl`)
- Library types in `bitfield/src/lib.rs`

## Input Contract

### Specifier Types (B1-B64)

```rust
use bitfield::*;

// B1 through B64 are marker types implementing Specifier
pub struct MyByte {
    a: B1,
    b: B3,
    c: B4,  // Total: 8 bits
}
```

### Bitfield Struct

```rust
#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}
```

### Enum Specifier

```rust
#[derive(BitfieldSpecifier)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

#[bitfield]
pub struct Entry {
    trigger: TriggerMode,  // 1 bit (2 variants = 1 bit)
    delivery: DeliveryMode,  // 3 bits (8 variants)
    reserved: B4,
}
```

### Bits Attribute

```rust
#[bitfield]
pub struct Entry {
    #[bits = 1]
    trigger: TriggerMode,
    #[bits = 3]
    delivery: DeliveryMode,
    reserved: B4,
}
```

### Input Constraints

| Constraint | Requirement |
|------------|-------------|
| Total bits | Must be multiple of 8 |
| Field types | Must implement `Specifier` trait |
| Enum variants | Power of 2 for derive, or use `#[bits = N]` |
| Discriminants | Must be explicit integer literals |

## Output Contract

### Specifier Trait

```rust
pub trait Specifier {
    /// Number of bits this type occupies
    const BITS: usize;
    /// Storage type for get/set operations
    type Bytes;  // or type Storage depending on design
}
```

### B1-B64 Types

```rust
pub enum B1 {}
impl Specifier for B1 {
    const BITS: usize = 1;
    type Bytes = u8;
}

pub enum B8 {}
impl Specifier for B8 {
    const BITS: usize = 8;
    type Bytes = u8;
}

pub enum B24 {}
impl Specifier for B24 {
    const BITS: usize = 24;
    type Bytes = u32;
}
```

### Generated Bitfield Struct

Input:
```rust
#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}
```

Output:
```rust
pub struct MyFourBytes {
    data: [u8; 4],  // 32 bits / 8 = 4 bytes
}

impl MyFourBytes {
    pub fn new() -> Self {
        MyFourBytes { data: [0; 4] }
    }

    pub fn get_a(&self) -> u8 {
        // Extract bit 0
        (self.data[0] & 0b00000001) as u8
    }

    pub fn set_a(&mut self, value: u8) {
        // Set bit 0
        self.data[0] = (self.data[0] & !0b00000001) | (value & 0b1);
    }

    pub fn get_b(&self) -> u8 {
        // Extract bits 1-3
        ((self.data[0] >> 1) & 0b111) as u8
    }

    pub fn set_b(&mut self, value: u8) {
        // Set bits 1-3
        self.data[0] = (self.data[0] & !0b00001110) | ((value & 0b111) << 1);
    }

    pub fn get_c(&self) -> u8 {
        // Extract bits 4-7
        ((self.data[0] >> 4) & 0b1111) as u8
    }

    pub fn set_c(&mut self, value: u8) {
        self.data[0] = (self.data[0] & !0b11110000) | ((value & 0b1111) << 4);
    }

    pub fn get_d(&self) -> u32 {
        // Extract bits 8-31 (spanning bytes 1, 2, 3)
        let b1 = self.data[1] as u32;
        let b2 = self.data[2] as u32;
        let b3 = self.data[3] as u32;
        b1 | (b2 << 8) | (b3 << 16)
    }

    pub fn set_d(&mut self, value: u32) {
        self.data[1] = (value & 0xFF) as u8;
        self.data[2] = ((value >> 8) & 0xFF) as u8;
        self.data[3] = ((value >> 16) & 0xFF) as u8;
    }
}
```

### Generated Enum Specifier

Input:
```rust
#[derive(BitfieldSpecifier)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}
```

Output:
```rust
impl Specifier for TriggerMode {
    const BITS: usize = 1;  // ceil(log2(2)) = 1
    type Bytes = u8;
}

impl TriggerMode {
    fn into_bits(self) -> u8 {
        self as u8
    }

    fn from_bits(bits: u8) -> Self {
        match bits {
            0 => TriggerMode::Edge,
            1 => TriggerMode::Level,
            _ => panic!("invalid bits for TriggerMode"),
        }
    }
}
```

### Accessor Return Types

| Field Bits | Return Type |
|------------|-------------|
| 1-8 | `u8` |
| 9-16 | `u16` |
| 17-32 | `u32` |
| 33-64 | `u64` |

## Error Messages

### Not Multiple of 8

Input:
```rust
#[bitfield]
pub struct Bad {
    a: B1,
    b: B3,
    c: B3,  // Total: 7 bits
}
```

Output:
```
error: bitfield size is 7 bits which is not a multiple of 8
 --> tests/04-multiple-of-8bits.rs:5:1
  |
5 | pub struct Bad {
  | ^^^
```

### Enum Not Power of Two

Input:
```rust
#[derive(BitfieldSpecifier)]
pub enum Bad {
    A = 0,
    B = 1,
    C = 2,  // 3 variants, not power of 2
}
```

Output:
```
error: BitfieldSpecifier expected power of 2 variants, got 3
 --> tests/08-non-power-of-two.rs:3:1
  |
3 | pub enum Bad {
  | ^^^
```

### Bits Attribute Mismatch

Input:
```rust
#[bitfield]
pub struct Bad {
    #[bits = 2]  // Says 2 bits
    mode: TriggerMode,  // But TriggerMode is 1 bit
}
```

Output:
```
error: #[bits = 2] does not match TriggerMode::BITS (1)
```

## Test Coverage

| Test | Description | Type |
|------|-------------|------|
| 01-specifier-types.rs | B1-B64 types exist | pass |
| 02-storage.rs | Struct storage layout | pass |
| 03-accessors.rs | get_*/set_* methods | pass |
| 04-multiple-of-8bits.rs | Total bits validation | fail (expected) |
| 05-accessor-signatures.rs | Return type selection | pass |
| 06-enums.rs | BitfieldSpecifier derive | pass |
| 07-optional-discriminant.rs | Enum without explicit values | pass |
| 08-non-power-of-two.rs | Non-power-of-2 error | fail (expected) |
| 09-variant-out-of-range.rs | Discriminant too large | fail (expected) |
| 10-bits-attribute.rs | #[bits = N] works | pass |
| 11-bits-attribute-wrong.rs | #[bits = N] validation | fail (expected) |
| 12-accessors-edge.rs | Edge cases in bit access | pass |

