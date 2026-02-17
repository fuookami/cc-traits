use crate::{
	Collection, CollectionMut, CollectionRef, Get, GetMut, Iter, IterMut, Len, SimpleCollectionMut,
	SimpleCollectionRef,
};

impl<T> Collection for [T] {
	type Item = T;
}

impl<T> CollectionRef for [T] {
	type ItemRef<'a> = &'a T where Self: 'a;

	crate::covariant_item_ref!();
}

impl<T> CollectionMut for [T] {
	type ItemMut<'a> = &'a mut T where Self: 'a;

	crate::covariant_item_mut!();
}

impl<T> SimpleCollectionRef for [T] {
	crate::simple_collection_ref!();
}

impl<T> SimpleCollectionMut for [T] {
	crate::simple_collection_mut!();
}

impl<T> Len for [T] {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<T> Get<usize> for [T] {
	#[inline(always)]
	fn get(&self, index: usize) -> Option<&T> {
		self.get(index)
	}
}

impl<T> GetMut<usize> for [T] {
	#[inline(always)]
	fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.get_mut(index)
	}
}

impl<T> Iter for [T] {
	type Iter<'a> = core::slice::Iter<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<T> IterMut for [T] {
	type IterMut<'a> = core::slice::IterMut<'a, T> where Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}
