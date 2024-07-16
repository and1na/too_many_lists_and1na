
/*
The most important thing about a persistent list is that you can manipulate the tails of lists basically for free:

For instance, this isn't an uncommon workload to see with a persistent list:


list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D


Ventajas de esta manipulación eficiente:
Inmutabilidad: Las listas persistentes no cambian sus valores una vez creadas.
En lugar de modificar una lista existente, se crean nuevas versiones con los cambios,
 lo cual es seguro para concurrencia y facilita la razonabilidad del código.
Eficiencia: Dado que las nuevas listas comparten la mayor parte de su estructura con las
 listas originales, la creación de nuevas listas (como list2 y list3) es muy eficiente
 en términos de tiempo y memoria. Solo se necesita crear referencias adicionales y no duplicar
  toda la estructura de datos.

  Esta lista no se puede hacer con Box porque En Rust, Box se utiliza para asignar memoria en el
   heap y garantizar que haya un único propietario del dato. Esto significa que cuando el
    propietario (la caja) se libera, también se libera el dato.


    Ventajas de Rc:
Propiedad compartida: Rc permite que múltiples propietarios compartan el mismo dato. Cada vez que se
 clona una Rc, el conteo de referencias aumenta, y cuando una Rc se libera, el conteo de referencias
  disminuye.
*/
use std::rc::Rc;
use std::sync::Arc;
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }


    /*
    Prepend
    It takes a list and an element, and returns a List.

    we want to make a new node, that has the old list as its next value. The only novel thing
    is how to get that next value, because we're not allowed to mutate anything.+

    Rc in particular uses Clone as the way to increment the reference count.

    &self cuz i cant mutate the list
    */
    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Arc::new(Node {
            elem: elem,
            next: self.head.clone()
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List {
            //and then is an option funcion which have a difference with map.
            //the first order fn received as a parameter, here returns an Opt, not T
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    pub fn head(&self) -> Option<&T> {

        self.head.as_ref().map(|node| &node.elem)
    }


}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            /*Returns the inner value, if the Rc has exactly one strong reference*/
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        println!("Unwrap: {}",list.head().unwrap());
        if let Some(head) = list.head() {

            println!("{}", head)

        }

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        list.head().map(|node| println!("{}",node));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

       /* let list = list.prepend(2);
        assert_eq!(list.head(), Some(&2));*/

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
}