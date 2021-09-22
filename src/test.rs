#[macro_use]
extern crate lazy_static;

use std::any::{Any, TypeId, type_name};
fn is_string(s: &dyn Any) -> bool {
     TypeId::of::<String>() == s.type_id()
}
fn main()
{
    let unmoved = Unmovable::new("hello".to_string());
    println!("unmoved:{} data:{}", (&unmoved.slice as * const _) as i64, (&unmoved.data as * const _) as i64);

    // The pointer should point to the correct location,
    // so long as the struct hasn't moved.
    // Meanwhile, we are free to move the pointer around.
    let mut still_unmoved = unmoved;    
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
    println!("still unmoved:{} data:{}", (&still_unmoved.slice as * const _) as i64, (&still_unmoved.data as * const _) as i64);

    // Since our type doesn't implement Unpin, this will fail to compile:
    // let mut new_unmoved = Unmovable::new("world".to_string());
    // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);


    let mut value = "hello".to_string();
    let id = 1;
    let self_struct = SelfStruct::new(&mut value, &id);
    // self_struct.print();

    println!("test");    
    let str = "1".to_string();
    let mut b = A{
        a:&str,
        b:"111".to_owned(),
        ptr : NonNull::dangling()
    };

    {
        let mut a = A{
            a:&str,
            b:"222".to_owned(),
            ptr : NonNull::dangling()
        };
        a.ptr = NonNull::from(&a.b);

        println!("a ptr:{} a.a:{} a.b:{} ptr:{}", (&a as *const A<String>) as i64, (&a.a as *const &String) as i64, (&a.b as *const String) as i64, a.ptr.as_ptr() as i64);
        unsafe{println!("ptr value:{}", a.ptr.as_ref());}
        a.b = "333".to_owned();
        b = a;
    }
    {      
        b.ptr = NonNull::from(&b.b);
        b.b = "444".to_owned();
        println!("b ptr:{} b.a:{} b.b:{} ptr:{}", (&b as *const A<String>) as i64, (&b.a as *const &String) as i64, (&b.b as *const String) as i64, b.ptr.as_ptr() as i64);
        unsafe{
            println!("ptr value:{}", b.ptr.as_mut());
            let mut dref_ptr = &mut*(b.ptr.as_ptr() as * mut String);
            dref_ptr.clear();
            dref_ptr.insert_str(0, "5556666");
            println!("ptr value:{}", b.ptr.as_ref());
        }
    }

    let student = ComputerStudent{
        sql_type : OdbcSqlType::varchar,
    };

    let type1 = student.get_type();
    println!("type:{}", type1);

    let name = "rover".to_string();
    println!("name type id:{:?}", name.type_id());

    println!("string type id:{:?}", TypeId::of::<String>());

    println!("string type name:{}", type_name::<String>());

}


#[derive(Debug)]
struct A<'a, B>{
    pub a : &'a B,
    pub b : String,
    pub ptr : NonNull<String>,
}

trait S{

}
trait Student{
    type SqlType : S;
    fn name(&self)->String;
    fn Class(&self)->String;
    fn get_type(&self)->&Self::SqlType;
}


struct ComputerStudent<SqlType>{
    sql_type : SqlType
}

#[derive(Debug, Copy,Clone)]
enum OdbcSqlType{
    int,
    varchar,
    datetime,
    double,
    float,
}

impl  S for OdbcSqlType {
    
}


// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` 的 trait
impl std::fmt::Display for OdbcSqlType {   //implement 实现
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
       
        write!(f, "{}", *self as u8)
    }
}




impl<SqlType> Student for ComputerStudent<SqlType>
where SqlType : S
{
    type SqlType = SqlType;

    fn name(&self)->String{
        "xxxx".to_string()
    }

    fn Class(&self)->String{
        "Computer".to_string()
    }

    fn get_type(&self)->&Self::SqlType{
        &self.sql_type
    }
}

struct Test{

}

impl Test{

    fn test<F>(&mut self, fun : F)
    where F: Fn(&mut Test)
    {
        fun(self)
    }

    fn test_1(&mut self){
        self.test(|a|{
             a.test_2();
        });
    }

    fn test_2(&self){

    }
}


use std::pin::Pin;
use std::marker::{PhantomData, PhantomPinned};
use std::ptr::NonNull;

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}


impl Unmovable {
    // To ensure the data doesn't move when the function returns,
    // we place it in the heap where it will stay for the lifetime of the object,
    // and the only way to access it would be through a pointer to it.
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}



struct SelfStruct<'a>{
    data : &'a String,
    data_ref : Option<&'a String>,
    inner : Inner<'a>,
    inner_field_ref : Option<&'a i32>
}

struct Inner<'a>{
    pub field : &'a i32,
}

impl<'a> SelfStruct<'a>{
    fn new(data : &'a mut String, id: &'a i32)->Self{
        let mut a = SelfStruct{
            data : data,
            data_ref : None,
            inner : Inner{field : id},
            inner_field_ref : None,

        };
        a.data_ref = Some(&a.data);
        a.inner_field_ref = Some(&a.inner.field);

        let b = a.data_ref.unwrap();
        println!("a.data_ref:{}", b);

        a
    }

    fn print(&self){
        println!("value:{} ref value:{}", self.data, self.data_ref.unwrap());
    }
}

struct Node<'a, T> {
    data: &'static str,
    next: Option<Box<Node<'a, T>>>,
    unuse: PhantomData<&'a T>
 }
 struct ForwardList<'a, T> {
    head: Option<Box<Node<'a, T>>>,
 }


impl<'a, T> ForwardList<'a, T>{
    fn new() ->Self{
        let mut list : ForwardList<T> = ForwardList{            
            head : None,
        };
        list
    }

    fn append(&mut self, new_data : &'static str){        
        
        let mut a = Node {
            data : new_data,
            next : None,
            unuse : PhantomData,
        };

        let mut tmp = &mut self.head;
        while let Some(t ) = tmp.as_deref_mut() {            
            if let None = t.next{
                t.next = Some(Box::new(a));        
                break;
            }
            tmp = &mut t.next;
        } 
    }

    fn get_ref(&mut self, data : &'static str)->Option<&mut Node<'a, T>>{
        let mut tmp = &mut self.head;
        while let Some(t ) = tmp.as_deref_mut() {            
            if data == t.data{
                return Some(t);
            }
            tmp = &mut t.next;
        }
        None
    }
}

fn test_forward_list(){
    let mut list : ForwardList<&'static str> = ForwardList::new();
    list.append("aa");
    list.append("bb");
    list.append("cc");

}


struct DoubleLinkList{
    pub head : Option<NonNull<DoubleLinkNode>>,
    pub tail : Option<NonNull<DoubleLinkNode>>,
    _pin : PhantomPinned,
    list : Vec<DoubleLinkNode>,
}

struct DoubleLinkNode{
    pub id : i32,
    pub data : String,
    pub prev : Option<NonNull<DoubleLinkNode>>,
    pub next : Option<NonNull<DoubleLinkNode>>,
    _pin : PhantomPinned,
}

impl DoubleLinkList{
    fn new()->DoubleLinkList{       
        let link = DoubleLinkList{
            head : None,
            tail: None,
            _pin : PhantomPinned,
            list : Vec::new(),
        };
        link
    }

    fn append(&mut self, mut node : DoubleLinkNode){        
        let mut max_id = 0;
        for item in self.list.iter(){
            if item.id > max_id{
                max_id = item.id;
            }
        }
        node.id = max_id + 1;

        let index = self.list.len();
        self.list.insert(index, node);
        if self.head == None && self.tail == None{
            self.head = Some(NonNull::from(self.list.get(index).unwrap()));
            self.tail = Some(NonNull::from(self.list.get(index).unwrap()));
            self.head.as_mut().map(|item|{
                unsafe{
                    item.as_mut().prev = None;
                    item.as_mut().next = None;
                }
            });
        }
        else{
            self.tail = Some(NonNull::from(self.list.get(index).unwrap()));
        }
    }
}

