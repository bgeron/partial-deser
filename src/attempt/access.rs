#[cfg(doc)]
use serde::de::Deserializer;
use serde::de::{MapAccess, SeqAccess};

use crate::{options::ExtraOptions, reporter::Reporter, util::DeserializeKind};

use super::{erase_error_ref, AttemptState, DeserializeSeed, GlobalState, HaltingPoint};

pub(crate) struct Access<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState,
    pub(crate) kind: DeserializeKind,
    pub(crate) inner: Inner,
    pub(crate) collection_has_ended: bool,
    pub(crate) inside_element: Option<InsideElement>,
}

pub(crate) struct InsideElement {
    pub(crate) corresponding_halting_point: HaltingPoint,
    pub(crate) halting_point_is_on_stack: bool,
}

impl<'a, Inner, Extra> Access<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    fn enter_element(&mut self, corresponding_halting_point: HaltingPoint) {
        if self.inside_element.is_some() {
            error!(
                "access: enter before leaving previous element (maybe next_value_seed was not called?)"
            );
        }

        let halting_point_is_on_stack = self.should_backtrack_skip_item();

        if halting_point_is_on_stack {
            self.attempt
                .halting_point_stack
                .push(corresponding_halting_point);
        }

        self.inside_element = Some(InsideElement {
            corresponding_halting_point,
            halting_point_is_on_stack,
        });
    }

    fn leave_element(&mut self) {
        let Some(inside_element) = self.inside_element.take() else {
            error!("access: leave without entering element (maybe next_key_seed was not called?)");
            return;
        };

        if inside_element.halting_point_is_on_stack {
            let actual_point = self
                .attempt
                .halting_point_stack
                .pop()
                .expect("halting point stack did not stay balanced");
            debug_assert_eq!(actual_point, inside_element.corresponding_halting_point);
        }
    }
}

impl<Inner, Extra> Access<'_, Inner, Extra>
where
    Extra: ExtraOptions,
{
    /// Determine whether it's okay to fallback to skipping an item,
    /// depending on what we are deserializing for (the [`Deserializer`] method)
    pub(crate) fn should_fallback_skip_item(&self) -> bool {
        match self.kind {
            DeserializeKind::Seq => self.global.config.behavior.unstable_fallback_seq_skip_item,
            // Tuples don't have optional elements.
            DeserializeKind::Tuple { len: _ } => false,
            // Tuple structs may have optional fields.
            DeserializeKind::TupleStruct { name: _, len: _ } => {
                self.global.config.behavior.unstable_fallback_seq_skip_item
            }
            DeserializeKind::Map => self.global.config.behavior.unstable_fallback_map_skip_item,
            DeserializeKind::Struct { name: _, fields: _ } => {
                self.global
                    .config
                    .behavior
                    .unstable_fallback_struct_skip_field
            }
            DeserializeKind::Enum { .. } => false,

            _ => false,
        }
    }

    /// Determine whether it's okay to backtrack skipping an item
    pub(crate) fn should_backtrack_skip_item(&self) -> bool {
        match self.kind {
            DeserializeKind::Seq => self.global.config.behavior.unstable_backtrack_seq_skip_item,
            DeserializeKind::Tuple { len: _ } => false,
            DeserializeKind::TupleStruct { name: _, len: _ } => {
                self.global.config.behavior.unstable_backtrack_seq_skip_item
            }
            DeserializeKind::Map => self.global.config.behavior.unstable_backtrack_map_skip_item,
            DeserializeKind::Struct { name: _, fields: _ } => {
                self.global
                    .config
                    .behavior
                    .unstable_backtrack_struct_skip_field
            }
            DeserializeKind::Enum { .. } => false,
            _ => false,
        }
    }
}

impl<'de, Inner, Extra> SeqAccess<'de> for Access<'_, Inner, Extra>
where
    Inner: SeqAccess<'de>,
    Extra: ExtraOptions,
{
    type Error = Inner::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.collection_has_ended {
            self.global.reporter.report_access_past_end();
        }
        let Some(this_halting_point) = self
            .attempt
            .intervention_active
            .is_none()
            .then(|| self.attempt.new_halting_point_and_check_continue())
            .flatten()
        else {
            self.global.reporter.report_seq_next_element_skip();
            self.collection_has_ended = true;
            return Ok(None);
        };

        self.enter_element(this_halting_point);

        self.global.reporter.report_seq_next_element_start();
        let wrapped_seed = DeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            inner: seed,
        };
        let result = self.inner.next_element_seed(wrapped_seed);
        self.global.reporter.report_seq_next_element_finish(
            matches!(result, Ok(Some(_))),
            erase_error_ref(&result),
        );

        self.leave_element();

        match result {
            Ok(Some(v)) => Ok(Some(v)),
            Ok(None) => {
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(_) if self.should_fallback_skip_item() => {
                self.global.reporter.report_fallback_no_element();
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(e) => {
                self.collection_has_ended = true;
                Err(e)
            }
        }
    }
}

impl<'de, Inner, Extra> MapAccess<'de> for Access<'_, Inner, Extra>
where
    Inner: MapAccess<'de>,
    Extra: ExtraOptions,
{
    type Error = Inner::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.collection_has_ended {
            self.global.reporter.report_access_past_end();
        }
        let Some(this_halting_point) = self
            .attempt
            .intervention_active
            .is_none()
            .then(|| self.attempt.new_halting_point_and_check_continue())
            .flatten()
        else {
            self.global.reporter.report_map_next_key_skip();
            self.collection_has_ended = true;
            return Ok(None);
        };

        self.enter_element(this_halting_point);

        self.global.reporter.report_map_next_key_start();
        let wrapped_seed = DeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            inner: seed,
        };
        let result = self.inner.next_key_seed(wrapped_seed);
        self.global
            .reporter
            .report_map_next_key_finish(matches!(result, Ok(Some(_))), erase_error_ref(&result));

        match result {
            Ok(Some(v)) => Ok(Some(v)),
            Ok(None) => {
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(_) if self.should_fallback_skip_item() => {
                self.global.reporter.report_fallback_no_element();
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(e) => {
                self.collection_has_ended = true;
                Err(e)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if self.collection_has_ended {
            self.global.reporter.report_access_past_end();
        }

        self.global.reporter.report_map_next_value_start();
        let wrapped_seed = DeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            inner: seed,
        };
        let result = self.inner.next_value_seed(wrapped_seed);
        self.global
            .reporter
            .report_map_next_value_finish(erase_error_ref(&result));

        self.leave_element();

        if result.is_err() {
            self.collection_has_ended = true;
        }

        result
    }
}
