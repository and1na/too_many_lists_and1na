use std::mem;

//https://rust-unofficial.github.io/too-many-lists/
//[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
/*
This is a bad list implementation because...

We're allocating a node that just says "I'm not actually a Node"
One of our nodes isn't heap-allocated at all

Consider splitting a list in both layouts:

-layout 1:

    [Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)

    split off C:

    [Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
    [Elem C, ptr] -> (Empty *junk*)

-layout 2:

    [ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)

    split off C:

    [ptr] -> (Elem A, ptr) -> (Elem B, *null*)
    [ptr] -> (Elem C, *null*)


How do we rewrite our List? Well, we could do something like:
-It totally avoids allocating the Empty case,
-It also still suffers from non-uniformly allocating our elements.
-Complex

// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }
//
//
//
//
//
// */
// // pub enum List {
// //     Empty,
// //     Elem(i32, Box<List>),
// // }
//
// // struct Node {
// //     elem: i32,
// //     next: List,
// // }
//
// /*
//
// La variante More(Box<Node>) contiene un Box<Node>, que es un puntero inteligente.
// La optimización del puntero nulo se aplica en esta variante si el puede determinar
// que el puntero Box<Node> nunca será nulo. Esto puede ocurrir cuando:
//     -El Box<Node> siempre apunta a un nodo válido en el heap (No null).
//     -No se necesita almacenar un valor adicional para indicar que More contiene datos,
//     ya que la presencia de datos está implícita en la existencia del puntero Box<Node>.
//
// */
// // pub enum List {
// //     Empty,
// //     More(Box<Node>),
// // }
//
//
//
// //We marked the List as public
// //The problem is that the internals of an enum are totally public
//
// /*
// We could make all of Node totally public, but generally in Rust we
// favour keeping implementation details private. Let's make List a struct,
//  so that we can hide the implementation details:
//  */
//
// pub struct List {
//     head: Link
// }
//
// type Link = Option<Box<Node>>;
//
//
// struct Node {
//     elem: i32,
//     next: Link
// }
//
// impl List {
//
//     //Self is an alias for "that type I wrote at the top next to impl
//     pub fn new()->Self{
//
//         List{
//             //We refer to variants of an enum using ::
//             head: None
//         }
//     }
//
//
//     /*
//         There are 3 primary forms that self can take: self, &mut self, and &self.
//         These 3 forms represent the three primary forms of ownership in Rust:
//
//         -> self - Value
//
//             A value represents true ownership. You can do whatever you want with a
//              value: move it, destroy it, mutate it, or loan it out via a reference.
//               When you pass something by value, it's moved to the new location. The
//                new location now owns the value, and the old location can no longer
//                 access it. For this reason most methods don't want self
//
//
//
//
//         -> &mut self - mutable reference
//
//             A mutable reference represents temporary exclusive access to a value that you don't own.
//              You're allowed to do absolutely anything you want to a value you have a mutable
//              reference to as long you leave it in a valid state when you're done (it would be rude
//              to the owner otherwise!). This means you can actually completely overwrite the value.
//              A really useful special case of this is swapping a value out for another, which we'll
//              be using a lot. The only thing you can't do with an &mut is move the value out with no
//               replacement. &mut self is great for methods that want to mutate self.
//
//         -> &self - shared reference
//
//          shared reference represents temporary shared access to a value that you don't own. Because
//          you have shared access, you're generally not allowed to mutate anything. Think of & as
//           putting the value out on display in a museum. & is great for methods that only want to
//           observe self.
//     */
//
//     /*
//         push mutates the list, so we'll want to take &mut self. We also need to take an i32 to push:
//
//         We're trying to move the self.head field out to next
//
//          But Rust no permite mover o transferir valores fuera de una referencia mutable directamente.
//     */
//     pub fn push(&mut self, elem: i32) {
//
//         let new_node = Box::new(Node {
//             elem,
//             next: self.head.take(),
//         });
//
//         self.head = Some(new_node);
//     }
//
//
//     pub fn pop(&mut self) -> Option<i32> {
//
//         //Takes the value out of the option, leaving a None in its place
//         //so if its None, puts a None in its place
//         //if its something, returns that something and puts a None in its place
//         match self.head.take() {
//             None => None,
//             Some(node) => {
//                 self.head = node.next;
//                 Some(node.elem)
//             }
//         }
//     }
//
// }
//
// impl Drop for List {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = boxed_node.next.take();
//         }
//     }
// }


//
//
//
// pub struct List {
//     head: Link,
// }
//
// type Link = Option<Box<Node>>;
//
// struct Node {
//     elem: i32,
//     next: Link,
// }
//
// impl List {
//     pub fn new() -> Self {
//         List { head: None }
//     }
//
//     pub fn push(&mut self, elem: i32) {
//         let new_node = Box::new(Node {
//             elem: elem,
//             next: self.head.take(),
//         });
//
//         self.head = Some(new_node);
//     }
//
//     pub fn pop(&mut self) -> Option<i32> {
//         match self.head.take() {
//             None => None,
//             Some(node) => {
//                 self.head = node.next;
//                 Some(node.elem)
//             }
//         }
//     }
// }
//
// impl Drop for List {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = boxed_node.next.take();
//         }
//     }
// }



pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}





mod test {
    use crate::second::List;

    #[test]
    fn basics() {
        mod test {
            use crate::second::List;

            #[test]
            fn basics() {
                let mut list = List::new();

                // Check empty list behaves right
                assert_eq!(list.pop(), None);

                // Populate list
                list.push(1);
                list.push(2);
                list.push(3);

                // Check normal removal
                assert_eq!(list.pop(), Some(3));
                assert_eq!(list.pop(), Some(2));

                // Push some more just to make sure nothing's corrupted
                list.push(4);
                list.push(5);

                // Check normal removal
                assert_eq!(list.pop(), Some(5));
                assert_eq!(list.pop(), Some(4));

                // Check exhaustion
                assert_eq!(list.pop(), Some(1));
                assert_eq!(list.pop(), None);
            }
        }

    }
}