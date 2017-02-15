//! The default matrix data storage allocator.
//!
//! This will use stack-allocated buffers for matrices with dimensions known at compile-time, and
//! heap-allocated buffers for matrices with at least one dimension unknown at compile-time.

use std::mem;
use std::ops::Mul;

use typenum::Prod;
use generic_array::ArrayLength;

use core::Scalar;
use core::dimension::{Dim, DimName, Dynamic};
use core::allocator::Allocator;
use core::matrix_array::MatrixArray;
use core::matrix_vec::MatrixVec;

/*
 *
 * Allocator.
 *
 */
/// An allocator based on `GenericArray` and `MatrixVec` for statically-sized and dynamically-sized
/// matrices respectively.
pub struct DefaultAllocator;

// Static - Static
impl<N, R, C> Allocator<N, R, C> for DefaultAllocator
    where N: Scalar,
          R: DimName,
          C: DimName,
          R::Value: Mul<C::Value>,
          Prod<R::Value, C::Value>: ArrayLength<N> {
    type Buffer = MatrixArray<N, R, C>;

    #[inline]
    unsafe fn allocate_uninitialized(_: R, _: C) -> Self::Buffer {
        mem::uninitialized()
    }

    #[inline]
    fn allocate_from_iterator<I: IntoIterator<Item = N>>(nrows: R, ncols: C, iter: I) -> Self::Buffer {
        let mut res = unsafe { Self::allocate_uninitialized(nrows, ncols) };
        let mut count = 0;

        for (res, e) in res.iter_mut().zip(iter.into_iter()) {
            *res = e;
            count += 1;
        }

        assert!(count == nrows.value() * ncols.value(),
                "Matrix init. from iterator: iterator not long enough.");

        res
    }
}


// Dynamic - Static
// Dynamic - Dynamic
impl<N: Scalar, C: Dim> Allocator<N, Dynamic, C> for DefaultAllocator {
    type Buffer = MatrixVec<N, Dynamic, C>;

    #[inline]
    unsafe fn allocate_uninitialized(nrows: Dynamic, ncols: C) -> Self::Buffer {
        let mut res = Vec::new();
        let length = nrows.value() * ncols.value();
        res.reserve_exact(length);
        res.set_len(length);

        MatrixVec::new(nrows, ncols, res)
    }

    #[inline]
    fn allocate_from_iterator<I: IntoIterator<Item = N>>(nrows: Dynamic, ncols: C, iter: I) -> Self::Buffer {
        let it = iter.into_iter();
        let res: Vec<N> = it.collect();
        assert!(res.len() == nrows.value() * ncols.value(),
                "Allocation from iterator error: the iterator did not yield the correct number of elements.");

        MatrixVec::new(nrows, ncols, res)
    }
}


// Static - Dynamic
impl<N: Scalar, R: DimName> Allocator<N, R, Dynamic> for DefaultAllocator {
    type Buffer = MatrixVec<N, R, Dynamic>;

    #[inline]
    unsafe fn allocate_uninitialized(nrows: R, ncols: Dynamic) -> Self::Buffer {
        let mut res = Vec::new();
        let length = nrows.value() * ncols.value();
        res.reserve_exact(length);
        res.set_len(length);

        MatrixVec::new(nrows, ncols, res)
    }

    #[inline]
    fn allocate_from_iterator<I: IntoIterator<Item = N>>(nrows: R, ncols: Dynamic, iter: I) -> Self::Buffer {
        let it = iter.into_iter();
        let res: Vec<N> = it.collect();
        assert!(res.len() == nrows.value() * ncols.value(),
                "Allocation from iterator error: the iterator did not yield the correct number of elements.");

        MatrixVec::new(nrows, ncols, res)
    }
}
