use crate::{
	Collection, CollectionMut, CollectionRef, Get, GetMut, Iter, IterMut, Len, SimpleCollectionMut,
	SimpleCollectionRef,
};

impl<T, const D: usize> Collection for [T; D] {
	type Item = T;
}

impl<T, const D: usize> CollectionRef for [T; D] {
	type ItemRef<'a> = &'a T where Self: 'a;

	crate::covariant_item_ref!();
}

impl<T, const D: usize> CollectionMut for [T; D] {
	type ItemMut<'a> = &'a mut T where Self: 'a;

	crate::covariant_item_mut!();
}

impl<T, const D: usize> SimpleCollectionRef for [T; D] {
	crate::simple_collection_ref!();
}

impl<T, const D: usize> SimpleCollectionMut for [T; D] {
	crate::simple_collection_mut!();
}

impl<T, const D: usize> Len for [T; D] {
	#[inline(always)]
	fn len(&self) -> usize {
		D
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		D == 0
	}
}

impl<T, const D: usize> Get<usize> for [T; D] {
	#[inline(always)]
	fn get(&self, index: usize) -> Option<&T> {
		self.as_slice().get(index)
	}
}

impl<T, const D: usize> GetMut<usize> for [T; D] {
	#[inline(always)]
	fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.as_mut_slice().get_mut(index)
	}
}

impl<T, const D: usize> Iter for [T; D] {
	type Iter<'a> = core::slice::Iter<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter()
	}
}

impl<T, const D: usize> IterMut for [T; D] {
	type IterMut<'a> = core::slice::IterMut<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.as_mut_slice().iter_mut()
	}
}
