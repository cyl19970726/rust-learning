

pub mod read_only_dag{
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct Node {
        id: usize,
        downstream: Option<Rc<Node>>,
    }
    // Rc 是一个只读的引用计数器，你无法拿到 Rc 结构内部数据的可变引用，来修改这个数据
    // 因此我们无法修改指向
    impl Node {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                downstream: None,
            }
        }

        pub fn update_downstream(&mut self, downstream: Rc<Node>) {
            self.downstream = Some(downstream);
        }

        // 引用计数 +1 
        pub fn get_downstream(&self) -> Option<Rc<Node>> {
            self.downstream.as_ref().map(|v| v.clone())
        }
    }
}

pub mod read_write_dag{ 
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct Node {
        id: usize,
        // 使用 Rc<RefCell<T>> 让节点可以被修改
        downstream: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                downstream: None,
            }
        }

        pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
           self.downstream = Some(downstream);
        }

        pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
            self.downstream.as_ref().map(|v| v.clone())
        }
    }



}

mod tests {
    use std::rc::Rc;
    use super::read_only_dag::*;
    use super::read_write_dag::{Node as WRNode};
    #[test]
    fn test_dag() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);
        node3.update_downstream(Rc::new(node4)); //  count 1
    
        node1.update_downstream(Rc::new(node3)); 
        node2.update_downstream(node1.get_downstream().unwrap()); //count 2
        println!("node1: {:?}", node1);
        println!("node2: {:?}", node2);

        // calculate the reference_count for node3 
        let n3 = node1.get_downstream(); // count 3 
        let weak_count = n3.and_then(
            |node|{
                let count = Rc::strong_count(&node);
                println!("count: {:?}", count);
                Some(count)
            }
        );
        let node5 = Node::new(5);
        let node3 = node1.get_downstream().unwrap();

        
       
        // node3.update_downstream(Rc::new(node5)); 

        // happen the below error caused by the above code 
        //cannot borrow data in an `Rc` as mutable
        //trait `DerefMut` is required to modify through a dereference, 
        //but it is not implemented for `Rc<read_only_dag::Node>`
    }

    use std::cell::RefCell;

    #[test]
    fn test_ref_cell() {
        let data = RefCell::new(1);
        {
            // 如果没有这{} :运行代码 3，编译没有任何问题，但在运行到最后一行时，会得到：“already mutably borrowed: BorrowError” 这样的错误。可以看到，所有权的借用规则在此依旧有效，只不过它在运行时检测。
            // 获得 RefCell 内部数据的可变借用
            let mut v = data.borrow_mut();
            *v += 1;
        }
        println!("data: {:?}", data.borrow());
    }

    #[test]
    fn test_write_read_dag() {
        let mut node1 = Node::new(1); 
        let mut node2 = Node::new(2); 
        let mut node3 = Node::new(3); 
        let node4 = Node::new(4);

        node3.update_downstream(Rc::new(node1));
        node3.update_downstream(Rc::new(node2));

    }
}
