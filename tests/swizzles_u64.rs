// Generated by swizzlegen. Do not edit.
#[macro_use]
mod support;
use glam::*;

glam_test!(test_u64vec4_swizzles, {
    let v = u64vec4(1_u64, 2_u64, 3_u64, 4_u64);
    let rhs3 = u64vec3(11_u64, 12_u64, 13_u64);
    let rhs2 = u64vec2(11_u64, 12_u64);
    assert_eq!(v, v.xyzw());
    assert_eq!(v.xxxx(), u64vec4(1_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.xxxy(), u64vec4(1_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.xxxz(), u64vec4(1_u64, 1_u64, 1_u64, 3_u64));
    assert_eq!(v.xxxw(), u64vec4(1_u64, 1_u64, 1_u64, 4_u64));
    assert_eq!(v.xxyx(), u64vec4(1_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.xxyy(), u64vec4(1_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.xxyz(), u64vec4(1_u64, 1_u64, 2_u64, 3_u64));
    assert_eq!(v.xxyw(), u64vec4(1_u64, 1_u64, 2_u64, 4_u64));
    assert_eq!(v.xxzx(), u64vec4(1_u64, 1_u64, 3_u64, 1_u64));
    assert_eq!(v.xxzy(), u64vec4(1_u64, 1_u64, 3_u64, 2_u64));
    assert_eq!(v.xxzz(), u64vec4(1_u64, 1_u64, 3_u64, 3_u64));
    assert_eq!(v.xxzw(), u64vec4(1_u64, 1_u64, 3_u64, 4_u64));
    assert_eq!(v.xxwx(), u64vec4(1_u64, 1_u64, 4_u64, 1_u64));
    assert_eq!(v.xxwy(), u64vec4(1_u64, 1_u64, 4_u64, 2_u64));
    assert_eq!(v.xxwz(), u64vec4(1_u64, 1_u64, 4_u64, 3_u64));
    assert_eq!(v.xxww(), u64vec4(1_u64, 1_u64, 4_u64, 4_u64));
    assert_eq!(v.xyxx(), u64vec4(1_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.xyxy(), u64vec4(1_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.xyxz(), u64vec4(1_u64, 2_u64, 1_u64, 3_u64));
    assert_eq!(v.xyxw(), u64vec4(1_u64, 2_u64, 1_u64, 4_u64));
    assert_eq!(v.xyyx(), u64vec4(1_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.xyyy(), u64vec4(1_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.xyyz(), u64vec4(1_u64, 2_u64, 2_u64, 3_u64));
    assert_eq!(v.xyyw(), u64vec4(1_u64, 2_u64, 2_u64, 4_u64));
    assert_eq!(v.xyzx(), u64vec4(1_u64, 2_u64, 3_u64, 1_u64));
    assert_eq!(v.xyzy(), u64vec4(1_u64, 2_u64, 3_u64, 2_u64));
    assert_eq!(v.xyzz(), u64vec4(1_u64, 2_u64, 3_u64, 3_u64));
    assert_eq!(v.xywx(), u64vec4(1_u64, 2_u64, 4_u64, 1_u64));
    assert_eq!(v.xywy(), u64vec4(1_u64, 2_u64, 4_u64, 2_u64));
    assert_eq!(v.xywz(), u64vec4(1_u64, 2_u64, 4_u64, 3_u64));
    assert_eq!(v.xyww(), u64vec4(1_u64, 2_u64, 4_u64, 4_u64));
    assert_eq!(v.xzxx(), u64vec4(1_u64, 3_u64, 1_u64, 1_u64));
    assert_eq!(v.xzxy(), u64vec4(1_u64, 3_u64, 1_u64, 2_u64));
    assert_eq!(v.xzxz(), u64vec4(1_u64, 3_u64, 1_u64, 3_u64));
    assert_eq!(v.xzxw(), u64vec4(1_u64, 3_u64, 1_u64, 4_u64));
    assert_eq!(v.xzyx(), u64vec4(1_u64, 3_u64, 2_u64, 1_u64));
    assert_eq!(v.xzyy(), u64vec4(1_u64, 3_u64, 2_u64, 2_u64));
    assert_eq!(v.xzyz(), u64vec4(1_u64, 3_u64, 2_u64, 3_u64));
    assert_eq!(v.xzyw(), u64vec4(1_u64, 3_u64, 2_u64, 4_u64));
    assert_eq!(v.xzzx(), u64vec4(1_u64, 3_u64, 3_u64, 1_u64));
    assert_eq!(v.xzzy(), u64vec4(1_u64, 3_u64, 3_u64, 2_u64));
    assert_eq!(v.xzzz(), u64vec4(1_u64, 3_u64, 3_u64, 3_u64));
    assert_eq!(v.xzzw(), u64vec4(1_u64, 3_u64, 3_u64, 4_u64));
    assert_eq!(v.xzwx(), u64vec4(1_u64, 3_u64, 4_u64, 1_u64));
    assert_eq!(v.xzwy(), u64vec4(1_u64, 3_u64, 4_u64, 2_u64));
    assert_eq!(v.xzwz(), u64vec4(1_u64, 3_u64, 4_u64, 3_u64));
    assert_eq!(v.xzww(), u64vec4(1_u64, 3_u64, 4_u64, 4_u64));
    assert_eq!(v.xwxx(), u64vec4(1_u64, 4_u64, 1_u64, 1_u64));
    assert_eq!(v.xwxy(), u64vec4(1_u64, 4_u64, 1_u64, 2_u64));
    assert_eq!(v.xwxz(), u64vec4(1_u64, 4_u64, 1_u64, 3_u64));
    assert_eq!(v.xwxw(), u64vec4(1_u64, 4_u64, 1_u64, 4_u64));
    assert_eq!(v.xwyx(), u64vec4(1_u64, 4_u64, 2_u64, 1_u64));
    assert_eq!(v.xwyy(), u64vec4(1_u64, 4_u64, 2_u64, 2_u64));
    assert_eq!(v.xwyz(), u64vec4(1_u64, 4_u64, 2_u64, 3_u64));
    assert_eq!(v.xwyw(), u64vec4(1_u64, 4_u64, 2_u64, 4_u64));
    assert_eq!(v.xwzx(), u64vec4(1_u64, 4_u64, 3_u64, 1_u64));
    assert_eq!(v.xwzy(), u64vec4(1_u64, 4_u64, 3_u64, 2_u64));
    assert_eq!(v.xwzz(), u64vec4(1_u64, 4_u64, 3_u64, 3_u64));
    assert_eq!(v.xwzw(), u64vec4(1_u64, 4_u64, 3_u64, 4_u64));
    assert_eq!(v.xwwx(), u64vec4(1_u64, 4_u64, 4_u64, 1_u64));
    assert_eq!(v.xwwy(), u64vec4(1_u64, 4_u64, 4_u64, 2_u64));
    assert_eq!(v.xwwz(), u64vec4(1_u64, 4_u64, 4_u64, 3_u64));
    assert_eq!(v.xwww(), u64vec4(1_u64, 4_u64, 4_u64, 4_u64));
    assert_eq!(v.yxxx(), u64vec4(2_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.yxxy(), u64vec4(2_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.yxxz(), u64vec4(2_u64, 1_u64, 1_u64, 3_u64));
    assert_eq!(v.yxxw(), u64vec4(2_u64, 1_u64, 1_u64, 4_u64));
    assert_eq!(v.yxyx(), u64vec4(2_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.yxyy(), u64vec4(2_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.yxyz(), u64vec4(2_u64, 1_u64, 2_u64, 3_u64));
    assert_eq!(v.yxyw(), u64vec4(2_u64, 1_u64, 2_u64, 4_u64));
    assert_eq!(v.yxzx(), u64vec4(2_u64, 1_u64, 3_u64, 1_u64));
    assert_eq!(v.yxzy(), u64vec4(2_u64, 1_u64, 3_u64, 2_u64));
    assert_eq!(v.yxzz(), u64vec4(2_u64, 1_u64, 3_u64, 3_u64));
    assert_eq!(v.yxzw(), u64vec4(2_u64, 1_u64, 3_u64, 4_u64));
    assert_eq!(v.yxwx(), u64vec4(2_u64, 1_u64, 4_u64, 1_u64));
    assert_eq!(v.yxwy(), u64vec4(2_u64, 1_u64, 4_u64, 2_u64));
    assert_eq!(v.yxwz(), u64vec4(2_u64, 1_u64, 4_u64, 3_u64));
    assert_eq!(v.yxww(), u64vec4(2_u64, 1_u64, 4_u64, 4_u64));
    assert_eq!(v.yyxx(), u64vec4(2_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.yyxy(), u64vec4(2_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.yyxz(), u64vec4(2_u64, 2_u64, 1_u64, 3_u64));
    assert_eq!(v.yyxw(), u64vec4(2_u64, 2_u64, 1_u64, 4_u64));
    assert_eq!(v.yyyx(), u64vec4(2_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.yyyy(), u64vec4(2_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.yyyz(), u64vec4(2_u64, 2_u64, 2_u64, 3_u64));
    assert_eq!(v.yyyw(), u64vec4(2_u64, 2_u64, 2_u64, 4_u64));
    assert_eq!(v.yyzx(), u64vec4(2_u64, 2_u64, 3_u64, 1_u64));
    assert_eq!(v.yyzy(), u64vec4(2_u64, 2_u64, 3_u64, 2_u64));
    assert_eq!(v.yyzz(), u64vec4(2_u64, 2_u64, 3_u64, 3_u64));
    assert_eq!(v.yyzw(), u64vec4(2_u64, 2_u64, 3_u64, 4_u64));
    assert_eq!(v.yywx(), u64vec4(2_u64, 2_u64, 4_u64, 1_u64));
    assert_eq!(v.yywy(), u64vec4(2_u64, 2_u64, 4_u64, 2_u64));
    assert_eq!(v.yywz(), u64vec4(2_u64, 2_u64, 4_u64, 3_u64));
    assert_eq!(v.yyww(), u64vec4(2_u64, 2_u64, 4_u64, 4_u64));
    assert_eq!(v.yzxx(), u64vec4(2_u64, 3_u64, 1_u64, 1_u64));
    assert_eq!(v.yzxy(), u64vec4(2_u64, 3_u64, 1_u64, 2_u64));
    assert_eq!(v.yzxz(), u64vec4(2_u64, 3_u64, 1_u64, 3_u64));
    assert_eq!(v.yzxw(), u64vec4(2_u64, 3_u64, 1_u64, 4_u64));
    assert_eq!(v.yzyx(), u64vec4(2_u64, 3_u64, 2_u64, 1_u64));
    assert_eq!(v.yzyy(), u64vec4(2_u64, 3_u64, 2_u64, 2_u64));
    assert_eq!(v.yzyz(), u64vec4(2_u64, 3_u64, 2_u64, 3_u64));
    assert_eq!(v.yzyw(), u64vec4(2_u64, 3_u64, 2_u64, 4_u64));
    assert_eq!(v.yzzx(), u64vec4(2_u64, 3_u64, 3_u64, 1_u64));
    assert_eq!(v.yzzy(), u64vec4(2_u64, 3_u64, 3_u64, 2_u64));
    assert_eq!(v.yzzz(), u64vec4(2_u64, 3_u64, 3_u64, 3_u64));
    assert_eq!(v.yzzw(), u64vec4(2_u64, 3_u64, 3_u64, 4_u64));
    assert_eq!(v.yzwx(), u64vec4(2_u64, 3_u64, 4_u64, 1_u64));
    assert_eq!(v.yzwy(), u64vec4(2_u64, 3_u64, 4_u64, 2_u64));
    assert_eq!(v.yzwz(), u64vec4(2_u64, 3_u64, 4_u64, 3_u64));
    assert_eq!(v.yzww(), u64vec4(2_u64, 3_u64, 4_u64, 4_u64));
    assert_eq!(v.ywxx(), u64vec4(2_u64, 4_u64, 1_u64, 1_u64));
    assert_eq!(v.ywxy(), u64vec4(2_u64, 4_u64, 1_u64, 2_u64));
    assert_eq!(v.ywxz(), u64vec4(2_u64, 4_u64, 1_u64, 3_u64));
    assert_eq!(v.ywxw(), u64vec4(2_u64, 4_u64, 1_u64, 4_u64));
    assert_eq!(v.ywyx(), u64vec4(2_u64, 4_u64, 2_u64, 1_u64));
    assert_eq!(v.ywyy(), u64vec4(2_u64, 4_u64, 2_u64, 2_u64));
    assert_eq!(v.ywyz(), u64vec4(2_u64, 4_u64, 2_u64, 3_u64));
    assert_eq!(v.ywyw(), u64vec4(2_u64, 4_u64, 2_u64, 4_u64));
    assert_eq!(v.ywzx(), u64vec4(2_u64, 4_u64, 3_u64, 1_u64));
    assert_eq!(v.ywzy(), u64vec4(2_u64, 4_u64, 3_u64, 2_u64));
    assert_eq!(v.ywzz(), u64vec4(2_u64, 4_u64, 3_u64, 3_u64));
    assert_eq!(v.ywzw(), u64vec4(2_u64, 4_u64, 3_u64, 4_u64));
    assert_eq!(v.ywwx(), u64vec4(2_u64, 4_u64, 4_u64, 1_u64));
    assert_eq!(v.ywwy(), u64vec4(2_u64, 4_u64, 4_u64, 2_u64));
    assert_eq!(v.ywwz(), u64vec4(2_u64, 4_u64, 4_u64, 3_u64));
    assert_eq!(v.ywww(), u64vec4(2_u64, 4_u64, 4_u64, 4_u64));
    assert_eq!(v.zxxx(), u64vec4(3_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.zxxy(), u64vec4(3_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.zxxz(), u64vec4(3_u64, 1_u64, 1_u64, 3_u64));
    assert_eq!(v.zxxw(), u64vec4(3_u64, 1_u64, 1_u64, 4_u64));
    assert_eq!(v.zxyx(), u64vec4(3_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.zxyy(), u64vec4(3_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.zxyz(), u64vec4(3_u64, 1_u64, 2_u64, 3_u64));
    assert_eq!(v.zxyw(), u64vec4(3_u64, 1_u64, 2_u64, 4_u64));
    assert_eq!(v.zxzx(), u64vec4(3_u64, 1_u64, 3_u64, 1_u64));
    assert_eq!(v.zxzy(), u64vec4(3_u64, 1_u64, 3_u64, 2_u64));
    assert_eq!(v.zxzz(), u64vec4(3_u64, 1_u64, 3_u64, 3_u64));
    assert_eq!(v.zxzw(), u64vec4(3_u64, 1_u64, 3_u64, 4_u64));
    assert_eq!(v.zxwx(), u64vec4(3_u64, 1_u64, 4_u64, 1_u64));
    assert_eq!(v.zxwy(), u64vec4(3_u64, 1_u64, 4_u64, 2_u64));
    assert_eq!(v.zxwz(), u64vec4(3_u64, 1_u64, 4_u64, 3_u64));
    assert_eq!(v.zxww(), u64vec4(3_u64, 1_u64, 4_u64, 4_u64));
    assert_eq!(v.zyxx(), u64vec4(3_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.zyxy(), u64vec4(3_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.zyxz(), u64vec4(3_u64, 2_u64, 1_u64, 3_u64));
    assert_eq!(v.zyxw(), u64vec4(3_u64, 2_u64, 1_u64, 4_u64));
    assert_eq!(v.zyyx(), u64vec4(3_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.zyyy(), u64vec4(3_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.zyyz(), u64vec4(3_u64, 2_u64, 2_u64, 3_u64));
    assert_eq!(v.zyyw(), u64vec4(3_u64, 2_u64, 2_u64, 4_u64));
    assert_eq!(v.zyzx(), u64vec4(3_u64, 2_u64, 3_u64, 1_u64));
    assert_eq!(v.zyzy(), u64vec4(3_u64, 2_u64, 3_u64, 2_u64));
    assert_eq!(v.zyzz(), u64vec4(3_u64, 2_u64, 3_u64, 3_u64));
    assert_eq!(v.zyzw(), u64vec4(3_u64, 2_u64, 3_u64, 4_u64));
    assert_eq!(v.zywx(), u64vec4(3_u64, 2_u64, 4_u64, 1_u64));
    assert_eq!(v.zywy(), u64vec4(3_u64, 2_u64, 4_u64, 2_u64));
    assert_eq!(v.zywz(), u64vec4(3_u64, 2_u64, 4_u64, 3_u64));
    assert_eq!(v.zyww(), u64vec4(3_u64, 2_u64, 4_u64, 4_u64));
    assert_eq!(v.zzxx(), u64vec4(3_u64, 3_u64, 1_u64, 1_u64));
    assert_eq!(v.zzxy(), u64vec4(3_u64, 3_u64, 1_u64, 2_u64));
    assert_eq!(v.zzxz(), u64vec4(3_u64, 3_u64, 1_u64, 3_u64));
    assert_eq!(v.zzxw(), u64vec4(3_u64, 3_u64, 1_u64, 4_u64));
    assert_eq!(v.zzyx(), u64vec4(3_u64, 3_u64, 2_u64, 1_u64));
    assert_eq!(v.zzyy(), u64vec4(3_u64, 3_u64, 2_u64, 2_u64));
    assert_eq!(v.zzyz(), u64vec4(3_u64, 3_u64, 2_u64, 3_u64));
    assert_eq!(v.zzyw(), u64vec4(3_u64, 3_u64, 2_u64, 4_u64));
    assert_eq!(v.zzzx(), u64vec4(3_u64, 3_u64, 3_u64, 1_u64));
    assert_eq!(v.zzzy(), u64vec4(3_u64, 3_u64, 3_u64, 2_u64));
    assert_eq!(v.zzzz(), u64vec4(3_u64, 3_u64, 3_u64, 3_u64));
    assert_eq!(v.zzzw(), u64vec4(3_u64, 3_u64, 3_u64, 4_u64));
    assert_eq!(v.zzwx(), u64vec4(3_u64, 3_u64, 4_u64, 1_u64));
    assert_eq!(v.zzwy(), u64vec4(3_u64, 3_u64, 4_u64, 2_u64));
    assert_eq!(v.zzwz(), u64vec4(3_u64, 3_u64, 4_u64, 3_u64));
    assert_eq!(v.zzww(), u64vec4(3_u64, 3_u64, 4_u64, 4_u64));
    assert_eq!(v.zwxx(), u64vec4(3_u64, 4_u64, 1_u64, 1_u64));
    assert_eq!(v.zwxy(), u64vec4(3_u64, 4_u64, 1_u64, 2_u64));
    assert_eq!(v.zwxz(), u64vec4(3_u64, 4_u64, 1_u64, 3_u64));
    assert_eq!(v.zwxw(), u64vec4(3_u64, 4_u64, 1_u64, 4_u64));
    assert_eq!(v.zwyx(), u64vec4(3_u64, 4_u64, 2_u64, 1_u64));
    assert_eq!(v.zwyy(), u64vec4(3_u64, 4_u64, 2_u64, 2_u64));
    assert_eq!(v.zwyz(), u64vec4(3_u64, 4_u64, 2_u64, 3_u64));
    assert_eq!(v.zwyw(), u64vec4(3_u64, 4_u64, 2_u64, 4_u64));
    assert_eq!(v.zwzx(), u64vec4(3_u64, 4_u64, 3_u64, 1_u64));
    assert_eq!(v.zwzy(), u64vec4(3_u64, 4_u64, 3_u64, 2_u64));
    assert_eq!(v.zwzz(), u64vec4(3_u64, 4_u64, 3_u64, 3_u64));
    assert_eq!(v.zwzw(), u64vec4(3_u64, 4_u64, 3_u64, 4_u64));
    assert_eq!(v.zwwx(), u64vec4(3_u64, 4_u64, 4_u64, 1_u64));
    assert_eq!(v.zwwy(), u64vec4(3_u64, 4_u64, 4_u64, 2_u64));
    assert_eq!(v.zwwz(), u64vec4(3_u64, 4_u64, 4_u64, 3_u64));
    assert_eq!(v.zwww(), u64vec4(3_u64, 4_u64, 4_u64, 4_u64));
    assert_eq!(v.wxxx(), u64vec4(4_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.wxxy(), u64vec4(4_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.wxxz(), u64vec4(4_u64, 1_u64, 1_u64, 3_u64));
    assert_eq!(v.wxxw(), u64vec4(4_u64, 1_u64, 1_u64, 4_u64));
    assert_eq!(v.wxyx(), u64vec4(4_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.wxyy(), u64vec4(4_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.wxyz(), u64vec4(4_u64, 1_u64, 2_u64, 3_u64));
    assert_eq!(v.wxyw(), u64vec4(4_u64, 1_u64, 2_u64, 4_u64));
    assert_eq!(v.wxzx(), u64vec4(4_u64, 1_u64, 3_u64, 1_u64));
    assert_eq!(v.wxzy(), u64vec4(4_u64, 1_u64, 3_u64, 2_u64));
    assert_eq!(v.wxzz(), u64vec4(4_u64, 1_u64, 3_u64, 3_u64));
    assert_eq!(v.wxzw(), u64vec4(4_u64, 1_u64, 3_u64, 4_u64));
    assert_eq!(v.wxwx(), u64vec4(4_u64, 1_u64, 4_u64, 1_u64));
    assert_eq!(v.wxwy(), u64vec4(4_u64, 1_u64, 4_u64, 2_u64));
    assert_eq!(v.wxwz(), u64vec4(4_u64, 1_u64, 4_u64, 3_u64));
    assert_eq!(v.wxww(), u64vec4(4_u64, 1_u64, 4_u64, 4_u64));
    assert_eq!(v.wyxx(), u64vec4(4_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.wyxy(), u64vec4(4_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.wyxz(), u64vec4(4_u64, 2_u64, 1_u64, 3_u64));
    assert_eq!(v.wyxw(), u64vec4(4_u64, 2_u64, 1_u64, 4_u64));
    assert_eq!(v.wyyx(), u64vec4(4_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.wyyy(), u64vec4(4_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.wyyz(), u64vec4(4_u64, 2_u64, 2_u64, 3_u64));
    assert_eq!(v.wyyw(), u64vec4(4_u64, 2_u64, 2_u64, 4_u64));
    assert_eq!(v.wyzx(), u64vec4(4_u64, 2_u64, 3_u64, 1_u64));
    assert_eq!(v.wyzy(), u64vec4(4_u64, 2_u64, 3_u64, 2_u64));
    assert_eq!(v.wyzz(), u64vec4(4_u64, 2_u64, 3_u64, 3_u64));
    assert_eq!(v.wyzw(), u64vec4(4_u64, 2_u64, 3_u64, 4_u64));
    assert_eq!(v.wywx(), u64vec4(4_u64, 2_u64, 4_u64, 1_u64));
    assert_eq!(v.wywy(), u64vec4(4_u64, 2_u64, 4_u64, 2_u64));
    assert_eq!(v.wywz(), u64vec4(4_u64, 2_u64, 4_u64, 3_u64));
    assert_eq!(v.wyww(), u64vec4(4_u64, 2_u64, 4_u64, 4_u64));
    assert_eq!(v.wzxx(), u64vec4(4_u64, 3_u64, 1_u64, 1_u64));
    assert_eq!(v.wzxy(), u64vec4(4_u64, 3_u64, 1_u64, 2_u64));
    assert_eq!(v.wzxz(), u64vec4(4_u64, 3_u64, 1_u64, 3_u64));
    assert_eq!(v.wzxw(), u64vec4(4_u64, 3_u64, 1_u64, 4_u64));
    assert_eq!(v.wzyx(), u64vec4(4_u64, 3_u64, 2_u64, 1_u64));
    assert_eq!(v.wzyy(), u64vec4(4_u64, 3_u64, 2_u64, 2_u64));
    assert_eq!(v.wzyz(), u64vec4(4_u64, 3_u64, 2_u64, 3_u64));
    assert_eq!(v.wzyw(), u64vec4(4_u64, 3_u64, 2_u64, 4_u64));
    assert_eq!(v.wzzx(), u64vec4(4_u64, 3_u64, 3_u64, 1_u64));
    assert_eq!(v.wzzy(), u64vec4(4_u64, 3_u64, 3_u64, 2_u64));
    assert_eq!(v.wzzz(), u64vec4(4_u64, 3_u64, 3_u64, 3_u64));
    assert_eq!(v.wzzw(), u64vec4(4_u64, 3_u64, 3_u64, 4_u64));
    assert_eq!(v.wzwx(), u64vec4(4_u64, 3_u64, 4_u64, 1_u64));
    assert_eq!(v.wzwy(), u64vec4(4_u64, 3_u64, 4_u64, 2_u64));
    assert_eq!(v.wzwz(), u64vec4(4_u64, 3_u64, 4_u64, 3_u64));
    assert_eq!(v.wzww(), u64vec4(4_u64, 3_u64, 4_u64, 4_u64));
    assert_eq!(v.wwxx(), u64vec4(4_u64, 4_u64, 1_u64, 1_u64));
    assert_eq!(v.wwxy(), u64vec4(4_u64, 4_u64, 1_u64, 2_u64));
    assert_eq!(v.wwxz(), u64vec4(4_u64, 4_u64, 1_u64, 3_u64));
    assert_eq!(v.wwxw(), u64vec4(4_u64, 4_u64, 1_u64, 4_u64));
    assert_eq!(v.wwyx(), u64vec4(4_u64, 4_u64, 2_u64, 1_u64));
    assert_eq!(v.wwyy(), u64vec4(4_u64, 4_u64, 2_u64, 2_u64));
    assert_eq!(v.wwyz(), u64vec4(4_u64, 4_u64, 2_u64, 3_u64));
    assert_eq!(v.wwyw(), u64vec4(4_u64, 4_u64, 2_u64, 4_u64));
    assert_eq!(v.wwzx(), u64vec4(4_u64, 4_u64, 3_u64, 1_u64));
    assert_eq!(v.wwzy(), u64vec4(4_u64, 4_u64, 3_u64, 2_u64));
    assert_eq!(v.wwzz(), u64vec4(4_u64, 4_u64, 3_u64, 3_u64));
    assert_eq!(v.wwzw(), u64vec4(4_u64, 4_u64, 3_u64, 4_u64));
    assert_eq!(v.wwwx(), u64vec4(4_u64, 4_u64, 4_u64, 1_u64));
    assert_eq!(v.wwwy(), u64vec4(4_u64, 4_u64, 4_u64, 2_u64));
    assert_eq!(v.wwwz(), u64vec4(4_u64, 4_u64, 4_u64, 3_u64));
    assert_eq!(v.wwww(), u64vec4(4_u64, 4_u64, 4_u64, 4_u64));
    assert_eq!(v.xxx(), u64vec3(1_u64, 1_u64, 1_u64));
    assert_eq!(v.xxy(), u64vec3(1_u64, 1_u64, 2_u64));
    assert_eq!(v.xxz(), u64vec3(1_u64, 1_u64, 3_u64));
    assert_eq!(v.xxw(), u64vec3(1_u64, 1_u64, 4_u64));
    assert_eq!(v.xyx(), u64vec3(1_u64, 2_u64, 1_u64));
    assert_eq!(v.xyy(), u64vec3(1_u64, 2_u64, 2_u64));
    assert_eq!(v.xyz(), u64vec3(1_u64, 2_u64, 3_u64));
    assert_eq!(v.xyw(), u64vec3(1_u64, 2_u64, 4_u64));
    assert_eq!(v.xzx(), u64vec3(1_u64, 3_u64, 1_u64));
    assert_eq!(v.xzy(), u64vec3(1_u64, 3_u64, 2_u64));
    assert_eq!(v.xzz(), u64vec3(1_u64, 3_u64, 3_u64));
    assert_eq!(v.xzw(), u64vec3(1_u64, 3_u64, 4_u64));
    assert_eq!(v.xwx(), u64vec3(1_u64, 4_u64, 1_u64));
    assert_eq!(v.xwy(), u64vec3(1_u64, 4_u64, 2_u64));
    assert_eq!(v.xwz(), u64vec3(1_u64, 4_u64, 3_u64));
    assert_eq!(v.xww(), u64vec3(1_u64, 4_u64, 4_u64));
    assert_eq!(v.yxx(), u64vec3(2_u64, 1_u64, 1_u64));
    assert_eq!(v.yxy(), u64vec3(2_u64, 1_u64, 2_u64));
    assert_eq!(v.yxz(), u64vec3(2_u64, 1_u64, 3_u64));
    assert_eq!(v.yxw(), u64vec3(2_u64, 1_u64, 4_u64));
    assert_eq!(v.yyx(), u64vec3(2_u64, 2_u64, 1_u64));
    assert_eq!(v.yyy(), u64vec3(2_u64, 2_u64, 2_u64));
    assert_eq!(v.yyz(), u64vec3(2_u64, 2_u64, 3_u64));
    assert_eq!(v.yyw(), u64vec3(2_u64, 2_u64, 4_u64));
    assert_eq!(v.yzx(), u64vec3(2_u64, 3_u64, 1_u64));
    assert_eq!(v.yzy(), u64vec3(2_u64, 3_u64, 2_u64));
    assert_eq!(v.yzz(), u64vec3(2_u64, 3_u64, 3_u64));
    assert_eq!(v.yzw(), u64vec3(2_u64, 3_u64, 4_u64));
    assert_eq!(v.ywx(), u64vec3(2_u64, 4_u64, 1_u64));
    assert_eq!(v.ywy(), u64vec3(2_u64, 4_u64, 2_u64));
    assert_eq!(v.ywz(), u64vec3(2_u64, 4_u64, 3_u64));
    assert_eq!(v.yww(), u64vec3(2_u64, 4_u64, 4_u64));
    assert_eq!(v.zxx(), u64vec3(3_u64, 1_u64, 1_u64));
    assert_eq!(v.zxy(), u64vec3(3_u64, 1_u64, 2_u64));
    assert_eq!(v.zxz(), u64vec3(3_u64, 1_u64, 3_u64));
    assert_eq!(v.zxw(), u64vec3(3_u64, 1_u64, 4_u64));
    assert_eq!(v.zyx(), u64vec3(3_u64, 2_u64, 1_u64));
    assert_eq!(v.zyy(), u64vec3(3_u64, 2_u64, 2_u64));
    assert_eq!(v.zyz(), u64vec3(3_u64, 2_u64, 3_u64));
    assert_eq!(v.zyw(), u64vec3(3_u64, 2_u64, 4_u64));
    assert_eq!(v.zzx(), u64vec3(3_u64, 3_u64, 1_u64));
    assert_eq!(v.zzy(), u64vec3(3_u64, 3_u64, 2_u64));
    assert_eq!(v.zzz(), u64vec3(3_u64, 3_u64, 3_u64));
    assert_eq!(v.zzw(), u64vec3(3_u64, 3_u64, 4_u64));
    assert_eq!(v.zwx(), u64vec3(3_u64, 4_u64, 1_u64));
    assert_eq!(v.zwy(), u64vec3(3_u64, 4_u64, 2_u64));
    assert_eq!(v.zwz(), u64vec3(3_u64, 4_u64, 3_u64));
    assert_eq!(v.zww(), u64vec3(3_u64, 4_u64, 4_u64));
    assert_eq!(v.wxx(), u64vec3(4_u64, 1_u64, 1_u64));
    assert_eq!(v.wxy(), u64vec3(4_u64, 1_u64, 2_u64));
    assert_eq!(v.wxz(), u64vec3(4_u64, 1_u64, 3_u64));
    assert_eq!(v.wxw(), u64vec3(4_u64, 1_u64, 4_u64));
    assert_eq!(v.wyx(), u64vec3(4_u64, 2_u64, 1_u64));
    assert_eq!(v.wyy(), u64vec3(4_u64, 2_u64, 2_u64));
    assert_eq!(v.wyz(), u64vec3(4_u64, 2_u64, 3_u64));
    assert_eq!(v.wyw(), u64vec3(4_u64, 2_u64, 4_u64));
    assert_eq!(v.wzx(), u64vec3(4_u64, 3_u64, 1_u64));
    assert_eq!(v.wzy(), u64vec3(4_u64, 3_u64, 2_u64));
    assert_eq!(v.wzz(), u64vec3(4_u64, 3_u64, 3_u64));
    assert_eq!(v.wzw(), u64vec3(4_u64, 3_u64, 4_u64));
    assert_eq!(v.wwx(), u64vec3(4_u64, 4_u64, 1_u64));
    assert_eq!(v.wwy(), u64vec3(4_u64, 4_u64, 2_u64));
    assert_eq!(v.wwz(), u64vec3(4_u64, 4_u64, 3_u64));
    assert_eq!(v.www(), u64vec3(4_u64, 4_u64, 4_u64));
    assert_eq!(v.with_xyz(rhs3), u64vec4(11_u64, 12_u64, 13_u64, 4_u64));
    assert_eq!(v.with_xyw(rhs3), u64vec4(11_u64, 12_u64, 3_u64, 13_u64));
    assert_eq!(v.with_xzy(rhs3), u64vec4(11_u64, 13_u64, 12_u64, 4_u64));
    assert_eq!(v.with_xzw(rhs3), u64vec4(11_u64, 2_u64, 12_u64, 13_u64));
    assert_eq!(v.with_xwy(rhs3), u64vec4(11_u64, 13_u64, 3_u64, 12_u64));
    assert_eq!(v.with_xwz(rhs3), u64vec4(11_u64, 2_u64, 13_u64, 12_u64));
    assert_eq!(v.with_yxz(rhs3), u64vec4(12_u64, 11_u64, 13_u64, 4_u64));
    assert_eq!(v.with_yxw(rhs3), u64vec4(12_u64, 11_u64, 3_u64, 13_u64));
    assert_eq!(v.with_yzx(rhs3), u64vec4(13_u64, 11_u64, 12_u64, 4_u64));
    assert_eq!(v.with_yzw(rhs3), u64vec4(1_u64, 11_u64, 12_u64, 13_u64));
    assert_eq!(v.with_ywx(rhs3), u64vec4(13_u64, 11_u64, 3_u64, 12_u64));
    assert_eq!(v.with_ywz(rhs3), u64vec4(1_u64, 11_u64, 13_u64, 12_u64));
    assert_eq!(v.with_zxy(rhs3), u64vec4(12_u64, 13_u64, 11_u64, 4_u64));
    assert_eq!(v.with_zxw(rhs3), u64vec4(12_u64, 2_u64, 11_u64, 13_u64));
    assert_eq!(v.with_zyx(rhs3), u64vec4(13_u64, 12_u64, 11_u64, 4_u64));
    assert_eq!(v.with_zyw(rhs3), u64vec4(1_u64, 12_u64, 11_u64, 13_u64));
    assert_eq!(v.with_zwx(rhs3), u64vec4(13_u64, 2_u64, 11_u64, 12_u64));
    assert_eq!(v.with_zwy(rhs3), u64vec4(1_u64, 13_u64, 11_u64, 12_u64));
    assert_eq!(v.with_wxy(rhs3), u64vec4(12_u64, 13_u64, 3_u64, 11_u64));
    assert_eq!(v.with_wxz(rhs3), u64vec4(12_u64, 2_u64, 13_u64, 11_u64));
    assert_eq!(v.with_wyx(rhs3), u64vec4(13_u64, 12_u64, 3_u64, 11_u64));
    assert_eq!(v.with_wyz(rhs3), u64vec4(1_u64, 12_u64, 13_u64, 11_u64));
    assert_eq!(v.with_wzx(rhs3), u64vec4(13_u64, 2_u64, 12_u64, 11_u64));
    assert_eq!(v.with_wzy(rhs3), u64vec4(1_u64, 13_u64, 12_u64, 11_u64));
    assert_eq!(v.xx(), u64vec2(1_u64, 1_u64));
    assert_eq!(v.xy(), u64vec2(1_u64, 2_u64));
    assert_eq!(v.xz(), u64vec2(1_u64, 3_u64));
    assert_eq!(v.xw(), u64vec2(1_u64, 4_u64));
    assert_eq!(v.yx(), u64vec2(2_u64, 1_u64));
    assert_eq!(v.yy(), u64vec2(2_u64, 2_u64));
    assert_eq!(v.yz(), u64vec2(2_u64, 3_u64));
    assert_eq!(v.yw(), u64vec2(2_u64, 4_u64));
    assert_eq!(v.zx(), u64vec2(3_u64, 1_u64));
    assert_eq!(v.zy(), u64vec2(3_u64, 2_u64));
    assert_eq!(v.zz(), u64vec2(3_u64, 3_u64));
    assert_eq!(v.zw(), u64vec2(3_u64, 4_u64));
    assert_eq!(v.wx(), u64vec2(4_u64, 1_u64));
    assert_eq!(v.wy(), u64vec2(4_u64, 2_u64));
    assert_eq!(v.wz(), u64vec2(4_u64, 3_u64));
    assert_eq!(v.ww(), u64vec2(4_u64, 4_u64));
    assert_eq!(v.with_xy(rhs2), u64vec4(11_u64, 12_u64, 3_u64, 4_u64));
    assert_eq!(v.with_xz(rhs2), u64vec4(11_u64, 2_u64, 12_u64, 4_u64));
    assert_eq!(v.with_xw(rhs2), u64vec4(11_u64, 2_u64, 3_u64, 12_u64));
    assert_eq!(v.with_yx(rhs2), u64vec4(12_u64, 11_u64, 3_u64, 4_u64));
    assert_eq!(v.with_yz(rhs2), u64vec4(1_u64, 11_u64, 12_u64, 4_u64));
    assert_eq!(v.with_yw(rhs2), u64vec4(1_u64, 11_u64, 3_u64, 12_u64));
    assert_eq!(v.with_zx(rhs2), u64vec4(12_u64, 2_u64, 11_u64, 4_u64));
    assert_eq!(v.with_zy(rhs2), u64vec4(1_u64, 12_u64, 11_u64, 4_u64));
    assert_eq!(v.with_zw(rhs2), u64vec4(1_u64, 2_u64, 11_u64, 12_u64));
    assert_eq!(v.with_wx(rhs2), u64vec4(12_u64, 2_u64, 3_u64, 11_u64));
    assert_eq!(v.with_wy(rhs2), u64vec4(1_u64, 12_u64, 3_u64, 11_u64));
    assert_eq!(v.with_wz(rhs2), u64vec4(1_u64, 2_u64, 12_u64, 11_u64));
});

glam_test!(test_u64vec3_swizzles, {
    let v = u64vec3(1_u64, 2_u64, 3_u64);
    let rhs2 = u64vec2(11_u64, 12_u64);
    assert_eq!(v, v.xyz());
    assert_eq!(v.xxxx(), u64vec4(1_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.xxxy(), u64vec4(1_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.xxxz(), u64vec4(1_u64, 1_u64, 1_u64, 3_u64));
    assert_eq!(v.xxyx(), u64vec4(1_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.xxyy(), u64vec4(1_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.xxyz(), u64vec4(1_u64, 1_u64, 2_u64, 3_u64));
    assert_eq!(v.xxzx(), u64vec4(1_u64, 1_u64, 3_u64, 1_u64));
    assert_eq!(v.xxzy(), u64vec4(1_u64, 1_u64, 3_u64, 2_u64));
    assert_eq!(v.xxzz(), u64vec4(1_u64, 1_u64, 3_u64, 3_u64));
    assert_eq!(v.xyxx(), u64vec4(1_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.xyxy(), u64vec4(1_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.xyxz(), u64vec4(1_u64, 2_u64, 1_u64, 3_u64));
    assert_eq!(v.xyyx(), u64vec4(1_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.xyyy(), u64vec4(1_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.xyyz(), u64vec4(1_u64, 2_u64, 2_u64, 3_u64));
    assert_eq!(v.xyzx(), u64vec4(1_u64, 2_u64, 3_u64, 1_u64));
    assert_eq!(v.xyzy(), u64vec4(1_u64, 2_u64, 3_u64, 2_u64));
    assert_eq!(v.xyzz(), u64vec4(1_u64, 2_u64, 3_u64, 3_u64));
    assert_eq!(v.xzxx(), u64vec4(1_u64, 3_u64, 1_u64, 1_u64));
    assert_eq!(v.xzxy(), u64vec4(1_u64, 3_u64, 1_u64, 2_u64));
    assert_eq!(v.xzxz(), u64vec4(1_u64, 3_u64, 1_u64, 3_u64));
    assert_eq!(v.xzyx(), u64vec4(1_u64, 3_u64, 2_u64, 1_u64));
    assert_eq!(v.xzyy(), u64vec4(1_u64, 3_u64, 2_u64, 2_u64));
    assert_eq!(v.xzyz(), u64vec4(1_u64, 3_u64, 2_u64, 3_u64));
    assert_eq!(v.xzzx(), u64vec4(1_u64, 3_u64, 3_u64, 1_u64));
    assert_eq!(v.xzzy(), u64vec4(1_u64, 3_u64, 3_u64, 2_u64));
    assert_eq!(v.xzzz(), u64vec4(1_u64, 3_u64, 3_u64, 3_u64));
    assert_eq!(v.yxxx(), u64vec4(2_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.yxxy(), u64vec4(2_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.yxxz(), u64vec4(2_u64, 1_u64, 1_u64, 3_u64));
    assert_eq!(v.yxyx(), u64vec4(2_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.yxyy(), u64vec4(2_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.yxyz(), u64vec4(2_u64, 1_u64, 2_u64, 3_u64));
    assert_eq!(v.yxzx(), u64vec4(2_u64, 1_u64, 3_u64, 1_u64));
    assert_eq!(v.yxzy(), u64vec4(2_u64, 1_u64, 3_u64, 2_u64));
    assert_eq!(v.yxzz(), u64vec4(2_u64, 1_u64, 3_u64, 3_u64));
    assert_eq!(v.yyxx(), u64vec4(2_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.yyxy(), u64vec4(2_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.yyxz(), u64vec4(2_u64, 2_u64, 1_u64, 3_u64));
    assert_eq!(v.yyyx(), u64vec4(2_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.yyyy(), u64vec4(2_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.yyyz(), u64vec4(2_u64, 2_u64, 2_u64, 3_u64));
    assert_eq!(v.yyzx(), u64vec4(2_u64, 2_u64, 3_u64, 1_u64));
    assert_eq!(v.yyzy(), u64vec4(2_u64, 2_u64, 3_u64, 2_u64));
    assert_eq!(v.yyzz(), u64vec4(2_u64, 2_u64, 3_u64, 3_u64));
    assert_eq!(v.yzxx(), u64vec4(2_u64, 3_u64, 1_u64, 1_u64));
    assert_eq!(v.yzxy(), u64vec4(2_u64, 3_u64, 1_u64, 2_u64));
    assert_eq!(v.yzxz(), u64vec4(2_u64, 3_u64, 1_u64, 3_u64));
    assert_eq!(v.yzyx(), u64vec4(2_u64, 3_u64, 2_u64, 1_u64));
    assert_eq!(v.yzyy(), u64vec4(2_u64, 3_u64, 2_u64, 2_u64));
    assert_eq!(v.yzyz(), u64vec4(2_u64, 3_u64, 2_u64, 3_u64));
    assert_eq!(v.yzzx(), u64vec4(2_u64, 3_u64, 3_u64, 1_u64));
    assert_eq!(v.yzzy(), u64vec4(2_u64, 3_u64, 3_u64, 2_u64));
    assert_eq!(v.yzzz(), u64vec4(2_u64, 3_u64, 3_u64, 3_u64));
    assert_eq!(v.zxxx(), u64vec4(3_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.zxxy(), u64vec4(3_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.zxxz(), u64vec4(3_u64, 1_u64, 1_u64, 3_u64));
    assert_eq!(v.zxyx(), u64vec4(3_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.zxyy(), u64vec4(3_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.zxyz(), u64vec4(3_u64, 1_u64, 2_u64, 3_u64));
    assert_eq!(v.zxzx(), u64vec4(3_u64, 1_u64, 3_u64, 1_u64));
    assert_eq!(v.zxzy(), u64vec4(3_u64, 1_u64, 3_u64, 2_u64));
    assert_eq!(v.zxzz(), u64vec4(3_u64, 1_u64, 3_u64, 3_u64));
    assert_eq!(v.zyxx(), u64vec4(3_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.zyxy(), u64vec4(3_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.zyxz(), u64vec4(3_u64, 2_u64, 1_u64, 3_u64));
    assert_eq!(v.zyyx(), u64vec4(3_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.zyyy(), u64vec4(3_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.zyyz(), u64vec4(3_u64, 2_u64, 2_u64, 3_u64));
    assert_eq!(v.zyzx(), u64vec4(3_u64, 2_u64, 3_u64, 1_u64));
    assert_eq!(v.zyzy(), u64vec4(3_u64, 2_u64, 3_u64, 2_u64));
    assert_eq!(v.zyzz(), u64vec4(3_u64, 2_u64, 3_u64, 3_u64));
    assert_eq!(v.zzxx(), u64vec4(3_u64, 3_u64, 1_u64, 1_u64));
    assert_eq!(v.zzxy(), u64vec4(3_u64, 3_u64, 1_u64, 2_u64));
    assert_eq!(v.zzxz(), u64vec4(3_u64, 3_u64, 1_u64, 3_u64));
    assert_eq!(v.zzyx(), u64vec4(3_u64, 3_u64, 2_u64, 1_u64));
    assert_eq!(v.zzyy(), u64vec4(3_u64, 3_u64, 2_u64, 2_u64));
    assert_eq!(v.zzyz(), u64vec4(3_u64, 3_u64, 2_u64, 3_u64));
    assert_eq!(v.zzzx(), u64vec4(3_u64, 3_u64, 3_u64, 1_u64));
    assert_eq!(v.zzzy(), u64vec4(3_u64, 3_u64, 3_u64, 2_u64));
    assert_eq!(v.zzzz(), u64vec4(3_u64, 3_u64, 3_u64, 3_u64));
    assert_eq!(v.xxx(), u64vec3(1_u64, 1_u64, 1_u64));
    assert_eq!(v.xxy(), u64vec3(1_u64, 1_u64, 2_u64));
    assert_eq!(v.xxz(), u64vec3(1_u64, 1_u64, 3_u64));
    assert_eq!(v.xyx(), u64vec3(1_u64, 2_u64, 1_u64));
    assert_eq!(v.xyy(), u64vec3(1_u64, 2_u64, 2_u64));
    assert_eq!(v.xzx(), u64vec3(1_u64, 3_u64, 1_u64));
    assert_eq!(v.xzy(), u64vec3(1_u64, 3_u64, 2_u64));
    assert_eq!(v.xzz(), u64vec3(1_u64, 3_u64, 3_u64));
    assert_eq!(v.yxx(), u64vec3(2_u64, 1_u64, 1_u64));
    assert_eq!(v.yxy(), u64vec3(2_u64, 1_u64, 2_u64));
    assert_eq!(v.yxz(), u64vec3(2_u64, 1_u64, 3_u64));
    assert_eq!(v.yyx(), u64vec3(2_u64, 2_u64, 1_u64));
    assert_eq!(v.yyy(), u64vec3(2_u64, 2_u64, 2_u64));
    assert_eq!(v.yyz(), u64vec3(2_u64, 2_u64, 3_u64));
    assert_eq!(v.yzx(), u64vec3(2_u64, 3_u64, 1_u64));
    assert_eq!(v.yzy(), u64vec3(2_u64, 3_u64, 2_u64));
    assert_eq!(v.yzz(), u64vec3(2_u64, 3_u64, 3_u64));
    assert_eq!(v.zxx(), u64vec3(3_u64, 1_u64, 1_u64));
    assert_eq!(v.zxy(), u64vec3(3_u64, 1_u64, 2_u64));
    assert_eq!(v.zxz(), u64vec3(3_u64, 1_u64, 3_u64));
    assert_eq!(v.zyx(), u64vec3(3_u64, 2_u64, 1_u64));
    assert_eq!(v.zyy(), u64vec3(3_u64, 2_u64, 2_u64));
    assert_eq!(v.zyz(), u64vec3(3_u64, 2_u64, 3_u64));
    assert_eq!(v.zzx(), u64vec3(3_u64, 3_u64, 1_u64));
    assert_eq!(v.zzy(), u64vec3(3_u64, 3_u64, 2_u64));
    assert_eq!(v.zzz(), u64vec3(3_u64, 3_u64, 3_u64));
    assert_eq!(v.xx(), u64vec2(1_u64, 1_u64));
    assert_eq!(v.xy(), u64vec2(1_u64, 2_u64));
    assert_eq!(v.xz(), u64vec2(1_u64, 3_u64));
    assert_eq!(v.yx(), u64vec2(2_u64, 1_u64));
    assert_eq!(v.yy(), u64vec2(2_u64, 2_u64));
    assert_eq!(v.yz(), u64vec2(2_u64, 3_u64));
    assert_eq!(v.zx(), u64vec2(3_u64, 1_u64));
    assert_eq!(v.zy(), u64vec2(3_u64, 2_u64));
    assert_eq!(v.zz(), u64vec2(3_u64, 3_u64));
    assert_eq!(v.with_xy(rhs2), u64vec3(11_u64, 12_u64, 3_u64));
    assert_eq!(v.with_xz(rhs2), u64vec3(11_u64, 2_u64, 12_u64));
    assert_eq!(v.with_yx(rhs2), u64vec3(12_u64, 11_u64, 3_u64));
    assert_eq!(v.with_yz(rhs2), u64vec3(1_u64, 11_u64, 12_u64));
    assert_eq!(v.with_zx(rhs2), u64vec3(12_u64, 2_u64, 11_u64));
    assert_eq!(v.with_zy(rhs2), u64vec3(1_u64, 12_u64, 11_u64));
});

glam_test!(test_u64vec2_swizzles, {
    let v = u64vec2(1_u64, 2_u64);
    assert_eq!(v, v.xy());
    assert_eq!(v.xxxx(), u64vec4(1_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.xxxy(), u64vec4(1_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.xxyx(), u64vec4(1_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.xxyy(), u64vec4(1_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.xyxx(), u64vec4(1_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.xyxy(), u64vec4(1_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.xyyx(), u64vec4(1_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.xyyy(), u64vec4(1_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.yxxx(), u64vec4(2_u64, 1_u64, 1_u64, 1_u64));
    assert_eq!(v.yxxy(), u64vec4(2_u64, 1_u64, 1_u64, 2_u64));
    assert_eq!(v.yxyx(), u64vec4(2_u64, 1_u64, 2_u64, 1_u64));
    assert_eq!(v.yxyy(), u64vec4(2_u64, 1_u64, 2_u64, 2_u64));
    assert_eq!(v.yyxx(), u64vec4(2_u64, 2_u64, 1_u64, 1_u64));
    assert_eq!(v.yyxy(), u64vec4(2_u64, 2_u64, 1_u64, 2_u64));
    assert_eq!(v.yyyx(), u64vec4(2_u64, 2_u64, 2_u64, 1_u64));
    assert_eq!(v.yyyy(), u64vec4(2_u64, 2_u64, 2_u64, 2_u64));
    assert_eq!(v.xxx(), u64vec3(1_u64, 1_u64, 1_u64));
    assert_eq!(v.xxy(), u64vec3(1_u64, 1_u64, 2_u64));
    assert_eq!(v.xyx(), u64vec3(1_u64, 2_u64, 1_u64));
    assert_eq!(v.xyy(), u64vec3(1_u64, 2_u64, 2_u64));
    assert_eq!(v.yxx(), u64vec3(2_u64, 1_u64, 1_u64));
    assert_eq!(v.yxy(), u64vec3(2_u64, 1_u64, 2_u64));
    assert_eq!(v.yyx(), u64vec3(2_u64, 2_u64, 1_u64));
    assert_eq!(v.yyy(), u64vec3(2_u64, 2_u64, 2_u64));
    assert_eq!(v.xx(), u64vec2(1_u64, 1_u64));
    assert_eq!(v.yx(), u64vec2(2_u64, 1_u64));
    assert_eq!(v.yy(), u64vec2(2_u64, 2_u64));
});
