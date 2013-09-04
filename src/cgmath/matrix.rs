// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Column major, square matrix types and traits.

use std::num::{Zero, zero, One, one, sin, cos};

use array::*;
use quaternion::{Quat, ToQuat};
use vector::*;
use util::half;

/// A 2 x 2, column major matrix
#[deriving(Clone, Eq, Zero)]
pub struct Mat2<S> { x: Vec2<S>, y: Vec2<S> }

/// A 3 x 3, column major matrix
#[deriving(Clone, Eq, Zero)]
pub struct Mat3<S> { x: Vec3<S>, y: Vec3<S>, z: Vec3<S> }

/// A 4 x 4, column major matrix
#[deriving(Clone, Eq, Zero)]
pub struct Mat4<S> { x: Vec4<S>, y: Vec4<S>, z: Vec4<S>, w: Vec4<S> }

approx_eq!(impl<S> Mat2<S>)
approx_eq!(impl<S> Mat3<S>)
approx_eq!(impl<S> Mat4<S>)

// Conversion traits
pub trait ToMat2<S: Clone + Num> { fn to_mat2(&self) -> Mat2<S>; }
pub trait ToMat3<S: Clone + Num> { fn to_mat3(&self) -> Mat3<S>; }
pub trait ToMat4<S: Clone + Num> { fn to_mat4(&self) -> Mat4<S>; }

impl<S: Clone + Num> Mat2<S> {
    #[inline]
    pub fn new(c0r0: S, c0r1: S,
               c1r0: S, c1r1: S) -> Mat2<S> {
        Mat2::from_cols(Vec2::new(c0r0, c0r1),
                        Vec2::new(c1r0, c1r1))
    }

    #[inline]
    pub fn from_cols(c0: Vec2<S>, c1: Vec2<S>) -> Mat2<S> {
        Mat2 { x: c0, y: c1 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat2<S> {
        Mat2::new(value.clone(), zero(),
                  zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat2<S> {
        Mat2::from_value(zero())
    }

    #[inline]
    pub fn ident() -> Mat2<S> {
        Mat2::from_value(one())
    }
}

impl<S: Clone + Float> Mat2<S> {
    #[inline]
    pub fn from_angle(radians: S) -> Mat2<S> {
        let cos_theta = cos(radians.clone());
        let sin_theta = sin(radians.clone());

        Mat2::new(cos_theta.clone(),  -sin_theta.clone(),
                  sin_theta.clone(),  cos_theta.clone())
    }
}

impl<S: Clone + Num> Mat3<S> {
    #[inline]
    pub fn new(c0r0:S, c0r1:S, c0r2:S,
               c1r0:S, c1r1:S, c1r2:S,
               c2r0:S, c2r1:S, c2r2:S) -> Mat3<S> {
        Mat3::from_cols(Vec3::new(c0r0, c0r1, c0r2),
                        Vec3::new(c1r0, c1r1, c1r2),
                        Vec3::new(c2r0, c2r1, c2r2))
    }

    #[inline]
    pub fn from_cols(c0: Vec3<S>, c1: Vec3<S>, c2: Vec3<S>) -> Mat3<S> {
        Mat3 { x: c0, y: c1, z: c2 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat3<S> {
        Mat3::new(value.clone(), zero(), zero(),
                  zero(), value.clone(), zero(),
                  zero(), zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat3<S> {
        Mat3::from_value(zero())
    }

    #[inline]
    pub fn ident() -> Mat3<S> {
        Mat3::from_value(one())
    }
}

impl<S: Clone + Float> Mat3<S> {
    pub fn look_at(dir: &Vec3<S>, up: &Vec3<S>) -> Mat3<S> {
        let dir  = dir.normalize();
        let side = dir.cross(&up.normalize());
        let up   = side.cross(&dir).normalize();

        Mat3::from_cols(up, side, dir)
    }
}

impl<S: Clone + Num> Mat4<S> {
    #[inline]
    pub fn new(c0r0: S, c0r1: S, c0r2: S, c0r3: S,
               c1r0: S, c1r1: S, c1r2: S, c1r3: S,
               c2r0: S, c2r1: S, c2r2: S, c2r3: S,
               c3r0: S, c3r1: S, c3r2: S, c3r3: S) -> Mat4<S>  {
        Mat4::from_cols(Vec4::new(c0r0, c0r1, c0r2, c0r3),
                        Vec4::new(c1r0, c1r1, c1r2, c1r3),
                        Vec4::new(c2r0, c2r1, c2r2, c2r3),
                        Vec4::new(c3r0, c3r1, c3r2, c3r3))
    }

    #[inline]
    pub fn from_cols(c0: Vec4<S>, c1: Vec4<S>, c2: Vec4<S>, c3: Vec4<S>) -> Mat4<S> {
        Mat4 { x: c0, y: c1, z: c2, w: c3 }
    }

    #[inline]
    pub fn from_value(value: S) -> Mat4<S> {
        Mat4::new(value.clone(), zero(), zero(), zero(),
                  zero(), value.clone(), zero(), zero(),
                  zero(), zero(), value.clone(), zero(),
                  zero(), zero(), zero(), value.clone())
    }

    #[inline]
    pub fn zero() -> Mat4<S> {
        Mat4::from_value(zero())
    }

    #[inline]
    pub fn ident() -> Mat4<S> {
        Mat4::from_value(one())
    }
}

impl<S: Clone + Float> One for Mat2<S> { #[inline] fn one() -> Mat2<S> { Mat2::ident() } }
impl<S: Clone + Float> One for Mat3<S> { #[inline] fn one() -> Mat3<S> { Mat3::ident() } }
impl<S: Clone + Float> One for Mat4<S> { #[inline] fn one() -> Mat4<S> { Mat4::ident() } }

array!(impl<S> Mat2<S> -> [Vec2<S>, ..2])
array!(impl<S> Mat3<S> -> [Vec3<S>, ..3])
array!(impl<S> Mat4<S> -> [Vec4<S>, ..4])

pub trait Matrix
<
    S: Clone + Float, Slice,
    V: Clone + Vector<S, VSlice> + Array<S, VSlice>, VSlice
>
:   Array<V, Slice>
+   Neg<Self>
+   Zero + One
+   ApproxEq<S>
{
    #[inline]
    fn c<'a>(&'a self, c: uint) -> &'a V { self.i(c) }

    #[inline]
    fn mut_c<'a>(&'a mut self, c: uint) -> &'a mut V { self.mut_i(c) }

    #[inline]
    fn swap_c(&mut self, a: uint, b: uint) {
        let tmp = self.c(a).clone();
        *self.mut_c(a) = self.c(b).clone();
        *self.mut_c(b) = tmp;
    }

    fn r(&self, r: uint) -> V;

    #[inline]
    fn swap_r(&mut self, a: uint, b: uint) {
        for c in self.mut_iter() { c.swap(a, b) }
    }

    #[inline]
    fn cr<'a>(&'a self, c: uint, r: uint) -> &'a S { self.i(c).i(r) }

    #[inline]
    fn mut_cr<'a>(&'a mut self, c: uint, r: uint) -> &'a mut S {
        self.mut_i(c).mut_i(r)
    }

    #[inline]
    fn swap_cr(&mut self, a: (uint, uint), b: (uint, uint)) {
        let (ca, ra) = a;
        let (cb, rb) = b;
        let tmp = self.cr(ca, ra).clone();
        *self.mut_cr(ca, ra) = self.cr(cb, rb).clone();
        *self.mut_cr(cb, rb) = tmp;
    }

    // fn swap_cr(&mut self, (ca, ra): (uint, uint), (cb, rb): (uint, uint)) {
    //     let tmp = self.cr(ca, ra).clone();
    //     *self.mut_cr(ca, ra) = self.cr(cb, rb).clone();
    //     *self.mut_cr(cb, rb) = tmp;
    // }

    #[inline] fn neg_self(&mut self) { for c in self.mut_iter() { *c = c.neg() } }

    #[inline] fn mul_s(&self, s: S) -> Self { self.map(|c| c.mul_s(s.clone())) }
    #[inline] fn div_s(&self, s: S) -> Self { self.map(|c| c.div_s(s.clone())) }
    #[inline] fn rem_s(&self, s: S) -> Self { self.map(|c| c.rem_s(s.clone())) }

    #[inline] fn mul_self_s(&mut self, s: S) { for c in self.mut_iter() { *c = c.mul_s(s.clone()) } }
    #[inline] fn div_self_s(&mut self, s: S) { for c in self.mut_iter() { *c = c.div_s(s.clone()) } }
    #[inline] fn rem_self_s(&mut self, s: S) { for c in self.mut_iter() { *c = c.rem_s(s.clone()) } }

    fn mul_v(&self, v: &V) -> V;

    #[inline] fn add_m(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.add_v(b) ) }
    #[inline] fn sub_m(&self, other: &Self) -> Self { self.bimap(other, |a, b| a.sub_v(b) ) }

    #[inline] fn add_self_m(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.add_v(b) } }
    #[inline] fn sub_self_m(&mut self, other: &Self) { for (a, b) in self.mut_iter().zip(other.iter()) { *a = a.sub_v(b) } }

    fn mul_m(&self, other: &Self) -> Self;

    fn transpose(&self) -> Self;
    fn transpose_self(&mut self);
    fn trace(&self) -> S;
    fn determinant(&self) -> S;
    fn invert(&self) -> Option<Self>;

    #[inline]
    fn invert_self(&mut self) {
        *self = self.invert().expect("Attempted to invert a matrix with zero determinant.");
    }

    #[inline]
    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&zero())
    }

    #[inline]
    fn is_identity(&self) -> bool {
        self.approx_eq(&one())
    }

    #[inline]
    fn is_rotated(&self) -> bool {
        !self.approx_eq(&one())
    }

    fn is_diagonal(&self) -> bool;
    fn is_symmetric(&self) -> bool;
}

impl<S: Clone + Float> Neg<Mat2<S>> for Mat2<S> { #[inline] fn neg(&self) -> Mat2<S> { self.map(|c| c.neg()) } }
impl<S: Clone + Float> Neg<Mat3<S>> for Mat3<S> { #[inline] fn neg(&self) -> Mat3<S> { self.map(|c| c.neg()) } }
impl<S: Clone + Float> Neg<Mat4<S>> for Mat4<S> { #[inline] fn neg(&self) -> Mat4<S> { self.map(|c| c.neg()) } }

impl<S: Clone + Float>
Matrix<S, [Vec2<S>, ..2], Vec2<S>, [S, ..2]>
for Mat2<S>
{
    #[inline]
    fn r(&self, r: uint) -> Vec2<S> {
        Vec2::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone())
    }

    fn mul_v(&self, v: &Vec2<S>) -> Vec2<S> {
        Vec2::new(self.r(0).dot(v),
                  self.r(1).dot(v))
    }

    fn mul_m(&self, other: &Mat2<S>) -> Mat2<S> {
        Mat2::new(self.r(0).dot(other.c(0)), self.r(1).dot(other.c(0)),
                  self.r(0).dot(other.c(1)), self.r(1).dot(other.c(1)))
    }

    fn transpose(&self) -> Mat2<S> {
        Mat2::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone())
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
    }

    #[inline]
    fn trace(&self) -> S {
        *self.cr(0, 0) + *self.cr(1, 1)
    }

    #[inline]
    fn determinant(&self) -> S {
        *self.cr(0, 0) * *self.cr(1, 1) - *self.cr(1, 0) * *self.cr(0, 1)
    }

    #[inline]
    fn invert(&self) -> Option<Mat2<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) {
            None
        } else {
            Some(Mat2::new( self.cr(1, 1) / det, -self.cr(0, 1) / det,
                           -self.cr(1, 0) / det,  self.cr(0, 0) / det))
        }
    }

    #[inline]
    fn is_diagonal(&self) -> bool {
        self.cr(0, 1).approx_eq(&zero()) &&
        self.cr(1, 0).approx_eq(&zero())
    }


    #[inline]
    fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(1, 0).approx_eq(self.cr(0, 1))
    }
}

impl<S: Clone + Float>
Matrix<S, [Vec3<S>, ..3], Vec3<S>, [S, ..3]>
for Mat3<S>
{
    #[inline]
    fn r(&self, r: uint) -> Vec3<S> {
        Vec3::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone(),
                  self.i(2).i(r).clone())
    }

    fn mul_v(&self, v: &Vec3<S>) -> Vec3<S> {
        Vec3::new(self.r(0).dot(v),
                  self.r(1).dot(v),
                  self.r(2).dot(v))
    }

    fn mul_m(&self, other: &Mat3<S>) -> Mat3<S> {
        Mat3::new(self.r(0).dot(other.c(0)),self.r(1).dot(other.c(0)),self.r(2).dot(other.c(0)),
                  self.r(0).dot(other.c(1)),self.r(1).dot(other.c(1)),self.r(2).dot(other.c(1)),
                  self.r(0).dot(other.c(2)),self.r(1).dot(other.c(2)),self.r(2).dot(other.c(2)))
    }

    fn transpose(&self) -> Mat3<S> {
        Mat3::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                  self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone())
    }

    #[inline]
    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((1, 2), (2, 1));
    }

    #[inline]
    fn trace(&self) -> S {
        *self.cr(0, 0) + *self.cr(1, 1) + *self.cr(2, 2)
    }

    fn determinant(&self) -> S {
        *self.cr(0, 0) * (*self.cr(1, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(1, 2)) -
        *self.cr(1, 0) * (*self.cr(0, 1) * *self.cr(2, 2) - *self.cr(2, 1) * *self.cr(0, 2)) +
        *self.cr(2, 0) * (*self.cr(0, 1) * *self.cr(1, 2) - *self.cr(1, 1) * *self.cr(0, 2))
    }

    fn invert(&self) -> Option<Mat3<S>> {
        let det = self.determinant();
        if det.approx_eq(&zero()) { None } else {
            Some(Mat3::from_cols(self.c(1).cross(self.c(2)).div_s(det.clone()),
                                 self.c(2).cross(self.c(0)).div_s(det.clone()),
                                 self.c(0).cross(self.c(1)).div_s(det.clone())).transpose())
        }
    }

    fn is_diagonal(&self) -> bool {
        self.cr(0, 1).approx_eq(&zero()) &&
        self.cr(0, 2).approx_eq(&zero()) &&

        self.cr(1, 0).approx_eq(&zero()) &&
        self.cr(1, 2).approx_eq(&zero()) &&

        self.cr(2, 0).approx_eq(&zero()) &&
        self.cr(2, 1).approx_eq(&zero())
    }

    fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(0, 2).approx_eq(self.cr(2, 0)) &&

        self.cr(1, 0).approx_eq(self.cr(0, 1)) &&
        self.cr(1, 2).approx_eq(self.cr(2, 1)) &&

        self.cr(2, 0).approx_eq(self.cr(0, 2)) &&
        self.cr(2, 1).approx_eq(self.cr(1, 2))
    }
}

impl<S: Clone + Float>
Matrix<S, [Vec4<S>, ..4], Vec4<S>, [S, ..4]>
for Mat4<S>
{
    #[inline]
    fn r(&self, r: uint) -> Vec4<S> {
        Vec4::new(self.i(0).i(r).clone(),
                  self.i(1).i(r).clone(),
                  self.i(2).i(r).clone(),
                  self.i(2).i(r).clone())
    }

    fn mul_v(&self, v: &Vec4<S>) -> Vec4<S> {
        Vec4::new(self.r(0).dot(v),
                  self.r(1).dot(v),
                  self.r(2).dot(v),
                  self.r(3).dot(v))
    }

    fn mul_m(&self, other: &Mat4<S>) -> Mat4<S> {
        Mat4::new(self.r(0).dot(other.c(0)), self.r(1).dot(other.c(0)), self.r(2).dot(other.c(0)), self.r(3).dot(other.c(0)),
                  self.r(0).dot(other.c(1)), self.r(1).dot(other.c(1)), self.r(2).dot(other.c(1)), self.r(3).dot(other.c(1)),
                  self.r(0).dot(other.c(2)), self.r(1).dot(other.c(2)), self.r(2).dot(other.c(2)), self.r(3).dot(other.c(2)),
                  self.r(0).dot(other.c(3)), self.r(1).dot(other.c(3)), self.r(2).dot(other.c(3)), self.r(3).dot(other.c(3)))
    }

    fn transpose(&self) -> Mat4<S> {
        Mat4::new(self.cr(0, 0).clone(), self.cr(1, 0).clone(), self.cr(2, 0).clone(), self.cr(3, 0).clone(),
                  self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                  self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                  self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone())
    }

    fn transpose_self(&mut self) {
        self.swap_cr((0, 1), (1, 0));
        self.swap_cr((0, 2), (2, 0));
        self.swap_cr((0, 3), (3, 0));
        self.swap_cr((1, 2), (2, 1));
        self.swap_cr((1, 3), (3, 1));
        self.swap_cr((2, 3), (3, 2));
    }

    #[inline]
    fn trace(&self) -> S {
        *self.cr(0, 0) + *self.cr(1, 1) + *self.cr(2, 2) + *self.cr(3, 3)
    }

    fn determinant(&self) -> S {
        let m0 = Mat3::new(self.cr(1, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(1, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(1, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m1 = Mat3::new(self.cr(0, 1).clone(), self.cr(2, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(2, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(2, 3).clone(), self.cr(3, 3).clone());
        let m2 = Mat3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(3, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(3, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(3, 3).clone());
        let m3 = Mat3::new(self.cr(0, 1).clone(), self.cr(1, 1).clone(), self.cr(2, 1).clone(),
                           self.cr(0, 2).clone(), self.cr(1, 2).clone(), self.cr(2, 2).clone(),
                           self.cr(0, 3).clone(), self.cr(1, 3).clone(), self.cr(2, 3).clone());

        self.cr(0, 0) * m0.determinant() -
        self.cr(1, 0) * m1.determinant() +
        self.cr(2, 0) * m2.determinant() -
        self.cr(3, 0) * m3.determinant()
    }

    fn invert(&self) -> Option<Mat4<S>> {
        if self.is_invertible() {
            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]

            let mut A = self.clone();
            let mut I = Mat4::ident();

            for j in range(0u, 4u) {
                // Find largest element in col j
                let mut i1 = j;
                for i in range(j + 1, 4) {
                    if A.cr(j, i).abs() > A.cr(j, i1).abs() {
                        i1 = i;
                    }
                }

                // Swap columns i1 and j in A and I to
                // put pivot on diagonal
                A.swap_c(i1, j);
                I.swap_c(i1, j);

                // Scale col j to have a unit diagonal
                *I.mut_c(j) = I.c(j).div_s(A.cr(j, j).clone());
                *A.mut_c(j) = A.c(j).div_s(A.cr(j, j).clone());

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for i in range(0u, 4u) {
                    if i != j {
                        let ij_mul_aij = I.c(j).mul_s(A.cr(i, j).clone());
                        let aj_mul_aij = A.c(j).mul_s(A.cr(i, j).clone());
                        *I.mut_c(i) = I.c(i).sub_v(&ij_mul_aij);
                        *A.mut_c(i) = A.c(i).sub_v(&aj_mul_aij);
                    }
                }
            }
            Some(I)
        } else {
            None
        }
    }

    fn is_diagonal(&self) -> bool {
        self.cr(0, 1).approx_eq(&zero()) &&
        self.cr(0, 2).approx_eq(&zero()) &&
        self.cr(0, 3).approx_eq(&zero()) &&

        self.cr(1, 0).approx_eq(&zero()) &&
        self.cr(1, 2).approx_eq(&zero()) &&
        self.cr(1, 3).approx_eq(&zero()) &&

        self.cr(2, 0).approx_eq(&zero()) &&
        self.cr(2, 1).approx_eq(&zero()) &&
        self.cr(2, 3).approx_eq(&zero()) &&

        self.cr(3, 0).approx_eq(&zero()) &&
        self.cr(3, 1).approx_eq(&zero()) &&
        self.cr(3, 2).approx_eq(&zero())
    }

    fn is_symmetric(&self) -> bool {
        self.cr(0, 1).approx_eq(self.cr(1, 0)) &&
        self.cr(0, 2).approx_eq(self.cr(2, 0)) &&
        self.cr(0, 3).approx_eq(self.cr(3, 0)) &&

        self.cr(1, 0).approx_eq(self.cr(0, 1)) &&
        self.cr(1, 2).approx_eq(self.cr(2, 1)) &&
        self.cr(1, 3).approx_eq(self.cr(3, 1)) &&

        self.cr(2, 0).approx_eq(self.cr(0, 2)) &&
        self.cr(2, 1).approx_eq(self.cr(1, 2)) &&
        self.cr(2, 3).approx_eq(self.cr(3, 2)) &&

        self.cr(3, 0).approx_eq(self.cr(0, 3)) &&
        self.cr(3, 1).approx_eq(self.cr(1, 3)) &&
        self.cr(3, 2).approx_eq(self.cr(2, 3))
    }
}

impl<S:Clone + Float> ToQuat<S> for Mat3<S> {
    /// Convert the matrix to a quaternion
    fn to_quat(&self) -> Quat<S> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf

        let mut s;
        let w; let x; let y; let z;
        let trace = self.trace();

        cond! (
            (trace >= zero::<S>()) {
                s = (one::<S>() + trace).sqrt();
                w = half::<S>() * s;
                s = half::<S>() / s;
                x = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
            }
            ((*self.cr(0, 0) > *self.cr(1, 1))
            && (*self.cr(0, 0) > *self.cr(2, 2))) {
                s = (half::<S>() + (*self.cr(0, 0) - *self.cr(1, 1) - *self.cr(2, 2))).sqrt();
                w = half::<S>() * s;
                s = half::<S>() / s;
                x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                y = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                z = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
            }
            (*self.cr(1, 1) > *self.cr(2, 2)) {
                s = (half::<S>() + (*self.cr(1, 1) - *self.cr(0, 0) - *self.cr(2, 2))).sqrt();
                w = half::<S>() * s;
                s = half::<S>() / s;
                x = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
                y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                z = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
            }
            _ {
                s = (half::<S>() + (*self.cr(2, 2) - *self.cr(0, 0) - *self.cr(1, 1))).sqrt();
                w = half::<S>() * s;
                s = half::<S>() / s;
                x = (*self.cr(2, 0) - *self.cr(0, 2)) * s;
                y = (*self.cr(1, 2) - *self.cr(2, 1)) * s;
                z = (*self.cr(0, 1) - *self.cr(1, 0)) * s;
            }
        )
        Quat::new(w, x, y, z)
    }
}
