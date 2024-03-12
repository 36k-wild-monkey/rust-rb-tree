pub mod rbtree_mod {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    #[derive(Debug, Clone, PartialEq)]
    pub enum Color {
        Red,
        Black
    }

    #[derive(Debug, Clone)]
    pub struct Node {
        value: i32,
        color: Color,
        left: Option<Rc<RefCell<Node>>>,
        right: Option<Rc<RefCell<Node>>>,
        parent: Option<Weak<RefCell<Node>>>
    }

    impl Node {
        pub fn new(value:i32, color:Color, parent:Option<Weak<RefCell<Node>>>) -> Self {
            Self {
                value: value,
                color: color,
                left: None,
                right: None,
                parent: parent
            }
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.value.partial_cmp(&other.value)
        }
    }

    #[derive(Debug, Clone)]
    pub struct RedBlackTree {
        root:Option<Rc<RefCell<Node>>>,
        len:usize
    }
    impl RedBlackTree {
        pub fn new() -> Self {
            Self {
                root: None,
                len: 0
            }
        }
        fn get_parent(&self, x:&Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
            match x.as_ref().borrow().parent.clone() {
                None => None,
                Some(x) => {
                    x.upgrade()
                }
            }
        }
        fn get_brother(&self, x:&Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
            match self.get_parent(x) {
                None => None,
                Some(p) => {
                    let p_node = p.as_ref().borrow();
                    if p_node.left == Some(x.clone()) {
                        p_node.right.clone()
                    } else {
                        p_node.left.clone()
                    }
                }
            }
        }
        fn is_left_node(&self, x:&Rc<RefCell<Node>>) -> bool {
            match self.get_parent(x) {
                None => false,
                Some(p) => {
                    p.as_ref().borrow().left == Some(x.clone())
                }
            }
        }
        fn is_right_node(&self, x:&Rc<RefCell<Node>>) -> bool {
            match self.get_parent(x) {
                None => false,
                Some(p) => {
                    p.as_ref().borrow().right == Some(x.clone())
                }
            }
        }
        fn swap_value(&self, a:&Rc<RefCell<Node>>, b:&Rc<RefCell<Node>>) {
            let mut a_write = a.as_ref().borrow_mut();
            let mut b_write = b.as_ref().borrow_mut();
            (b_write.value, a_write.value) = (a_write.value, b_write.value);
        }
        fn set_color(&self, x:&Rc<RefCell<Node>>, color:Color) {
            let mut x_write = x.as_ref().borrow_mut();
            x_write.color = color.clone();
        }
        fn drop_node(&mut self, x:Rc<RefCell<Node>>) {
            if let Some(p) = self.get_parent(&x) {
                let mut p_write = p.as_ref().borrow_mut();
                if p_write.left == Some(x.clone()) {
                    p_write.left = None;
                } else {
                    p_write.right = None;
                }
            }
            if self.root == Some(x.clone()) {
                self.root = None;
            }
            drop(x);
            self.len -= 1;
        }
        fn insert_search(&self, value:i32, cmp: bool) -> Option<Rc<RefCell<Node>>> {
            let mut pv = self.root.clone();
            while let Some(temp) = pv.clone() {
                let pv_node = temp.as_ref().borrow();
                if value < pv_node.value {
                    if pv_node.left.is_some() {
                        pv = pv_node.left.clone();
                        continue;
                    }
                } else if value > pv_node.value {
                    if pv_node.right.is_some() {
                        pv = pv_node.right.clone();
                        continue;
                    } 
                } else if cmp && value == pv_node.value {
                    return pv;
                }
                break;
            }
            if cmp {
                None
            } else {
                pv
            }
        }
        fn search_max_node(&self, x:Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
            let mut x = x;
            while let Some(n) = x.clone() {
                let n_node = n.as_ref().borrow();
                if n_node.right.is_some() {
                    x = n_node.right.clone();
                }  else {
                    break;
                }
            }
            x
        }
        fn search_min_node(&self, x:Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
            let mut x = x;
            while let Some(n) = x.clone() {
                let n_node = n.as_ref().borrow();
                if n_node.left.is_some() {
                    x = n_node.left.clone();
                }  else {
                    break;
                }
            }
            x
        }
        fn delete_search(&self, x:&Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
            let x_node = x.as_ref().borrow();
            if let Some(x) = x_node.right.clone() {
                self.search_min_node(Some(x))
            } else if let Some(x) = x_node.left.clone() {
                self.search_max_node(Some(x))
            } else {
                None
            }
        }
        fn root_fix(&self) {
            if let Some(root) = self.root.clone() {
                let mut root_write = root.as_ref().borrow_mut();
                if root_write.color != Color::Black {
                    root_write.color = Color::Black;
                }
            }
        }
        fn left_rotate(&mut self, x:&Rc<RefCell<Node>>) {
            if let Some(p) = self.get_parent(x) {
                let x_node = x.as_ref().borrow().clone();
                let p_node = p.as_ref().borrow().clone();
                if p_node.right == Some(x.clone()) {
                    if let Some(g) = self.get_parent(&p) {
                        let mut g_node = g.as_ref().borrow_mut();
                        if g_node.left == Some(p.clone()) {
                            g_node.left = Some(x.clone());
                        } else {
                            g_node.right = Some(x.clone());
                        }
                    }
                    {
                        let mut x_write = x.as_ref().borrow_mut();
                        let mut p_write = p.as_ref().borrow_mut();
                        if let Some(x_left) = x_node.left.clone() {
                            let mut x_left_write = x_left.as_ref().borrow_mut();
                            x_left_write.parent = Some(Rc::downgrade(&p));
                        }
                        p_write.right = x_node.left.clone();
                        p_write.parent = Some(Rc::downgrade(x));
                        x_write.left = Some(p.clone());
                        x_write.parent = p_node.parent.clone();
                    }
                    if self.root == Some(p) {
                        self.root = Some(x.clone());
                    }
                }
            }
        }
        fn right_rotate(&mut self, x:&Rc<RefCell<Node>>) {
            if let Some(p) = self.get_parent(x) {
                let x_node = x.as_ref().borrow().clone();
                let p_node = p.as_ref().borrow().clone();
                if p_node.left == Some(x.clone()) {
                    if let Some(g) = self.get_parent(&p) {
                        let mut g_node = g.as_ref().borrow_mut();
                        if g_node.left == Some(p.clone()) {
                            g_node.left = Some(x.clone());
                        } else {
                            g_node.right = Some(x.clone());
                        }
                    }
                    {
                        let mut x_write = x.as_ref().borrow_mut();
                        let mut p_write = p.as_ref().borrow_mut();
                        if let Some(x_right) = x_node.right.clone() {
                            let mut x_right_write = x_right.as_ref().borrow_mut();
                            x_right_write.parent = Some(Rc::downgrade(&p));
                        }
                        p_write.left = x_node.right.clone();
                        p_write.parent = Some(Rc::downgrade(x));
                        x_write.right = Some(p.clone());
                        x_write.parent = p_node.parent.clone();
                    }
                    if self.root == Some(p) {
                        self.root = Some(x.clone());
                    }
                }
            }
        }
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn get(&self, value:i32) -> Option<Rc<RefCell<Node>>> {
            self.insert_search(value, true)
        }
        pub fn add(&mut self, value:i32) {
            match self.insert_search(value, false) {
                None => {
                    self.root = Some(Rc::new(RefCell::new(Node::new(value, Color::Black, None))));
                    self.len = 1;
                },
                Some(pv) => {
                    let pv_node = pv.as_ref().borrow().clone();
                    let x = Rc::new(RefCell::new(Node::new(value, Color::Red, Some(Rc::downgrade(&pv)))));
                    {
                        let mut pv_write = pv.as_ref().borrow_mut();
                        if value < pv_node.value  {
                            pv_write.left = Some(x.clone());
                            self.len += 1;
                        } else if value > pv_node.value {
                            pv_write.right = Some(x.clone());
                            self.len += 1;
                        } else if value == pv_node.value {
                            pv_write.value = value;
                        }
                    }
                    self.insert_fix(x);
                }
            }
        }
        fn insert_fix(&mut self, x:Rc<RefCell<Node>>) {
            let mut x = x;
            loop {
                let x_node = x.as_ref().borrow().clone();
                if x_node.color == Color::Red {
                    if let Some(p) = self.get_parent(&x) {
                        let p_node = p.as_ref().borrow().clone();
                        if p_node.color == Color::Red {
                            if let Some(g) = self.get_parent(&p) {
                                let g_node = g.clone().as_ref().borrow().clone();
                                if g_node.color == Color::Black {
                                    let next;
                                    {
                                        let mut x = x;
                                        let mut p = p;
                                        let u = self.get_brother(&p);
                                        // 不能判断U，U可能不存在，用P判断
                                        if self.is_right_node(&p) {
                                            if self.is_left_node(&x) {
                                                self.right_rotate(&x);
                                            } else {
                                                (x, p) = (p, x);
                                            }
                                            self.left_rotate(&x);
                                        } else {
                                            if self.is_right_node(&x) {
                                                self.left_rotate(&x);
                                            } else {
                                                (x, p) = (p, x);
                                            }
                                            self.right_rotate(&x);
                                        }
                                        if let Some(u) = u {
                                            let u_node = u.as_ref().borrow().clone();
                                            if u_node.color == Color::Red {
                                                let mut p_write = p.as_ref().borrow_mut();
                                                p_write.color = Color::Black;
                                            } else /*if u_node.color == Color::Black*/ {
                                                let mut x_write = x.as_ref().borrow_mut();
                                                x_write.color = Color::Black;
                                                let mut g_write = g.as_ref().borrow_mut();
                                                g_write.color = Color::Red;
                                            }
                                        } else /* u.is_none() */ {
                                            let mut p_write = p.as_ref().borrow_mut();
                                            p_write.color = Color::Black;
                                        }
                                        next = x;
                                    }
                                    x = next;
                                    continue;
                                }
                            }
                        }
                    }
                }
                break;
            }
            self.root_fix();
        }
        fn delete_fix(&mut self, x:Rc<RefCell<Node>>) {
            let mut x = x;
            loop {
                let x_node = x.as_ref().borrow().clone();
                if x_node.color == Color::Black {
                    if let (Some(p), Some(b)) = (self.get_parent(&x), self.get_brother(&x)) {
                        let p_node = p.as_ref().borrow().clone();
                        let b_node = b.as_ref().borrow().clone();
                        match p_node.color {
                            Color::Red => {
                                let cl;
                                let cr;
                                let cl_node;
                                let cr_node;
                                if self.is_right_node(&x) {
                                    cl  = b_node.left.clone().unwrap();
                                    cr = b_node.right.clone().unwrap();
                                } else {
                                    cl  = b_node.right.clone().unwrap();
                                    cr = b_node.left.clone().unwrap();
                                }
                                cl_node = cl.as_ref().borrow().clone();
                                cr_node = cr.as_ref().borrow().clone();
                                match (cl_node.color, cr_node.color) {
                                    (Color::Black, Color::Black) => {
                                        //规则1 P红 B黑 双C黑
                                        self.set_color(&p, Color::Black);
                                        self.set_color(&b, Color::Red);
                                    },(Color::Red, Color::Red) => {
                                        //规则2 P红 B黑 双C红
                                        if self.is_right_node(&x) {
                                            self.right_rotate(&b);
                                        } else {
                                            self.left_rotate(&b);
                                        }
                                        self.set_color(&cl, Color::Black);
                                        self.set_color(&p, Color::Black);
                                        self.set_color(&b, Color::Red);
                                    },(Color::Red, Color::Black) => {
                                        //规则3 P红 B黑 CL红 CR黑
                                        if self.is_right_node(&x) {
                                            self.right_rotate(&b);
                                        } else {
                                            self.left_rotate(&b);
                                        }
                                    },(Color::Black, Color::Red) => {
                                        //规则4 P红 B黑 CL黑 CR红
                                        if self.is_right_node(&x) {
                                            self.left_rotate(&cr);
                                            self.right_rotate(&cr);
                                            self.set_color(&b, Color::Red);
                                            self.set_color(&cr, Color::Black);
                                        } else {
                                            self.right_rotate(&cr);
                                            self.left_rotate(&cr);
                                            self.set_color(&b, Color::Red);
                                            self.set_color(&cr, Color::Black);
                                        }
                                    }
                                }
                            },
                            Color::Black => {
                                match b_node.color {
                                    Color::Red => {
                                        let c;
                                        let c_node;
                                        let l;
                                        let r;
                                        let l_node;
                                        let r_node;
                                        if self.is_right_node(&x) {
                                            c = b_node.right.clone().unwrap();
                                            c_node = c.as_ref().borrow().clone();
                                            l = c_node.left.clone().unwrap();
                                            r = c_node.right.clone().unwrap();
                                        } else {
                                            c = b_node.left.clone().unwrap();
                                            c_node = c.as_ref().borrow().clone();
                                            l = c_node.right.clone().unwrap();
                                            r = c_node.left.clone().unwrap();
                                        }
                                        l_node = l.as_ref().borrow().clone();
                                        r_node = r.as_ref().borrow().clone();    
                                        match (l_node.color, r_node.color) {
                                            (Color::Black, Color::Black) => {
                                                //规则5 P黑 B红 C双子(双黑)
                                                if self.is_right_node(&x) {
                                                    self.right_rotate(&b);
                                                } else {
                                                    self.left_rotate(&b);
                                                }
                                                self.set_color(&b, Color::Black);
                                                self.set_color(&c, Color::Red);
                                            },
                                            (Color::Red, Color::Red) => {
                                                //规则6 P黑 B红 C双子(双红)
                                                if self.is_right_node(&x) {
                                                    self.right_rotate(&b);
                                                    self.right_rotate(&c);
                                                    self.left_rotate(&c);
                                                } else {
                                                    self.left_rotate(&b);
                                                    self.left_rotate(&c);
                                                    self.right_rotate(&c);
                                                }
                                                self.set_color(&l, Color::Black);
                                            },
                                            (Color::Red, Color::Black) => {
                                                //规则7 P黑 B红 C双子(左红右黑)
                                                if self.is_right_node(&x) {
                                                    self.right_rotate(&b);
                                                    self.right_rotate(&c);
                                                    self.left_rotate(&c);
                                                } else {
                                                    self.left_rotate(&b);
                                                    self.left_rotate(&c);
                                                    self.right_rotate(&c);
                                                }
                                                self.set_color(&l, Color::Black);
                                            },
                                            (Color::Black, Color::Red) => {
                                                //规则8 P黑 B红 C双子(左黑右红)
                                                if self.is_right_node(&x) {
                                                    self.right_rotate(&b);
                                                    self.left_rotate(&r);
                                                    self.right_rotate(&r);
                                                } else {
                                                    self.left_rotate(&b);
                                                    self.right_rotate(&r);
                                                    self.left_rotate(&r);
                                                }
                                                self.set_color(&b, Color::Black);
                                            }
                                        }
                                    },
                                    Color::Black => {
                                        let cl;
                                        let cr;
                                        let cl_node;
                                        let cr_node;
                                        if self.is_right_node(&x) {
                                            cl  = b_node.left.clone().unwrap();
                                            cr = b_node.right.clone().unwrap();
                                        } else {
                                            cl  = b_node.right.clone().unwrap();
                                            cr = b_node.left.clone().unwrap();
                                        }
                                        cl_node = cl.as_ref().borrow().clone();
                                        cr_node = cr.as_ref().borrow().clone();
                                        match (cl_node.color, cr_node.color) {
                                            (Color::Black, Color::Black) => {
                                                //规则9 P黑 B黑 双C黑(递归P)
                                                self.set_color(&b, Color::Red);
                                                x = p;
                                                continue;
                                            },(Color::Red, Color::Red) => {
                                                //规则10 P黑 B黑 双C红
                                                if self.is_right_node(&x) {
                                                    self.right_rotate(&b);
                                                } else {
                                                    self.left_rotate(&b);
                                                }
                                                self.set_color(&cl, Color::Black);
                                            },(Color::Black, Color::Red) => {
                                                //规则11 P黑 B黑 CL黑 CR红
                                                if self.is_right_node(&x) {
                                                    self.left_rotate(&cr);
                                                    self.right_rotate(&cr);
                                                } else {
                                                    self.right_rotate(&cr);
                                                    self.left_rotate(&cr);
                                                }
                                                self.set_color(&cr, Color::Black);
                                            },(Color::Red, Color::Black) => {
                                                //规则12 P黑 B黑 CL红 CR黑
                                                if self.is_right_node(&x) {
                                                    self.right_rotate(&b);
                                                } else {
                                                    self.left_rotate(&b);
                                                }
                                                self.set_color(&cl, Color::Black);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break;
            }
        }
        pub fn del(&mut self, value:i32) {
            match self.get(value) {
                None => return,
                Some(x) => {
                    let mut x = x.clone();
                    if let Some(pv) = self.delete_search(&x) {
                        self.swap_value(&x, &pv);
                        x = pv.clone();
                    }
                    loop {
                        let x_node = x.as_ref().borrow().clone();
                        match x_node.color {
                            // 规则1 X红
                            Color::Red => {
                                self.drop_node(x);
                            },
                            Color::Black => {
                                // 规则2 X黑 无父无子
                                if x_node.parent.is_none() && x_node.left.is_none() && x_node.right.is_none() {
                                    self.drop_node(x);
                                } else {
                                    // 规则3 X黑有一个子节点
                                    if let Some(c) = x_node.left.clone() {
                                        self.swap_value(&x, &c);
                                        x = c;
                                        continue;
                                    } else if let Some(c) = x_node.right.clone() {
                                        self.swap_value(&x, &c);
                                        x = c;
                                        continue;
                                    } else {
                                        if let (Some(p), Some(mut b)) = (self.get_parent(&x), self.get_brother(&x)) {
                                            let p_node = p.as_ref().borrow().clone();
                                            let b_node = b.as_ref().borrow().clone();
                                            match p_node.color {
                                                Color::Red => {
                                                    if b_node.color != Color::Black {
                                                        panic!("节点不平衡");
                                                    }
                                                    //规则4 X黑 P红 B黑
                                                    let (c_a, c_b);
                                                    if self.is_left_node(&x) {
                                                        c_a = b_node.left.clone();
                                                        c_b = b_node.right.clone();
                                                    } else {
                                                        c_a = b_node.right.clone();
                                                        c_b = b_node.left.clone();
                                                    }
                                                    if let Some(c) = c_a {
                                                        let c_node = c.as_ref().borrow().clone();
                                                        if c_node.color != Color::Red {
                                                            panic!("节点不平衡");
                                                        }
                                                        self.swap_value(&x, &p);
                                                        self.swap_value(&p, &c);
                                                        x = c;
                                                        continue;
                                                    } if let Some(c) = c_b {
                                                        let c_node = c.as_ref().borrow().clone();
                                                        if c_node.color != Color::Red {
                                                            panic!("节点不平衡");
                                                        }
                                                        self.swap_value(&x, &p);
                                                        self.swap_value(&p, &b);
                                                        self.swap_value(&b, &c);
                                                        x = c;
                                                        continue;
                                                    } else {
                                                        self.set_color(&p, Color::Black);
                                                        self.set_color(&b, Color::Red);
                                                        self.drop_node(x);
                                                    }
                                                }, Color::Black => {
                                                    match b_node.color {
                                                        Color::Red => {
                                                            //规则5 X黑 P黑 B红
                                                            let c;
                                                            if self.is_left_node(&x) {
                                                                c = b_node.left.clone();
                                                            } else {
                                                                c = b_node.right.clone();
                                                            }
                                                            if let Some(c) = c {
                                                                let c_node = c.as_ref().borrow().clone();
                                                                if c_node.color != Color::Black {
                                                                    panic!("节点不平衡");
                                                                }
                                                                // C可能有红子节点
                                                                let c_node = c.as_ref().borrow().clone();
                                                                let (c_a, c_b);
                                                                if self.is_left_node(&x) {
                                                                    c_a = c_node.left.clone();
                                                                    c_b = c_node.right.clone();
                                                                } else {
                                                                    c_a = c_node.right.clone();
                                                                    c_b = c_node.left.clone();
                                                                }
                                                                if let Some(c_a) = c_a {
                                                                    self.swap_value(&x, &p);
                                                                    self.swap_value(&p, &c_a);
                                                                    x = c_a;
                                                                    // 转到规则1
                                                                    continue;
                                                                } else if let Some(c_b) = c_b {
                                                                    self.swap_value(&x, &p);
                                                                    self.swap_value(&p, &c);
                                                                    self.swap_value(&c, &c_b);
                                                                    x = c_b;
                                                                    // 转到规则1
                                                                    continue;
                                                                } else {
                                                                    self.swap_value(&x, &p);
                                                                    self.swap_value(&p, &c);
                                                                    x = c;
                                                                    // 转到规则4
                                                                    continue;
                                                                }
                                                            } else {
                                                                panic!("节点不平衡");
                                                            }
                                                        },
                                                        Color::Black => {
                                                            //规则6 X黑 P黑 B黑
                                                            let (c_a, c_b);
                                                            if self.is_left_node(&x) {
                                                                c_a = b_node.left.clone();
                                                                c_b = b_node.right.clone();
                                                            } else {
                                                                c_a = b_node.right.clone();
                                                                c_b = b_node.left.clone();
                                                            }
                                                            if let Some(c) = c_a {
                                                                let c_node = c.as_ref().borrow().clone();
                                                                if c_node.color != Color::Red {
                                                                    panic!("节点不平衡");
                                                                }
                                                                self.swap_value(&x, &p);
                                                                self.swap_value(&p, &c);
                                                                x = c;
                                                                continue;
                                                            } else if let Some(c) = c_b {
                                                                let c_node = c.as_ref().borrow().clone();
                                                                if c_node.color != Color::Red {
                                                                    panic!("节点不平衡");
                                                                }
                                                                self.swap_value(&x, &p);
                                                                self.swap_value(&p, &b);
                                                                self.swap_value(&b, &c);
                                                                x = c;
                                                                continue;
                                                            } else {
                                                                /* 无子节点 */
                                                                self.set_color(&p, Color::Black);
                                                                self.set_color(&b, Color::Red);
                                                                self.set_color(&x, Color::Red);
                                                                // P有父
                                                                if p_node.parent.is_some() {
                                                                    if self.is_left_node(&x) {
                                                                        if self.is_right_node(&p) {
                                                                            self.swap_value(&x, &b);
                                                                            (x, b) = (b, x);
                                                                            self.swap_value(&b, &p);
                                                                        }
                                                                    } else {
                                                                        if self.is_left_node(&p) {
                                                                            self.swap_value(&x, &b);
                                                                            (x, b) = (b, x);
                                                                            self.swap_value(&b, &p);
                                                                        }
                                                                    }
                                                                    self.drop_node(x);
                                                                    self.delete_fix(p);
                                                                }
                                                                // P无父
                                                                else {
                                                                    self.drop_node(x);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            panic!("节点不平衡");
                                        }
                                    }
                                }
                            }
                        }
                        break;
                    }
                    self.root_fix();
                }
            }
        }
        pub fn clear(&mut self) {
            while let Some(x) = self.root.clone() {
                let v = x.as_ref().borrow().clone().value;
                self.del(v);
            }
        }
        fn check_tree(&self) -> bool {
            let mut hight = None;
            let mut result = true;
            self._check_rb_hight(self.root.clone(), 0, &mut |x| {
                if hight.is_none() {
                    hight = Some(x);
                }
                if result {
                    result = hight == Some(x);
                }
            });
            result
        }
        fn _check_rb_hight(&self, x:Option<Rc<RefCell<Node>>>, i:i32, callback: &mut dyn FnMut(i32)) {
            let mut i = i;
            if x.is_some() {
                let temp = x.clone().unwrap();
                let x_node = temp.as_ref().borrow().clone();
                if x_node.color == Color::Black {
                    i+=1;
                }
                self._check_rb_hight(x_node.left.clone(), i, callback);
                self._check_rb_hight(x_node.right.clone(), i, callback);
        
            } else {
                callback(i);
            }
        }
    }
}