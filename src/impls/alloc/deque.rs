use crate::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, GetMut, Iter, IterMut, Len,
	PopBack, PushBack, Reserve, SimpleCollectionMut, SimpleCollectionRef, WithCapacity,
};
use alloc::collections::VecDeque;

impl<T> Collection for VecDeque<T> {
	type Item = T;
}

impl<T> CollectionRef for VecDeque<T> {
	type ItemRef<'a> = &'a T where Self: 'a;

	crate::covariant_item_ref!();
}

impl<T> CollectionMut for VecDeque<T> {
	type ItemMut<'a> = &'a mut T where Self: 'a;

	crate::covariant_item_mut!();
}

impl<T> SimpleCollectionRef for VecDeque<T> {
	crate::simple_collection_ref!();
}

impl<T> SimpleCollectionMut for VecDeque<T> {
	crate::simple_collection_mut!();
}

impl<T> WithCapacity for VecDeque<T> {
	#[inline(always)]
	fn with_capacity(capacity: usize) -> Self {
		VecDeque::with_capacity(capacity)
	}
}

impl<T> Len for VecDeque<T> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<T> Capacity for VecDeque<T> {
	#[inline(always)]
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<T> Reserve for VecDeque<T> {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl<T> Get<usize> for VecDeque<T> {
	#[inline(always)]
	fn get(&self, key: usize) -> Option<&T> {
		self.get(key)
	}
}

impl<T> GetMut<usize> for VecDeque<T> {
	#[inline(always)]
	fn get_mut(&mut self, key: usize) -> Option<&mut T> {
		self.get_mut(key)
	}
}

impl<T> PushBack for VecDeque<T> {
	type Output = ();

	#[inline(always)]
	fn push_back(&mut self, t: T) {
		self.push_back(t)
	}
}

impl<T> PopBack for VecDeque<T> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<T> {
		self.pop_back()
	}
}

impl<T> Clear for VecDeque<T> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> Iter for VecDeque<T> {
	type Iter<'a>
		= alloc::collections::vec_deque::Iter<'a, T>
	where
		Self: 'a;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<T> IterMut for VecDeque<T> {
	type IterMut<'a>
		= alloc::collections::vec_deque::IterMut<'a, T>
	where
		Self: 'a;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}
