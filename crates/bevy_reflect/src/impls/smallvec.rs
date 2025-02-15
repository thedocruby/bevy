use smallvec::{Array, SmallVec};
use std::any::Any;

use crate::{serde::Serializable, FromReflect, List, ListIter, Reflect, ReflectMut, ReflectRef};

impl<T: Array + Send + Sync + 'static> List for SmallVec<T>
where
    T::Item: FromReflect + Clone,
{
    fn get(&self, index: usize) -> Option<&dyn Reflect> {
        if index < SmallVec::len(self) {
            Some(&self[index] as &dyn Reflect)
        } else {
            None
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        if index < SmallVec::len(self) {
            Some(&mut self[index] as &mut dyn Reflect)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        <SmallVec<T>>::len(self)
    }

    fn push(&mut self, value: Box<dyn Reflect>) {
        let value = value.take::<T::Item>().unwrap_or_else(|value| {
            <T as Array>::Item::from_reflect(&*value).unwrap_or_else(|| {
                panic!(
                    "Attempted to push invalid value of type {}.",
                    value.type_name()
                )
            })
        });
        SmallVec::push(self, value);
    }

    fn iter(&self) -> ListIter {
        ListIter {
            list: self,
            index: 0,
        }
    }
}

// SAFE: any and any_mut both return self
unsafe impl<T: Array + Send + Sync + 'static> Reflect for SmallVec<T>
where
    T::Item: FromReflect + Clone,
{
    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn any(&self) -> &dyn Any {
        self
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_reflect(&self) -> &dyn Reflect {
        self
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        self
    }

    fn apply(&mut self, value: &dyn Reflect) {
        crate::list_apply(self, value);
    }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
        *self = value.take()?;
        Ok(())
    }

    fn reflect_ref(&self) -> ReflectRef {
        ReflectRef::List(self)
    }

    fn reflect_mut(&mut self) -> ReflectMut {
        ReflectMut::List(self)
    }

    fn clone_value(&self) -> Box<dyn Reflect> {
        Box::new(self.clone_dynamic())
    }

    fn reflect_hash(&self) -> Option<u64> {
        None
    }

    fn reflect_partial_eq(&self, value: &dyn Reflect) -> Option<bool> {
        crate::list_partial_eq(self, value)
    }

    fn serializable(&self) -> Option<Serializable> {
        None
    }
}

impl<T: Array + Send + Sync + 'static> FromReflect for SmallVec<T>
where
    T::Item: FromReflect + Clone,
{
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::List(ref_list) = reflect.reflect_ref() {
            let mut new_list = Self::with_capacity(ref_list.len());
            for field in ref_list.iter() {
                new_list.push(<T as Array>::Item::from_reflect(field)?);
            }
            Some(new_list)
        } else {
            None
        }
    }
}
