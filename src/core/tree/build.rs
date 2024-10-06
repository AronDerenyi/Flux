use super::{change::Change, node::Node, Tree};
use crate::{
    core::{Context, View},
    utils::id_vec::Id,
};
use itertools::{EitherOrBoth, Itertools};
use std::{any::Any, collections::HashMap, rc::Rc};

impl Tree {
    pub fn build(&mut self, states: &mut HashMap<Id, Rc<dyn Any>>, id: Id) {
        let node = &self[id];
        if !node.change.contains(Change::VIEW) {
            return;
        }

        let mut context = Context::new(id, states);
        let children = node.view.build(&mut context);
        let (paired_children, unused_children) =
            self.pair_children(children.into_iter(), node.children.iter().copied());

        for id in unused_children {
            self.remove(id);
        }

        self[id].children = paired_children
            .into_iter()
            .map(|(child_view, child_id)| {
                let child_id = if let Some(child_id) = child_id {
                    let child_node = &mut self[child_id];
                    child_node.parent = Some(id);

                    // underlying struct is immutable (if no interior mutability is used)
                    // so if the references match the structs match too
                    if !Rc::ptr_eq(&child_node.view, &child_view) && *child_node.view != *child_view
                    {
                        child_node.view = child_view;
                        child_node.change.add(Change::VIEW);
                    }
                    child_id
                } else {
                    self.insert(id, child_view)
                };

                self.build(states, child_id);
                child_id
            })
            .collect();
    }

    fn pair_children(
        &self,
        children: impl Iterator<Item = Rc<dyn View>>,
        child_indices: impl Iterator<Item = Id>,
    ) -> (Vec<(Rc<dyn View>, Option<Id>)>, Vec<Id>) {
        let mut paired_children = Vec::new();
        let mut unused_children = Vec::new();
        for pair in children.zip_longest(child_indices) {
            match pair {
                EitherOrBoth::Both(child, id) => {
                    let old_child = &self[id];
                    if old_child.view.type_id() == child.type_id() {
                        paired_children.push((child, Some(id)));
                    } else {
                        unused_children.push(id);
                        paired_children.push((child, None));
                    }
                }
                EitherOrBoth::Left(child) => paired_children.push((child, None)),
                EitherOrBoth::Right(index) => unused_children.push(index),
            }
        }
        (paired_children, unused_children)
    }
}
