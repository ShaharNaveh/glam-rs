// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3A, Vec3Swizzles, Vec4};

use core::arch::wasm32::*;

impl Vec3Swizzles for Vec3A {
    type Vec2 = Vec2;

    type Vec4 = Vec4;

    #[inline]
    #[must_use]
    fn xx(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn with_xy(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn with_xz(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y)
    }

    #[inline]
    #[must_use]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn with_yx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yy(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn with_yz(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y)
    }

    #[inline]
    #[must_use]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn with_zx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x)
    }

    #[inline]
    #[must_use]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn with_zy(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x)
    }

    #[inline]
    #[must_use]
    fn zz(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 0, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 0, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 0, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 1, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 1, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 2, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 2, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<0, 2, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 0, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 0, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 0, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 1, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 1, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 1, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 2, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 2, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<1, 2, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 0, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 0, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 0, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 1, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 1, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 1, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 2, 4, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 2, 5, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> Vec3A {
        Vec3A(i32x4_shuffle::<2, 2, 6, 4>(self.0, self.0).into())
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 6, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 4, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 4, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 4, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 5, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 5, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 5, 6>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 6, 4>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 6, 5>(self.0, self.0))
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 6, 6>(self.0, self.0))
    }
}
