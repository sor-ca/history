use undo::{Edit, Record};
pub trait Editable {
    type Element: Clone;
    type Key;
    fn add(&mut self, new: &Self::Element) -> Self::Key;
    fn delete(&mut self, key: &Self::Key) -> Option<Self::Element>;
    fn edit(&mut self, key: &Self::Key, edit: &Self::Element) -> Option<Self::Element>;
    fn insert(&mut self, key: &Self::Key, insert: &Self::Element);
}

#[derive(Clone, Debug)]
pub struct Numbers(pub Vec<i32>);

impl Editable for Numbers {
    type Element = i32;
    type Key = usize;
    fn add(&mut self, new: &Self::Element) -> Self::Key {
        self.0.push(*new);
        self.0.len() - 1
    }
    fn delete(&mut self, key: &Self::Key) -> Option<Self::Element> {
        if *key >= self.0.len() {
            None
        } else {
            let el = self.0.remove(*key);
            Some(el)
        }
    }
    fn edit(&mut self, key: &Self::Key, edit: &Self::Element) -> Option<Self::Element> {
        if let Some(el) = self.0.get_mut(*key) {
            let output = *el;
            *el = edit.clone();
            Some(output)
        } else {
            None
        }
    }
    fn insert(&mut self, key: &Self::Key, insert: &Self::Element) {
        if *key >= self.0.len() {
        } else {
            self.0.insert(*key, *insert);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Strings(pub Vec<String>);

impl Editable for Strings {
    type Element = String;
    type Key = usize;
    fn add(&mut self, new: &Self::Element) -> Self::Key {
        self.0.push(new.clone());
        self.0.len() - 1
    }
    fn delete(&mut self, key: &Self::Key) -> Option<Self::Element> {
        if *key >= self.0.len() {
            None
        } else {
            let el = self.0.remove(*key);
            Some(el)
        }
    }
    fn edit(&mut self, key: &Self::Key, edit: &Self::Element) -> Option<Self::Element> {
        if let Some(el) = self.0.get_mut(*key) {
            let output = el.clone();
            *el = edit.clone();
            Some(output)
        } else {
            None
        }
    }
    fn insert(&mut self, key: &Self::Key, insert: &Self::Element) {
        if *key >= self.0.len() {
        } else {
            self.0.insert(*key, insert.clone());
        }
    }
}

pub enum Action<T: Editable + Clone> {
    Delete(Delete<T>),
    Add(Add<T>),
    Edit(EditAct<T>),
}

impl<T: Editable + Clone> Edit for Action<T> {
    type Target = T;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        match self {
            Action::Delete(del) => {
                let res = target.delete(&del.key);
                del.result = res;
            }
            Action::Add(add) => {
                let key = target.add(&add.el);
                add.key = Some(key);
            }
            Action::Edit(ed) => {
                if let Some(t) = target.edit(&ed.key, &ed.new) {
                    ed.prev = Some(t);
                }
            }
        }
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        match self {
            Action::Delete(del) => match &del.result {
                None => {}
                Some(el) => {
                    target.insert(&del.key, &el);
                    del.result = None;
                }
            },
            Action::Add(add) => {
                if let Some(key) = &add.key {
                    target.delete(key);
                }
                add.key = None;
            }
            Action::Edit(ed) => {
                if let Some(t) = &ed.prev {
                    target.edit(&ed.key, t).unwrap();
                    ed.prev = None;
                }
            }
        }
    }
}
pub struct Delete<T: Editable + Clone> {
    pub key: T::Key,
    pub result: Option<T::Element>,
}

pub struct Add<T: Editable + Clone> {
    pub el: T::Element,
    pub key: Option<T::Key>,
}

pub struct EditAct<T: Editable + Clone> {
    pub new: T::Element,
    pub key: T::Key,
    pub prev: Option<T::Element>,
}

pub enum LocationAndAction {
    Nums(Action<Numbers>),
    Strs(Action<Strings>),
}

impl Edit for LocationAndAction {
    type Target = Project;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        match self {
            LocationAndAction::Nums(action) => {
                let target_field = &mut target.nums;
                action.edit(target_field);
            }
            LocationAndAction::Strs(action) => {
                let target_field = &mut target.strs;
                action.edit(target_field);
            }
        }
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        match self {
            LocationAndAction::Nums(action) => {
                let target_field = &mut target.nums;
                action.undo(target_field);
            }
            LocationAndAction::Strs(action) => {
                let target_field = &mut target.strs;
                action.undo(target_field);
            }
        }
    }
}

pub struct App {
    pub record: Record<LocationAndAction>,
    pub project: Project,
}

#[derive(Clone, Debug)]
pub struct Project {
    pub nums: Numbers,
    pub strs: Strings,
}

fn main() {
    let nums = Numbers(vec![1, 2, 3]);
    let strs = Strings(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]);
    let project = Project { nums, strs };
    let record = Record::new();
    let mut app = App { record, project };

    app.record.edit(
        &mut app.project,
        LocationAndAction::Nums(Action::Delete(Delete {
            key: 0,
            result: None,
        })),
    );
    assert_eq!(app.project.nums.0, vec![2, 3]);
    app.record.edit(
        &mut app.project,
        LocationAndAction::Strs(Action::Delete(Delete {
            key: 0,
            result: None,
        })),
    );
    assert_eq!(app.project.strs.0, vec!["b".to_owned(), "c".to_owned()]);
    app.record.undo(&mut app.project);
    assert_eq!(
        app.project.strs.0,
        vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
    );
    assert_eq!(app.project.nums.0, vec![2, 3]);
    app.record.undo(&mut app.project);
    assert_eq!(app.project.nums.0, vec![1, 2, 3]);

    app.record.edit(
        &mut app.project,
        LocationAndAction::Nums(Action::Add(Add {
            el: 0i32,
            key: None,
        })),
    );
    assert_eq!(app.project.nums.0, vec![1, 2, 3, 0]);

    app.record.undo(&mut app.project);
    assert_eq!(app.project.nums.0, vec![1, 2, 3]);

    app.record.edit(
        &mut app.project,
        LocationAndAction::Nums(Action::Edit(EditAct {
            new: 10i32,
            key: 0,
            prev: None,
        })),
    );
    assert_eq!(app.project.nums.0, vec![10, 2, 3]);
    app.record.undo(&mut app.project);
    assert_eq!(app.project.nums.0, vec![1, 2, 3]);
}

/*
pub trait Edit {
    type Target;
    type Output;

    // Required methods
    fn edit(&mut self, target: &mut Self::Target) -> Self::Output;
    fn undo(&mut self, target: &mut Self::Target) -> Self::Output;

    // Provided methods
    fn redo(&mut self, target: &mut Self::Target) -> Self::Output { ... }
    fn merge(&mut self, other: Self) -> Merged<Self>
       where Self: Sized { ... }
}
*/
