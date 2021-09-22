use std::sync::{Arc};
use std::os::raw::{c_void};
use std::marker::PhantomData;
const NULL:* mut c_void = 0 as *mut c_void;

#[derive(Debug)]
pub struct Raii<'p, T> {
    //Invariant: Should always point to a valid odbc Object
    handle: *mut T,
    // we use phantom data to tell the borrow checker that we need to keep the data source alive
    // for the lifetime of the handle
    parent: PhantomData<&'p ()>,
}

fn test(a : i32, b: i32)->i32{
    a+b
}

fn main() {

    let en = "a";
    let zh = "上海永大菌业有限公司";
    let utf =  String::from_utf8(zh.as_bytes().to_vec());

    let mut v: Vec<u16> = zh.encode_utf16().collect();   
    let u = String::from_utf16(&v).unwrap();
    let ptr = v.as_ptr() as * const u8;
    let slice = unsafe{ std::slice::from_raw_parts(ptr, v.len() * std::mem::size_of::<u16>()) };
    let utf16 = encoding_rs::UTF_16LE.encode(zh).0;
    let gb18030 = encoding_rs::GB18030.encode(zh).0;
    let gbk = encoding_rs::GBK.encode(zh).0;

    let cn = encoding_rs::GB18030.decode(&gb18030).0;

    println!("en:{} len:{} zh:{:?} len:{} code:{:?} gb18030:{:?} gbk:{:?} cn:{} utf16:{:?}", en, en.len(), zh.to_string().as_bytes(), zh.len(), slice, gb18030, gbk, cn, utf16);
        


    // &str    -> String--| String::from(s) or s.to_string() or s.to_owned()
    let str = "xxx";
    let str1 = str.to_string();
    let str21:String = str.to_owned();
    let str3 = String::from(str);

    // &str    -> &[u8]---| s.as_bytes()
    let bytes = str.as_bytes();

    // &str    -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
    let v = str.as_bytes().to_vec();

    // String  -> &str----| &s if possible* else s.as_str()
    let str2:&str = &str1;
    let str2:&str = str1.as_str();

    // String  -> &[u8]---| s.as_bytes()
    let bytes: &[u8] =  str1.as_bytes();

    // String  -> Vec<u8>-| s.into_bytes()
    let v: Vec<u8> =  str21.into_bytes();

    // &[u8]   -> &str----| s.to_vec() or s.to_owned()
    let str = bytes.to_vec().as_slice();

    // &[u8]   -> String--| std::str::from_utf8(s).unwrap(), but don't**
    let str1 : &str = std::str::from_utf8(bytes).unwrap();

    // &[u8]   -> Vec<u8>-| String::from_utf8(s).unwrap(), but don't**
    let str1 : String = String::from_utf8(bytes.to_vec()).unwrap();

    // Vec<u8> -> &str----| &s if possible* else s.as_slice()
    let str1 : &[u8] = &v;
    let str1 : &[u8] = v.as_slice();

    // Vec<u8> -> str--| std::str::from_utf8(&s).unwrap(), but don't**
    let str1 : &str = std::str::from_utf8(&v).unwrap();

    // Vec<u8> -> String---| String::from_utf8(s).unwrap(), but don't**
    let str1 : String = String::from_utf8(v).unwrap();


    println!("Hello, world!");  
    let c = test(1, 2);
    println!("c:{}", c);
    
    //*mut
    let mut c:*mut c_void=1 as * mut c_void;
    println!("c:{},{:?}",std::mem::size_of_val(&c),c);

    c = 2 as *mut c_void;
    println!("c:{},{:?}",std::mem::size_of_val(&c),c);

    c = NULL;
    println!("c:{},{:?}",std::mem::size_of_val(&c),c);

    //ok
    let mut d:c_void;
    //fail
    //d = 5 as c_void;

    //*const
    let mut e: *const c_void = 3 as *const c_void;
    println!("e:{},{:?}",std::mem::size_of_val(&e),e);

    e = 4 as *const c_void;
    println!("e:{},{:?}",std::mem::size_of_val(&e),e);

    e = NULL;
    println!("e:{},{:?}",std::mem::size_of_val(&e),e);

    let x = 5;
    let raw = &x as *const i32;
    // println!("x:{}={}", x, raw);

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;

    let x0 = 1;
    let y0 = 2;
    let point = Point{
        x : &x0,
        y : &y0,
    };
    unsafe{
        println!("point:{}", point);
    }

    let point1 = point;
    unsafe{
        println!("point:{}", point1);
    }

    unsafe{
        println!("point:{}", point);

    }
}

#[derive(Clone, Copy)]
struct Point{
    x : *const i32,
    y : *const i32,
}

impl std::fmt::Display for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe{
            write!(f, "({}, {})", *self.x, *self.y)
        }
    }
}


pub struct Environment{
    safe : EnvSafe
}


impl Environment{
    fn as_safe(&self)->&EnvSafe{
        &self.safe
    }
}

pub struct EnvSafe{
    pub handle : String    
}


pub struct RawConnection<'env> {
    safe: ConnSafe<'env>,
}

#[allow(dead_code)]
impl<'env> RawConnection<'env>{
    fn establish()->RawConnection<'env>{
        let env = Environment{
            safe:EnvSafe{
                handle:"000".to_string()
            }
        };
        let conn = RawConnection::new(env);
        conn
    }

    fn new(env : Environment)->RawConnection<'env>{                
        RawConnection{
           safe: ConnSafe{
               handle : 1,
               env : env,
               phantom : std::marker::PhantomData
           }
        }
    }

    fn prepare_query(&'env self, sql : *mut String) -> Statement{
        Statement{
            str1 : sql,
        }      
    }

    fn load(&self, sql : *mut String)
    {
        self.prepare_query(sql);       
    }
}

pub struct ConnSafe<'env>{
    handle : i32,
    env : Environment,
    phantom : std::marker::PhantomData<&'env String>
}


impl<'env> ConnSafe<'env>{
}

struct Statement{
    str1 : *mut String,   
}

#[derive(Debug)]
struct W<'env>{
    pub env : &'env i32,
}

type T<'env> = W<'env>;

#[derive(Debug)]
struct V<T>{
    t : T
}

fn test_w<'env>(t : T<'env>){

    // let t = T{env : &i};
    println!("t={:?}", t);    
}


fn test_v<'env>(v : V<T<'env>>){

    // let t = T{env : &i};
    println!("t={:?}", v);    
}


struct A{
    pub b : B,
}

impl A{
    fn test(mut self)
    {
        let c = self.b;
    }

    fn test1(){

        let b = B{
            b1:1 as * const i32,
            b2:2 as * const i32
        };

        let mut a = A{b};
        
        let a1 = &mut a;        
        a1.b = B{
            b1:1 as * const i32,
            b2:2 as * const i32
        };

        let a2 = &mut a;
        //A::test2(&mut a);
        //let c = a.b;        
    }

    fn test2(a: &mut A)
    {        
        a.b = B{
            b1:1 as * const i32,
            b2:2 as * const i32
        };
                
    }
}

struct B{
    pub b1 : *const i32,
    pub b2 : *const i32,
}