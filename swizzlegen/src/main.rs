use std::fs::File;
use std::io::{Result, Write};

const E: [char; 4] = ['x', 'y', 'z', 'w']; // element name
const B: [&str; 4] = ["00", "01", "10", "11"]; // shuffle bits
const V: [&str; 4] = ["1.0", "2.0", "3.0", "4.0"]; //element value
const VEC4: &str = "Vec4";
const VEC3A: &str = "Vec3A";
const VEC3: &str = "Vec3";
const VEC2: &str = "Vec2";

fn write_swizzle_head(out: &mut impl Write) -> Result<()> {
    writeln!(out, "// Generated by swizzlegen. Do not edit.")?;
    Ok(())
}

fn write_loops<W, F4, F3, F2>(
    out: &mut W,
    size: usize,
    vec4fn: F4,
    vec3fn: F3,
    vec2fn: F2,
) -> Result<()>
where
    W: Write,
    F4: Fn(&mut W, usize, usize, usize, usize) -> Result<()>,
    F3: Fn(&mut W, usize, usize, usize) -> Result<()>,
    F2: Fn(&mut W, usize, usize) -> Result<()>,
{
    for e0 in 0..size {
        for e1 in 0..size {
            for e2 in 0..size {
                for e3 in 0..size {
                    if size == 4 && e0 == 0 && e1 == 1 && e2 == 2 && e3 == 3 {
                        continue;
                    }
                    vec4fn(out, e0, e1, e2, e3)?;
                }
            }
        }
    }

    for e0 in 0..size {
        for e1 in 0..size {
            for e2 in 0..size {
                if size == 3 && e0 == 0 && e1 == 1 && e2 == 2 {
                    continue;
                }
                vec3fn(out, e0, e1, e2)?;
            }
        }
    }

    for e0 in 0..size {
        for e1 in 0..size {
            if size == 2 && e0 == 0 && e1 == 1 {
                continue;
            }
            vec2fn(out, e0, e1)?;
        }
    }

    Ok(())
}

fn write_swizzle_trait(
    out: &mut impl Write,
    size: usize,
    vec4t: &str,
    vec3t: &str,
    vec2t: &str,
) -> Result<()> {
    let t = match size {
        4 => vec4t,
        3 => vec3t,
        2 => vec2t,
        _ => unreachable!(),
    };

    write!(
        out,
        r#"
pub trait {}Swizzles {{"#,
        t
    )?;

    write_loops(
        out,
        size,
        |out, e0, e1, e2, e3| {
            write!(
                out,
                r#"
    fn {}{}{}{}(self) -> Vec4;"#,
                E[e0], E[e1], E[e2], E[e3]
            )
        },
        |out, e0, e1, e2| {
            write!(
                out,
                r#"
    fn {}{}{}(self) -> {};"#,
                E[e0], E[e1], E[e2], vec3t
            )
        },
        |out, e0, e1| {
            write!(
                out,
                r#"
    fn {}{}(self) -> Vec2;"#,
                E[e0], E[e1]
            )
        },
    )?;

    writeln!(
        out,
        r#"
}}"#
    )?;

    Ok(())
}

fn write_swizzle_vec4(out: &mut impl Write) -> Result<()> {
    const SIZE: usize = 4;

    write!(
        out,
        r#"
use super::{{Vec2, Vec3, Vec4}};

#[cfg(all(vec4_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec4_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;
"#
    )?;

    write_swizzle_trait(out, SIZE, VEC4, VEC3, VEC2)?;

    write!(
        out,
        r#"
impl Vec4Swizzles for Vec4 {{"#
    )?;

    write_loops(
        out,
        SIZE,
        |out, e0, e1, e2, e3| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}{}(self) -> Vec4 {{
        #[cfg(vec4_sse2)]
        unsafe {{
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_{}))
        }}

        #[cfg(vec4_f32)]
        {{
            Vec4 {{
                x: self.{},
                y: self.{},
                z: self.{},
                w: self.{},
            }}
        }}
    }}"#,
                E[e0], E[e1], E[e2], E[e3], B[e3], B[e2], B[e1], B[e0], E[e0], E[e1], E[e2], E[e3],
            )
        },
        |out, e0, e1, e2| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}(self) -> Vec3 {{
        #[cfg(vec4_sse2)]
        unsafe {{
            Vec3::from(Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_{}_{}_{})))
        }}

        #[cfg(vec4_f32)]
        {{
            Vec3 {{
                x: self.{},
                y: self.{},
                z: self.{},
            }}
        }}
    }}"#,
                E[e0], E[e1], E[e2], B[e2], B[e1], B[e0], E[e0], E[e1], E[e2]
            )
        },
        |out, e0, e1| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}(self) -> Vec2 {{
        #[cfg(vec4_sse2)]
        unsafe {{
            Vec2::from(Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_{}_{})))
        }}

        #[cfg(vec4_f32)]
        {{
            Vec2 {{
                x: self.{},
                y: self.{},
            }}
        }}
    }}"#,
                E[e0], E[e1], B[e1], B[e0], E[e0], E[e1]
            )
        },
    )?;

    write!(out, "\n}}\n")?;

    Ok(())
}

fn write_swizzle_vec3a(out: &mut impl Write) -> Result<()> {
    const SIZE: usize = 3;

    write!(
        out,
        r#"
use super::{{Vec2, Vec3A, Vec4}};

#[cfg(all(vec3a_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec3a_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;
"#
    )?;

    write_swizzle_trait(out, SIZE, VEC4, VEC3A, VEC2)?;

    write!(
        out,
        r#"
#[rustfmt::skip]
impl Vec3ASwizzles for Vec3A {{"#
    )?;

    write_loops(
        out,
        SIZE,
        |out, e0, e1, e2, e3| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}{}(self) -> Vec4 {{
        #[cfg(vec3a_sse2)]
        unsafe {{
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_{}))
        }}

        #[cfg(vec3a_f32)]
        {{
            Vec4 {{ x: self.{}, y: self.{}, z: self.{}, w: self.{} }}
        }}
    }}"#,
                E[e0], E[e1], E[e2], E[e3], B[e3], B[e2], B[e1], B[e0], E[e0], E[e1], E[e2], E[e3],
            )
        },
        |out, e0, e1, e2| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}(self) -> Vec3A {{
        #[cfg(vec3a_sse2)]
        unsafe {{
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_{}_{}_{}))
        }}

        #[cfg(vec3a_f32)]
        {{
            Vec3A {{ x: self.{}, y: self.{}, z: self.{} }}
        }}
    }}"#,
                E[e0], E[e1], E[e2], B[e2], B[e1], B[e0], E[e0], E[e1], E[e2]
            )
        },
        |out, e0, e1| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}(self) -> Vec2 {{
        #[cfg(vec3a_sse2)]
        unsafe {{
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_{}_{})))
        }}

        #[cfg(vec3a_f32)]
        {{
            Vec2 {{ x: self.{}, y: self.{} }}
        }}
    }}"#,
                E[e0], E[e1], B[e1], B[e0], E[e0], E[e1]
            )
        },
    )?;

    write!(out, "\n}}\n")?;

    Ok(())
}

fn write_swizzle_vec3(out: &mut impl Write) -> Result<()> {
    const SIZE: usize = 3;

    write!(
        out,
        r#"
use super::{{Vec2, Vec3, Vec4}};
"#
    )?;

    write_swizzle_trait(out, SIZE, VEC4, VEC3, VEC2)?;

    write!(
        out,
        r#"
impl Vec3Swizzles for Vec3 {{"#
    )?;

    write_loops(
        out,
        SIZE,
        |out, e0, e1, e2, e3| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}{}(self) -> Vec4 {{
        Vec4::new(self.{}, self.{}, self.{}, self.{})
    }}"#,
                E[e0], E[e1], E[e2], E[e3], E[e0], E[e1], E[e2], E[e3],
            )
        },
        |out, e0, e1, e2| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}(self) -> Vec3 {{
        Vec3 {{
            x: self.{},
            y: self.{},
            z: self.{},
        }}
    }}"#,
                E[e0], E[e1], E[e2], E[e0], E[e1], E[e2]
            )
        },
        |out, e0, e1| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}(self) -> Vec2 {{
        Vec2 {{
            x: self.{},
            y: self.{},
        }}
    }}"#,
                E[e0], E[e1], E[e0], E[e1]
            )
        },
    )?;

    write!(out, "\n}}\n")?;

    Ok(())
}

fn write_swizzle_vec2(out: &mut impl Write) -> Result<()> {
    const SIZE: usize = 2;

    write!(
        out,
        r#"
use super::{{Vec2, Vec3, Vec4}};
"#
    )?;

    write_swizzle_trait(out, SIZE, VEC4, VEC3, VEC2)?;

    write!(
        out,
        r#"
impl Vec2Swizzles for Vec2 {{"#
    )?;

    write_loops(
        out,
        SIZE,
        |out, e0, e1, e2, e3| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}{}(self) -> Vec4 {{
        Vec4::new(self.{}, self.{}, self.{}, self.{})
    }}"#,
                E[e0], E[e1], E[e2], E[e3], E[e0], E[e1], E[e2], E[e3],
            )
        },
        |out, e0, e1, e2| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}{}(self) -> Vec3 {{
        Vec3 {{
            x: self.{},
            y: self.{},
            z: self.{},
        }}
    }}"#,
                E[e0], E[e1], E[e2], E[e0], E[e1], E[e2]
            )
        },
        |out, e0, e1| {
            write!(
                out,
                r#"
    #[inline]
    fn {}{}(self) -> Vec2 {{
        Vec2 {{
            x: self.{},
            y: self.{},
        }}
    }}"#,
                E[e0], E[e1], E[e0], E[e1]
            )
        },
    )?;

    write!(out, "\n}}\n")?;

    Ok(())
}

fn write_src() -> Result<()> {
    let mut out = File::create("../src/f32/vec4_swizzle.rs")?;
    write_swizzle_head(&mut out)?;
    write_swizzle_vec4(&mut out)?;

    let mut out = File::create("../src/f32/vec3a_swizzle.rs")?;
    write_swizzle_head(&mut out)?;
    write_swizzle_vec3a(&mut out)?;

    let mut out = File::create("../src/f32/vec3_swizzle.rs")?;
    write_swizzle_head(&mut out)?;
    write_swizzle_vec3(&mut out)?;

    let mut out = File::create("../src/f32/vec2_swizzle.rs")?;
    write_swizzle_head(&mut out)?;
    write_swizzle_vec2(&mut out)?;

    Ok(())
}

fn write_test_head(out: &mut impl Write) -> Result<()> {
    write!(
        out,
        r#"// Generated by swizzles. Do not edit.
use glam::{{swizzles::*, vec2, vec3, vec3a, vec4}};
"#
    )?;
    Ok(())
}

fn write_test_vec4(out: &mut impl Write) -> Result<()> {
    const SIZE: usize = 4;

    write!(
        out,
        r#"
#[test]
fn test_vec4_swizzles() {{
    let v = vec4(1.0, 2.0, 3.0, 4.0);
"#
    )?;

    write_test_loops(out, SIZE, "vec3")?;

    writeln!(out, "}}")?;

    Ok(())
}

fn write_test_vec3(out: &mut impl Write, vec3t: &str) -> Result<()> {
    const SIZE: usize = 3;

    write!(
        out,
        r#"
#[test]
fn test_{}_swizzles() {{
    let v = {}(1.0, 2.0, 3.0);
"#,
        vec3t, vec3t,
    )?;

    write_test_loops(out, SIZE, vec3t)?;

    writeln!(out, "}}")?;

    Ok(())
}

fn write_test_vec2(out: &mut impl Write) -> Result<()> {
    const SIZE: usize = 2;

    write!(
        out,
        r#"
#[test]
fn test_vec2_swizzles() {{
    let v = vec2(1.0, 2.0);
"#
    )?;

    write_test_loops(out, SIZE, "vec3")?;

    writeln!(out, "}}")?;

    Ok(())
}

fn write_test_loops(out: &mut impl Write, size: usize, vec3t: &str) -> Result<()> {
    write_loops(
        out,
        size,
        |out, e0, e1, e2, e3| {
            writeln!(
                out,
                "    assert_eq!(v.{}{}{}{}(), vec4({}, {}, {}, {}));",
                E[e0], E[e1], E[e2], E[e3], V[e0], V[e1], V[e2], V[e3]
            )
        },
        |out, e0, e1, e2| {
            writeln!(
                out,
                "    assert_eq!(v.{}{}{}(), {}({}, {}, {}));",
                E[e0], E[e1], E[e2], vec3t, V[e0], V[e1], V[e2]
            )
        },
        |out, e0, e1| {
            writeln!(
                out,
                "    assert_eq!(v.{}{}(), vec2({}, {}));",
                E[e0], E[e1], V[e0], V[e1]
            )
        },
    )
}

fn write_test() -> Result<()> {
    let mut out = File::create("../tests/swizzle.rs")?;
    write_test_head(&mut out)?;
    write_test_vec4(&mut out)?;
    write_test_vec3(&mut out, "vec3a")?;
    write_test_vec3(&mut out, "vec3")?;
    write_test_vec2(&mut out)?;
    Ok(())
}

fn main() -> Result<()> {
    write_src()?;
    write_test()?;
    Ok(())
}
