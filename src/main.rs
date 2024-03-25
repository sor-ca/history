use undo::{Edit, Record};
pub trait Editable {
    type Element: Clone;
    type Key;
    fn add(&mut self, new: &Self::Element) -> Self::Key;
    fn delete(&mut self, key: &Self::Key) -> Option<Self::Element>;
    fn edit(&mut self, key: &Self::Key, edit: Self::Element) -> Option<Self::Element>;
    fn insert(&mut self, key: &Self::Key, insert: Self::Element);
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
    fn edit(&mut self, key: &Self::Key, edit: Self::Element) -> Option<Self::Element> {
        if let Some(el) = self.0.get_mut(*key) {
            let output = *el;
            *el = edit;
            Some(output)
        } else {
            None
        }
    }
    fn insert(&mut self, key: &Self::Key, insert: Self::Element) {
        if *key >= self.0.len() {
        } else {
            self.0.insert(*key, insert);
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
    fn edit(&mut self, key: &Self::Key, edit: Self::Element) -> Option<Self::Element> {
        if let Some(el) = self.0.get_mut(*key) {
            let output = el.clone();
            *el = edit;
            Some(output)
        } else {
            None
        }
    }
    fn insert(&mut self, key: &Self::Key, insert: Self::Element) {
        if *key >= self.0.len() {
        } else {
            self.0.insert(*key, insert);
        }
    }
}

pub enum Action<T: Editable + Clone> {
    Delete(Delete<T>),
    Add(Add<T>),
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
        }
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        match self {
            Action::Delete(del) => match &del.result {
                None => {}
                Some(el) => {
                    target.insert(&del.key, el.clone());
                    del.result = None;
                }
            },
            Action::Add(add) => {
                if let Some(key) = &add.key {
                    target.delete(key);
                }
                add.key = None;
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

// pub struct App {
//     pub record: Record<Action<Project>>,
//     pub project: Project,
// }

// pub struct Project {
//     pub nums: Numbers,
//     pub strs: Strings,
// }

fn main() {
    let mut numbers = Numbers(vec![1, 2, 3]);
    //let mut strings = Strings(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]);

    let mut record = Record::new();
    record.edit(
        &mut numbers,
        Action::Delete(Delete {
            key: 0,
            result: None,
        }),
    );
    dbg!(&numbers);
    record.undo(&mut numbers);
    dbg!(&numbers);

    record.edit(
        &mut numbers,
        Action::Add(Add {
            el: 0i32,
            key: None,
        }),
    );
    dbg!(&numbers);

    record.undo(&mut numbers);
    dbg!(&numbers);
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
