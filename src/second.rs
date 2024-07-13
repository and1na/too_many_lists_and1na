
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
            elem,
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

    pub fn peek(&self) -> Option<&T> {
        //map() takes self by value (it doesnt care about what type of parameter
        // i have -mut ref , ref, value-), so we have to use
        //as_ref()
        //
        //El método as_ref en Rust es muy útil cuando se trabaja con tipos como Option y Result
        // necesitas convertir una referencia a una opción (&Option<T>) en una opción que contiene
        // una referencia (Option<&T>). Esto es especialmente importante cuando quieres acceder al
        // contenido del Option sin mover o consumir el Option original.


        // Converts from &Option<T> to Option<&T>

        //Usar as_ref te permite trabajar con una referencia al contenido del Option en lugar de moverlo.

        //pub fn map<U, F>(self (SE LO LLEVA), f: F)
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        //lo que devuelvo es el head, tengo que habilitarlo como muy
        /*
            con as_mut() devuelvo una referencia a un option con referencia mutable
            el self que consume/mueve seria el head??o el self normal idk, estariamos
             moviendo el ownership del option, lo que no se puede porque tenemos como
              parametro una referencia
        */
        self.head.as_mut().map(|node| {
            &mut node.elem
        })



    }


    pub fn into_iter(self) -> IntoIter<T> {--
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        // head = Option<Box<Node<T>>>;
        // Converts from &Option<T>)to Option<&T>
        //In this case it converts from Option<&Box<Node<T>>> to
        // Option<Box<&Node<T>>>;
        //to Option<&>
        Iter { next: self.head.as_deref().map(|node| &*node) }
    }

    pub fn iter_mut(&self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
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





//Collections are iterated in Rust using the Iterator trait
/*
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

Type Item.  is declaring that every implementation of Iterator has an associated type called Item.
 In this case, this is the type that it can spit out when you call next.
*/




//Iter is generic over *some* lifetime, it doesn't care
//definition with a lifetime and a type
//the lifetime indicates how much time the references inside Iter
// are valid, to set this lifetime to a ref, mark it with ´a

/*
So what does fn foo<'a>(&'a A) -> &'a B mean? In practical terms, all it means is
 that the input must live at least as long as the output. So if you keep the output
  around for a long time, this will expand the region that the input must be valid for.
   Once you stop using the output, the compiler will know it's ok for the input to become
    invalid too.*/
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        //si self.next es Some, aplica una funcion con el como parametro
        self.next.map(|node| {

            // Option<Box<Node<T>>>`= node.next.map(|node| &node);
            // Lo que daría error, pero sacando el valor de la ref
            //que tenemos a node con "*" y anteponiendole el ampersand
            //podemos obtener esa referencia a &Node<T> que necesitamos

            /*
             El node que nos proporciona este next  (tipo  Node)
             nos devuelve Option<Box<Node<T>>> pero lo que nosotros
             queremos es un Node<T>
            */

            // node -> Option<&'a Node<T>>
            // node.next = Option<Box<Node<T>>>;
            // node.next.as_ref = &Box<Node<T>>
            // node.next.as_deref = &Node<T>

            /*
            INTERESTING
            I can do the same as "as_deref()" using as_ref (adding an indirection layer)
            an then deref it using value operator "*" twice (not recommended)
            */

            /*
            pub fn map<U, F>(self, f: F) -> Option<U>
                where F: FnOnce(T) -> U, (F es funcion o closure fn (T) -> U
            */
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
//this iterator traverses the list transferring the ownership
pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    // Define que el tipo de los elementos que este iterador devolverá es T
    type Item = T;
    //Este método debe devolver Option<Self::Item>, lo que en este caso es Option<T>.
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}




mod test {
    use crate::second::List;
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            //valor de la referencia
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }
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



/*
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}


type Item. This is declaring that every implementation of Iterator has an associated type
called Item. In this case, this is the type that it can spit out when you call next.
*/



























